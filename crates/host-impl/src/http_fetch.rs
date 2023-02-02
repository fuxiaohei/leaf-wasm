wasmtime::component::bindgen!({
    world:"http-fetch",
    path: "../../wit/http-fetch.wit",
    async: true,
});

use async_trait::async_trait;
use http_fetch::{FetchOptions, HttpError, RedirectPolicy, Request, Response};
use reqwest::redirect;
use std::str::FromStr;
use tokio::time::Instant;
use tracing::{info, warn};

impl Default for FetchOptions {
    fn default() -> Self {
        FetchOptions {
            timeout: 30,
            decompress: false,
            redirect: RedirectPolicy::Follow,
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

impl TryFrom<http_fetch::RedirectPolicy> for redirect::Policy {
    type Error = anyhow::Error;
    fn try_from(value: http_fetch::RedirectPolicy) -> Result<Self, Self::Error> {
        match value {
            http_fetch::RedirectPolicy::Follow => Ok(redirect::Policy::default()),
            http_fetch::RedirectPolicy::Error => Ok(redirect::Policy::custom(|attempt| {
                attempt.error(anyhow::anyhow!("redirect policy is error"))
            })),
            http_fetch::RedirectPolicy::Manual => Ok(redirect::Policy::none()),
        }
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

        let fetch_body = match request.body {
            Some(b) => b,
            None => vec![],
        };

        let client = reqwest::Client::builder()
            .redirect(options.redirect.try_into()?)
            .build()?;
        let fetch_response = match client
            .request(
                reqwest::Method::from_str(request.method.as_str()).unwrap(),
                request.uri.clone(),
            )
            .timeout(std::time::Duration::from_secs(options.timeout as u64))
            .body(reqwest::Body::from(fetch_body))
            .send()
            .await
        {
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
            body: Some(fetch_response.bytes().await?.to_vec()),
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

    let mut fetch_impl = FetchImpl::new(0);
    let req = Request {
        method: "GET".to_string(),
        uri: "http://www.rust-lang.org".to_string(),
        headers: vec![],
        body: None,
    };
    let fetch_options = FetchOptions {
        redirect: RedirectPolicy::Manual,
        ..Default::default()
    };
    let resp = fetch_impl.fetch(req, fetch_options).await.unwrap().unwrap();
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
    let mut fetch_impl = FetchImpl::new(0);
    let req = Request {
        method: "GET".to_string(),
        uri: "https://www.rust-lang.org".to_string(),
        headers: vec![],
        body: None,
    };
    let resp = fetch_impl
        .fetch(req, FetchOptions::default())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resp.status, 200);
}
