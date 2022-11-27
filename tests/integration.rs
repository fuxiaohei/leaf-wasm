use std::process::Command;

/// rust sample test
#[test]
fn run_wasm_component_exports_test() {
    // run command 'cargo build -p "leaf-abi-impl" --release --target wasm32-unknown-unknown'
    let result = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("leaf-abi-impl")
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .output()
        .expect("failed to build leaf-abi-impl");
    println!("status: {}", result.status);

    let target_file = "target/wasm32-unknown-unknown/release/leaf_abi_impl.wasm";
    // check target file exist
    assert!(std::path::Path::new(target_file).exists());

    // encode wasm module to wasm component
    let component_file = match component_compiler::encode(target_file, None) {
        Ok(file) => file,
        Err(err) => panic!("encode component failed: {}", err),
    };
    // check component generated
    assert!(std::path::Path::new(&component_file).exists());

    // check http handler is exported in wasm component
    assert!(component_compiler::check_http_handler(component_file.to_str().unwrap()).is_ok());
}
