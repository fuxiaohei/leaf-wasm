use clap::Parser;

mod base;
mod cmd;
mod server;

/// Leaf Command line
#[derive(Parser)]
#[clap(
    name = "leaf",
    version = base::get_version(),
)]
enum Leaf {
    /// init creates a new leaf project
    Init(cmd::Init),
    /// build compiles the leaf project
    Build(cmd::Build),
    /// serve runs the leaf project
    Serve(cmd::Serve),
}

#[tokio::main]
async fn main() {
    base::init_tracing();

    let args = Leaf::parse();
    match args {
        Leaf::Init(cmd) => cmd.run().await,
        Leaf::Build(cmd) => cmd.run().await,
        Leaf::Serve(cmd) => cmd.run().await,
    }
}
