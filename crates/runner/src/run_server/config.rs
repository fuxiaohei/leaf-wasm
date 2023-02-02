use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerConfig {
    pub wasm_dir: String,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            wasm_dir: String::from("./wasm_dir"),
        }
    }
}

impl RunnerConfig {
    pub fn from_file(file: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file)?;
        let config: RunnerConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
