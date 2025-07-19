use std::fs;

fn main() {
    if let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") {
        fs::write("./hello", dir).expect("Failed to write to target directory");
    }
}
