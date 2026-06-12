use super::common::*;
use nyash_rust::backend::wasm::{
    compile_hako_native_shape_bytes, WasmBackend, WasmHakoDefaultLanePlan,
};

#[test]
fn wasm_demo_default_hako_lane_native_pilot_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p4_min_const_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
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
    assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
}

#[test]
fn wasm_demo_default_hako_lane_bridge_webcanvas_fixture_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_g4_min3_webcanvas_fixture_min.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
}

#[test]
fn wasm_demo_default_hako_lane_bridge_canvas_advanced_fixture_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::BridgeRustBackend);
}

#[test]
fn wasm_demo_default_hako_lane_native_const_copy_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_const_binop_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p9_min1_const_binop_return.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_p10_min4_loop_extern_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p10_min4_loop_extern_native.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_p10_min6_warn_loop_extern_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p10_min6_loop_extern_warn_native.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_p10_min7_info_loop_extern_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p10_min7_loop_extern_info_native.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_p10_min8_error_loop_extern_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p10_min8_loop_extern_error_native.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = WasmBackend::new();
    let (_wasm_bytes, plan) = backend
        .compile_hako_default_lane(mir_module)
        .expect("default hako-lane compile must succeed");
    assert_eq!(plan, WasmHakoDefaultLanePlan::NativeShapeTable);
}

#[test]
fn wasm_demo_default_hako_lane_native_p10_min9_debug_loop_extern_shape_contract() {
    let fixture_rel = "apps/tests/phase29cc_wsm_p10_min9_loop_extern_debug_native.hako";
    let mir_module = compile_fixture_to_mir_module(fixture_rel);
    let mut backend = nyash_rust::backend::wasm::WasmBackend::new();
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

    let backend = nyash_rust::backend::wasm::WasmBackend::new();
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
