use super::common::*;

#[test]
fn wasm_demo_min_const_return_binary_writer_parity_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let emitted = compile_fixture_to_wasm_direct(fixture_rel);

    let backend = nyash_rust::backend::wasm::WasmBackend::new();
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

    let backend = nyash_rust::backend::wasm::WasmBackend::new();
    let baseline = backend
        .build_minimal_i32_const_wasm(-1)
        .expect("baseline writer must succeed");

    assert_eq!(
        emitted, baseline,
        "binary-writer pilot parity mismatch for const-return(-1) fixture"
    );
}

#[test]
fn wasm_demo_g4_min3_webcanvas_fixture_compile_to_wat_contract() {
    let wat = compile_fixture_to_wat_direct(
        "apps/tests/phase29cc_wsm_g4_min3_webcanvas_fixture_min.hako",
    );
    assert!(wat.contains("(export \"main\" (func $main))"));
    assert!(wat.contains("\"console_log\""));
    assert!(wat.contains("\"canvas_clear\""));
    assert!(wat.contains("\"canvas_fillRect\""));
    assert!(wat.contains("\"canvas_fillText\""));
}

#[test]
fn wasm_demo_g4_min4_canvas_advanced_fixture_compile_to_wat_contract() {
    let wat = compile_fixture_to_wat_direct(
        "apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako",
    );
    assert!(wat.contains("(export \"main\" (func $main))"));
    assert!(wat.contains("\"console_log\""));
    assert!(wat.contains("\"console_info\""));
    assert!(wat.contains("\"canvas_clear\""));
    assert!(wat.contains("\"canvas_fillRect\""));
    assert!(wat.contains("\"canvas_fillText\""));
}

#[test]
fn wasm_demo_g4_min8_global_call_probe_compile_to_wat_contract() {
    let wat = compile_fixture_to_wat_direct(
        "apps/tests/phase29cc_wsm_g4_min8_global_call_probe_min.hako",
    );
    assert!(wat.contains("(export \"main\" (func $main))"));
    assert!(wat.contains("(func $WsmProbeBox.ping/1"));
    assert!(wat.contains("call $WsmProbeBox.ping/1"));
}
