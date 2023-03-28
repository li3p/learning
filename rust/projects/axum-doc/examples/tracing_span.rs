use tracing::{info, instrument, span};
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

#[instrument]
fn some_operation() {
    let span = span!(tracing::Level::INFO, "inner_span");
    let _enter = span.enter();

    info!("This is an event inside the inner span");
}

fn main() {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let span = span!(tracing::Level::INFO, "outer_span");
    let _enter = span.enter();

    info!("This is an event inside the outer span");

    some_operation();
}
