use std::process::Command;

fn main() {
    let dir_compiler= "prop2sql/";

    println!("cargo:rerun-if-changed={}", dir_compiler);

    let status = Command::new("cabal")
        .arg("build")
        .arg("prop2sql")
        .current_dir(dir_compiler)
        .status()
        .expect("Failed to execute cabal. Is it installed?");

    if !status.success() {
        panic!("Cabal failed to build the Haskell compiler!");
    }
}
