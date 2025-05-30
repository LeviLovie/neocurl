use std::process::Command;

fn main() {
    // Try to find python3.11
    if let Ok(output) = Command::new("which").arg("python3.11").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=PYTHON_SYS_EXECUTABLE={}", path);
        } else {
            eprintln!(
                "Failed to find python3.11: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
