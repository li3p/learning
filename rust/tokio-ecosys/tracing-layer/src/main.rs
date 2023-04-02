use hyper::{Body, Request, Response};
use std::net::SocketAddr;
use tower::{Service, ServiceBuilder};
use tower_http::{trace::DefaultMakeSpan, trace::MakeSpan, trace::TraceLayer};
use tracing::{info, Span};
use tracing_futures::Instrument;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // your request handling logic here
    Ok(Response::new(Body::from("Hello, World!")))
}

fn make_span(req: &Request<Body>) -> Span {
    tracing::info_span!("http.request", method = %req.method(), uri = %req.uri())
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let service = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http().make_span(make_span as DefaultMakeSpan))
        .service_fn(handle_request);

    let server = hyper::Server::bind(&addr).serve(service);

    info!("Listening on http://{}", addr);

    server
        .instrument(info_span!("http.server", addr = %addr))
        .await
        .unwrap();
}
