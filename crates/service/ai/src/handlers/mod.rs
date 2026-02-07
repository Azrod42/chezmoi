pub(crate) mod ai;
pub(crate) mod writer;

use lambda_http::{
    http::{Method, StatusCode},
    Body, Error, Request, Response,
};
use lambda_http::RequestExt;
use std::sync::Arc;
use std::time::Instant;

use crate::State;

pub(crate) async fn router(req: Request, state: Arc<State>) -> Result<Response<Body>, Error> {
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let lambda_ctx = req.lambda_context_ref().cloned();
    let mut user_id: Option<String> = None;

    let result = if method == Method::OPTIONS {
        Ok(cors_headers(Response::builder())
            .status(StatusCode::NO_CONTENT)
            .body(Body::Empty)
            .unwrap())
    } else {
        match auth_middleware::with_user(req, state.pool()).await {
            Ok(req) => {
                user_id = req
                    .extensions()
                    .get::<auth_middleware::AuthenticatedUser>()
                    .map(|user| user.id.to_string());

                match (req.method(), req.uri().path()) {
                    (&Method::POST, writer::PATH) => writer::handle(req, state).await,
                    (&Method::POST, ai::PATH) => ai::handle(req, state).await,
                    _ => Ok(err(StatusCode::NOT_FOUND, "not found")),
                }
            }
            Err(auth_err) => Ok(err(auth_err.status, auth_err.message)),
        }
    };

    let duration_ms = start.elapsed().as_millis();
    let request_id = lambda_ctx
        .as_ref()
        .map(|ctx| ctx.request_id.as_str())
        .unwrap_or("unknown");
    let xray_trace_id = lambda_ctx
        .as_ref()
        .and_then(|ctx| ctx.xray_trace_id.as_deref());

    http_utils::log_http_result(
        "ai",
        &method,
        &path,
        request_id,
        xray_trace_id,
        user_id.as_deref(),
        duration_ms,
        &result,
    );

    result
}

pub(crate) use http_utils::body_bytes;
pub(crate) use http_utils::cors_headers;
pub(crate) use http_utils::err;
pub(crate) use http_utils::json;
