use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accept from {}", addr);

        tokio::spawn(async {
            eprintln!("task 2");
        });

        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());

            let (mut w, mut r) = framed.split();

            while let Some(line) = r.next().await {
                w.send(format!(">> {}", line?)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
