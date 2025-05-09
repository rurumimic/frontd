use tonic::{Request, Response, Status};

use crate::proto::core::v1::RequestMeta;
use crate::proto::rerank::v1::rerank_server::Rerank;
use crate::proto::rerank::v1::Result as RerankResult;
use crate::proto::rerank::v1::{RerankRequest, RerankResponse};

#[derive(Debug)]
pub struct RerankService {}

#[tonic::async_trait]
impl Rerank for RerankService {
    async fn rank_documents(
        &self,
        request: Request<RerankRequest>,
    ) -> Result<Response<RerankResponse>, Status> {
        let request = request.into_inner();
        println!("Received request: {:?}", request);

        let results = vec![
            RerankResult {
                index: 0,
                score: 0.95,
            },
            RerankResult {
                index: 1,
                score: 0.85,
            },
            RerankResult {
                index: 2,
                score: 0.75,
            },
        ];

        let response = RerankResponse {
            meta: request.meta.clone(),
            results,
            error: String::new(),
        };

        Ok(Response::new(response))
    }
}
