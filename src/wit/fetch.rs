wasmtime::component::bindgen!({
    path: "./wit/http-fetch.wit",
    async: true,
});

use async_trait::async_trait;
use http_fetch::{FetchOptions, HttpError, Request, Response};
use hyper::http::{Request as httpRequest, Response as httpResponse};
use hyper::Client;
use hyper::Uri;
use hyper_timeout::TimeoutConnector;
use log::{info, warn};
use std::time::Duration;
use tokio::time::Instant;

impl Default for FetchOptions {
    fn default() -> Self {
        FetchOptions {
            timeout: 30,
            decompress: false,
        }
    }
}

pub struct FetchImpl {
    pub req_id: u64,
    pub counter: u16,
}

impl FetchImpl {
    pub fn new(req_id: u64) -> Self {
        FetchImpl { req_id, counter: 0 }
    }
}

#[async_trait]
impl http_fetch::HttpFetch for FetchImpl {
    async fn fetch(
        &mut self,
        request: Request,
        options: FetchOptions,
    ) -> anyhow::Result<std::result::Result<Response, HttpError>> {
        info!(
            "[Fetch] request: {} {}, id={}",
            request.method, request.uri, self.req_id
        );
        self.counter += 1;
        let st = Instant::now();
        let mut fetch_request = httpRequest::builder()
            .method(request.method.as_str())
            .uri(request.uri.clone());
        for (key, value) in request.headers {
            fetch_request = fetch_request.header(key, value);
        }

        let fetch_body = match request.body {
            Some(b) => b,
            None => vec![],
        };
        let fetch_request = fetch_request.body(hyper::Body::from(fetch_body)).unwrap();
        info!("[Fetch] request: {:?}", fetch_request);

        let uri = request.uri.parse::<Uri>().unwrap();
        if uri.scheme().is_none() {
            warn!(
                "[Fetch] request failed: invalid uri={}, id={}, {}ms",
                request.uri,
                self.req_id,
                st.elapsed().as_millis()
            );
            return Ok(Err(HttpError::InvalidRequest));
        }

        // check uri schema is https
        let is_https = uri.scheme_str().unwrap() == "https";
        let fetch_response: httpResponse<hyper::Body> = if is_https {
            let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .build();

            // set client timeout
            let mut timeout = TimeoutConnector::new(https_connector);
            timeout.set_connect_timeout(Some(Duration::from_secs(options.timeout as u64)));
            timeout.set_read_timeout(Some(Duration::from_secs(options.timeout as u64)));
            timeout.set_write_timeout(Some(Duration::from_secs(options.timeout as u64)));

            let client = Client::builder().build(timeout);
            match client.request(fetch_request).await {
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
            }
        } else {
            let default_connector = hyper::client::HttpConnector::new();

            let mut timeout = TimeoutConnector::new(default_connector);
            timeout.set_connect_timeout(Some(Duration::from_secs(options.timeout as u64)));
            timeout.set_read_timeout(Some(Duration::from_secs(options.timeout as u64)));
            timeout.set_write_timeout(Some(Duration::from_secs(options.timeout as u64)));

            let client = Client::builder().build(timeout);
            match client.request(fetch_request).await {
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

#[tokio::test]
async fn run_fetch_impl_test() {
    use http_fetch::HttpFetch;
    let options = FetchOptions {
        timeout: 30,
        decompress: false,
    };

    let mut fetch_impl = FetchImpl::new(0);
    let req = Request {
        method: "GET".to_string(),
        uri: "http://www.rust-lang.org".to_string(),
        headers: vec![],
        body: None,
    };
    let resp = fetch_impl.fetch(req, options).await.unwrap().unwrap();
    assert_eq!(resp.status, 301);
    for (key, value) in resp.headers {
        if key == "location" {
            assert_eq!(value, "https://www.rust-lang.org/");
        }
    }
}

#[tokio::test]
async fn run_fetch_impl_https_test() {
    use http_fetch::HttpFetch;
    let fetch_options = FetchOptions {
        timeout: 30,
        decompress: false,
    };

    let mut fetch_impl = FetchImpl::new(0);
    let req = Request {
        method: "GET".to_string(),
        uri: "https://www.rust-lang.org".to_string(),
        headers: vec![],
        body: None,
    };
    let resp = fetch_impl.fetch(req, fetch_options).await.unwrap().unwrap();
    assert_eq!(resp.status, 200);
}
