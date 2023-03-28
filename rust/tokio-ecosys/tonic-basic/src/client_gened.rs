mod pb;

use pb::helloworld::greeter2_client::Greeter2Client;
use pb::helloworld::HelloRequest2;

// pub mod gretter_cli {
//     tonic::include_proto!("helloworld");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Greeter2Client::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest2 {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
