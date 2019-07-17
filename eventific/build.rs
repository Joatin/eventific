use std::process::Command;
use walkdir::WalkDir;
extern crate protoc_grpcio;

fn main() {
    {
        let proto_root = "examples/proto";
        let proto_out = "examples/proto";
        println!("cargo:rerun-if-changed={}", proto_root);

        protoc_grpcio::compile_grpc_protos(&["service.proto"], &[proto_root], &proto_out).unwrap();
    }
    {
        let proto_root = "tests/proto";
        let proto_out = "tests/proto";
        println!("cargo:rerun-if-changed={}", proto_root);

        protoc_grpcio::compile_grpc_protos(&["service.proto"], &[proto_root], &proto_out).unwrap();
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
