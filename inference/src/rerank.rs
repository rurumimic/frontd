pub struct Rerank {
    query: String,
    documents: Vec<String>,
}

pub struct RerankResult {
    pub index: usize,
    pub score: f32,
}

impl Rerank {
    pub fn new(query: String, documents: Vec<String>) -> Self {
        Rerank { query, documents }
    }

    pub fn rerank_documents(&self) -> Vec<RerankResult> {
        let mut results = Vec::new();
        for (index, _document) in self.documents.iter().enumerate() {
            let score = rand::random::<f32>();
            results.push(RerankResult { index, score });
        }
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }
}
