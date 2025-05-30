use std::process::Command;

fn main() {
    // Only rerun build.rs if this file changes (i.e., not on every build)
    println!("cargo:rerun-if-changed=build.rs");

    // Find python3.11 and emit the path
    if let Ok(output) = Command::new("which").arg("python3.11").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Emit a warning if this value is different than before
            println!("cargo:rustc-env=PYTHON_SYS_EXECUTABLE={}", path);
        } else {
            eprintln!(
                "Failed to find python3.11: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
