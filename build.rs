//! `BackMate` UI pre-build script

use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=input.css");
    println!("cargo::rerun-if-changed=tailwind.config.js");
    Command::new("npx")
        .arg("tailwindcss")
        .args(["-i", "./input.css"])
        .args(["-o", "./assets/style.css"])
        .output()
        .expect("Failed to run Tailwind");
}
