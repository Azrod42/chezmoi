use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::Serialize;
use sqlx::Row;
use std::sync::Arc;

use crate::State;

use super::{
    auth::{verify_password, AuthRequest},
    body_bytes, err, json,
};

pub(crate) const PATH: &str = "/login";

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    token_type: &'static str,
    expires_in: u64,
}

pub(crate) async fn handle(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let payload: AuthRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;

    let row = sqlx::query("SELECT id::text as id, password_hash FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(state.pool())
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

    let token = shared::issue_jwt(&user_id, &payload.email)
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
