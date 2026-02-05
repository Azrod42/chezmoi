use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::State;

use auth_middleware::AuthenticatedUser;

use super::{body_bytes, json};

pub(crate) const PATH: &str = "/ai";

#[derive(Debug, Deserialize)]
struct AiRequest {
    prompt: String,
}

pub(crate) async fn handle(req: Request, _state: Arc<State>) -> Result<Response<Body>, Error> {
    let user = req
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or_else(|| lambda_http::Error::from("missing auth context"))?
        .clone();

    let payload: AiRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;

    Ok(json(
        StatusCode::OK,
        serde_json::json!({
          "id": format!("req-{}", Uuid::new_v4()),
          "model": "mock-1",
          "user_id": user.id.to_string(),
          "email": user.email,
          "input_prompt_len": payload.prompt.len(),
          "answer": "Ceci est une réponse mockée (PoC)."
        }),
    ))
}
