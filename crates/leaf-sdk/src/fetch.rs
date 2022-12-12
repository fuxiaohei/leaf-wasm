use bytes::Bytes;
use http::Response as httpResponse;

use super::http::{Error, Request, Response};

include!("../../../wit/leaf-http-fetch.rs");

pub fn fetch(req: Request) -> Result<Response, Error> {
    // convert leaf_http::Request to http_fetch::Request
    let mut headers = vec![];
    for (key, value) in req.headers() {
        headers.push((key.as_str(), value.to_str().unwrap()));
    }
    let body = match req.body() {
        Some(b) => b.to_vec(),
        None => Vec::new(),
    };
    let uri = req.uri().clone().to_string();
    let fetch_req = http_fetch::Request {
        method: req.method().as_str(),
        uri: uri.as_str(),
        headers: &headers,
        body: Some(body.as_slice()),
    };

    // call host function to fetch
    let fetch_resp = http_fetch::fetch(fetch_req)?;

    // convert http_fetch::Response to leaf_http::Response
    let body = match fetch_resp.body {
        Some(b) => Some(Bytes::from(b)),
        None => None,
    };
    let mut builder = httpResponse::builder().status(fetch_resp.status);
    for (key, value) in headers {
        builder = builder.header(key, value);
    }
    let resp = builder.body(body).unwrap();
    Ok(resp)
}

pub use http_fetch::HttpError;
