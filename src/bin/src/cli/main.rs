mod flags;
mod server;

use clap::Parser;
use flags::LeafCli;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    init_tracing();

    let args = LeafCli::parse();
    match args {
        LeafCli::Init(cmd) => cmd.run().await,
        LeafCli::Build(cmd) => cmd.run().await,
        LeafCli::Serve(cmd) => cmd.run().await,
    }
}

fn init_tracing() {
    if std::env::var("RUST_LOG").ok().is_none() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "debug")
        } else {
            std::env::set_var("RUST_LOG", "info")
        }
    }

    let timer = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
        )
        .unwrap(),
    );

    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();
}
