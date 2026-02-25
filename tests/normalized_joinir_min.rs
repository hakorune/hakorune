#![cfg(all(feature = "normalized_dev", debug_assertions))]

use nyash_rust::backend::{mir_interpreter::MirInterpreter, VMValue};
use nyash_rust::mir::join_ir::normalized::dev_env::{
    normalized_dev_enabled, test_ctx, NormalizedDevEnvGuard, NormalizedTestContext,
};
use nyash_rust::mir::join_ir::normalized::fixtures::{
    build_jsonparser_atoi_real_structured_for_normalized_dev,
    build_jsonparser_atoi_structured_for_normalized_dev,
    build_jsonparser_parse_array_continue_skip_ws_structured_for_normalized_dev,
    build_jsonparser_parse_number_real_structured_for_normalized_dev,
    build_jsonparser_parse_object_continue_skip_ws_structured_for_normalized_dev,
    build_jsonparser_skip_ws_real_structured_for_normalized_dev,
    build_jsonparser_skip_ws_structured_for_normalized_dev,
    build_jsonparser_unescape_string_step2_min_structured_for_normalized_dev,
    build_parse_array_min_structured_for_normalized_dev,
    build_parse_object_min_structured_for_normalized_dev,
    build_parse_string_composite_min_structured_for_normalized_dev,
    build_pattern2_break_fixture_structured, build_pattern2_minimal_structured,
    build_pattern3_if_sum_min_structured_for_normalized_dev,
    build_pattern3_if_sum_multi_min_structured_for_normalized_dev,
    build_pattern3_json_if_sum_min_structured_for_normalized_dev,
    build_pattern4_continue_min_structured_for_normalized_dev,
    build_pattern_continue_return_min_structured_for_normalized_dev,
    build_selfhost_args_parse_p2_structured_for_normalized_dev,
    build_selfhost_if_sum_p3_ext_structured_for_normalized_dev,
    build_selfhost_if_sum_p3_structured_for_normalized_dev,
    build_selfhost_stmt_count_p3_structured_for_normalized_dev,
    build_selfhost_token_scan_p2_accum_structured_for_normalized_dev,
    build_selfhost_token_scan_p2_structured_for_normalized_dev,
};
use nyash_rust::mir::join_ir::{
    normalize_pattern1_minimal, normalize_pattern2_minimal, normalized_pattern1_to_structured,
    normalized_pattern2_to_structured, BinOpKind, ConstValue, JoinContId, JoinFuncId, JoinFunction,
    JoinInst, JoinIrPhase, JoinModule, MirLikeInst,
};
use nyash_rust::mir::join_ir_ops::JoinValue;
use nyash_rust::mir::join_ir_runner::run_joinir_function;
use nyash_rust::mir::join_ir_vm_bridge::{convert_join_module_to_mir_with_meta, run_joinir_via_vm};
use nyash_rust::mir::ValueId;
use std::collections::BTreeMap;

#[path = "normalized_joinir_min/basic.rs"]
mod basic;
#[path = "normalized_joinir_min/ownership.rs"]
mod ownership;
#[path = "normalized_joinir_min/selfhost.rs"]
mod selfhost;
#[path = "normalized_joinir_min/shapes.rs"]
mod shapes;
fn normalized_dev_test_ctx() -> NormalizedTestContext<'static> {
    let ctx = test_ctx();
    assert!(
        normalized_dev_enabled(),
        "Phase 40: normalized_dev must be enabled for normalized_* tests (feature + NYASH_JOINIR_NORMALIZED_DEV_RUN=1)"
    );
    ctx
}

fn assert_normalized_dev_ready() {
    assert!(
        normalized_dev_enabled(),
        "Phase 40: normalized_dev must be enabled for normalized_* tests (feature + NYASH_JOINIR_NORMALIZED_DEV_RUN=1)"
    );
}

fn run_joinir_runner(
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
    normalized: bool,
) -> JoinValue {
    let _guard = NormalizedDevEnvGuard::new(normalized);
    if normalized {
        assert_normalized_dev_ready();
    }
    let mut vm = MirInterpreter::new();
    run_joinir_function(&mut vm, module, entry, args).expect("JoinIR runner should succeed")
}

fn run_joinir_vm_bridge(
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
    normalized: bool,
) -> JoinValue {
    let _guard = NormalizedDevEnvGuard::new(normalized);
    if normalized {
        assert_normalized_dev_ready();
    }
    run_joinir_via_vm(module, entry, args).expect("JoinIR→MIR execution should succeed")
}

fn run_joinir_vm_bridge_structured_only(
    module: &JoinModule,
    entry: JoinFuncId,
    args: &[JoinValue],
) -> JoinValue {
    let mir =
        convert_join_module_to_mir_with_meta(module, &BTreeMap::new()).expect("structured bridge");
    let mut vm = MirInterpreter::new();
    let entry_name = format!("join_func_{}", entry.0);
    let vm_args: Vec<VMValue> = args.iter().cloned().map(|v| v.into_vm_value()).collect();
    let result = vm
        .execute_function_with_args(&mir, &entry_name, &vm_args)
        .expect("VM execution should succeed");
    JoinValue::from_vm_value(&result).expect("result conversion")
}

// moved into tests/normalized_joinir_min/basic.rs

// moved into tests/normalized_joinir_min/{selfhost.rs,shapes.rs}

// moved into tests/normalized_joinir_min/shapes.rs

// moved into tests/normalized_joinir_min/ownership.rs
