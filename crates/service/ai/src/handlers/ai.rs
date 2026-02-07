use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::State;

use auth_middleware::AuthenticatedUser;

use super::{body_bytes, err, json};

pub(crate) const PATH: &str = "/ai";

#[derive(Debug, Deserialize)]
struct AiRequest {
    prompt: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct AiResponse {
    answer: String,
}

pub(crate) async fn handle(req: Request, _state: Arc<State>) -> Result<Response<Body>, Error> {
    let user = match req.extensions().get::<AuthenticatedUser>() {
        Some(user) => user.clone(),
        None => return Ok(err(StatusCode::UNAUTHORIZED, "missing auth context")),
    };

    let api_key = match user.api_key.clone() {
        Some(key) => key,
        None => return Ok(err(StatusCode::BAD_REQUEST, "missing openrouter api key")),
    };

    let payload: AiRequest = match serde_json::from_slice(&body_bytes(req.body())) {
        Ok(payload) => payload,
        Err(_) => return Ok(err(StatusCode::BAD_REQUEST, "invalid json")),
    };
    let AiRequest { prompt, model } = payload;

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base("https://openrouter.ai/api/v1");
    let client = Client::with_config(config);

    let message = match ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()
    {
        Ok(message) => message,
        Err(_) => return Ok(err(StatusCode::BAD_REQUEST, "invalid chat request")),
    };

    let request = match CreateChatCompletionRequestArgs::default()
        .model(model.clone())
        .messages([message.into()])
        .build()
    {
        Ok(request) => request,
        Err(_) => return Ok(err(StatusCode::BAD_REQUEST, "invalid chat request")),
    };

    let response = match client.chat().create(request).await {
        Ok(response) => response,
        Err(openai_err) => return Ok(openai_error_response(openai_err)),
    };

    let answer = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.clone())
        .unwrap_or_default();

    Ok(json(
        StatusCode::OK,
        serde_json::to_value(AiResponse { answer }).unwrap(),
    ))
}

fn openai_error_response(openai_err: OpenAIError) -> Response<Body> {
    match openai_err {
        OpenAIError::ApiError(api_err) => {
            let mut details = api_err.message;
            if let Some(code) = api_err.code {
                details = format!("{details} (code: {code})");
            }
            if let Some(type_value) = api_err.r#type {
                details = format!("{details} (type: {type_value})");
            }
            err(StatusCode::BAD_REQUEST, &details)
        }
        OpenAIError::InvalidArgument(msg) => err(StatusCode::BAD_REQUEST, &msg),
        OpenAIError::Reqwest(req_err) => {
            let status = req_err
                .status()
                .and_then(|s| StatusCode::from_u16(s.as_u16()).ok())
                .unwrap_or(StatusCode::BAD_GATEWAY);
            err(status, &req_err.to_string())
        }
        other => err(StatusCode::BAD_GATEWAY, &other.to_string()),
    }
}
