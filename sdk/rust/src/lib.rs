pub mod http {
    use anyhow::Result;
    use bytes::Bytes;

    pub type Request = http::Request<Bytes>;
    pub type Response = http::Response<Bytes>;

    pub fn error_response(status: http::StatusCode, message: String) -> Result<Response> {
        let mut response = Response::new(message.into());
        *response.status_mut() = status;
        Ok(response)
    }
}

// Re-export leaf_sdk_macro,
// make all functions in leaf_sdk
pub use leaf_sdk_macro::http_main;

// mod abi;

#[cfg(test)]
mod sdk_tests {
    use crate::sdk_tests::http_handler::HttpHandler;

    use super::http::*;
    use super::http_main;
    use http::StatusCode;

    #[http_main]
    fn main(req: Request) -> Response {
        let mut response = Response::new("Hello, world!".into());
        response.headers_mut().insert(
            "X-Request-Method",
            http::HeaderValue::from_str(req.method().as_str()).unwrap(),
        );
        *response.status_mut() = StatusCode::OK;
        response
    }

    #[test]
    fn test_main() {
        let req = http_handler::Request {
            method: "GET".to_string(),
            uri: "/".to_string(),
            headers: vec![],
            body: Some("Hello".as_bytes().to_vec()),
        };
        let resp = HttpImpl::handle_request(req);

        println!("resp: {:?}", resp);

        assert_eq!(resp.status, StatusCode::OK);
        assert_eq!(resp.body.unwrap(), "Hello, world!".as_bytes());
        resp.headers
            .iter()
            .find(|(k, v)| {
                if k == "X-Request-Method" {
                    assert_eq!(v, "GET");
                }
                true
            })
            .unwrap();
    }
}
