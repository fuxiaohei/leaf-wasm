#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// failed to read manifest file
    #[error("failed to read manifest file '{1}': {0}")]
    ReadManifestFile(std::io::Error, String),
    /// failed to unmarshal manifest toml file
    #[error("failed to unmarshal manifest toml file: {0}")]
    UnmarshalManifestToml(toml::de::Error),
    /// failed to init wasmtime engine
    #[error("failed to init wasmtime engine: {0}")]
    InitEngine(anyhow::Error),
    /// failed to load wasmtime component
    #[error("failed to load wasmtime component: {0}, file: {1}")]
    ReadWasmComponent(anyhow::Error, String),
    /// failed to instantiate wasmtime component
    #[error("failed to instantiate wasmtime component: {0}")]
    InstantiateWasmComponent(anyhow::Error),
    /// failed to init component manager pool
    #[error("failed to init component manager pool: {0}")]
    InitComponentManagerPool(anyhow::Error),
}
