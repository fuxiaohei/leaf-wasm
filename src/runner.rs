mod base;

use clap::Parser;
use log::info;

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
}

fn main() {
    base::init_tracing();

    let args = RunnerArgs::parse();

    info!("Hello World!, config: {}", args.config);
}
