use lambda_http::{
    http::{self, Method, StatusCode},
    Body, Error, Response,
};
use tracing::{error, info, warn};

pub fn cors_headers(builder: http::response::Builder) -> http::response::Builder {
    builder
        .header("content-type", "application/json")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "GET,POST,OPTIONS")
        .header("access-control-allow-headers", "content-type,authorization")
}

pub fn json(status: StatusCode, value: serde_json::Value) -> Response<Body> {
    let body = Body::Text(value.to_string());
    cors_headers(Response::builder())
        .status(status)
        .body(body)
        .unwrap()
}

pub fn err(status: StatusCode, msg: &str) -> Response<Body> {
    json(status, serde_json::json!({ "error": msg }))
}

pub fn body_bytes(body: &Body) -> Vec<u8> {
    match body {
        Body::Text(s) => s.as_bytes().to_vec(),
        Body::Binary(b) => b.clone(),
        Body::Empty => vec![],
        _ => vec![],
    }
}

pub fn log_http_result(
    service: &str,
    method: &Method,
    path: &str,
    request_id: &str,
    xray_trace_id: Option<&str>,
    user_id: Option<&str>,
    duration_ms: u128,
    result: &Result<Response<Body>, Error>,
) {
    match result {
        Ok(response) => {
            let status = response.status().as_u16();
            match status {
                500..=599 => log_response_error(
                    service,
                    method,
                    path,
                    request_id,
                    xray_trace_id,
                    user_id,
                    status,
                    duration_ms,
                ),
                400..=499 => log_response_warn(
                    service,
                    method,
                    path,
                    request_id,
                    xray_trace_id,
                    user_id,
                    status,
                    duration_ms,
                ),
                _ => log_response_info(
                    service,
                    method,
                    path,
                    request_id,
                    xray_trace_id,
                    user_id,
                    status,
                    duration_ms,
                ),
            }
        }
        Err(err) => {
            error!(
                service = service,
                request_id = request_id,
                method = %method,
                path = %path,
                duration_ms = duration_ms,
                error = %err,
                "request failed"
            );
        }
    }
}

fn log_response_info(
    service: &str,
    method: &Method,
    path: &str,
    request_id: &str,
    xray_trace_id: Option<&str>,
    user_id: Option<&str>,
    status: u16,
    duration_ms: u128,
) {
    match (user_id, xray_trace_id) {
        (Some(user_id), Some(xray_trace_id)) => {
            info!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed"
            );
        }
        (Some(user_id), None) => {
            info!(
                service = service,
                request_id = request_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed"
            );
        }
        (None, Some(xray_trace_id)) => {
            info!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed"
            );
        }
        (None, None) => {
            info!(
                service = service,
                request_id = request_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed"
            );
        }
    }
}

fn log_response_warn(
    service: &str,
    method: &Method,
    path: &str,
    request_id: &str,
    xray_trace_id: Option<&str>,
    user_id: Option<&str>,
    status: u16,
    duration_ms: u128,
) {
    match (user_id, xray_trace_id) {
        (Some(user_id), Some(xray_trace_id)) => {
            warn!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed with warning"
            );
        }
        (Some(user_id), None) => {
            warn!(
                service = service,
                request_id = request_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed with warning"
            );
        }
        (None, Some(xray_trace_id)) => {
            warn!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed with warning"
            );
        }
        (None, None) => {
            warn!(
                service = service,
                request_id = request_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request completed with warning"
            );
        }
    }
}

fn log_response_error(
    service: &str,
    method: &Method,
    path: &str,
    request_id: &str,
    xray_trace_id: Option<&str>,
    user_id: Option<&str>,
    status: u16,
    duration_ms: u128,
) {
    match (user_id, xray_trace_id) {
        (Some(user_id), Some(xray_trace_id)) => {
            error!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request failed"
            );
        }
        (Some(user_id), None) => {
            error!(
                service = service,
                request_id = request_id,
                user_id = user_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request failed"
            );
        }
        (None, Some(xray_trace_id)) => {
            error!(
                service = service,
                request_id = request_id,
                xray_trace_id = xray_trace_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request failed"
            );
        }
        (None, None) => {
            error!(
                service = service,
                request_id = request_id,
                method = %method,
                path = %path,
                status = status,
                duration_ms = duration_ms,
                "request failed"
            );
        }
    }
}
