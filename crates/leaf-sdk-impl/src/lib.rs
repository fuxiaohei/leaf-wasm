use bytes::Bytes;
use leaf_sdk::http::{Request, Response};
use std::str::FromStr;

wit_bindgen_guest_rust::generate!("../../wit/leaf-http.wit");

fn handle_sdk_http(req: Request) -> Response {
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

impl TryFrom<leaf_http::Request> for http::Request<Option<bytes::Bytes>> {
    type Error = anyhow::Error;

    fn try_from(leaf_req: leaf_http::Request) -> Result<Self, Self::Error> {
        let mut http_req = http::Request::builder()
            .method(http::Method::from_str(leaf_req.method.as_str())?)
            .uri(&leaf_req.uri);

        for (key, value) in leaf_req.headers {
            http_req = http_req.header(key, value);
        }

        let body = match leaf_req.body {
            Some(b) => b.to_vec(),
            None => Vec::new(),
        };

        let body = Some(bytes::Bytes::from(body));

        Ok(http_req.body(body)?)
    }
}

impl TryFrom<http::Response<Option<bytes::Bytes>>> for leaf_http::Response {
    type Error = anyhow::Error;

    fn try_from(http_res: http::Response<Option<bytes::Bytes>>) -> Result<Self, Self::Error> {
        let status = http_res.status().as_u16();
        let mut headers: Vec<(String, String)> = vec![];
        for (key, value) in http_res.headers() {
            headers.push((key.to_string(), value.to_str()?.to_string()));
        }
        let body = http_res.body().as_ref().map(|b| b.to_vec());
        Ok(leaf_http::Response {
            status,
            headers,
            body,
        })
    }
}

struct HttpImpl;

impl leaf_http::LeafHttp for HttpImpl {
    fn handle_request(req: leaf_http::Request) -> leaf_http::Response {
        let http_req: Request = match req.try_into() {
            Ok(r) => r,
            Err(e) => {
                return leaf_http::Response {
                    status: 500,
                    headers: vec![],
                    body: Some(
                        format!("Request Convert Error: {:?}", e)
                            .as_bytes()
                            .to_vec(),
                    ),
                }
            }
        };
        let http_resp = handle_sdk_http(http_req);
        match http_resp.try_into() {
            Ok(r) => r,
            Err(e) => leaf_http::Response {
                status: 500,
                headers: vec![],
                body: Some(
                    format!("Response Convert Error: {:?}", e)
                        .as_bytes()
                        .to_vec(),
                ),
            },
        }
    }
}

export_leaf_http!(HttpImpl);
