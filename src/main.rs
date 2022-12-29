use clap::Parser;
use lazy_static::lazy_static;
mod cmd;
mod embed;
mod errors;
mod server;
mod vars;
mod wasm;
mod wit;

fn build_info() -> String {
    format!(
        "{} ({} {})",
        env!("VERGEN_BUILD_SEMVER"),
        env!("VERGEN_GIT_SHA_SHORT"),
        env!("VERGEN_GIT_COMMIT_DATE")
    )
}

lazy_static! {
    pub static ref VERSION: String = build_info();
}

fn get_version() -> &'static str {
    &VERSION
}

fn init_tracing() {
    use tracing_subscriber::fmt::time::OffsetTime;

    let timer = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
        )
        .unwrap(),
    );

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(timer)
        .with_target(false)
        .init();
}

/// Leaf Command line
#[derive(Parser)]
#[clap(
    name = "leaf-cli",
    version = get_version(),
)]
enum LeafCli {
    /// New creates a new leaf project
    New(cmd::NewCommand),
    /// Compile compiles the leaf project
    Compile(cmd::CompileCommand),
    /// Up runs the leaf project
    Up(cmd::UpCommand),
}

#[tokio::main]
async fn main() {
    init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::New(cmd) => cmd.run().await,
        LeafCli::Compile(cmd) => cmd.run().await,
        LeafCli::Up(cmd) => cmd.run().await,
    }
}
