use hyper::Server;
use log::{error, info};
use std::net::SocketAddr;

mod service;

mod wasm;
pub use wasm::ComponentContext;
pub use wasm::ComponentPool;

mod wit;
pub use wit::LeafHttp;

pub async fn start(addr: SocketAddr, wasm: String) {
    let svc = match service::ServerContext::new(wasm) {
        Ok(svc) => svc,
        Err(e) => {
            error!("init server context error: {}", e);
            return;
        }
    };

    let server = match Server::try_bind(&addr) {
        Ok(server) => server.serve(svc),
        Err(e) => {
            error!("Starting server error, failed to bind: {}", e);
            return;
        }
    };

    info!("Starting server on {}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("Starting server error: {}", e);
    }
}
