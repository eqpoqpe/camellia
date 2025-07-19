use landmap_camellia::env;

fn main() {
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        println!("CARGO_TARGET_DIR: {}", target_dir);
    } else {
        println!("CARGO_TARGET_DIR is not set.");
    }

    println!("Hello, world! {:?}", env::app_env());
}
