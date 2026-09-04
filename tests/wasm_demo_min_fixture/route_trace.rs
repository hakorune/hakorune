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
