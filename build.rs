use cc::Build;
use std::process::Command;

fn main() {
    // Assemble eadk.s
    println!("cargo:rerun-if-changed=eadk/eadk.s");
    Build::new().file("eadk/eadk.s").compile("asm");

    // Turn icon.png into a linker-embeddable file
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = Command::new("./eadk/inliner.py")
        .args(&["src/icon.png", "target/icon.ld"])
        .output().expect("Failure to launch process");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
