use crate::common::errors::Error;
use crate::common::vars::*;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub language: String,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            manifest: String::from("v1"),
            name: String::from("leaf"),
            description: String::from("leaf wasm project"),
            authors: vec![],
            language: String::from("rust"),
        }
    }
}

impl Manifest {
    /// from_file reads the manifest file and returns a Manifest struct
    pub fn from_file(file: &str) -> Result<Manifest, Error> {
        let content = std::fs::read_to_string(file)
            .map_err(|e| Error::ReadManifestFile(e, String::from(file)))?;
        let manifest: Manifest = toml::from_str(&content).map_err(Error::UnmarshalManifestToml)?;
        Ok(manifest)
    }

    /// to_file writes the manifest struct to a file
    pub fn to_file(&self, file: &str) -> Result<(), Error> {
        let content = toml::to_string(&self).map_err(Error::MarshalManifestToml)?;
        std::fs::write(file, content).map_err(|e| Error::WriteManifestFile(e, file.to_string()))?;
        Ok(())
    }

    /// determine compiling target file, before optimized
    pub fn compiling_target(&self) -> Result<String> {
        let name = self.name.replace('-', "_");
        let dir = env!("CARGO_MANIFEST_DIR");
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(format!(
                "{}/{}/{}.wasm",
                dir, RUST_TARGET_WASM_RELEASE_DIR, name
            )),
            PROJECT_LANGUAGE_JS => Ok(format!("{}.wasm", name)),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine optimized final target file
    pub fn final_target(&self) -> Result<String> {
        let name = self.name.replace('-', "_");
        let dir = env!("CARGO_MANIFEST_DIR");
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(format!(
                "{}/{}/{}.wasm",
                dir, RUST_TARGET_WASM_RELEASE_DIR, name
            )),
            PROJECT_LANGUAGE_JS => Ok(format!("{}_wizer.wasm", name)),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine compile arch
    pub fn compile_arch(&self) -> Result<String> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(COMPILE_TARGET_WASM32_UNKNOWN_UNKNOWN.to_string()),
            PROJECT_LANGUAGE_JS => Ok(COMPILE_TARGET_WASM32_WASI.to_string()),
            _ => Err(anyhow::Error::msg("unknown compile target")),
        }
    }

    /// determine enable wasi
    pub fn is_enable_wasi(&self) -> Result<bool> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(false),
            PROJECT_LANGUAGE_JS => Ok(true),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }
}
