fn main() {
    // Only build the C shim when the `c-shim` feature is enabled.
    let use_c = std::env::var("CARGO_FEATURE_C_SHIM").is_ok();
    if !use_c {
        println!("cargo:warning=nyash-tlv: c-shim feature disabled; using Rust stub");
        return;
    }
    cc::Build::new()
        .file("src/tlv.c")
        .flag_if_supported("-std=c99")
        .compile("nyash_tlv_c");
}
