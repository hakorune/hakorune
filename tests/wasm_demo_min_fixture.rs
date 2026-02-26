#![cfg(feature = "wasm-backend")]

use nyash_rust::backend::wasm::WasmBackend;
use nyash_rust::mir::MirCompiler;
use nyash_rust::parser::NyashParser;
use std::fs;
use std::path::PathBuf;
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

fn compile_fixture_to_wasm_direct(rel: &str) -> Vec<u8> {
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
        .compile_module(mir_module)
        .expect("fixture should compile to WASM")
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
fn wasm_demo_min_fixture_compile_to_wasm_contract() {
    let wasm = compile_fixture_to_wasm_direct("apps/tests/phase29cc_wsm02d_demo_min.hako");
    assert!(
        wasm.starts_with(&[0x00, 0x61, 0x73, 0x6d]),
        "wasm binary must start with \\0asm magic"
    );
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
fn wasm_demo_wat2wasm_ascii_guard_contract() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let backend = WasmBackend::new();
    let err = backend
        .convert_wat_to_wasm("(module (func (export \"main\") (result i32) i32.const 0 ;; あ))")
        .expect_err("ascii guard should fail fast");
    let msg = err.to_string();
    assert!(msg.contains("WAT source contains non-ASCII characters"));
}

#[test]
fn wasm_demo_wat2wasm_invalid_wat_contract() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let backend = WasmBackend::new();
    let err = backend
        .convert_wat_to_wasm("(module (func")
        .expect_err("invalid WAT should fail fast");
    let msg = err.to_string();
    assert!(msg.contains("WAT to WASM conversion failed"));
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

#[test]
fn wasm_demo_min_fixture_compile_wasm_cli_emits_wasm_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm02d_demo_min.hako";
    let fixture = wasm_common::fixture_path(fixture_rel);
    let mut out_base = wasm_common::target_temp_wat_path("phase29cc_compile_wasm_cli");
    out_base.set_extension("");
    let out_file = PathBuf::from(format!("{}.wasm", out_base.to_string_lossy()));
    let _ = fs::remove_file(&out_file);

    let status = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_base)
        .arg(&fixture)
        .status()
        .expect("hakorune --compile-wasm must launch");
    assert!(status.success(), "--compile-wasm should succeed");

    let emitted = fs::read(&out_file).expect("compile-wasm output should be readable");
    let _ = fs::remove_file(&out_file);
    assert!(
        emitted.starts_with(&[0x00, 0x61, 0x73, 0x6d]),
        "compile-wasm must emit wasm binary with \\0asm magic"
    );
}
