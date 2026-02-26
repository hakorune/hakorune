#![cfg(feature = "wasm-backend")]

use nyash_rust::backend::wasm::WasmBackend;
use nyash_rust::mir::MirCompiler;
use nyash_rust::parser::NyashParser;
use std::fs;
use std::process::Command;

#[path = "common/wasm.rs"]
mod wasm_common;

fn compile_fixture_to_wat_direct(rel: &str) -> String {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let fixture = wasm_common::fixture_path(rel);
    let source = fs::read_to_string(&fixture).expect("fixture should be readable");
    let ast = NyashParser::parse_from_string(&source).expect("fixture should parse");
    let mut compiler = MirCompiler::new();
    let mir_module = compiler
        .compile(ast)
        .expect("fixture should lower to MIR")
        .module;
    let mut wasm_backend = WasmBackend::new();
    wasm_backend
        .compile_to_wat(mir_module)
        .expect("fixture should compile to WAT")
}

#[test]
fn wasm_demo_min_fixture_compile_to_wat_contract() {
    let wat = compile_fixture_to_wat_direct("apps/tests/phase29cc_wsm02d_demo_min.hako");

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

    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_unsupported_boundary_min.hako");

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

#[test]
fn wasm_demo_min_fixture_emit_wat_parity_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm02d_demo_min.hako";
    let fixture = wasm_common::fixture_path(fixture_rel);
    let baseline = compile_fixture_to_wat_direct(fixture_rel);

    let out = wasm_common::target_temp_wat_path("phase29cc_wat_parity");
    let _ = fs::remove_file(&out);

    let status = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .arg("--emit-wat")
        .arg(&out)
        .arg(&fixture)
        .status()
        .expect("hakorune --emit-wat must launch");
    assert!(status.success(), "--emit-wat should succeed");

    let emitted = fs::read_to_string(&out).expect("emit-wat output should be readable");
    let _ = fs::remove_file(&out);

    assert_eq!(
        emitted, baseline,
        "WAT parity mismatch between CLI --emit-wat and direct compile_to_wat"
    );
}
