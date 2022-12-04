use clap::Args;

mod new;
pub use new::Manifest;
pub use new::NewCommand;

mod up;
pub use up::UpCommand;

#[derive(Args, Debug)]
pub struct CompileCommand {
    /// Set output filename
    #[clap(long)]
    pub output: Option<String>,
}
