mod base;
mod run_server;

use clap::Parser;
use run_server::RunnerConfig;
use std::net::SocketAddr;
use std::path::Path;
use tracing::log::debug;

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
    /// The port to listen on admin address
    #[clap(long, default_value("0.0.0.0:19989"))]
    pub admin_addr: Option<SocketAddr>,
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = RunnerArgs::parse();

    // load config from file
    let config = if Path::new(&args.config).exists() {
        debug!("[Runner] load config from file: {}", &args.config);
        RunnerConfig::from_file(&args.config).unwrap()
    } else {
        debug!(
            "[Runner] config file not found, use default config: {}",
            &args.config
        );
        RunnerConfig::default()
    };
    debug!("[Runner] config: {:?}", config);

    let config2 = config.clone();

    tokio::spawn(async move {
        run_server::start_admin(args.admin_addr.unwrap(), config.clone()).await;
    });

    // start server server
    run_server::start(args.addr.unwrap(), config2).await;
}
