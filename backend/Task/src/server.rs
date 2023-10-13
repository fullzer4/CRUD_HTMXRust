use tonic::{transport::Server, Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloResponse, HelloRequest};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn send_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("INFO: Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = HelloResponse {
            successful: true,
            message: format!("Hello {}", req.hello).into(), 
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hello_service = HelloService::default();

    Server::builder()
        .add_service(HelloServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(()) 
}
