use clap::{Args, Parser};
use std::net::SocketAddr;
use tracing::Instrument;

/// Leaf Command line
#[derive(Parser)]
#[clap(name = "leaf-cli", version = "0.1..0")]
pub enum LeafCli {
    /// Init creates a new leaf project
    Init(Init),
    /// Build compiles the leaf project
    Build(Build),
    /// Serve runs the leaf project
    Serve(Serve),
}

/// Init command
#[derive(Args, Debug)]
pub struct Init {
    /// The name of the project
    pub name: String,
    /// The template to use
    #[clap(long, default_value("rust-basic"))]
    pub template: Option<String>,
}

impl Init {
    pub async fn run(&self) {
        println!("Init command: {:?}", self);
    }
}

/// Build command
#[derive(Args, Debug)]
pub struct Build {}

impl Build {
    pub async fn run(&self) {
        println!("Build command: {:?}", self);
    }
}

/// Server command with addr
#[derive(Args, Debug)]
pub struct Serve {
    /// The address to serve on
    #[clap(long, default_value("127.0.0.1:18080"))]
    pub addr: Option<SocketAddr>,
}

impl Serve {
    pub async fn run(&self) {
        super::server::start(self.addr.unwrap())
            .instrument(tracing::info_span!("[Server]"))
            .await
            .unwrap();
    }
}
