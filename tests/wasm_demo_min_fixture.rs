#![cfg(feature = "wasm-backend")]

use nyash_rust::backend::wasm::WasmBackend;
use nyash_rust::mir::MirCompiler;
use nyash_rust::parser::NyashParser;
use std::fs;
use std::path::PathBuf;

#[test]
fn wasm_demo_min_fixture_compile_to_wat_contract() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();

    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("apps/tests/phase29cc_wsm02d_demo_min.hako");

    let source = fs::read_to_string(&fixture).expect("fixture should be readable");
    let ast = NyashParser::parse_from_string(&source).expect("fixture should parse");
    let mut compiler = MirCompiler::new();
    let mir_module = compiler
        .compile(ast)
        .expect("fixture should lower to MIR")
        .module;

    let mut wasm_backend = WasmBackend::new();
    let wat = wasm_backend
        .compile_to_wat(mir_module)
        .expect("fixture should compile to WAT");

    assert!(wat.contains("(export \"main\" (func $main))"));
    assert!(wat.contains("\"console_log\""));
    assert!(wat.contains("\"console_warn\""));
    assert!(wat.contains("\"console_error\""));
    assert!(wat.contains("\"console_info\""));
    assert!(wat.contains("\"console_debug\""));
}

#[test]
fn wasm_demo_unsupported_boundary_fails_fast_contract() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();

    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("apps/tests/phase29cc_wsm02d_demo_unsupported_boundary_min.hako");

    let source = fs::read_to_string(&fixture).expect("fixture should be readable");
    let ast = NyashParser::parse_from_string(&source).expect("fixture should parse");
    let mut compiler = MirCompiler::new();
    let mir_module = compiler
        .compile(ast)
        .expect("fixture should lower to MIR")
        .module;

    let mut wasm_backend = WasmBackend::new();
    let err = wasm_backend
        .compile_to_wat(mir_module)
        .expect_err("scope-out method should fail-fast");

    let msg = err.to_string();
    assert!(msg.contains("Unsupported instruction"));
    assert!(msg.contains("Unsupported BoxCall method: group"));
    assert!(msg.contains("supported:"));
}
