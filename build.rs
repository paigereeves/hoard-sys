use std::process::Command;
use std::path::PathBuf;

const VERSION: &'static str = "b24588fe050a230f79666b50495f1f239aa537d9";

fn main() {
    // assert!(false, "OUT_DIR={:?}", PathBuf::from(std::env::var("OUT_DIR").expect("No OUT_DIR.")));
    let mut current_dir = PathBuf::from(std::env::var("OUT_DIR").expect("No OUT_DIR."));
    if !current_dir.join("Hoard").exists() {
        let mut git = Command::new("git");
        git
            .current_dir(&current_dir)
            .args(&["clone", "-n", "https://github.com/emeryberger/Hoard"])
            .status().expect("Failed to clone Hoard.");
    }
    current_dir.push("Hoard");
    let mut git = Command::new("git");
    git
        .current_dir(&current_dir)
        .args(&["checkout", VERSION])
        .status().expect("Failed to checkout.");

    current_dir.push("src");
    let mut make = Command::new("make");
    make
        .current_dir(&current_dir)
        .status().expect("Failed to make Hoard.");
    println!("cargo:rustc-link-lib=dylib=hoard");
    println!("cargo:rustc-link-search=native={}", &current_dir.to_str().unwrap());
}