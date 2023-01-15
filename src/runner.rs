mod base;

use clap::Parser;
use std::net::SocketAddr;
use tracing::info;

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

fn main() {
    base::init_tracing();

    let args = RunnerArgs::parse();

    info!("Hello World!, config: {}", args.config);
}
