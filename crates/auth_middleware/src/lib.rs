use lambda_http::{http::StatusCode, Request};
use once_cell::sync::Lazy;
use sqlx::{PgPool, Row};
use std::{collections::HashMap, sync::Mutex, time::Duration};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AuthError {
    pub status: StatusCode,
    pub message: &'static str,
}

const RATE_LIMIT: u32 = 15;
const RATE_WINDOW: Duration = Duration::from_secs(60);

#[derive(Clone, Debug)]
struct RateCounter {
    count: u32,
    window_start: std::time::Instant,
}

static RATE_LIMITER: Lazy<Mutex<HashMap<Uuid, RateCounter>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn extract_bearer(req: &Request) -> Option<String> {
    let h = req.headers().get("authorization")?.to_str().ok()?;
    let prefix = "Bearer ";
    if h.starts_with(prefix) {
        Some(h[prefix.len()..].trim().to_string())
    } else {
        None
    }
}

pub async fn with_user(mut req: Request, pool: &PgPool) -> Result<Request, AuthError> {
    let token = extract_bearer(&req).ok_or(AuthError {
        status: StatusCode::UNAUTHORIZED,
        message: "missing bearer",
    })?;
    let claims = shared::verify_jwt(&token).map_err(|_| AuthError {
        status: StatusCode::UNAUTHORIZED,
        message: "invalid token",
    })?;

    let user_id = claims.sub.parse::<Uuid>().map_err(|_| AuthError {
        status: StatusCode::UNAUTHORIZED,
        message: "invalid token",
    })?;
    enforce_rate_limit(user_id)?;

    let row = sqlx::query("SELECT api_key FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AuthError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "db error",
        })?;

    let Some(row) = row else {
        return Err(AuthError {
            status: StatusCode::UNAUTHORIZED,
            message: "user not found",
        });
    };

    let api_key: Option<String> = row.try_get("api_key").map_err(|_| AuthError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "db error",
    })?;

    req.extensions_mut().insert(AuthenticatedUser {
        id: user_id,
        email: claims.email,
        api_key,
    });

    Ok(req)
}

fn enforce_rate_limit(user_id: Uuid) -> Result<(), AuthError> {
    let now = std::time::Instant::now();
    let mut limiter = RATE_LIMITER.lock().map_err(|_| AuthError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "rate limiter error",
    })?;
    let counter = limiter
        .entry(user_id)
        .or_insert(RateCounter {
            count: 0,
            window_start: now,
        });

    if now.duration_since(counter.window_start) >= RATE_WINDOW {
        counter.count = 0;
        counter.window_start = now;
    }

    if counter.count >= RATE_LIMIT {
        return Err(AuthError {
            status: StatusCode::TOO_MANY_REQUESTS,
            message: "rate limit exceeded",
        });
    }

    counter.count += 1;
    Ok(())
}
