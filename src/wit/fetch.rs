wasmtime::component::bindgen!({
    path: "./wit/leaf-http-fetch.wit",
    async: true,
});

use async_trait::async_trait;
use http_fetch::{HttpError, Request, Response};
use hyper::http::Request as httpRequest;
use hyper::Client;

pub struct FetchImpl {}

#[async_trait]
impl http_fetch::HttpFetch for FetchImpl {
    async fn fetch(
        &mut self,
        request: Request,
    ) -> wasmtime::component::Result<Response, HttpError> {
        let fetch_request = httpRequest::builder()
            .method(request.method.as_str())
            .uri("http://example.org/")
            .header("content-type", "application/json")
            .body(hyper::Body::from(request.body.unwrap()))
            .unwrap();

        let client = Client::new();
        let fetch_response = client.request(fetch_request).await.unwrap();

        let resp = Response {
            status: fetch_response.status().as_u16(),
            headers: vec![],
            body: Some(
                hyper::body::to_bytes(fetch_response.into_body())
                    .await
                    .unwrap()
                    .to_vec(),
            ),
        };
        Ok(resp)
    }
}

pub use http_fetch::add_to_linker;
