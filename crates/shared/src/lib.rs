use anyhow::Result;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

pub use jwt_validation::ttl_seconds;
use jwt_validation::{issuer, jwt_secret, Claims};

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

pub fn issue_jwt(user_id: &str, email: &str) -> Result<String> {
    let secret = jwt_secret()?;
    let iss = issuer();
    let exp = (now_epoch_seconds() + ttl_seconds()) as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iss,
        exp,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}
