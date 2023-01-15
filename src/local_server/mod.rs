use hyper::Server;
use std::net::SocketAddr;
use tracing::{error, info};

mod service;

pub async fn start(addr: SocketAddr, wasm: String, enable_wasi: bool) {
    let svc = match service::ServerContext::new(wasm, enable_wasi) {
        Ok(svc) => svc,
        Err(e) => {
            error!("[Server] init error: {}", e);
            return;
        }
    };

    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(svc),
        Err(e) => {
            error!("[Server] starting failed to bind: {}", e);
            return;
        }
    };

    info!("[Server] starting on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("[Server] starting error: {}", e);
    }
}

#[tokio::test]
async fn run_local_server_test() {
    let handle = tokio::spawn(async {
        start(
            "127.0.0.1:18899".parse().expect("wrong socket addr"),
            "./tests/sample.wasm".to_string(),
            false,
        )
        .await;
    });

    // wait for server to start
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let resp = reqwest::get("http://127.0.0.1:18899/abc")
        .await
        .expect("failed to get /abc");
    println!("{:?}", resp);

    assert!(resp.status().is_success());
    let headers = resp.headers();
    assert_eq!(headers.get("content-length").unwrap(), "12");
    assert_eq!(headers.get("x-request-url").unwrap(), "/abc");
    assert_eq!(headers.get("x-request-method").unwrap(), "GET");
    assert!(resp.text().await.unwrap().contains("Hello, World"));

    handle.abort();
    assert!(handle.await.unwrap_err().is_cancelled());
}
