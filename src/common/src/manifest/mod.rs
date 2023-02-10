mod build;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Lang is the project language
#[derive(Debug)]
pub enum Language {
    Rust,
    Go,
    JavaScript,
}

/// Manifest is the manifest struct
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Manifest {
    pub manifest: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<build::BuildInfo>,
}

impl Manifest {
    /// read manifest from toml file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Manifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    /// write manifest to toml file
    pub fn to_file(&self, path: &str) -> Result<()> {
        let content = toml::to_string(&self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// test manifest from_file
    #[test]
    fn from_file() {
        let manifest = super::Manifest::from_file("../../tests/data/manifest.toml").unwrap();
        assert_eq!(manifest.manifest, "v1");
        assert_eq!(manifest.name, "rust-basic");
        assert_eq!(manifest.description, "example rust project");
        assert_eq!(manifest.authors, vec!["leaf"]);
        assert_eq!(manifest.language, "rust");
        assert_eq!(
            manifest.build.as_ref().unwrap().rust_target_dir,
            Some("./target".to_string())
        );
        assert_eq!(
            manifest.build.as_ref().unwrap().rust_enable_wasi,
            Some(true)
        );
    }

    /// test manifest to file
    #[test]
    fn to_file() {
        let manifest = super::Manifest::from_file("../../tests/data/manifest.toml").unwrap();
        manifest.to_file("../../tests/data/manifest2.toml").unwrap();
        let manifest2 = super::Manifest::from_file("../../tests/data/manifest2.toml").unwrap();
        assert_eq!(manifest.manifest, manifest2.manifest);
        assert_eq!(manifest.name, manifest2.name);
        assert_eq!(manifest.description, manifest2.description);
        assert_eq!(manifest.authors, manifest2.authors);
        assert_eq!(manifest.language, manifest2.language);
        assert_eq!(
            manifest.build.as_ref().unwrap().rust_target_dir,
            manifest2.build.as_ref().unwrap().rust_target_dir
        );
        assert_eq!(
            manifest.build.as_ref().unwrap().rust_enable_wasi,
            manifest2.build.as_ref().unwrap().rust_enable_wasi
        );

        std::fs::remove_file("../../tests/data/manifest2.toml").unwrap();
    }
}
