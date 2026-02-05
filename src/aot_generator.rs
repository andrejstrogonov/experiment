use std::fs::{create_dir_all, write};
use std::process::Command;

/// Generate a small Rust binary crate under out_dir 'aot_bin' that implements AOT functions
/// for given hot sequences (seq, helper_name) and runs simple micro-benchmarks.
pub fn generate_and_run_aot(hot: &[(String,String)]) {
    let out_dir = "aot_bin";
    let _ = create_dir_all(format!("{}/src", out_dir));

    // Cargo.toml
    let cargo = r#"[package]
name = "aot_bin"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 3
"#;
    let _ = write(format!("{}/Cargo.toml", out_dir), cargo);

    // main.rs: generate functions and bench harness
    let mut code = String::new();
    code.push_str("use std::time::Instant;\n\n");

    // generate stub functions based on hot sequences
    for (seq, name) in hot {
        code.push_str(&format!("fn {}(iterations: usize) {{\n", name));
        code.push_str("    let mut pos = 0.0f64; let vel = 5.0f64; let dt = 0.016f64;\n");
        code.push_str("    for _ in 0..iterations {\n");
        for token in seq.split(';') {
            let t = token.trim();
            if t == "move" {
                code.push_str("        pos += vel * dt;\n");
            } else if t == "collide" {
                code.push_str("        let _ = pos;\n");
            } else if t.starts_with("takeDamage") {
                code.push_str("        let _d = 10; let _ = _d;\n");
            } else {
                code.push_str("        let _ = 0;\n");
            }
        }
        code.push_str("    }\n}\n\n");
    }

    // bench main
    code.push_str("fn main() {\n    let iters = 10_000_000usize;\n");
    for (_seq, name) in hot {
        code.push_str(&format!("    let t0 = Instant::now(); {}(iters); let d = t0.elapsed(); println!(\"AOT {}: {{:?}}\", d);\n", name, name));
    }
    code.push_str("}\n");

    let _ = write(format!("{}/src/main.rs", out_dir), code);

    // Build release
    println!("Building AOT benchmark...");
    let mut cmd = Command::new("cargo");
    cmd.args(["build","--release","--manifest-path", "aot_bin/Cargo.toml"]);
    if let Ok(status) = cmd.status() {
        println!("cargo build status: {}", status);
    }

    // Run the built binary
    let exe = if cfg!(windows) { r"aot_bin\target\release\aot_bin.exe" } else { "aot_bin/target/release/aot_bin" };
    if std::path::Path::new(exe).exists() {
        let _ = Command::new(exe).status();
    } else {
        println!("AOT binary not found at {}", exe);
    }
}
