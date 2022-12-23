use crate::errors::Error;
use crate::vars::DEFAULT_MANIFEST_FILE;
use clap::Args;
use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct NewCommand {
    /// The name of the project
    pub name: String,
    /// The template to use
    #[clap(long, default_value("hello-rust"))]
    pub template: Option<String>,
}

impl NewCommand {
    fn determine_language(&self) -> String {
        if self.template.is_some() {
            let template = self.template.as_ref().unwrap();
            if template.contains("rust") {
                return String::from("rust");
            }
        }
        String::from("rust")
    }
    pub async fn run(&self) {
        // check manifest file exist
        debug!("New command: run {:?}", self);

        // create dir by name
        if !Path::new(&self.name).exists() {
            std::fs::create_dir(&self.name).unwrap();
            info!("Created dir: {}", &self.name)
        }

        let dpath = PathBuf::from(&self.name);

        // create leaf.toml
        let manifest = Manifest {
            name: self.name.clone(),
            language: self.determine_language(),
            ..Default::default()
        };
        let manifest_file = dpath.join(DEFAULT_MANIFEST_FILE);
        if Path::new(manifest_file.to_str().unwrap()).exists() {
            error!(
                "manifest file already exist: {}",
                manifest_file.to_str().unwrap()
            );
            //return;
        }
        manifest.to_file(manifest_file.to_str().unwrap()).unwrap();
        info!("Created manifest: {}", manifest_file.to_str().unwrap());

        // create sample project
        create_project(&self.name, self.template.as_ref().unwrap().as_str());
    }
}

fn create_project(name: &str, template: &str) {
    let cargotoml_content = include_str!("../../etc/sample-rust/Cargo.toml.tpl");
    let cargotoml_content = cargotoml_content.replace("{{name}}", name);
    let cargotoml = Path::new(name).join("Cargo.toml");
    std::fs::write(cargotoml, cargotoml_content).unwrap();

    let mut code_content = "";
    if template == "hello-rust" {
        code_content = include_str!("../../etc/sample-rust/http-hello.rs");
    } else if template == "fetch-rust" {
        code_content = include_str!("../../etc/sample-rust/http-fetch-hello.rs");
    }
    let codefile = Path::new(name).join("src/lib.rs");
    std::fs::create_dir_all(codefile.parent().unwrap()).unwrap();
    std::fs::write(codefile, code_content).unwrap();
}

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
}
