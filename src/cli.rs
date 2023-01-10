mod base;
mod cmd;
mod common;
mod server;
mod wasm;
mod wit;

use clap::Parser;

/// Leaf Command line
#[derive(Parser)]
#[clap(
    name = "leaf-cli",
    version = base::get_version(),
)]
enum LeafCli {
    /// Init creates a new leaf project
    Init(cmd::cli::Init),
    /// Build compiles the leaf project
    Build(cmd::cli::Build),
    /// Serve runs the leaf project
    Serve(cmd::cli::Serve),
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::Init(cmd) => cmd.run().await,
        LeafCli::Build(cmd) => cmd.run().await,
        LeafCli::Serve(cmd) => cmd.run().await,
    }
}
