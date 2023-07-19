extern crate rpc;

use tonic::{transport::Channel, Request, Status};
use rpc::user::user_service_client::UserServiceClient;
use rpc::user::{SetNameRequest, GetNameRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = UserServiceClient::new(channel);

    let request = tonic::Request::new(SetNameRequest {
        name: "Alice".into(),
    });

    let response = client.set_name(request).await?.into_inner();

    println!("Response: {}", response.message);

    let request = Request::new(GetNameRequest {});

    let response = client.get_name(request).await?.into_inner();

    println!("Response: {}", response.name);

    Ok(())
}
