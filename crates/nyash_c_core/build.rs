fn main() {
    println!("cargo:rerun-if-changed=src/c_core.c");
    cc::Build::new()
        .file("src/c_core.c")
        .warnings(false)
        .compile("nyash_c_core_c");
}
