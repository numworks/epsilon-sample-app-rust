use cc::Build;
use std::process::Command;

fn main() {
    Build::new().file("eadk/eadk.s").compile("asm");
    println!("cargo:rerun-if-changed=eadk/eadk.s");
    Command::new("python3")
        .args(&["eadk/inliner.py", "src/icon.png", "target/icon.ld"])
        .output()
        .expect("failed to process icon");
    println!("cargo:rerun-if-changed=src/icon.png");
}
