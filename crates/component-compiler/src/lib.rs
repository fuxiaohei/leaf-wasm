use std::path::PathBuf;
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
