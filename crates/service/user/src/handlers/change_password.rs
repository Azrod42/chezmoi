use auth_middleware::AuthenticatedUser;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

use crate::State;

use super::{
    auth::{hash_password, verify_password},
    body_bytes, err, json,
};

pub(crate) const PATH: &str = "/change-password";

#[derive(Debug, Deserialize)]
struct ChangePasswordRequest {
    current_password: String,
    new_password: String,
}

#[derive(Debug, Serialize)]
struct ChangePasswordResponse {
    status: &'static str,
}

pub(crate) async fn handle(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let user = match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => user.clone(),
        None => return Ok(err(StatusCode::UNAUTHORIZED, "missing auth context")),
    };

    let payload: ChangePasswordRequest = match serde_json::from_slice(&body_bytes(req.body())) {
        Ok(payload) => payload,
        Err(_) => return Ok(err(StatusCode::BAD_REQUEST, "invalid json")),
    };

    if payload.new_password.len() < 8 {
        return Ok(err(StatusCode::BAD_REQUEST, "password too short (min 8)"));
    }

    let row = sqlx::query("SELECT password_hash FROM users WHERE id = $1")
        .bind(user.id)
        .fetch_optional(state.pool())
        .await
        .map_err(|_| lambda_http::Error::from("db error"))?;

    let Some(row) = row else {
        return Ok(err(StatusCode::UNAUTHORIZED, "user not found"));
    };

    let password_hash: String = row
        .try_get("password_hash")
        .map_err(|_| lambda_http::Error::from("db error"))?;

    let ok = verify_password(&password_hash, &payload.current_password)
        .map_err(|_| lambda_http::Error::from("verify error"))?;

    if !ok {
        return Ok(err(StatusCode::UNAUTHORIZED, "invalid credentials"));
    }

    let new_hash =
        hash_password(&payload.new_password).map_err(|_| lambda_http::Error::from("hash error"))?;

    sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(new_hash)
        .bind(user.id)
        .execute(state.pool())
        .await
        .map_err(|_| lambda_http::Error::from("db error"))?;

    Ok(json(
        StatusCode::OK,
        serde_json::to_value(ChangePasswordResponse { status: "ok" }).unwrap(),
    ))
}
