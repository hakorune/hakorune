#![cfg(feature = "wasm-backend")]

use nyash_rust::backend::wasm::{compile_hako_native_shape_bytes, WasmBackend};
use nyash_rust::mir::MirCompiler;
use nyash_rust::parser::NyashParser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[path = "common/wasm.rs"]
mod wasm_common;

fn compile_fixture_to_wat_direct(rel: &str) -> String {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let mir_module = compile_fixture_to_mir_module(rel);
    let mut wasm_backend = WasmBackend::new();
    wasm_backend
        .compile_to_wat(mir_module)
        .expect("fixture should compile to WAT")
}

fn compile_fixture_to_mir_module(rel: &str) -> nyash_rust::mir::MirModule {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    let fixture = wasm_common::fixture_path(rel);
    let source = fs::read_to_string(&fixture).expect("fixture should be readable");
    let ast = NyashParser::parse_from_string(&source).expect("fixture should parse");
    let mut compiler = MirCompiler::new();
    compiler
        .compile(ast)
        .expect("fixture should lower to MIR")
        .module
}

fn compile_fixture_to_wasm_direct(rel: &str) -> Vec<u8> {
    let mir_module = compile_fixture_to_mir_module(rel);
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

#[test]
fn wasm_demo_min_fixture_legacy_route_policy_rejected_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let fixture = wasm_common::fixture_path(fixture_rel);

    let mut out_default_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_default");
    out_default_base.set_extension("");
    let out_default = PathBuf::from(format!("{}.wasm", out_default_base.to_string_lossy()));
    let _ = fs::remove_file(&out_default);

    let mut out_legacy_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_legacy");
    out_legacy_base.set_extension("");
    let out_legacy = PathBuf::from(format!("{}.wasm", out_legacy_base.to_string_lossy()));
    let _ = fs::remove_file(&out_legacy);

    let status_default = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "default")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_default_base)
        .arg(&fixture)
        .status()
        .expect("default route compile-wasm must launch");
    assert!(
        status_default.success(),
        "default route compile-wasm should succeed"
    );

    let output_legacy = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "legacy-wasm-rust")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_legacy_base)
        .arg(&fixture)
        .output()
        .expect("legacy route compile-wasm must launch");
    assert!(
        !output_legacy.status.success(),
        "legacy policy compile-wasm must fail-fast after hard-remove lock"
    );
    let stderr_legacy = String::from_utf8_lossy(&output_legacy.stderr);
    assert!(
        stderr_legacy
            .contains("[freeze:contract][wasm/route-policy] NYASH_WASM_ROUTE_POLICY='legacy-wasm-rust' (allowed: default)"),
        "legacy policy reject freeze tag must be emitted"
    );

    let bytes_default = fs::read(&out_default).expect("default route output should be readable");

    let _ = fs::remove_file(&out_default);
    let _ = fs::remove_file(&out_legacy);

    assert!(
        bytes_default.starts_with(&[0x00, 0x61, 0x73, 0x6d]),
        "default route must emit wasm binary"
    );
}

#[test]
fn wasm_demo_default_hako_lane_native_pilot_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(
        plan,
        nyash_rust::backend::wasm::WasmHakoDefaultLanePlan::NativeShapeTable
    );
    assert!(
        wasm_bytes.starts_with(&[0x00, 0x61, 0x73, 0x6d]),
        "native pilot path must emit wasm binary"
    );
}

#[test]
fn wasm_demo_default_hako_lane_bridge_non_pilot_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm02d_demo_min.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(
        plan,
        nyash_rust::backend::wasm::WasmHakoDefaultLanePlan::BridgeRustBackend
    );
}

#[test]
fn wasm_demo_default_hako_lane_native_const_copy_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(
        plan,
        nyash_rust::backend::wasm::WasmHakoDefaultLanePlan::NativeShapeTable
    );
}

#[test]
fn wasm_demo_default_route_pilot_uses_native_helper_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let bytes = compile_hako_native_shape_bytes(&mir_module)
        .expect("native helper should succeed")
        .expect("pilot shape should be emitted by native helper");

    let backend = WasmBackend::new();
    let baseline = backend
        .build_minimal_i32_const_wasm(7)
        .expect("baseline writer must succeed");
    assert_eq!(
        bytes, baseline,
        "default-route native helper output mismatch for pilot fixture"
    );
}

#[test]
fn wasm_demo_default_route_native_helper_rejects_non_pilot_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm02d_demo_min.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let bytes = compile_hako_native_shape_bytes(&mir_module)
        .expect("native helper should return Ok(None) for non-pilot");
    assert!(
        bytes.is_none(),
        "native helper must reject non-pilot fixture and keep bridge fallback boundary explicit"
    );
}

#[test]
fn wasm_demo_default_route_const_copy_uses_native_helper_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let bytes = compile_hako_native_shape_bytes(&mir_module)
        .expect("native helper should succeed")
        .expect("const-copy-return shape should be emitted by native helper");

    let backend = WasmBackend::new();
    let baseline = backend
        .build_minimal_i32_const_wasm(8)
        .expect("baseline writer must succeed");
    assert_eq!(
        bytes, baseline,
        "default-route native helper output mismatch for const-copy fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_default_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako");
    let mut out_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native");
    out_base.set_extension("");
    let out_file = out_base.with_extension("wasm");
    let _ = fs::remove_file(&out_file);

    let output = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "default")
        .env("NYASH_WASM_ROUTE_TRACE", "1")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_base)
        .arg(&fixture)
        .output()
        .expect("default route compile-wasm with trace must launch");
    assert!(output.status.success(), "default route compile-wasm should succeed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p5.main_return_i32_const_via_copy.v0"
        ),
        "route trace must include native shape_id for const-copy-return fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_bridge_and_legacy_policy_rejected_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_min.hako");
    let mut out_default_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_bridge");
    out_default_base.set_extension("");
    let out_default = out_default_base.with_extension("wasm");
    let _ = fs::remove_file(&out_default);

    let output_default = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "default")
        .env("NYASH_WASM_ROUTE_TRACE", "1")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_default_base)
        .arg(&fixture)
        .output()
        .expect("default route compile-wasm with trace must launch");
    assert!(output_default.status.success(), "default route compile-wasm should succeed");
    let stderr_default = String::from_utf8_lossy(&output_default.stderr);
    assert!(
        stderr_default.contains("[wasm/route-trace] policy=default plan=bridge-rust-backend shape_id=-"),
        "default non-native fixture must report bridge plan in route trace"
    );

    let mut out_legacy_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_legacy");
    out_legacy_base.set_extension("");
    let out_legacy = out_legacy_base.with_extension("wasm");
    let _ = fs::remove_file(&out_legacy);

    let output_legacy = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "legacy-wasm-rust")
        .env("NYASH_WASM_ROUTE_TRACE", "1")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_legacy_base)
        .arg(&fixture)
        .output()
        .expect("legacy route compile-wasm with trace must launch");
    assert!(
        !output_legacy.status.success(),
        "legacy policy compile-wasm must fail-fast after hard-remove lock"
    );
    let stderr_legacy = String::from_utf8_lossy(&output_legacy.stderr);
    assert!(
        !stderr_legacy.contains("[wasm/route-trace] policy=legacy-wasm-rust"),
        "legacy policy parse-fail must stop before route-trace emission"
    );
    assert!(
        stderr_legacy
            .contains("[freeze:contract][wasm/route-policy] NYASH_WASM_ROUTE_POLICY='legacy-wasm-rust' (allowed: default)"),
        "legacy policy reject freeze tag must be emitted"
    );
}

#[test]
fn wasm_demo_min_fixture_route_policy_default_noop_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let fixture = wasm_common::fixture_path(fixture_rel);

    let mut out_unset_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_noop_unset");
    out_unset_base.set_extension("");
    let out_unset = out_unset_base.with_extension("wasm");
    let _ = fs::remove_file(&out_unset);

    let mut out_default_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_noop_default_env");
    out_default_base.set_extension("");
    let out_default = out_default_base.with_extension("wasm");
    let _ = fs::remove_file(&out_default);

    let output_unset = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_unset_base)
        .arg(&fixture)
        .output()
        .expect("compile-wasm without route policy env must launch");
    assert!(
        output_unset.status.success(),
        "compile-wasm without route policy env should succeed"
    );

    let output_default = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "default")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_default_base)
        .arg(&fixture)
        .output()
        .expect("compile-wasm with default route policy env must launch");
    assert!(
        output_default.status.success(),
        "compile-wasm with NYASH_WASM_ROUTE_POLICY=default should succeed"
    );

    let bytes_unset = fs::read(&out_unset).expect("unset route policy output should be readable");
    let bytes_default =
        fs::read(&out_default).expect("default route policy output should be readable");
    let _ = fs::remove_file(&out_unset);
    let _ = fs::remove_file(&out_default);
    assert_eq!(
        bytes_unset, bytes_default,
        "route policy env should be no-op when value is default"
    );
}

#[test]
fn wasm_demo_min_const_return_binary_writer_parity_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let emitted = compile_fixture_to_wasm_direct(fixture_rel);

    let backend = WasmBackend::new();
    let baseline = backend
        .build_minimal_i32_const_wasm(7)
        .expect("baseline writer must succeed");

    assert_eq!(
        emitted, baseline,
        "binary-writer pilot parity mismatch for const-return fixture"
    );
}

#[test]
fn wasm_demo_min_const_return_neg1_binary_writer_parity_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return_neg1.hako";
    let emitted = compile_fixture_to_wasm_direct(fixture_rel);

    let backend = WasmBackend::new();
    let baseline = backend
        .build_minimal_i32_const_wasm(-1)
        .expect("baseline writer must succeed");

    assert_eq!(
        emitted, baseline,
        "binary-writer pilot parity mismatch for const-return(-1) fixture"
    );
}
