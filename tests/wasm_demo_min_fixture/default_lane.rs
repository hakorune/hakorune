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
