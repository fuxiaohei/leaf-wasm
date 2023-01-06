use super::Manifest;
use crate::common::vars::*;
use clap::Args;
use log::{error, info};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use which::which;
use wit_component::ComponentEncoder;

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
    /// Set optimization progress
    #[clap(long, default_value("false"))]
    pub optimize: bool,
    /// Set compiling debug mode
    #[clap(long, default_value("false"))]
    pub debug: bool,
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
        match self.do_compile(&manifest) {
            Ok(_) => info!("Compile success"),
            Err(e) => error!("Compile failed: {}", e),
        }
    }

    fn do_compile(&self, manifest: &Manifest) -> anyhow::Result<()> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        if !self.debug {
            cmd.arg("--release");
        }
        let child = cmd
            .arg("--target")
            .arg(manifest.determine_compile_target()?)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute cargo child process");
        let output = child
            .wait_with_output()
            .expect("failed to wait on cargo child process");
        if output.status.success() {
            info!("Cargo build wasm success");
        } else {
            return Err(anyhow::anyhow!("Cargo build wasm failed: {:?}", output));
        }
        let target_wasm_file = manifest.determine_target()?;
        if !PathBuf::from(&target_wasm_file).exists() {
            return Err(anyhow::anyhow!("Wasm file not found: {}", target_wasm_file));
        }
        if self.optimize {
            self.try_wasm_optimize(&target_wasm_file);
        }
        self.convert_rust_component(&target_wasm_file);
        Ok(())
    }

    fn try_wasm_optimize(&self, path: &str) {
        let cmd = match which("wasm-opt") {
            Ok(cmd) => cmd,
            Err(_) => {
                info!("Command wasm-opt not found, skip wasm-opt");
                return;
            }
        };
        let child = Command::new(cmd)
            .arg("--strip-debug")
            .arg("-o")
            .arg(path)
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute wasm-opt child process");
        let output = child
            .wait_with_output()
            .expect("failed to wait on wasm-opt child process");
        if output.status.success() {
            info!("Wasm-opt success");
        } else {
            panic!("Wasm-opt failed: {:?}", output);
        }
    }

    fn convert_rust_component(&self, path: &str) {
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
        info!("Convert wasm module to component success, {}", path)
    }
}
