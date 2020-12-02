fn main() {
    println!("cargo:rustc-link-lib=dylib=ibverbs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}