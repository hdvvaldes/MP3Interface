use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=haskell_compiler_dir/");

    let status = Command::new("cabal")
        .arg("build")
        .current_dir("haskell_compiler_dir") // Path to your Haskell project
        .status()
        .expect("Failed to execute cabal. Is it installed?");

    if !status.success() {
        panic!("Cabal failed to build the Haskell compiler!");
    }
}
