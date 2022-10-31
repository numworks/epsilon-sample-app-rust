use std::process::Command;

fn main() {
    // Turn icon.png into icon.nwi
    println!("cargo:rerun-if-changed=src/icon.png");
    let output = Command::new("npm")
        .args(&[
            "exec",
            "--yes",
            "--",
            "nwlink",
            "png-nwi",
            "src/icon.png",
            "target/icon.nwi",
        ])
        .output()
        .expect("Failure to launch process");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
