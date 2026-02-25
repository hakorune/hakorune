fn main() {
    println!("cargo:rerun-if-changed=src/kernel_min.c");
    cc::Build::new()
        .file("src/kernel_min.c")
        .warnings(false)
        .compile("nyash_kernel_min_c");
}
