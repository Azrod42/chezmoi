use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::State;

use super::{
    auth::{hash_password, AuthRequest},
    body_bytes, err, json,
};

pub(crate) const PATH: &str = "/register";

#[derive(Debug, Serialize)]
struct RegisterResponse {
    id: String,
    email: String,
}

pub(crate) async fn handle(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
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
        .execute(state.pool())
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
