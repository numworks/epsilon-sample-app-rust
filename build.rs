use std::process::Command;
use cc::Build;


fn main() {

    Build::new().file("src/svc.s").compile("asm"); // <- NEW!

    // rebuild if `asm.s` changed
    println!("cargo:rerun-if-changed=src/svc.s"); // <- NEW!
    




    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    let hash = match output {
        Ok(o) => String::from_utf8(o.stdout).unwrap(),
        Err(_) => String::from(""),
    };
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", hash);

    let output = Command::new("git")
        .args(&["log", "--pretty=format:'%ad'", "-n1", "--date=short"])
        .output();
    let date = match output {
        Ok(o) => String::from_utf8(o.stdout)
            .unwrap()
            .trim_matches('\'')
            .to_string(),
        Err(_) => String::from(""),
    };
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", date);
}
