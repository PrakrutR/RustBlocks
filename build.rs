fn main() {
    // This file is used to set up the build process
    // For now, it's minimal, but we'll add more as needed for Android builds
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/");
}