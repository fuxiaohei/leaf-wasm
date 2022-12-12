use bytes::Bytes;
use leaf_sdk::http::{Request, Response, fetch};
use leaf_sdk_macro::http_main;
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(req: Request) -> Response {
    let url = req.uri().clone();
    let method = req.method().to_string().to_uppercase();
    let fetch_response = fetch(req).unwrap();
    let resp = http::Response::builder()
        .header("X-Request-Url", url.to_string())
        .header("X-Request-Method", method)
        .status(200)
        .body(fetch_response.body().clone())
        .unwrap();
    resp
}