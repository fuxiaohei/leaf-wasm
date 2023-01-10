use crate::common::embed::TemplatesAsset;
use crate::common::manifest::Manifest;
use crate::common::vars::*;
use crate::server;
use clap::Args;
use log::{debug, error, info};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct Init {
    /// The name of the project
    pub name: String,
    /// The template to use
    #[clap(long, default_value("hello-rust"))]
    pub template: Option<String>,
}

impl Init {
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
        if self.create_project(&self.name, self.template.as_ref().unwrap().as_str()) {
            info!("Created project: {}", &self.name);
        } else {
            error!("Create project failed");
        }
    }
    fn create_project(&self, name: &str, template: &str) -> bool {
        // copy cargo.toml
        let cargotoml_path = Path::new(template).join("Cargo.toml.tpl");
        debug!("[New] use cargo.toml path: {:?}", cargotoml_path);

        if let Some(c) = TemplatesAsset::get(cargotoml_path.to_str().unwrap()) {
            let mut cargotoml_content = std::str::from_utf8(&c.data).unwrap().to_string();
            cargotoml_content = cargotoml_content.replace("{{name}}", name);
            let cargotoml = Path::new(name).join("Cargo.toml");
            std::fs::write(cargotoml, cargotoml_content).unwrap();
        };

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
                std::fs::create_dir_all(target_path.parent().unwrap()).unwrap();
                std::fs::write(target_path, content).unwrap();
            }
        });

        true
    }
}

#[derive(Args, Debug)]
pub struct Build {
    /// Set optimization progress
    #[clap(long, default_value("false"))]
    pub optimize: bool,
    /// Set compiling debug mode
    #[clap(long, default_value("false"))]
    pub debug: bool,
    /// Set js engine wasm file
    #[clap(long)]
    pub js_engine: Option<String>,
}

impl Build {
    pub async fn run(&self) {
        // load manifest file
        let manifest_file = DEFAULT_MANIFEST_FILE;
        let manifest = match Manifest::from_file(manifest_file) {
            Ok(manifest) => manifest,
            Err(e) => {
                error!("read manifest file error: {}", e);
                return;
            }
        };
        info!("Read manifest '{:?}'", manifest_file);
        match self.build(&manifest) {
            Ok(_) => info!("Compile success"),
            Err(e) => error!("Compile failed: {}", e),
        }
    }

    fn build(&self, manifest: &Manifest) -> anyhow::Result<()> {
        if manifest.language == PROJECT_LANGUAGE_RUST {
            return leaf_compiler::compile_rust(
                manifest.compile_arch().unwrap(),
                manifest.compiling_target().unwrap(),
                self.optimize,
                self.debug,
            );
        }
        if manifest.language == PROJECT_LANGUAGE_JS {
            return leaf_compiler::compile_js(
                manifest.compiling_target().unwrap(),
                "src/index.js".to_string(),
                self.js_engine.clone(),
            );
        }
        anyhow::bail!("Unsupported language: {}", manifest.language)
    }
}

#[derive(Args, Debug)]
pub struct Serve {
    /// The port to listen on
    #[clap(long, default_value("0.0.0.0:18899"))]
    pub addr: Option<SocketAddr>,
    /// The wasm file to run, ignore current project
    #[clap(long)]
    pub wasm: Option<String>,
    /// Enable wasi
    #[clap(long, default_value("false"))]
    pub enable_wasi: bool,
}

impl Serve {
    pub async fn run(&self) {
        let mut enable_wasi = self.enable_wasi;
        let wasm_file = if self.wasm.is_some() {
            info!(
                "[Main] use wasm file from command line: {:?}",
                self.wasm.as_ref().unwrap()
            );
            self.wasm.clone().unwrap()
        } else {
            // load manifest file
            let manifest_file = DEFAULT_MANIFEST_FILE;
            let manifest = match Manifest::from_file(manifest_file) {
                Ok(manifest) => manifest,
                Err(e) => {
                    error!("read manifest file error: {}", e);
                    return;
                }
            };
            info!("[Main] read manifest '{:?}'", manifest_file);

            if !enable_wasi {
                enable_wasi = match manifest.is_enable_wasi() {
                    Ok(enable_wasi) => {
                        info!("[Main] enable wasm32-wasi");
                        enable_wasi
                    }
                    Err(e) => {
                        error!("determine enable_wasi error: {}", e);
                        return;
                    }
                };
            }

            match manifest.final_target() {
                Ok(file) => file,
                Err(e) => {
                    error!("[Main] find wasm error: {}", e);
                    return;
                }
            }
        };

        if !std::path::PathBuf::from(&wasm_file).exists() {
            error!("[Worker] file not found: {}", &wasm_file);
            info!("[Worker] Try to run 'leaf-cli compile' to build wasm file");
            return;
        }
        info!("[Worker] use file: {}", &wasm_file);

        // start local server
        server::start(self.addr.unwrap(), wasm_file, enable_wasi).await;
    }
}

#[derive(Args, Debug)]
pub struct Component {
    /// Input file
    pub input: String,
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
}

impl Component {
    pub async fn run(&self) {
        if !std::path::PathBuf::from(&self.input).exists() {
            error!("File not found: {}", &self.input);
            return;
        }
        let output = self
            .output
            .clone()
            .unwrap_or_else(|| "component.wasm".to_string());
        leaf_compiler::encode_wasm_component(&self.input, Some(output));
    }
}
