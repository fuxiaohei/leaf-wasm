use super::Manifest;
use crate::vars::{DEFAULT_MANIFEST_FILE, PROJECT_LANGUAGE_RUST, RUST_TARGET_WASM_RELEASE_DIR};
use clap::Args;
use log::{error, info};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use wit_component::ComponentEncoder;

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
}

impl CompileCommand {
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
        info!("Read manifest '{:?}'", manifest_file);
        if manifest.language == PROJECT_LANGUAGE_RUST {
            do_rust_compile(&manifest);
        }
    }
}

fn do_rust_compile(manifest: &Manifest) {
    // run cargo build --release wasm32-unknow-unknown
    let child = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute cargo child process");
    let output = child
        .wait_with_output()
        .expect("failed to wait on cargo child process");
    if output.status.success() {
        info!("Cargo build wasm success");
    } else {
        panic!("Cargo build wasm failed: {:?}", output);
    }

    // check target wasm file
    let target_wasm_file = format!(
        "{}/{}.wasm",
        RUST_TARGET_WASM_RELEASE_DIR,
        manifest.name.replace('-', "_")
    );
    if !PathBuf::from(&target_wasm_file).exists() {
        panic!("Wasm file not found: {}", &target_wasm_file);
    }
    convert_rust_component(&target_wasm_file);
}

fn convert_rust_component(path: &str) {
    let file_bytes = std::fs::read(path).expect("Read wasm file error");
    let wasi_adapter = include_bytes!("../../wit/wasi_snapshot_preview1.wasm");

    let component = ComponentEncoder::default()
        .module(file_bytes.as_slice())
        .expect("Pull custom sections from module")
        .validate(true)
        .adapter("wasi_snapshot_preview1", wasi_adapter)
        .unwrap()
        .encode()
        .expect("Encode component");

    std::fs::write(path, component).expect("Write component file error");
    info!("Convert wasm module to component success")
}
