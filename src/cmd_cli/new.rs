use crate::common::embed::TemplatesAsset;
use crate::common::errors::Error;
use crate::common::vars::*;
use anyhow::Result;
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
            if template.contains("js") {
                return String::from("js");
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
        if create_project(&self.name, self.template.as_ref().unwrap().as_str()) {
            info!("Created project: {}", &self.name);
        } else {
            error!("Create project failed");
        }
    }
}

fn create_project(name: &str, template: &str) -> bool {
    // copy cargo.toml
    let cargotoml_path = Path::new(template).join("Cargo.toml.tpl");
    debug!("[New] use cargo.toml path: {:?}", cargotoml_path);
    TemplatesAsset::get(cargotoml_path.to_str().unwrap()).map(|c| {
        let mut cargotoml_content = std::str::from_utf8(&c.data).unwrap().to_string();
        cargotoml_content = cargotoml_content.replace("{{name}}", name);
        let cargotoml = Path::new(name).join("Cargo.toml");
        std::fs::write(cargotoml, cargotoml_content).unwrap();
    });

    // create src dir
    let librs_target = Path::new(name).join("src");
    std::fs::create_dir_all(librs_target.parent().unwrap()).unwrap();

    // copy src files
    let librs_path = Path::new(template).join("src");
    TemplatesAsset::iter().for_each(|t| {
        if t.starts_with(librs_path.to_str().unwrap()) {
            let src_path = Path::new(t.as_ref())
                .strip_prefix(librs_path.to_str().unwrap())
                .unwrap();
            let file = TemplatesAsset::get(t.as_ref()).unwrap();
            let content = std::str::from_utf8(&file.data).unwrap().to_string();
            let target_path = librs_target.join(src_path);
            debug!("[New] src_path: {:?}, {:?}", src_path, target_path);
            std::fs::write(target_path, content).unwrap();
        }
    });

    true
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

    /// determine compiling target file, before optimized
    pub fn determine_compiling_target(&self) -> Result<String> {
        let name = self.name.replace('-', "_");
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(format!("{}/{}.wasm", RUST_TARGET_WASM_RELEASE_DIR, name)),
            PROJECT_LANGUAGE_JS => Ok(format!("{}.wasm", name)),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine optimized target file
    pub fn determine_optimized_target(&self) -> Result<String> {
        let name = self.name.replace('-', "_");
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(format!("{}/{}.wasm", RUST_TARGET_WASM_RELEASE_DIR, name)),
            PROJECT_LANGUAGE_JS => Ok(format!("{}_wizer.wasm", name)),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }

    /// determine compile arch
    pub fn determine_compile_arch(&self) -> Result<String> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(COMPILE_TARGET_WASM32_UNKNOWN_UNKNOWN.to_string()),
            PROJECT_LANGUAGE_JS => Ok(COMPILE_TARGET_WASM32_WASI.to_string()),
            _ => Err(anyhow::Error::msg("unknown compile target")),
        }
    }

    /// determine enable wasi
    pub fn determine_enable_wasi(&self) -> Result<bool> {
        match self.language.as_str() {
            PROJECT_LANGUAGE_RUST => Ok(false),
            PROJECT_LANGUAGE_JS => Ok(true),
            _ => Err(anyhow::Error::msg("unknown language")),
        }
    }
}
