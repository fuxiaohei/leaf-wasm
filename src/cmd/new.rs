use crate::errors::Error;
use clap::Args;
use log::debug;
use serde_derive::Deserialize;

#[derive(Args, Debug)]
pub struct NewCommand {
    /// The name of the project
    #[clap(long)]
    pub name: Option<String>,
    /// The template to use
    #[clap(long, default_value("hello-world"))]
    pub template: Option<String>,
}

impl NewCommand {
    pub async fn run(&self) {
        // check manifest file exist
        debug!("New command: run {:?}", self);
    }
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub manifest: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub language: String,
}

impl Manifest {
    /// from_file reads the manifest file and returns a Manifest struct
    pub fn from_file(file: &str) -> Result<Manifest, Error> {
        let content = std::fs::read_to_string(file).map_err(Error::ReadManifestFile)?;
        let manifest: Manifest =
            toml::from_str(&content).map_err(Error::UnmarshalManifestToml)?;
        Ok(manifest)
    }
}
