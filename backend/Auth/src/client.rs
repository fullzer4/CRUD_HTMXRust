use hello::hello_client::HelloClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut client = HelloClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        HelloRequest {
            hello: "funcionou".to_owned()
        }
    );

    let response = client.send_hello(request).await?;

    println!("INFO: response - {:?}", response);

    Ok(())
}
