pub(crate) mod auth;
pub(crate) mod health;
pub(crate) mod login;
pub(crate) mod register;

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

    match (req.method(), req.uri().path()) {
        (&Method::GET, health::PATH) => health::handle(req, state).await,
        (&Method::POST, register::PATH) => register::handle(req, state).await,
        (&Method::POST, login::PATH) => login::handle(req, state).await,
        _ => Ok(err(StatusCode::NOT_FOUND, "not found")),
    }
}

pub(crate) use http_utils::body_bytes;
pub(crate) use http_utils::cors_headers;
pub(crate) use http_utils::err;
pub(crate) use http_utils::json;
