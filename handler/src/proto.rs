pub mod core {
    pub mod v1 {
        tonic::include_proto!("core.v1");
    }
}

pub mod rerank {
    pub mod v1 {
        tonic::include_proto!("rerank.v1");
    }
}
