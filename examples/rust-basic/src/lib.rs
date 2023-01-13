use bytes::Bytes;
use leaf_sdk::{
    http::{Request, Response},
    http_main,
};
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(mut req: Request) -> Response {
    let url = req.uri().clone();
    let method = req.method().to_string().to_uppercase();
    http::Response::builder()
        .status(200)
        .header("X-Request-Url", url.to_string())
        .header("X-Request-Method", method)
        .body(Some(Bytes::from("Hello, World")))
        .unwrap()
}
