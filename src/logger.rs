use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init_logger() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true))
        .try_init();
}
