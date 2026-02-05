use lambda_http::{
    http::{self, StatusCode},
    Body, Response,
};

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
