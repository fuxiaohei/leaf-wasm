wasmtime::component::bindgen!({
    path: "./wit/leaf-http-fetch.wit",
    async: true,
});

use async_trait::async_trait;
use http_fetch::{HttpError, Request, Response};

pub struct FetchImpl {}

#[async_trait]
impl http_fetch::HttpFetch for FetchImpl {
    async fn fetch(
        &mut self,
        _request: Request,
    ) -> wasmtime::component::Result<Response, HttpError> {
        let resp = Response {
            status: 200,
            headers: vec![],
            body: Some("Hello Fetch".as_bytes().to_vec()),
        };
        Ok(resp)
    }
}

pub use http_fetch::add_to_linker;
