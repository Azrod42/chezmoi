use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub iss: String,
    pub exp: usize,
}

pub fn issuer() -> String {
    std::env::var("JWT_ISSUER").unwrap_or_else(|_| "poc".to_string())
}

pub fn ttl_seconds() -> u64 {
    std::env::var("JWT_TTL_SECONDS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(3600)
}

pub fn jwt_secret() -> Result<String> {
    std::env::var("JWT_SECRET").map_err(|_| anyhow!("JWT_SECRET missing"))
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let secret = jwt_secret()?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[issuer()]); // verify iss + exp

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}
