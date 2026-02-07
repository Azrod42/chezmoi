use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::State;

use auth_middleware::AuthenticatedUser;

use super::{body_bytes, json};

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
    let user = req
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or_else(|| lambda_http::Error::from("missing auth context"))?
        .clone();

    let api_key = user
        .api_key
        .clone()
        .ok_or_else(|| lambda_http::Error::from("missing openrouter api key"))?;

    let payload: AiRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;
    let AiRequest { prompt, model } = payload;

    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base("https://openrouter.ai/api/v1");
    let client = Client::with_config(config);

    let message = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()
        .map_err(|_| lambda_http::Error::from("invalid chat request"))?;
    let request = CreateChatCompletionRequestArgs::default()
        .model(model.clone())
        .messages([message.into()])
        .build()
        .map_err(|_| lambda_http::Error::from("invalid chat request"))?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|_| lambda_http::Error::from("openrouter error"))?;

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
