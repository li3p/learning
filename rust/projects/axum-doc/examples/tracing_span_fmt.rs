// use std::io::{self, Write};
// use tracing::{Level, Span};
// use tracing_subscriber::fmt::{format::FmtSpan, Subscriber};
// use tracing_subscriber::prelude::*;

// fn main() {
//     tracing_subscriber::fmt()
//         .event_format(move |buf, event| {
//             let span = event.parent().unwrap_or_else(|| Span::current());

//             writeln!(
//                 buf,
//                 "{} {} {}",
//                 event.metadata().level(),
//                 &span.name(),
//                 event.metadata().target()
//             )
//         })
//         .init();

//     tracing::debug!("This is a debug message");
//     tracing::info!("This is an info message");
//     tracing::warn!("This is a warning message");
//     tracing::error!("This is an error message");
// }

fn main() {}
