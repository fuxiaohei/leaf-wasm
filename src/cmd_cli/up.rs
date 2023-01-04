use super::Manifest;
use crate::{common::vars::DEFAULT_MANIFEST_FILE, server};
use clap::Args;
use log::{error, info};
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
        info!("[Main] read manifest '{:?}'", manifest_file);

        let wasm_file = match manifest.determine_target() {
            Ok(file) => file,
            Err(e) => {
                error!("[Main] find wasm error: {}", e);
                return;
            }
        };

        if !std::path::PathBuf::from(&wasm_file).exists() {
            error!("[Worker] file not found: {}", &wasm_file);
            info!("[Worker] Try to run 'leaf-cli compile' to compile wasm file");
            return;
        }
        info!("[Worker] use file: {}", &wasm_file);

        // start local server
        server::start(self.addr.unwrap(), wasm_file).await;
    }
}
