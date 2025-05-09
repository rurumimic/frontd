use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = out_dir.join("descriptor.bin");

    tonic_build::configure()
        .out_dir(&out_dir) // or "src/pb"
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(
            &[
                "../proto/core/v1/meta.proto",
                "../proto/rerank/v1/rerank.proto",
            ],
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}
