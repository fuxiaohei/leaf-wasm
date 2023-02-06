use anyhow::Result;
use std::net::SocketAddr;
use tracing::info;

pub async fn start(addr: SocketAddr) -> Result<()> {
    use warp::Filter;

    info!("[Server] listening on {addr}");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(addr).await;

    Ok(())
}

mod db;
pub use db::init_db;
pub use db::DB;

mod migrator;
