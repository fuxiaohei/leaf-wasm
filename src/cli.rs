mod base;

use clap::Parser;

/// Leaf Command line
#[derive(Parser)]
#[clap(
    name = "leaf-cli",
    version = base::get_version(),
)]
enum LeafCli {
    /// Init creates a new leaf project
    Init(leaf_cli::flags::Init),
    /// Build compiles the leaf project
    Build(leaf_cli::flags::Build),
    /// Serve runs the leaf project
    Serve(leaf_cli::flags::Serve),
    /// Component convert wasm module to component
    Component(leaf_cli::flags::Component),
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::Init(cmd) => cmd.run().await,
        LeafCli::Build(cmd) => cmd.run().await,
        LeafCli::Serve(cmd) => cmd.run().await,
        LeafCli::Component(cmd) => cmd.run().await,
    }
}
