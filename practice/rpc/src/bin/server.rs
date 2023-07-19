extern crate rpc;

use std::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};
use rpc::user::user_service_server::{UserService, UserServiceServer};
use rpc::user::{SetNameRequest, SetNameResponse, GetNameRequest, GetNameResponse};


#[derive(Default)]
pub struct MyUserService {
    name: Mutex<String>,
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn set_name(
        &self,
        request: Request<SetNameRequest>,
    ) -> Result<Response<SetNameResponse>, Status> {
        let name = request.into_inner().name;
        let mut name_guard = self.name.lock().unwrap();
        *name_guard = name.clone();
        let reply = SetNameResponse {
            message: format!("Name set to {}", name),
        };
        Ok(Response::new(reply))
    }

    async fn get_name(
        &self,
        _request: Request<GetNameRequest>,
    ) -> Result<Response<GetNameResponse>, Status> {
        let name_guard = self.name.lock().unwrap();
        let reply = GetNameResponse {
            name: name_guard.clone(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user_service = MyUserService::default();

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
