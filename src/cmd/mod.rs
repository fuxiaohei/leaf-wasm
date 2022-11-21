use crate::server;
use clap::Args;
use std::net::SocketAddr;

#[derive(Args, Debug)]
pub struct NewCommand {
    /// The name of the project
    #[clap(long)]
    pub name: Option<String>,
    /// The template to use
    #[clap(long, default_value("hello-world"))]
    pub template: Option<String>,
}

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpCommand {
    // The port to listen on
    #[clap(long, default_value("0.0.0.0:18899"))]
    pub addr: Option<SocketAddr>,
}

impl UpCommand {
    pub async fn run(&self) {
        server::start(self.addr.unwrap()).await;
    }
}
