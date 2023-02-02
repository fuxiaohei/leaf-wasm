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
}
