mod flags;
mod server;

use clap::Parser;
use flags::LeafCli;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let args = LeafCli::parse();
    match args {
        LeafCli::Init(cmd) => cmd.run().await,
        LeafCli::Build(cmd) => cmd.run().await,
        LeafCli::Serve(cmd) => cmd.run().await,
    }
}
