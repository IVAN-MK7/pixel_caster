/// Necessary in combination with:
/// Cargo.toml:
/// [target.'cfg(target_os = "linux")'.package]
/// build = "src/build.rs"
/// 
/// and
/// 
/// sudo apt-get install libx11-dev
/// 
/// to make it compile on Linux
fn main() {
    println!("cargo:rustc-link-lib=X11");
}
