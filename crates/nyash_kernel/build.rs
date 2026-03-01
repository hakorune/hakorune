fn main() {
    println!("cargo:rerun-if-changed=src/hako_forward_registry.c");
    println!("cargo:rerun-if-changed=../../include/nyrt.h");
    cc::Build::new()
        .file("src/hako_forward_registry.c")
        .warnings(false)
        .compile("nyash_hako_forward_registry");
}
