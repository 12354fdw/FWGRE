use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*, registry};

pub fn init() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    registry().with(fmt::layer()).with(filter_layer).init();
}
