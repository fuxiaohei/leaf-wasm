use std::path::PathBuf;
use wasmtime::{
    component::{Component, Linker},
    AsContextMut, Config, Engine, Store,
};
use wit_component::ComponentEncoder;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// read file failed error
    #[error("read file failed: {0}")]
    ReadFileFailed(std::io::Error),
    /// encode wasm component failed
    #[error("encode wasm component failed: {0}")]
    EncodeWasmComponentFailed(anyhow::Error),
    /// write file failed
    #[error("write file failed: {0}")]
    WriteFileFailed(std::io::Error),
    /// init engine error
    #[error("init engine error: {0}")]
    InitEngineError(anyhow::Error),
    /// read wasm component failed
    #[error("read wasm component failed: {0}")]
    ReadWasmComponentFailed(anyhow::Error),
}

pub fn encode(input: &str, output: Option<String>) -> Result<PathBuf, Error> {
    let file_bytes = std::fs::read(input).map_err(Error::ReadFileFailed)?;

    // TODO: add wasi support maybe
    let component = ComponentEncoder::default()
        .module(file_bytes.as_slice())
        .expect("Pull custom sections from module")
        .validate(true)
        .encode()
        .map_err(Error::EncodeWasmComponentFailed)?;

    let output_file = match output {
        Some(output) => PathBuf::from(output),
        None => {
            let filename = PathBuf::from(input)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                + "_component.wasm";
            PathBuf::from(filename)
        }
    };

    std::fs::write(&output_file, component).map_err(Error::WriteFileFailed)?;

    Ok(output_file)
}

pub fn check_http_handler(path: &str) -> Result<(), Error> {
    let config = create_wasmtime_config();
    let engine = Engine::new(&config).map_err(Error::InitEngineError)?;
    let component = Component::from_file(&engine, path).map_err(Error::ReadWasmComponentFailed)?;

    let mut store = Store::new(&engine, ());
    let linker: Linker<()> = Linker::new(&engine);
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let store_context = store.as_context_mut();
    let mut exports = instance.exports(store_context);
    if exports.root().func("handle-request").is_none() {
        return Err(Error::ReadWasmComponentFailed(anyhow::Error::msg(
            "Component does not have a handle-request function",
        )));
    }

    Ok(())
}

/// create_wasmtime_config creates a wasmtime config with the component model
fn create_wasmtime_config() -> Config {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config
}

mod codegen;
pub use codegen::code_gen;