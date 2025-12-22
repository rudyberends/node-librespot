use std::env;

fn main() {
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
  if target_os == "macos" {
    // Allow unresolved N-API symbols to be resolved at runtime by Node on macOS.
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
  }
}
