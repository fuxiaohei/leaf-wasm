use clap::Parser;
mod cmd;
mod common;
mod server;
mod wasm;
mod wit;

use cmd::cli;

/// Leaf Command line
#[derive(Parser)]
#[clap(
    name = "leaf-cli",
    version = cmd::get_version(),
)]
enum LeafCli {
    /// New creates a new leaf project
    New(cli::NewCommand),
    /// Compile compiles the leaf project
    Compile(cli::CompileCommand),
    /// Up runs the leaf project
    Up(cli::UpCommand),
}

#[tokio::main]
async fn main() {
    cmd::init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::New(cmd) => cmd.run().await,
        LeafCli::Compile(cmd) => cmd.run().await,
        LeafCli::Up(cmd) => cmd.run().await,
    }
}
