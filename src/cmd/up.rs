use super::Manifest;
use crate::server;
use crate::vars::{DEFAULT_MANIFEST_FILE, RUST_TARGET_WASM_RELEASE_DIR};
use clap::Args;
use log::{debug, error};
use std::net::SocketAddr;

#[derive(Args, Debug)]
pub struct UpCommand {
    // The port to listen on
    #[clap(long, default_value("0.0.0.0:18899"))]
    pub addr: Option<SocketAddr>,
}

impl UpCommand {
    pub async fn run(&self) {
        // load manifest file
        let manifest_file = DEFAULT_MANIFEST_FILE;
        let manifest = match Manifest::from_file(manifest_file) {
            Ok(manifest) => manifest,
            Err(e) => {
                error!("read manifest file error: {}", e);
                return;
            }
        };
        debug!("manifest: {:?}", manifest);

        // find wasm file
        let wasm_file = format!("{}/{}_component.wasm", RUST_TARGET_WASM_RELEASE_DIR, manifest.name);
        if !std::path::PathBuf::from(&wasm_file).exists() {
            // TODO: try compile wasm file
            error!("wasm file not found: {}", &wasm_file);
            return;
        }
        debug!("wasm file: {}", &wasm_file);

        // start local server
        server::start(self.addr.unwrap(), wasm_file).await;
    }
}
