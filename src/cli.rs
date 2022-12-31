mod base;
mod cmd_cli;
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
    /// New creates a new leaf project
    New(cmd_cli::NewCommand),
    /// Compile compiles the leaf project
    Compile(cmd_cli::CompileCommand),
    /// Up runs the leaf project
    Up(cmd_cli::UpCommand),
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::New(cmd) => cmd.run().await,
        LeafCli::Compile(cmd) => cmd.run().await,
        LeafCli::Up(cmd) => cmd.run().await,
    }
}
