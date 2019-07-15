use std::process::Command;
use walkdir::WalkDir;

fn main() {
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
