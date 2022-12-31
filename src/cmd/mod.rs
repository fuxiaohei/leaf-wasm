pub mod cli;

use lazy_static::lazy_static;

/// build_info returns the version information of the current build.
pub fn build_info() -> String {
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

// get_version returns the version of the current build.
pub fn get_version() -> &'static str {
    &VERSION
}

// init_tracing initializes the tracing subscriber.
pub fn init_tracing() {
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
