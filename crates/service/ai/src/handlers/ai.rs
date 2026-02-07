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

pub(crate) const PATH: &str = "/ai/writer";

const TRANSLATE_PROMPT: &str = " You are a translation assistant. Your task is to accurately translate the given text into the specified target language. Follow these steps:

1. You will be provided with a source text to translate at the end of this prompt.
   
2. The target language for translation is:

{language}

3. Translate the source text into the target language. Ensure that you maintain the original meaning, tone, and style as closely as possible while adhering to the grammatical and idiomatic norms of the target language.

4. Output only the translated text, without any additional comments or explanations.

Remember:
- Do not provide any explanations or notes about the translation process.
- Do not include the original text in your output.
- Ensure that your translation is accurate and natural-sounding in the target language.

The text to translate:
{text}";

const CORRECT_PROMPT: &str = "You are tasked with correcting a sentence in a specified language and returning only the corrected part. Follow these steps carefully:

1. the sentence to be corrected is at the end of this prompt after –:

2. The language for correction is:
{language}

3. Analyze the sentence for grammatical errors, spelling mistakes, or improper word usage in the specified language.

4. Make the necessary corrections to the sentence. Pay attention to:
   - Proper grammar
   - Correct spelling
   - Appropriate word choice
   - Punctuation
   - Capitalization

Do not include any explanations or additional comments.

Remember, your task is to correct the sentence and return the entire sentence.

The text to coorect:
{text}";

#[derive(Debug, Deserialize)]
struct AiRequest {
    prompt: String,
    language: String,
    function: Function,
    model: String,
}

#[derive(Debug, Deserialize)]
pub enum Function {
    TRANSLATE,
    CORRECT,
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
        .or_else(|| std::env::var("OPENROUTER_API_KEY").ok())
        .ok_or_else(|| lambda_http::Error::from("missing openrouter api key"))?;

    let payload: AiRequest = serde_json::from_slice(&body_bytes(req.body()))
        .map_err(|_| lambda_http::Error::from("invalid json"))?;
    let AiRequest {
        prompt: input,
        language,
        function,
        model,
    } = payload;
    let prompt = match function {
        Function::TRANSLATE => TRANSLATE_PROMPT
            .replace("{language}", &language)
            .replace("{text}", &input),
        Function::CORRECT => CORRECT_PROMPT.replace("{text}", &input),
    };

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
