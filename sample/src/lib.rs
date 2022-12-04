// use leaf_sdk::http::{Request, Response};
use leaf_sdk_macro::http_main;

#[http_main]
pub fn handle_sdk_http(req: Request) -> Response {
    let url = req.uri();
    let method = req.method().as_str().to_uppercase();

    use http::header::HeaderName;

    let mut builder = http::Response::builder().status(200);
    let headers = builder.headers_mut().unwrap();
    headers.insert(
        HeaderName::from_static("X-Request-Method"),
        method.parse().unwrap(),
    );
    headers.insert(
        HeaderName::from_static("X-Request-Url"),
        url.to_string().parse().unwrap(),
    );

    builder.body(Some(Bytes::from("Hello, World"))).unwrap()
}
