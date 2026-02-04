use lambda_http::{
    http::{self, Method, StatusCode},
    run, service_fn, Body, Error, Request, Response,
};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use jwt_validation::verify_jwt;

#[derive(Clone)]
struct State {
    pool: PgPool,
}

#[derive(Debug, Deserialize)]
struct AiRequest {
    prompt: String,
}

fn cors_headers(builder: http::response::Builder) -> http::response::Builder {
    builder
        .header("content-type", "application/json")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "GET,POST,OPTIONS")
        .header("access-control-allow-headers", "content-type,authorization")
}

fn json(status: StatusCode, value: serde_json::Value) -> Response<Body> {
    cors_headers(Response::builder())
        .status(status)
        .body(Body::Text(value.to_string()))
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

fn extract_bearer(req: &Request) -> Option<String> {
    let h = req.headers().get("authorization")?.to_str().ok()?;
    let prefix = "Bearer ";
    if h.starts_with(prefix) {
        Some(h[prefix.len()..].trim().to_string())
    } else {
        None
    }
}

async fn ai(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let token = extract_bearer(&req).ok_or_else(|| lambda_http::Error::from("missing bearer"))?;
    let claims = verify_jwt(&token).map_err(|_| lambda_http::Error::from("invalid token"))?;

    // Check en DB que l'user existe (histoire de “relier” ce service à Postgres aussi)
    let user_id = claims
        .sub
        .parse::<Uuid>()
        .map_err(|_| lambda_http::Error::from("invalid token"))?;
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| lambda_http::Error::from("db error"))?;

    if !exists {
        return Ok(err(StatusCode::UNAUTHORIZED, "user not found"));
    }

    let payload: AiRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;

    Ok(json(
        StatusCode::OK,
        serde_json::json!({
          "id": format!("req-{}", Uuid::new_v4()),
          "model": "mock-1",
          "user_id": claims.sub,
          "email": claims.email,
          "input_prompt_len": payload.prompt.len(),
          "answer": "Ceci est une réponse mockée (PoC)."
        }),
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
        (&Method::POST, "/ai") => ai(req, state).await,
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

    info!("ai_service started");

    let state = Arc::new(State { pool });

    run(service_fn(move |req| {
        let state = state.clone();
        async move { handler(req, state).await }
    }))
    .await
}
