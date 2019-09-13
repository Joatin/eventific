use std::process::Command;
use walkdir::WalkDir;
extern crate protoc_rust_grpc;

fn main() {
    {
        let proto_root = "examples/proto";
        let proto_out = "examples/proto";
        println!("cargo:rerun-if-changed={}", proto_root);

        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: proto_out,
            includes: &[proto_root],
            input: &["examples/proto/service.proto"],
            rust_protobuf: true, // also generate protobuf messages, not just services
            ..Default::default()
        }).expect("protoc-rust-grpc");

        // protoc_grpcio::compile_grpc_protos(&["service.proto"], &[proto_root], &proto_out).unwrap();
    }
    {
        let proto_root = "tests/proto";
        let proto_out = "tests/proto";
        println!("cargo:rerun-if-changed={}", proto_root);

        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: proto_out,
            includes: &[proto_root],
            input: &["tests/proto/service.proto"],
            rust_protobuf: true, // also generate protobuf messages, not just services
            ..Default::default()
        }).expect("protoc-rust-grpc");
    }
    {
        let proto_root = "proto";
        let proto_out = "src/grpc";
        println!("cargo:rerun-if-changed={}", proto_root);

        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: proto_out,
            includes: &[proto_root],
            input: &["proto/health.proto"],
            rust_protobuf: true, // also generate protobuf messages, not just services
            ..Default::default()
        }).expect("protoc-rust-grpc");
    }
}
