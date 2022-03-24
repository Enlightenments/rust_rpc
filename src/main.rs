use grpc_lib::server;

#[tokio::main]
async fn main() {
    server(([127, 0, 0, 1],19999).into()).await;
}

