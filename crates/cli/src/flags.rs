use super::embed::TemplateAssets;
use super::local_server;
use anyhow::Result;
use clap::Args;
use leaf_common::vars::*;
use leaf_common::Manifest;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};

#[derive(Args, Debug)]
pub struct Init {
    /// The name of the project
    pub name: String,
    /// The template to use
    #[clap(long, default_value("rust-basic"))]
    pub template: Option<String>,
}

impl Init {
    fn determine_language(&self) -> String {
        if self.template.is_some() {
            let template = self.template.as_ref().unwrap();
            if template.starts_with("rust-") {
                return String::from("rust");
            }
            if template.starts_with("js-") {
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
            description: "leaf wasm project with template ".to_owned()
                + self.template.as_ref().unwrap().as_str(),
            ..Default::default()
        };
        let manifest_file = dpath.join(DEFAULT_MANIFEST_FILE);
        if Path::new(manifest_file.to_str().unwrap()).exists() {
            warn!(
                "Manifest file already exist: {}",
                manifest_file.to_str().unwrap()
            );
            //return;
        }
        manifest.to_file(manifest_file.to_str().unwrap()).unwrap();
        info!("Created manifest: {}", manifest_file.to_str().unwrap());

        match self.create_project(&self.name, self.template.as_ref().unwrap().as_str()) {
            Ok(_) => info!("Created project: {}", &self.name),
            Err(e) => panic!("Create project failed: {e}"),
        }
    }

    fn create_project(&self, name: &str, template: &str) -> anyhow::Result<()> {
        // if template is rust, copy cargo.toml
        let toml = Path::new(template).join("Cargo.toml");
        debug!("[New] use cargo.toml path: {:?}", toml);
        if let Some(c) = TemplateAssets::get(toml.to_str().unwrap()) {
            let mut content = std::str::from_utf8(&c.data)?.to_string();
            content = content.replace(template, name);
            content = content.replace(
                "path = \"../../crates/rust-sdk\"",
                "git = \"https://github.com/fuxiaohei/leaf-wasm\"",
            );
            content = content.replace("[build]\ntarget_dir = \"../../target\"", "");
            let target = Path::new(name).join("Cargo.toml");
            std::fs::write(target, content)?;
        };

        // create src dir
        let src_dir = Path::new(name).join("src");
        std::fs::create_dir_all(src_dir.parent().unwrap()).unwrap();

        // copy src files
        let tpl_dir = Path::new(template).join("src");
        TemplateAssets::iter().for_each(|t| {
            if t.starts_with(tpl_dir.to_str().unwrap()) {
                let src_path = Path::new(t.as_ref())
                    .strip_prefix(tpl_dir.to_str().unwrap())
                    .unwrap();
                let file = TemplateAssets::get(t.as_ref()).unwrap();
                let content = std::str::from_utf8(&file.data).unwrap().to_string();
                let target_path = src_dir.join(src_path);
                debug!("[New] src_path: {:?}, {:?}", src_path, target_path);
                std::fs::create_dir_all(target_path.parent().unwrap()).unwrap();
                std::fs::write(target_path, content).unwrap();
            }
        });

        Ok(())
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
                panic!("read manifest file error: {e}");
            }
        };
        info!("Read manifest '{:?}'", manifest_file);
        match self.build(&manifest) {
            Ok(_) => info!("Compile success"),
            Err(e) => panic!("Compile failed: {e}"),
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
                    panic!("read manifest file error: {e}");
                }
            };
            info!("[Main] read manifest '{:?}'", manifest_file);

            if !enable_wasi {
                enable_wasi = match manifest.is_enable_wasi() {
                    Ok(enable_wasi) => enable_wasi,
                    Err(e) => {
                        panic!("determine enable_wasi error: {e}");
                    }
                };
            }
            if enable_wasi {
                info!("[Main] enable wasm32-wasi");
            }

            match manifest.final_target() {
                Ok(file) => file,
                Err(e) => {
                    panic!("[Main] find wasm error: {e}");
                }
            }
        };

        if !std::path::PathBuf::from(&wasm_file).exists() {
            error!("[Worker] file not found: {}", &wasm_file);
            panic!("[Worker] Try to run 'leaf-cli compile' to build wasm file");
        }
        info!("[Worker] use file: {}", &wasm_file);

        // start local server
        local_server::start(self.addr.unwrap(), wasm_file, enable_wasi).await;
    }
}

#[derive(Args, Debug)]
pub struct Component {
    /// Input file
    pub input: String,
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
    /// Set enabled wasi
    #[clap(long, default_value("true"))]
    pub enable_wasi: bool,
}

impl Component {
    pub async fn run(&self) {
        if !std::path::PathBuf::from(&self.input).exists() {
            panic!("File not found: {}", &self.input);
        }
        let output = self
            .output
            .clone()
            .unwrap_or_else(|| "component.wasm".to_string());
        leaf_compiler::encode_wasm_component(&self.input, Some(output), self.enable_wasi);
    }
}

#[derive(Args, Debug)]
pub struct Test {
    /// Set enabled wasi
    #[clap(long, default_value("true"))]
    pub enable_wasi: bool,
}

impl Test {
    pub async fn run(&self) -> Result<()> {
        // load manifest file
        let manifest_file = DEFAULT_MANIFEST_FILE;
        let manifest = match Manifest::from_file(manifest_file) {
            Ok(manifest) => manifest,
            Err(e) => {
                panic!("read manifest file error: {e}");
            }
        };
        info!("[Main] read manifest '{:?}'", manifest_file);

        let enable_wasi = match manifest.is_enable_wasi() {
            Ok(enable_wasi) => enable_wasi,
            Err(e) => {
                panic!("determine enable_wasi error: {e}");
            }
        };

        let wasm_file = match manifest.final_target() {
            Ok(file) => file,
            Err(e) => {
                panic!("[Main] find wasm error: {e}");
            }
        };

        let mut worker = leaf_worker::Worker::new(&wasm_file, enable_wasi).await?;

        let headers: Vec<(&str, &str)> = vec![];
        let req = leaf_host_impl::http::Request {
            request_id: 1,
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("testing".as_bytes()),
        };

        let resp = worker.handle_request(req).await.unwrap();
        println!("resp: {:?}", resp);
        println!(
            "body: {:?}",
            std::str::from_utf8(&resp.body.unwrap()).unwrap()
        );

        Ok(())
    }
}
