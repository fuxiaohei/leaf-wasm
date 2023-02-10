use clap::{Args, Parser};
use common::manifest::Manifest;
use std::net::SocketAddr;
use tracing::Instrument;
use tracing::{debug, info};

const DEFAULT_MANIFEST_FILE: &str = "manifest.toml";

fn must_read_manifest(path: &str) -> Manifest {
    let manifest = Manifest::from_file(path)
        .map_err(|e| {
            tracing::error!("read manifest file failed: {}", e);
            std::process::exit(1);
        })
        .unwrap();
    manifest
}

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
        // 1. load manifest file
        let filename = DEFAULT_MANIFEST_FILE;
        let entered = tracing::error_span!("[Manifest]", manifest = filename).entered();
        let manifest = must_read_manifest(filename);
        debug!("manifest: {:?}", manifest);
        info!("manifest load ok");

        // 2. get target wasm file
        entered.exit();

        // 3. start local http server
        super::server::start(self.addr.unwrap())
            .instrument(tracing::info_span!("[Server]"))
            .await
            .unwrap();
    }
}
