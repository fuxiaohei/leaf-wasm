use super::Manifest;
use crate::common::vars::*;
use clap::Args;
use log::{error, info};

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
    /// Set optimization progress
    #[clap(long, default_value("false"))]
    pub optimize: bool,
    /// Set compiling debug mode
    #[clap(long, default_value("false"))]
    pub debug: bool,
}

impl CompileCommand {
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
        match self.do_compile(&manifest) {
            Ok(_) => info!("Compile success"),
            Err(e) => error!("Compile failed: {}", e),
        }
    }

    fn do_compile(&self, manifest: &Manifest) -> anyhow::Result<()> {
        if manifest.language == PROJECT_LANGUAGE_RUST {
            return leaf_codegen::compile::compile_rust(
                manifest.determine_compile_arch().unwrap(),
                manifest.determine_compiling_target().unwrap(),
                self.optimize,
                self.debug,
            );
        }
        if manifest.language == PROJECT_LANGUAGE_JS {
            return leaf_codegen::compile::compile_js(
                manifest.determine_compiling_target().unwrap(),
                "src/index.js".to_string(),
            );
        }
        anyhow::bail!("Unsupported language: {}", manifest.language)
    }
}
