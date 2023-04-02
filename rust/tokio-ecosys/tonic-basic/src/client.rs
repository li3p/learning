use gretter_cli::greeter2_client::Greeter2Client;
use gretter_cli::HelloRequest2;

pub mod gretter_cli {
    tonic::include_proto!("helloworld");
}

pub mod types {
    tonic::include_proto!("types");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Greeter2Client::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest2 {
        name: "Tonic".into(),
        enum_type: types::EnumType::A as i32,
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
