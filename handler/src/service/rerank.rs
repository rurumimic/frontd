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

        let results = frontd_inference::rerank::Rerank::new(request.query, request.documents)
            .rerank_documents()
            .iter()
            .map(|doc| RerankResult {
                index: doc.index as u32,
                score: doc.score,
            })
            .collect::<Vec<RerankResult>>();

        let response = RerankResponse {
            meta: request.meta.clone(),
            results,
            error: String::new(),
        };

        Ok(Response::new(response))
    }
}
