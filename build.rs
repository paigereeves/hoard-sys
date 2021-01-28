use std::process::Command;
use std::path::PathBuf;

const VERSION: &'static str = "b24588fe050a230f79666b50495f1f239aa537d9";

fn main() {
    let mut current_dir = PathBuf::from(std::env::var("OUT_DIR").expect("No OUT_DIR."));
    if !current_dir.join("Hoard").exists() {
        let mut cmd = Command::new("git");
        cmd
            .current_dir(&current_dir)
            .args(&["clone", "-n", "https://github.com/emeryberger/Hoard"])
            .status().expect("Failed to clone Hoard.");
    }

    current_dir.push("Hoard");

    let mut cmd = Command::new("git");
    cmd
        .current_dir(&current_dir)
        .args(&["checkout", VERSION])
        .status().expect("Failed to checkout.");

    current_dir.push("src");

    let mut cmd = Command::new("make");
    cmd
        .current_dir(&current_dir)
        .arg("Heap-Layers")
        .status().expect("Failed to clone Heap Layers.");

    let mut cmd = Command::new("make");
    cmd
        .current_dir("./")
        .args(&["Hoard", &format!("out={}",&current_dir.to_str().unwrap())])
        .status().expect("Failed to make Hoard.");

    println!("cargo:rustc-link-lib=static=hoard");
    println!("cargo:rustc-link-search=native={}", &current_dir.to_str().unwrap());
}
