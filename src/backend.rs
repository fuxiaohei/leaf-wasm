mod base;

use clap::Parser;
use std::net::SocketAddr;
use tracing::{debug, info};

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
    debug!("Args: {:?}", args);
    info!("Starting leaf-backend {}", base::get_version());
}
