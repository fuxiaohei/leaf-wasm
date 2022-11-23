wit_bindgen_guest_rust::generate!("../../wit/leaf-http.wit");

struct HttpImpl;

use leaf_http::Request;
use leaf_http::Response;

impl leaf_http::LeafHttp for HttpImpl {
    fn handle_request(req: Request) -> Response {
        let url = req.uri;
        let method = req.method.to_uppercase();

        let mut headers = req.headers;
        headers.push(("X-Request-Method".to_string(), method));
        headers.push(("X-Request-Url".to_string(), url));

        Response {
            status: 200,
            headers,
            body: req.body,
        }
    }
}

export_leaf_http!(HttpImpl);
