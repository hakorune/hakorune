use super::common::make_function_with_call;
use super::*;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType, ValueId};

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
