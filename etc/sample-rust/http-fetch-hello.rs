use leaf_sdk::http::{fetch, FetchOptions, Request, Response};
use leaf_sdk_macro::http_main;
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(mut req: Request) -> Response {
    let url = req.uri().clone();
    let method = req.method().to_string().to_uppercase();

    let mut fetch_request = http::Request::builder()
        .method("GET")
        .uri("https://www.rust-lang.org/")
        .body(Some(bytes::Bytes::from("")))
        .unwrap();

    let fetch_response = fetch(
        req,
        FetchOptions {
            timeout: 30,
            decompress: false,
        },
    )
    .unwrap();
    let resp = http::Response::builder()
        .header("X-Request-Url", url.to_string())
        .header("X-Request-Method", method)
        .status(200)
        .body(fetch_response.body().clone())
        .unwrap();
    resp
}
