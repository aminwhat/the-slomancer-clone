use dotenvy::dotenv;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    // Load variables from .env into process environment
    dotenv().ok();

    let run_dir = env!("CARGO_MANIFEST_DIR");
    let godot_path = env::var("GODOT_PATH").expect("GODOT_PATH not set in .env");

    Command::new(godot_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(run_dir)
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "tried running Godot from {}. Make sure GODOT_PATH is set correctly in .env",
                run_dir
            )
        });
}
