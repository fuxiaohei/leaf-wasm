use clap::Args;

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
}

impl CompileCommand {
    pub async fn run(&self) {
        println!("Compile command: {:?}", self);
    }
}
