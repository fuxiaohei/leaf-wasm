use serde::{Deserialize, Serialize};

/// BuildInfo is the build section of the manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildInfo {
    pub rust_target_dir: Option<String>,
    pub rust_enable_wasi: Option<bool>,
}
