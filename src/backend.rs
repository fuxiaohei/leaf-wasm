mod base;

use clap::Parser;
use std::net::SocketAddr;

/// Leaf Backend
#[derive(Debug, Parser)]
#[clap(
    name = "leaf-backend",
    version = base::get_version(),
)]
struct BackendArgs {
    /// The config file
    #[clap(short, long, default_value("leaf-backend.toml"))]
    config: String,
    /// The port to listen on
    #[clap(long, default_value("0.0.0.0:21000"))]
    pub addr: Option<SocketAddr>,
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = BackendArgs::parse();

    // init database
    leaf_backend::init_db().await.unwrap();

    // init server
    leaf_backend::start(args.addr.unwrap()).await.unwrap();
}
