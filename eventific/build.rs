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

    for entry in WalkDir::new("playground").into_iter().filter_map(|e| e.ok()) {
        if !entry.path().display().to_string().contains("node_modules") && !entry.path().display().to_string().contains("build") {
            println!("{}", entry.path().display());
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    let status = Command::new("yarn")
        .current_dir("./playground")
        .args(&["install"])
        .status()
        .expect("failed to execute process");

    if !status.success() {
        panic!();
    }

    let status2 = Command::new("yarn")
        .current_dir("./playground")
        .args(&["build"])
        .status()
        .expect("failed to execute process");

    if !status2.success() {
        panic!();
    }
}
