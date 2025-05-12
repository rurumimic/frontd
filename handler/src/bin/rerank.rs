use tonic::{transport::Server, Request, Response, Status};

use frontd_handler::proto::rerank::v1::rerank_server::RerankServer;
use frontd_handler::service::rerank::RerankService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let server = Server::builder().add_service(RerankServer::new(RerankService {}));

    server.serve(addr).await?;

    Ok(())
}
