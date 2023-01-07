use hyper::Server;
use log::{error, info};
use std::net::SocketAddr;

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
