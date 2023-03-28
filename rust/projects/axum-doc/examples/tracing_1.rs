use tracing::{event, instrument, span, Level};
use tracing_subscriber::fmt;

#[instrument]
pub fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function` with the
    // field `my_arg`.
    event!(Level::INFO, "inside my_function!");
    // ...
}

#[derive(Debug)]
struct User {
    name: &'static str,
    email: &'static str,
}

fn main() {
    fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::TRACE)
        // .event_format(
        //     fmt::format()
        //         .with_target(true)
        //         .with_thread_names(false)
        //         .with_thread_ids(true),
        // )
        .init();
    my_function(250);

    span!(target: "app_spans", Level::TRACE, "my_span_a");
    event!(target: "app_events", Level::INFO, "something has happened!");

    let span = span!(Level::TRACE, "my_span_b");
    let _enter = span.enter();
    event!(
        parent: &span,
        Level::INFO,
        "something has happened in parent span!"
    );

    event!(
        parent: &span,
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything",
        "something has happened in parent span!"
    );

    //-------------------------------------------------------------------------------
    let user = User {
        name: "ferris",
        email: "ferris@rust-lang.org",
    };
    // the span will have the fields `user.name = "ferris"` and
    // `user.email = "ferris@rust-lang.org"`.
    span!(Level::TRACE, "login", user.name, user.email);
    event!(
        parent: &span,
        Level::INFO,
        "something has happened in fields span!"
    );

    //--------------------------------------------------------------------------------
    let user = "ferris";

    span!(Level::TRACE, "login", user);
    // is equivalent to:
    span!(Level::TRACE, "login", user = user);
}
