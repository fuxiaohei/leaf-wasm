wasmtime::component::bindgen!({
    path: "./wit/leaf-http-fetch.wit",
    async: true,
});

use async_trait::async_trait;
use http_fetch::{HttpError, Request, Response};
use hyper::http::Request as httpRequest;
use hyper::Client;
use log::{info, warn};

pub struct FetchImpl {
    pub req_id: u64,
}

#[async_trait]
impl http_fetch::HttpFetch for FetchImpl {
    async fn fetch(
        &mut self,
        request: Request,
    ) -> anyhow::Result<std::result::Result<Response, HttpError>> {
        info!(
            "[Fetch] request: {} {}, id={}",
            request.method, request.uri, self.req_id
        );
        let st = Instant::now();
        let mut fetch_request = httpRequest::builder()
            .method(request.method.as_str())
            .uri(request.uri);
        for (key, value) in request.headers {
            fetch_request = fetch_request.header(key, value);
        }
        let fetch_request = fetch_request
            .body(hyper::Body::from(request.body.unwrap()))
            .unwrap();
        info!("[Fetch] request: {:?}", fetch_request);

        let client = Client::new();
        let fetch_response = match client.request(fetch_request).await {
            Ok(r) => r,
            Err(e) => {
                warn!(
                    "[Fetch] request failed: {}, id={}, {}ms",
                    e,
                    self.req_id,
                    st.elapsed().as_millis()
                );
                return Ok(Err(HttpError::InvalidRequest));
            }
        };

        let mut resp_headers = vec![];
        for (key, value) in fetch_response.headers() {
            resp_headers.push((key.to_string(), value.to_str().unwrap().to_string()));
        }
        let resp = Response {
            status: fetch_response.status().as_u16(),
            headers: resp_headers,
            body: Some(
                hyper::body::to_bytes(fetch_response.into_body())
                    .await
                    .unwrap()
                    .to_vec(),
            ),
        };
        info!(
            "[Fetch] response: {}, id={}, {}ms",
            resp.status,
            self.req_id,
            st.elapsed().as_millis()
        );
        Ok(Ok(resp))
    }
}

pub use http_fetch::add_to_linker;
use tokio::time::Instant;
