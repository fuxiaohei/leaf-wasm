use bytes::Bytes;
use leaf_sdk::{
    http::{Request, Response},
    http_main, kv,
};
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(mut req: Request) -> Response {
    // set kv
    kv::set("key1".to_string(), "value1".to_string(), None).unwrap();

    // get kv
    let value = kv::get("key".to_string()).unwrap();

    http::Response::builder()
        .status(200)
        .body(Some(Bytes::from(format!("key1:{}", value))))
        .unwrap()
}
