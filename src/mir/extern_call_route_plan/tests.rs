use super::*;
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

fn make_function_with_call(callee: &str, args: Vec<ValueId>, dst: Option<ValueId>) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("STAGE1_SOURCE_TEXT".to_string()),
    });
    block.instructions.push(MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(callee.to_string())),
        args,
        effects: EffectMask::PURE,
    });
    function.blocks.insert(BasicBlockId::new(0), block);
    function
}

#[test]
fn refresh_function_extern_call_routes_records_env_get_plan_source() {
    let mut function =
        make_function_with_call("env.get/1", vec![ValueId::new(1)], Some(ValueId::new(2)));

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.env.get");
    assert_eq!(route.core_op(), "EnvGet");
    assert_eq!(route.symbol(), "nyash.env.get");
    assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(
        route.lowering_emit_kind(),
        LoweringPlanEmitKind::RuntimeCall
    );
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "env.get/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "string_handle_or_null");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["read.env"]);
}

#[test]
fn refresh_function_extern_call_routes_records_env_set_plan_source() {
    let mut function = make_function_with_call(
        "env.set/2",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.env.set");
    assert_eq!(route.core_op(), "EnvSet");
    assert_eq!(route.symbol(), "nyash.env.set");
    assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(
        route.lowering_emit_kind(),
        LoweringPlanEmitKind::RuntimeCall
    );
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "env.set/2");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["write.env"]);
}

#[test]
fn refresh_function_extern_call_routes_records_any_handle_live_route() {
    let mut function = make_function_with_call(
        "nyash.any.handle_live_h",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.any.handle_live");
    assert_eq!(route.core_op(), "AnyHandleLive");
    assert_eq!(route.symbol(), "nyash.any.handle_live_h");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.any.handle_live_h");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["read.any.handle_live"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_append_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_append_hh",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_append_any");
    assert_eq!(route.core_op(), "ArraySlotAppendAny");
    assert_eq!(route.symbol(), "nyash.array.slot_append_hh");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_append_hh");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_append"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_len_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_len_h",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_len_i64");
    assert_eq!(route.core_op(), "ArraySlotLenI64");
    assert_eq!(route.symbol(), "nyash.array.slot_len_h");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_len_h");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_len"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_load_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_load_hi",
        vec![ValueId::new(1), ValueId::new(2)],
        Some(ValueId::new(3)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_load_i64");
    assert_eq!(route.core_op(), "ArraySlotLoadI64");
    assert_eq!(route.symbol(), "nyash.array.slot_load_hi");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_load_hi");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), ValueId::new(3));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_load"]);
}

#[test]
fn refresh_function_extern_call_routes_records_array_slot_store_route() {
    let mut function = make_function_with_call(
        "nyash.array.slot_store_hii",
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
        Some(ValueId::new(4)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.array.slot_store_i64");
    assert_eq!(route.core_op(), "ArraySlotStoreI64");
    assert_eq!(route.symbol(), "nyash.array.slot_store_hii");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.array.slot_store_hii");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(3)));
    assert_eq!(route.result_value(), ValueId::new(4));
    assert_eq!(route.arity(), 3);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["array.slot_store_i64"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_mem_alloc_route() {
    let mut function = make_function_with_call(
        "hako_mem_alloc",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_mem.alloc");
    assert_eq!(route.core_op(), "HakoMemAlloc");
    assert_eq!(route.symbol(), "hako_mem_alloc");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_mem_alloc");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.mem.alloc"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_mem_free_route() {
    let mut function = make_function_with_call(
        "hako_mem_free/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_mem.free");
    assert_eq!(route.core_op(), "HakoMemFree");
    assert_eq!(route.symbol(), "hako_mem_free");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_mem_free/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "void_sentinel_i64_zero");
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.effect_tags(), &["hako.mem.free"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_osvm_routes() {
    let mut reserve = make_function_with_call(
        "hako_osvm_reserve_bytes_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );
    refresh_function_extern_call_routes(&mut reserve);
    assert_eq!(reserve.metadata.extern_call_routes.len(), 1);
    let route = &reserve.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.reserve_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmReserveBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_reserve_bytes_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_osvm_reserve_bytes_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "native_ptr_nullable");
    assert_eq!(route.value_demand(), "native_ptr_nullable");
    assert_eq!(route.effect_tags(), &["hako.osvm.reserve"]);

    let mut commit = make_function_with_call(
        "hako_osvm_commit_bytes_i64",
        vec![ValueId::new(3), ValueId::new(4)],
        Some(ValueId::new(5)),
    );
    refresh_function_extern_call_routes(&mut commit);
    assert_eq!(commit.metadata.extern_call_routes.len(), 1);
    let route = &commit.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.commit_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmCommitBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_commit_bytes_i64");
    assert_eq!(route.key_value(), ValueId::new(3));
    assert_eq!(route.value_value(), Some(ValueId::new(4)));
    assert_eq!(route.result_value(), ValueId::new(5));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.osvm.commit"]);

    let mut decommit = make_function_with_call(
        "hako_osvm_decommit_bytes_i64/2",
        vec![ValueId::new(6), ValueId::new(7)],
        Some(ValueId::new(8)),
    );
    refresh_function_extern_call_routes(&mut decommit);
    assert_eq!(decommit.metadata.extern_call_routes.len(), 1);
    let route = &decommit.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_osvm.decommit_bytes_i64");
    assert_eq!(route.core_op(), "HakoOsvmDecommitBytesI64");
    assert_eq!(route.symbol(), "hako_osvm_decommit_bytes_i64");
    assert_eq!(route.key_value(), ValueId::new(6));
    assert_eq!(route.value_value(), Some(ValueId::new(7)));
    assert_eq!(route.result_value(), ValueId::new(8));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.osvm.decommit"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hako_tls_cache_slot_routes() {
    let mut get = make_function_with_call(
        "hako_tls_cache_slot_get_i64/1",
        vec![ValueId::new(1)],
        Some(ValueId::new(2)),
    );
    refresh_function_extern_call_routes(&mut get);
    assert_eq!(get.metadata.extern_call_routes.len(), 1);
    let route = &get.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_tls.cache_slot_get_i64");
    assert_eq!(route.core_op(), "HakoTlsCacheSlotGetI64");
    assert_eq!(route.symbol(), "hako_tls_cache_slot_get_i64");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hako_tls_cache_slot_get_i64/1");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.tls.cache_slot_get"]);

    let mut set = make_function_with_call(
        "hako_tls_cache_slot_set_i64",
        vec![ValueId::new(3), ValueId::new(4)],
        Some(ValueId::new(5)),
    );
    refresh_function_extern_call_routes(&mut set);
    assert_eq!(set.metadata.extern_call_routes.len(), 1);
    let route = &set.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hako_tls.cache_slot_set_i64");
    assert_eq!(route.core_op(), "HakoTlsCacheSlotSetI64");
    assert_eq!(route.symbol(), "hako_tls_cache_slot_set_i64");
    assert_eq!(route.key_value(), ValueId::new(3));
    assert_eq!(route.value_value(), Some(ValueId::new(4)));
    assert_eq!(route.result_value(), ValueId::new(5));
    assert_eq!(route.arity(), 2);
    assert_eq!(route.return_shape(), "scalar_i64");
    assert_eq!(route.value_demand(), "runtime_i64");
    assert_eq!(route.effect_tags(), &["hako.tls.cache_slot_set"]);
}

#[test]
fn refresh_function_extern_call_routes_records_hostbridge_extern_invoke_global_source() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(10)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
        args: vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
        effects: EffectMask::IO,
    });
    function.blocks.insert(BasicBlockId::new(0), block);

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.hostbridge.extern_invoke");
    assert_eq!(route.core_op(), "HostBridgeExternInvoke");
    assert_eq!(route.symbol(), "nyash.hostbridge.extern_invoke");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "hostbridge.extern_invoke/3");
    assert_eq!(route.key_value(), ValueId::new(1));
    assert_eq!(route.value_value(), Some(ValueId::new(3)));
    assert_eq!(route.result_value(), ValueId::new(10));
    assert_eq!(route.arity(), 3);
    assert_eq!(route.return_shape(), "string_handle_or_null");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["hostbridge.extern"]);
}

#[test]
fn refresh_function_extern_call_routes_records_stage1_emit_program_json_extern_route() {
    let mut function = make_function_with_call(
        "nyash.stage1.emit_program_json_v0_h",
        vec![ValueId::new(0)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.stage1.emit_program_json_v0");
    assert_eq!(route.core_op(), "Stage1EmitProgramJson");
    assert_eq!(route.symbol(), "nyash.stage1.emit_program_json_v0_h");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(route.source_symbol(), "nyash.stage1.emit_program_json_v0_h");
    assert_eq!(route.key_value(), ValueId::new(0));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "string_handle");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["stage1.emit_program_json"]);
}

#[test]
fn refresh_function_extern_call_routes_records_stage1_emit_mir_from_source_extern_route() {
    let mut function = make_function_with_call(
        "nyash.stage1.emit_mir_from_source_v0_h",
        vec![ValueId::new(0)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(route.route_id(), "extern.stage1.emit_mir_from_source_v0");
    assert_eq!(route.core_op(), "Stage1EmitMirFromSource");
    assert_eq!(route.symbol(), "nyash.stage1.emit_mir_from_source_v0_h");
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(
        route.source_symbol(),
        "nyash.stage1.emit_mir_from_source_v0_h"
    );
    assert_eq!(route.key_value(), ValueId::new(0));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "string_handle");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["stage1.emit_mir_from_source"]);
}

#[test]
fn refresh_function_extern_call_routes_records_stage1_emit_mir_from_program_json_extern_route() {
    let mut function = make_function_with_call(
        "nyash.stage1.emit_mir_from_program_json_v0_h",
        vec![ValueId::new(0)],
        Some(ValueId::new(2)),
    );

    refresh_function_extern_call_routes(&mut function);

    assert_eq!(function.metadata.extern_call_routes.len(), 1);
    let route = &function.metadata.extern_call_routes[0];
    assert_eq!(
        route.route_id(),
        "extern.stage1.emit_mir_from_program_json_v0"
    );
    assert_eq!(route.core_op(), "Stage1EmitMirFromProgramJson");
    assert_eq!(
        route.symbol(),
        "nyash.stage1.emit_mir_from_program_json_v0_h"
    );
    assert_eq!(route.tier(), "ColdRuntime");
    assert_eq!(route.emit_kind(), "runtime_call");
    assert_eq!(route.proof(), "extern_registry");
    assert_eq!(
        route.source_symbol(),
        "nyash.stage1.emit_mir_from_program_json_v0_h"
    );
    assert_eq!(route.key_value(), ValueId::new(0));
    assert_eq!(route.value_value(), None);
    assert_eq!(route.result_value(), ValueId::new(2));
    assert_eq!(route.arity(), 1);
    assert_eq!(route.return_shape(), "string_handle");
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.effect_tags(), &["stage1.emit_mir_from_program_json"]);
}

#[test]
fn refresh_function_extern_call_routes_requires_dst_and_matching_arity() {
    let mut missing_dst = make_function_with_call("env.get/1", vec![ValueId::new(1)], None);
    refresh_function_extern_call_routes(&mut missing_dst);
    assert!(missing_dst.metadata.extern_call_routes.is_empty());

    let mut missing_arg = make_function_with_call("env.get/1", vec![], Some(ValueId::new(2)));
    refresh_function_extern_call_routes(&mut missing_arg);
    assert!(missing_arg.metadata.extern_call_routes.is_empty());

    let mut missing_value =
        make_function_with_call("env.set/2", vec![ValueId::new(1)], Some(ValueId::new(2)));
    refresh_function_extern_call_routes(&mut missing_value);
    assert!(missing_value.metadata.extern_call_routes.is_empty());
}
