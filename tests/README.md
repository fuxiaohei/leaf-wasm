# Tests Infomation

`sample.wasm` source is:

```rust
use leaf_sdk::http::{Request, Response};
use leaf_sdk_macro::http_main;
use std::str::FromStr;

#[http_main]
pub fn handle_sdk_http(req: Request) -> Response {
    let url = req.uri().clone();
    let method = req.method().to_string().to_uppercase();
    let body = req.into_body();
    let resp = http::Response::builder()
        .header("X-Request-Url", url.to_string())
        .header("X-Request-Method", method)
        .status(200)
        .body(body)
        .unwrap();
    resp
}
```

use `leaf-cli` to create sample project and replace `src/lib.rs` with `sample.wasm` source. Then run `leaf-cli compile` and target is in `target/wasm32-unknown-unknown/release/{project_name}.wasm`.
