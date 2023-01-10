use anyhow::Result;
use log::{debug, info};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use which::which;
use wit_component::ComponentEncoder;

pub fn compile_rust(arch: String, target: String, optimize: bool, debug: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if !debug {
        cmd.arg("--release");
    }
    let child = cmd
        .arg("--target")
        .arg(arch)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
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
    if !PathBuf::from(&target).exists() {
        return Err(anyhow::anyhow!("Wasm file not found: {}", target));
    }
    if optimize {
        try_wasm_optimize(&target);
    }
    convert_rust_component(&target);
    Ok(())
}

fn try_wasm_optimize(path: &str) {
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
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
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

fn convert_rust_component(path: &str) {
    let file_bytes = std::fs::read(path).expect("Read wasm file error");
    let wasi_adapter = include_bytes!("../engine/wasi_snapshot_preview1.wasm");

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

pub fn compile_js(target: String, src_js_path: String) -> Result<()> {
    // js need wizer command
    let cmd = match which("wizer") {
            Ok(cmd) => cmd,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Wizer not found \n\tplease install wizer first: \n\tcargo install wizer --all-features\n\tmore infomation see: https://github.com/bytecodealliance/wizer"
                ))
            }
        };

    let engine_wasm = include_bytes!("../engine/quickjs.wasm");
    debug!("Use engine_wasm len: {}", engine_wasm.len());
    debug!("Initialize target wasm file: {}", &target);
    std::fs::write(&target, engine_wasm)?;
    let src_content = std::fs::read(src_js_path)?;

    // wizer leaf_wasm_js.wasm -o leaf_wasm_js_wizer.wasm --allow-wasi --inherit-stdio=true --inherit-env=true
    let wizer_target = target.replace(".wasm", "_wizer.wasm");
    let mut child = Command::new(cmd)
        .arg(&target)
        .arg("-o")
        .arg(&wizer_target)
        .arg("--allow-wasi")
        .arg("--inherit-stdio=true")
        .arg("--inherit-env=true")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute wizer child process");
    let mut stdin = child.stdin.take().expect("failed to get stdin");

    std::thread::spawn(move || {
        stdin
            .write_all(src_content.as_slice())
            .expect("failed to write to stdin");
    });

    let output = child
        .wait_with_output()
        .expect("failed to wait on wizer child process");
    if output.status.success() {
        // print output
        debug!(
            "Wizer output: \n{}",
            std::str::from_utf8(&output.stdout).unwrap()
        );
        info!("Wizer success: {}", &wizer_target);
    } else {
        panic!("Wizer failed: {:?}", output);
    }
    
    convert_rust_component(&wizer_target);

    Ok(())
}
