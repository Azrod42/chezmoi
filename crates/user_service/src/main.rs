use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use lambda_http::{
    http::{self, Method, StatusCode},
    run, service_fn, Body, Error, Request, Response,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use shared::issue_jwt;

#[derive(Clone)]
struct State {
    pool: PgPool,
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    id: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    token_type: &'static str,
    expires_in: u64,
}

fn cors_headers(builder: http::response::Builder) -> http::response::Builder {
    builder
        .header("content-type", "application/json")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "GET,POST,OPTIONS")
        .header("access-control-allow-headers", "content-type,authorization")
}

fn json(status: StatusCode, value: serde_json::Value) -> Response<Body> {
    let body = Body::Text(value.to_string());
    cors_headers(Response::builder())
        .status(status)
        .body(body)
        .unwrap()
}

fn err(status: StatusCode, msg: &str) -> Response<Body> {
    json(status, serde_json::json!({ "error": msg }))
}

fn body_bytes(body: &Body) -> Vec<u8> {
    match body {
        Body::Text(s) => s.as_bytes().to_vec(),
        Body::Binary(b) => b.clone(),
        Body::Empty => vec![],
        _ => vec![],
    }
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| anyhow::anyhow!(err.to_string()))?
        .to_string();
    Ok(hash)
}

fn verify_password(hash: &str, password: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|err| anyhow::anyhow!(err.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

async fn register(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let payload: AuthRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;

    if !payload.email.contains('@') || payload.password.len() < 8 {
        return Ok(err(
            StatusCode::BAD_REQUEST,
            "email invalid or password too short (min 8)",
        ));
    }

    let user_id = Uuid::new_v4();
    let pw_hash =
        hash_password(&payload.password).map_err(|_| lambda_http::Error::from("hash error"))?;

    let res = sqlx::query("INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)")
        .bind(&user_id)
        .bind(&payload.email)
        .bind(&pw_hash)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => Ok(json(
            StatusCode::CREATED,
            serde_json::to_value(RegisterResponse {
                id: user_id.to_string(),
                email: payload.email,
            })
            .unwrap(),
        )),
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            Ok(err(StatusCode::CONFLICT, "email already exists"))
        }
        Err(_) => Ok(err(StatusCode::INTERNAL_SERVER_ERROR, "db error")),
    }
}

async fn login(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let payload: AuthRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;

    let row = sqlx::query("SELECT id::text as id, password_hash FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| lambda_http::Error::from("db error"))?;

    let Some(row) = row else {
        return Ok(err(StatusCode::UNAUTHORIZED, "invalid credentials"));
    };

    let user_id: String = row
        .try_get("id")
        .map_err(|_| lambda_http::Error::from("db error"))?;
    let password_hash: String = row
        .try_get("password_hash")
        .map_err(|_| lambda_http::Error::from("db error"))?;

    let ok = verify_password(&password_hash, &payload.password)
        .map_err(|_| lambda_http::Error::from("verify error"))?;

    if !ok {
        return Ok(err(StatusCode::UNAUTHORIZED, "invalid credentials"));
    }

    let token = issue_jwt(&user_id, &payload.email)
        .map_err(|_| lambda_http::Error::from("jwt error"))?;

    let ttl = shared::ttl_seconds();
    Ok(json(
        StatusCode::OK,
        serde_json::to_value(LoginResponse {
            token,
            token_type: "Bearer",
            expires_in: ttl,
        })
        .unwrap(),
    ))
}

async fn handler(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    if req.method() == Method::OPTIONS {
        return Ok(cors_headers(Response::builder())
            .status(StatusCode::NO_CONTENT)
            .body(Body::Empty)
            .unwrap());
    }

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => Ok(json(StatusCode::OK, serde_json::json!({"ok": true}))),
        (&Method::POST, "/register") => register(req, state).await,
        (&Method::POST, "/login") => login(req, state).await,
        _ => Ok(err(StatusCode::NOT_FOUND, "not found")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgres");

    ensure_schema(&pool)
        .await
        .expect("failed to ensure schema");

    info!("user_service started");

    let state = Arc::new(State { pool });

    run(service_fn(move |req| {
        let state = state.clone();
        async move { handler(req, state).await }
    }))
    .await
}

async fn ensure_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id uuid PRIMARY KEY,
            email text NOT NULL UNIQUE,
            password_hash text NOT NULL,
            created_at timestamptz NOT NULL DEFAULT now()
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
