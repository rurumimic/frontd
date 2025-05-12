use frontd_handler::proto::core::v1::RequestMeta;
use frontd_handler::proto::rerank::v1::rerank_client::RerankClient;
use frontd_handler::proto::rerank::v1::RerankRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RerankClient::connect("http://[::1]:50051").await?;

    // simple RPC
    let response = client
        .rank_documents(RerankRequest {
            meta: Some(RequestMeta {
                request_id: "1234567890".to_string(),
                model: "rerank-v1".to_string(),
            }),
            query: "hello world".to_string(),
            documents: vec![
                "hello world".to_string(),
                "hello world 2".to_string(),
                "hello world 3".to_string(),
            ],
        })
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
