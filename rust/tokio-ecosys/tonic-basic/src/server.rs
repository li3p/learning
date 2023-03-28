use tonic::{transport::Server, Request, Response, Status};

use gretter::greeter2_server::{Greeter2, Greeter2Server};
use gretter::greeter_server::{Greeter, GreeterServer};
use gretter::{HelloReply, HelloReply2, HelloRequest, HelloRequest2};

pub mod gretter {
    tonic::include_proto!("helloworld");
}

pub mod types {
    tonic::include_proto!("types");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = gretter::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl Greeter2 for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest2>,
    ) -> Result<Response<HelloReply2>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = gretter::HelloReply2 {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let greeter2 = MyGreeter::default();
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(Greeter2Server::new(greeter2))
        .serve(addr)
        .await?;

    Ok(())
}
