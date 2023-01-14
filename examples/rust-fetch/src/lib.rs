use leaf_sdk::{
    http::{fetch, FetchOptions, Request, Response},
    http_main,
};
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(mut _req: Request) -> Response {
    let fetch_request = http::Request::builder()
        .method("GET")
        .uri("https://www.rust-lang.org/")
        .body(None)
        .unwrap();

    let fetch_response = fetch(fetch_request, FetchOptions::default()).unwrap();

    http::Response::builder()
        .status(fetch_response.status())
        .body(fetch_response.body().clone())
        .unwrap()
}
