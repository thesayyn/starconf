fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=patches/");
    println!("cargo:warning=hello");
    patch_crate::run().expect("Failed while patching");
}