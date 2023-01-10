use clap::Args;
use std::net::SocketAddr;
use tracing::debug;

#[derive(Args, Debug)]
pub struct Init {
    /// The name of the project
    pub name: String,
    /// The template to use
    #[clap(long, default_value("hello-rust"))]
    pub template: Option<String>,
}

impl Init {
    pub async fn run(&self) {
        tracing::debug!("New command: run {:?}", self);
    }
}

#[derive(Args, Debug)]
pub struct Build {
    /// Set optimization progress
    #[clap(long, default_value("false"))]
    pub enable_optimize: bool,
}

impl Build {
    pub async fn run(&self) {
        debug!("Compile command: run {:?}", self);
    }
}

#[derive(Args, Debug)]
pub struct Serve {
    /// The port to listen on
    #[clap(long, default_value("0.0.0.0:18899"))]
    pub addr: Option<SocketAddr>,
}

impl Serve {
    pub async fn run(&self) {
        debug!("[Command] serve: {:?}", self);
    }
}
