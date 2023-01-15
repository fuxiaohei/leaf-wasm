mod base;
mod run_server;

use clap::Parser;
use std::net::SocketAddr;

/// Leaf Runner
#[derive(Parser)]
#[clap(
    name = "leaf-runner",
    version = base::get_version(),
)]
struct RunnerArgs {
    /// The config file
    #[clap(short, long, default_value("leaf-runner.toml"))]
    config: String,
    /// The port to listen on
    #[clap(long, default_value("0.0.0.0:19988"))]
    pub addr: Option<SocketAddr>,
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = RunnerArgs::parse();
    
    // start server server
    run_server::start(args.addr.unwrap()).await;
}
