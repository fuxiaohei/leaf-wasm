use std::path::Path;

use crate::common::errors::Error;
use crate::common::vars::*;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

/// ManifestBuild is the build section of the manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestBuild {
    pub rust_target_dir: Option<String>,
    pub rust_enable_wasi: Option<bool>,
}

impl Default for ManifestBuild {
    fn default() -> Self {
        Self {
            rust_target_dir: None,
            rust_enable_wasi: None,
        }
    }
}

/// Manifest is the manifest struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<ManifestBuild>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            manifest: String::from("v1"),
            name: String::from("leaf"),
            description: String::from("leaf wasm project"),
            authors: vec![],
            language: String::from("rust"),
            build: None,
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
        let target = self
            .build
            .clone()
            .unwrap_or(ManifestBuild::default())
            .rust_target_dir
            .unwrap_or("target".to_string());
        let arch = self.compile_arch().unwrap_or("rust".to_string());
        let target_dir = Path::new(&target).join(arch).join("release");
        let name = self.name.replace('-', "_") + ".wasm";
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(target_dir.join(name).to_str().unwrap().to_string()),
            PROJECT_LANGUAGE_JS => Ok(name),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine optimized final target file
    pub fn final_target(&self) -> Result<String> {
        let compile_target = self.compiling_target();
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(compile_target.unwrap()),
            PROJECT_LANGUAGE_JS => Ok(compile_target.unwrap().replace(".wasm", "_wizer.wasm")),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine compile arch
    pub fn compile_arch(&self) -> Result<String> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => {
                let flag = self
                    .build
                    .clone()
                    .unwrap_or(ManifestBuild::default())
                    .rust_enable_wasi
                    .unwrap_or(false);
                if flag {
                    return Ok(COMPILE_TARGET_WASM32_WASI.to_string());
                }
                Ok(COMPILE_TARGET_WASM32_UNKNOWN_UNKNOWN.to_string())
            }
            PROJECT_LANGUAGE_JS => Ok(COMPILE_TARGET_WASM32_WASI.to_string()),
            _ => Err(anyhow::Error::msg("unknown compile target")),
        }
    }

    /// determine enable wasi
    pub fn is_enable_wasi(&self) -> Result<bool> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => {
                let flag = self
                    .build
                    .clone()
                    .unwrap_or(ManifestBuild::default())
                    .rust_enable_wasi
                    .unwrap_or(false);
                Ok(flag)
            }
            PROJECT_LANGUAGE_JS => Ok(true),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }
}
