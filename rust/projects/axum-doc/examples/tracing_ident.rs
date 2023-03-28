use std::io;
use std::io::Write;
use tracing::{info, instrument, span};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::Layer as FmtLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::Context, Layer};

#[instrument]
fn some_operation() {
    let span = span!(tracing::Level::INFO, "inner_span");
    let _enter = span.enter();

    info!("This is an event inside the inner span");
}

struct IndentLayer<S> {
    fmt: FmtLayer<S>,
}

impl<S> Layer<S> for IndentLayer<S>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let span_count = ctx
            .event_span(event)
            .and_then(|span| {
                let id = span.id();
                let mut count = 0;
                let mut current = Some(id.clone());
                while let Some(parent) = current.and_then(|id| ctx.span(&id).unwrap().parent()) {
                    count += 1;
                    current = Some(parent.id());
                }
                Some(count)
            })
            .unwrap_or(0);

        let indent = "  ".repeat(span_count);

        let mut writer = io::stderr();
        write!(writer, "{}", indent).unwrap();
        self.fmt.on_event(event, ctx);
    }
}

fn main() {
    let fmt_layer = fmt::Layer::default();
    let indent_layer = IndentLayer { fmt: fmt_layer };

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(indent_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let span = span!(tracing::Level::INFO, "outer_span");
    let _enter = span.enter();

    info!("This is an event inside the outer span");

    some_operation();
}
