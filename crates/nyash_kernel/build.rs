fn main() {
    println!("cargo:rerun-if-changed=src/hako_forward_registry.c");
    println!("cargo:rerun-if-changed=../../include/nyrt.h");
    println!("cargo:rerun-if-changed=../../include/nyrt_fault_v1.h");
    let target = std::env::var("TARGET").expect("Cargo must provide TARGET");
    let target_source = format!("pub(super) const TARGET_TRIPLE: &str = {target:?};\n");
    std::fs::write(
        std::path::Path::new(&std::env::var("OUT_DIR").expect("Cargo must provide OUT_DIR"))
            .join("runtime_abi_target.rs"),
        target_source,
    )
    .expect("write runtime ABI target source");
    cc::Build::new()
        .file("src/hako_forward_registry.c")
        .warnings(false)
        .compile("nyash_hako_forward_registry");
}
