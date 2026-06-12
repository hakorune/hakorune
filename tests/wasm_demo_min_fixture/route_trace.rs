use super::wasm_common;
use std::fs;
use std::process::Command;

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_default_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p5.main_return_i32_const_via_copy.v0"
        ),
        "route trace must include native shape_id for const-copy-return fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_const_binop_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_p9_min1_const_binop_return.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native_binop");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p9.main_return_i32_const_binop.v0"
        ),
        "route trace must include native shape_id for const-binop-return fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_p10_min4_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_p10_min4_loop_extern_native.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native_p10_min4");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p10.main_loop_extern_call.fixed3.v0"
        ),
        "route trace must include native shape_id for p10 min4 loop/extern fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_p10_min6_warn_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_p10_min6_loop_extern_warn_native.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native_p10_min6_warn");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p10.main_loop_extern_call.warn.fixed4.v0"
        ),
        "route trace must include native shape_id for p10 min6 warn fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_p10_min7_info_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_p10_min7_loop_extern_info_native.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_native_p10_min7_info");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p10.main_loop_extern_call.info.fixed4.v0"
        ),
        "route trace must include native shape_id for p10 min7 info fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_p10_min8_error_contract() {
    let fixture = wasm_common::fixture_path(
        "apps/tests/phase29cc_wsm_p10_min8_loop_extern_error_native.hako",
    );
    let mut out_base = wasm_common::target_temp_wat_path(
        "phase29cc_wsm_route_trace_default_native_p10_min8_error",
    );
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p10.main_loop_extern_call.error.fixed4.v0"
        ),
        "route trace must include native shape_id for p10 min8 error fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_shape_id_for_native_p10_min9_debug_contract() {
    let fixture = wasm_common::fixture_path(
        "apps/tests/phase29cc_wsm_p10_min9_loop_extern_debug_native.hako",
    );
    let mut out_base = wasm_common::target_temp_wat_path(
        "phase29cc_wsm_route_trace_default_native_p10_min9_debug",
    );
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[wasm/route-trace] policy=default plan=native-shape-table shape_id=wsm.p10.main_loop_extern_call.debug.fixed4.v0"
        ),
        "route trace must include native shape_id for p10 min9 debug fixture"
    );
}

#[test]
fn wasm_demo_route_trace_reports_bridge_and_legacy_policy_rejected_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_min.hako");
    let mut out_default_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_default_bridge");
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
    assert!(
        output_default.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr_default = String::from_utf8_lossy(&output_default.stderr);
    assert!(
        stderr_default
            .contains("[wasm/route-trace] policy=default plan=bridge-rust-backend shape_id=-"),
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
            .contains("[freeze:contract][wasm/route-policy] NYASH_WASM_ROUTE_POLICY='legacy-wasm-rust' (allowed: default|rust_native)"),
        "legacy policy reject freeze tag must be emitted"
    );
}

#[test]
fn wasm_demo_route_trace_reports_rust_native_forced_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_min.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_forced_rust_native");
    out_base.set_extension("");
    let out_file = out_base.with_extension("wasm");
    let _ = fs::remove_file(&out_file);

    let output = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "rust_native")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_base)
        .arg(&fixture)
        .output()
        .expect("rust_native forced compile-wasm must launch");
    assert!(
        output.status.success(),
        "rust_native forced compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[wasm/route-trace] policy=rust_native plan=bridge-rust-backend shape_id=- route=rust_native"),
        "forced rust_native route must emit route-trace line with rust_native route"
    );
}

#[test]
fn wasm_demo_emit_wat_rejects_rust_native_policy_scope_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_min.hako");
    let out_wat = wasm_common::target_temp_wat_path("phase29cc_wsm_emit_wat_rust_native_scope");
    let _ = fs::remove_file(&out_wat);

    let output = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "rust_native")
        .arg("--emit-wat")
        .arg(&out_wat)
        .arg(&fixture)
        .output()
        .expect("emit-wat with rust_native route policy must launch");
    assert!(
        !output.status.success(),
        "emit-wat must fail-fast when rust_native policy is set"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[freeze:contract][wasm/route-policy-scope] NYASH_WASM_ROUTE_POLICY=rust_native is compile-wasm only"
        ),
        "emit-wat scope guard must emit freeze contract tag"
    );
    assert!(
        !out_wat.exists(),
        "emit-wat output file must not be produced when scope guard fails"
    );
}

#[test]
fn wasm_demo_route_trace_is_emitted_without_trace_env_contract() {
    let fixture = wasm_common::fixture_path("apps/tests/phase29cc_wsm02d_demo_min.hako");
    let mut out_base = wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_always_on");
    out_base.set_extension("");
    let out_file = out_base.with_extension("wasm");
    let _ = fs::remove_file(&out_file);

    let output = Command::new(wasm_common::hakorune_bin_path())
        .env("NYASH_USE_NY_COMPILER", "0")
        .env("NYASH_WASM_ROUTE_POLICY", "default")
        .arg("--compile-wasm")
        .arg("-o")
        .arg(&out_base)
        .arg(&fixture)
        .output()
        .expect("default compile-wasm must launch");
    assert!(
        output.status.success(),
        "default compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[wasm/route-trace] policy=default"),
        "compile-wasm must always emit wasm route-trace line"
    );
}

#[test]
fn wasm_demo_route_trace_reports_bridge_for_webcanvas_fixture_contract() {
    let fixture =
        wasm_common::fixture_path("apps/tests/phase29cc_wsm_g4_min3_webcanvas_fixture_min.hako");
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_webcanvas_bridge");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[wasm/route-trace] policy=default plan=bridge-rust-backend shape_id=-"),
        "webcanvas fixture must currently report bridge plan in route trace"
    );
}

#[test]
fn wasm_demo_route_trace_reports_bridge_for_canvas_advanced_fixture_contract() {
    let fixture = wasm_common::fixture_path(
        "apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako",
    );
    let mut out_base =
        wasm_common::target_temp_wat_path("phase29cc_wsm_route_trace_canvas_advanced_bridge");
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
    assert!(
        output.status.success(),
        "default route compile-wasm should succeed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("[wasm/route-trace] policy=default plan=bridge-rust-backend shape_id=-"),
        "canvas_advanced fixture must currently report bridge plan in route trace"
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
