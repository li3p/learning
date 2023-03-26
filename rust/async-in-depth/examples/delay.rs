// This example shows how to implement a future that will resolve after a
// certain amount of time has passed.
// https://tokio.rs/tokio/tutorial/async
use std::{future::Future, time::Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Done waiting!");
            std::task::Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + std::time::Duration::from_secs(1);
    let delay = Delay { when };

    println!("Waiting for the delay to finish...");
    let out = delay.await;
    assert_eq!(out, "done");
}
