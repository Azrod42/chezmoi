use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use std::sync::Arc;

use crate::State;

use super::json;

pub(crate) const PATH: &str = "/health";

pub(crate) async fn handle(
    _req: Request,
    _state: Arc<State>,
) -> Result<Response<Body>, Error> {
    Ok(json(StatusCode::OK, serde_json::json!({"ok": true})))
}
