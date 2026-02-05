use lambda_http::{http::StatusCode, Request};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct AuthError {
    pub status: StatusCode,
    pub message: &'static str,
}

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
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|_| AuthError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "db error",
        })?;

    if !exists {
        return Err(AuthError {
            status: StatusCode::UNAUTHORIZED,
            message: "user not found",
        });
    }

    req.extensions_mut().insert(AuthenticatedUser {
        id: user_id,
        email: claims.email,
    });

    Ok(req)
}
