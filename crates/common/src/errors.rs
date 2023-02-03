#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// failed to read manifest file
    #[error("failed to read manifest file '{1}': {0}")]
    ReadManifestFile(std::io::Error, String),

    /// failed to write manifest file
    #[error("failed to write manifest file '{1}': {0}")]
    WriteManifestFile(std::io::Error, String),

    /// failed to unmarshal manifest toml file
    #[error("failed to unmarshal manifest toml file: {0}")]
    UnmarshalManifestToml(toml::de::Error),

    /// failed to marshal manifest toml file
    #[error("failed to marshal manifest toml file: {0}")]
    MarshalManifestToml(toml::ser::Error),

    /// failed to init component manager pool
    #[error("failed to init component manager pool: {0}")]
    InitComponentManagerPool(anyhow::Error),

    /// failed to init wasmtime engine
    #[error("failed to init wasmtime engine: {0}")]
    InitEngine(anyhow::Error),

    /// failed to load wasmtime component
    #[error("failed to load wasmtime component: {0}, file: {1}")]
    ReadWasmComponent(anyhow::Error, String),

    /// failed to instantiate wasmtime component
    #[error("failed to instantiate wasmtime component: {0} at {1}")]
    InstantiateWasmComponent(anyhow::Error, String),

    /// failed to invoke component export function
    #[error("failed to invoke component export function: {0}")]
    InvokeComponentExportFunction(anyhow::Error),
}
