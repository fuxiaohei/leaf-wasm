use leaf_sdk::{
    http::{Request, Response},
    http_handler,
};

#[http_handler]
fn handle(request: Request) -> Response {
    leaf_sdk::http::internal_server_error("testing error".to_string()).unwrap()
}
