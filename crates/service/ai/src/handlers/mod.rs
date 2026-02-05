pub(crate) mod ai;
pub(crate) mod health;

use lambda_http::{
    http::{Method, StatusCode},
    Body, Error, Request, Response,
};
use std::sync::Arc;

use crate::State;

pub(crate) async fn router(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    if req.method() == Method::OPTIONS {
        return Ok(cors_headers(Response::builder())
            .status(StatusCode::NO_CONTENT)
            .body(Body::Empty)
            .unwrap());
    }

    if req.uri().path() == health::PATH {
        return health::handle(req, state).await;
    }

    let req = match auth_middleware::with_user(req, state.pool()).await {
        Ok(req) => req,
        Err(auth_err) => return Ok(err(auth_err.status, auth_err.message)),
    };

    match (req.method(), req.uri().path()) {
        (&Method::POST, ai::PATH) => ai::handle(req, state).await,
        _ => Ok(err(StatusCode::NOT_FOUND, "not found")),
    }
}

pub(crate) use http_utils::body_bytes;
pub(crate) use http_utils::cors_headers;
pub(crate) use http_utils::err;
pub(crate) use http_utils::json;
