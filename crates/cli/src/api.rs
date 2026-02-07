use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://api.cealum.dev";

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
struct AiRequest {
    prompt: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct WriterRequest {
    prompt: String,
    language: String,
    function: Function,
    model: String,
}

#[derive(Debug, Serialize)]
struct ChangePasswordRequest {
    current_password: String,
    new_password: String,
}

#[derive(Debug, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Function {
    TRANSLATE,
    CORRECT,
}

#[derive(Debug, Deserialize)]
pub struct AiResponse {
    pub answer: String,
}

pub async fn login_request(email: &str, password: &str) -> Result<LoginResponse> {
    let client = reqwest::Client::new();
    let url = format!("{}/login", api_base_url());
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await
        .context("login request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("login failed ({status}): {body}");
    }

    let payload: LoginResponse = response.json().await.context("invalid login response")?;
    Ok(payload)
}

pub async fn register_request(email: &str, password: &str) -> Result<RegisterResponse> {
    let client = reqwest::Client::new();
    let url = format!("{}/register", api_base_url());
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await
        .context("register request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("register failed ({status}): {body}");
    }

    let payload: RegisterResponse = response.json().await.context("invalid register response")?;
    Ok(payload)
}

pub async fn change_password_request(
    current_password: &str,
    new_password: &str,
    token: &str,
) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("{}/change-password", api_base_url());
    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&ChangePasswordRequest {
            current_password: current_password.to_string(),
            new_password: new_password.to_string(),
        })
        .send()
        .await
        .context("change password request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("change password failed ({status}): {body}");
    }

    Ok(())
}

pub async fn ai_writer(
    function: Function,
    language: &str,
    prompt: &str,
    model: &str,
    token: &str,
) -> Result<AiResponse> {
    let url = format!("{}/ai/writer", api_base_url());
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&WriterRequest {
            prompt: prompt.to_string(),
            language: language.to_string(),
            function,
            model: model.to_string(),
        })
        .send()
        .await
        .context("ai request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("ai request failed ({status}): {body}");
    }

    let payload: AiResponse = response.json().await.context("invalid ai response")?;
    Ok(payload)
}

pub async fn ai(prompt: &str, model: &str, token: &str) -> Result<AiResponse> {
    let url = format!("{}/ai", api_base_url());
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&AiRequest {
            prompt: prompt.to_string(),
            model: model.to_string(),
        })
        .send()
        .await
        .context("ai request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("ai request failed ({status}): {body}");
    }

    let payload: AiResponse = response.json().await.context("invalid ai response")?;
    Ok(payload)
}

fn api_base_url() -> String {
    std::env::var("CEALUM_API_BASE").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}
