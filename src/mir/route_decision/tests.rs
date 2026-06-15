use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
use crate::mir::function::{
    DirectStateFieldPlan, DirectStatePlan, RequiredFastPathRegion, TypedObjectFieldPlan,
    TypedObjectFieldStorage, TypedObjectPlan,
};
use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
use crate::mir::{
    BasicBlock, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};

fn method_call(
    dst: Option<u32>,
    box_name: &str,
    method: &str,
    receiver: u32,
    args: Vec<u32>,
) -> MirInstruction {
    MirInstruction::Call {
        dst: dst.map(ValueId::new),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: args.into_iter().map(ValueId::new).collect(),
        effects: EffectMask::PURE,
    }
}

fn direct_exact_hotcore_call_plan() -> DirectExactHotCoreCallPlan {
    DirectExactHotCoreCallPlan {
        block: BasicBlockId::new(3),
        instruction_index: 7,
        caller: "Main.runOne/2".to_string(),
        callee: "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1".to_string(),
        box_name: "HakoAllocObjectLifecycleHotCore".to_string(),
        method: "objectLifecycleSmallAlloc".to_string(),
        receiver_value: ValueId::new(1),
        result_value: Some(ValueId::new(9)),
        receiver_exact: true,
        same_module: true,
        dispatch_policy: "static_exact",
        call_boundary_policy: "thin_direct_call_candidate",
        return_shape: Some("scalar_i64"),
        value_demand: "read_scalar",
        callee_summary_status: "ok",
        lowering_consumer_enabled: true,
        generic_method_dispatch: false,
        dynamic_route: false,
        boxed_fallback: false,
        summary: "ok",
        failure_reason: None,
    }
}

fn map_lookup_fusion_route() -> MapLookupFusionRoute {
    crate::mir::map_lookup_fusion_plan::test_support::same_key_nonzero_json_fixture()
}

fn typed_object_route_plan() -> TypedObjectPlan {
    TypedObjectPlan {
        box_name: "Page".to_string(),
        type_id: 294019300,
        layout_kind: "typed_object_v0".to_string(),
        field_count: 3,
        fields: vec![
            TypedObjectFieldPlan {
                name: "capacity".to_string(),
                slot: 0,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::USize,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "used".to_string(),
                slot: 1,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
                is_weak: false,
            },
            TypedObjectFieldPlan {
                name: "next".to_string(),
                slot: 2,
                declared_type_name: Some("handle".to_string()),
                storage: TypedObjectFieldStorage::Handle,
                is_weak: false,
            },
        ],
    }
}

fn direct_state_ready_plan() -> DirectStatePlan {
    DirectStatePlan {
        box_name: "Page".to_string(),
        state_repr: "direct_v0".to_string(),
        field_decl_authority: true,
        selected_field_count: 2,
        unsupported_field_count: 0,
        materialization_boundary_known: true,
        positive_net_expected: true,
        fields: vec![
            DirectStateFieldPlan {
                name: "capacity".to_string(),
                slot: 0,
                declared_type_name: Some("usize".to_string()),
                storage: TypedObjectFieldStorage::USize,
            },
            DirectStateFieldPlan {
                name: "used".to_string(),
                slot: 1,
                declared_type_name: Some("i64".to_string()),
                storage: TypedObjectFieldStorage::I64,
            },
        ],
    }
}

#[test]
fn route_decision_reports_direct_array_selected_route() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    refresh_function_generic_method_routes(&mut function);
    refresh_function_direct_array_access_plans(&mut function);
    refresh_function_route_decisions(&mut function);

    assert_eq!(function.metadata.route_decisions.len(), 1);
    let decision = &function.metadata.route_decisions[0];
    assert_eq!(decision.site_id, "b0.i0");
    assert_eq!(decision.semantic_op, "ArrayGet");
    assert_eq!(decision.preferred_route, "direct_array_i64_load");
    assert_eq!(decision.selected_route, "direct_array_i64_load");
    assert_eq!(decision.fallback_route, "generic_array_get_helper");
    assert_eq!(decision.fallback_policy, "opportunistic");
    assert_eq!(decision.miss_reason, None);
    assert_eq!(decision.source_plan_kind, "DirectArrayAccessPlan");
}

#[test]
fn route_decision_reports_map_lookup_selected_route() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .map_lookup_fusion_routes
        .push(map_lookup_fusion_route());

    refresh_function_route_decisions(&mut function);

    assert_eq!(function.metadata.route_decisions.len(), 2);
    let get_decision = &function.metadata.route_decisions[0];
    assert_eq!(get_decision.site_id, "b4.i10");
    assert_eq!(get_decision.semantic_op, "MapGet");
    assert_eq!(get_decision.access_kind, "map_lookup_same_key_get");
    assert_eq!(get_decision.preferred_route, "map_lookup_const_fold");
    assert_eq!(get_decision.selected_route, "map_lookup_const_fold");
    assert_eq!(get_decision.fallback_route, "generic_map_get_helper");
    assert_eq!(get_decision.fallback_policy, "opportunistic");
    assert_eq!(get_decision.miss_reason, None);
    assert_eq!(get_decision.source_plan_kind, "MapLookupFusionRoute");
    assert_eq!(get_decision.selected_i64_const, Some(7));
    assert_eq!(get_decision.selected_bool_const, None);
    assert_eq!(
        get_decision.proof_ids,
        vec![
            "same_receiver_same_i64_key_scalar_get_has",
            "scalar_i64_nonzero"
        ]
    );

    let has_decision = &function.metadata.route_decisions[1];
    assert_eq!(has_decision.site_id, "b4.i12");
    assert_eq!(has_decision.semantic_op, "MapHas");
    assert_eq!(has_decision.access_kind, "map_lookup_same_key_has");
    assert_eq!(has_decision.preferred_route, "map_lookup_const_fold");
    assert_eq!(has_decision.selected_route, "map_lookup_const_fold");
    assert_eq!(has_decision.fallback_route, "generic_map_has_helper");
    assert_eq!(has_decision.fallback_policy, "opportunistic");
    assert_eq!(has_decision.miss_reason, None);
    assert_eq!(has_decision.source_plan_kind, "MapLookupFusionRoute");
    assert_eq!(has_decision.selected_i64_const, None);
    assert_eq!(has_decision.selected_bool_const, Some(true));
}

#[test]
fn route_decision_reports_map_missing_empty_selected_route() {
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
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    function.add_block(block);

    refresh_function_generic_method_routes(&mut function);
    refresh_function_route_decisions(&mut function);

    assert_eq!(function.metadata.route_decisions.len(), 1);
    let decision = &function.metadata.route_decisions[0];
    assert_eq!(decision.site_id, "b0.i2");
    assert_eq!(decision.semantic_op, "MapGet");
    assert_eq!(decision.access_kind, "map_missing_empty_get");
    assert_eq!(decision.preferred_route, "map_get_missing_empty_const_zero");
    assert_eq!(decision.selected_route, "map_get_missing_empty_const_zero");
    assert_eq!(decision.fallback_route, "generic_map_runtime_data_load_any");
    assert_eq!(decision.fallback_policy, "opportunistic");
    assert_eq!(decision.miss_reason, None);
    assert_eq!(decision.source_plan_kind, "MapMissingEmptyRoute");
    assert_eq!(decision.selected_i64_const, Some(0));
    assert_eq!(decision.selected_bool_const, None);
    assert_eq!(decision.receiver_box_name.as_deref(), Some("MapBox"));
    assert!(decision.proof_ids.contains(&"receiver_birth_is_new_mapbox"));
    assert!(decision.proof_ids.contains(&"i64_const_key"));
    assert!(decision
        .proof_ids
        .contains(&"runtime_data_facade_missing_shape"));
}

#[test]
fn route_decision_reports_typed_object_exact_slot_selected_route() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Page".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(1),
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(1),
        field: "used".to_string(),
        value: ValueId::new(3),
        declared_type: Some(MirType::Integer),
    });
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    let mut module = MirModule::new("typed_object_exact_slot_route_test".to_string());
    module
        .metadata
        .typed_object_plans
        .push(typed_object_route_plan());
    module.add_function(function);

    refresh_module_typed_object_exact_slot_route_decisions(&mut module);

    let function = &module.functions["main"];
    assert_eq!(function.metadata.route_decisions.len(), 2);

    let get_decision = &function.metadata.route_decisions[0];
    assert_eq!(get_decision.site_id, "b0.i1");
    assert_eq!(get_decision.semantic_op, "FieldGet");
    assert_eq!(get_decision.source_plan_kind, "TypedObjectExactSlotRoute");
    assert_eq!(
        get_decision.preferred_route,
        "hako.typed_object.slot_load_u64"
    );
    assert_eq!(
        get_decision.selected_route,
        "hako.typed_object.slot_load_u64"
    );
    assert_eq!(
        get_decision.selected_lowering_form,
        Some("exact_helper_bridge")
    );
    assert_eq!(
        get_decision.selected_bridge_symbol,
        Some("hako.object.exact_slot_get_u64_hii")
    );
    assert_eq!(get_decision.selected_slot, Some(0));
    assert_eq!(get_decision.selected_storage, Some("u64"));
    assert_eq!(get_decision.field_id.as_deref(), Some("capacity"));
    assert_eq!(get_decision.receiver_box_name.as_deref(), Some("Page"));
    assert_eq!(get_decision.fallback_policy, "fail_fast");

    let set_decision = &function.metadata.route_decisions[1];
    assert_eq!(set_decision.site_id, "b0.i2");
    assert_eq!(set_decision.semantic_op, "FieldSet");
    assert_eq!(set_decision.source_plan_kind, "TypedObjectExactSlotRoute");
    assert_eq!(
        set_decision.preferred_route,
        "hako.typed_object.slot_store_i64"
    );
    assert_eq!(
        set_decision.selected_route,
        "hako.typed_object.slot_store_i64"
    );
    assert_eq!(
        set_decision.selected_lowering_form,
        Some("exact_helper_bridge")
    );
    assert_eq!(
        set_decision.selected_bridge_symbol,
        Some("hako.object.exact_slot_set_i64_hii")
    );
    assert_eq!(set_decision.selected_slot, Some(1));
    assert_eq!(set_decision.selected_storage, Some("i64"));
    assert_eq!(set_decision.field_id.as_deref(), Some("used"));
    assert_eq!(set_decision.receiver_box_name.as_deref(), Some("Page"));
    assert_eq!(set_decision.fallback_policy, "fail_fast");
}

#[test]
fn route_decision_reports_typed_object_exact_slot_native_direct_when_direct_state_is_ready() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "Page".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(1),
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    let mut module = MirModule::new("typed_object_exact_slot_native_direct_test".to_string());
    module
        .metadata
        .typed_object_plans
        .push(typed_object_route_plan());
    module
        .metadata
        .direct_state_plans
        .push(direct_state_ready_plan());
    module.add_function(function);

    refresh_module_typed_object_exact_slot_route_decisions(&mut module);

    let function = &module.functions["main"];
    assert_eq!(function.metadata.route_decisions.len(), 1);

    let decision = &function.metadata.route_decisions[0];
    assert_eq!(decision.site_id, "b0.i1");
    assert_eq!(decision.semantic_op, "FieldGet");
    assert_eq!(decision.source_plan_kind, "TypedObjectExactSlotRoute");
    assert_eq!(decision.preferred_route, "hako.typed_object.slot_load_u64");
    assert_eq!(decision.selected_route, "hako.typed_object.slot_load_u64");
    assert_eq!(decision.selected_lowering_form, Some("native_direct"));
    assert_eq!(decision.selected_bridge_symbol, None);
    assert_eq!(decision.selected_slot, Some(0));
    assert_eq!(decision.selected_storage, Some("u64"));
    assert_eq!(decision.field_id.as_deref(), Some("capacity"));
    assert_eq!(decision.receiver_box_name.as_deref(), Some("Page"));
    assert_eq!(decision.fallback_policy, "fail_fast");
    assert!(decision
        .proof_ids
        .iter()
        .any(|proof| *proof == "native_direct_ready"));
}

#[test]
fn required_fastpath_region_marks_route_decision_required() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .required_fastpath_regions
        .push(RequiredFastPathRegion {
            region_id: 0,
            source_kind: "diagnostic_mode",
            relevant_access_policy: "direct_memory",
            route_requirement: "fastpath_plan_required",
            bounds_requirement: "checked_allowed",
            fallback_policy: "fail_fast",
        });
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    refresh_function_generic_method_routes(&mut function);
    refresh_function_direct_array_access_plans(&mut function);
    refresh_function_route_decisions(&mut function);

    assert_eq!(function.metadata.route_decisions.len(), 1);
    assert_eq!(
        function.metadata.route_decisions[0].fallback_policy,
        "require_fastpath"
    );
}

#[test]
fn diagnostic_region_marks_route_decision_report_if_slow() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .required_fastpath_regions
        .push(RequiredFastPathRegion {
            region_id: 0,
            source_kind: "diagnostic_mode",
            relevant_access_policy: "direct_memory",
            route_requirement: "fastpath_plan_required",
            bounds_requirement: "checked_allowed",
            fallback_policy: "report_if_slow",
        });
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.set_terminator(MirInstruction::Return { value: None });
    function.add_block(block);

    refresh_function_generic_method_routes(&mut function);
    refresh_function_direct_array_access_plans(&mut function);
    refresh_function_route_decisions(&mut function);

    assert_eq!(function.metadata.route_decisions.len(), 1);
    assert_eq!(
        function.metadata.route_decisions[0].fallback_policy,
        "report_if_slow"
    );
}

#[test]
fn hotcore_call_plan_appends_route_decision() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.runOne/2".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .direct_exact_hotcore_call_plans
        .push(direct_exact_hotcore_call_plan());
    let mut module = MirModule::new("route-decision-hotcore-test".to_string());
    module.add_function(function);

    refresh_module_hotcore_route_decisions(&mut module);

    let function = module.functions.get("Main.runOne/2").expect("function");
    assert_eq!(function.metadata.route_decisions.len(), 1);
    let decision = &function.metadata.route_decisions[0];
    assert_eq!(decision.site_id, "b3.i7");
    assert_eq!(decision.semantic_op, "MethodCall");
    assert_eq!(decision.access_kind, "hotcore_call");
    assert_eq!(decision.preferred_route, "static_exact_call");
    assert_eq!(decision.selected_route, "static_exact_call");
    assert_eq!(decision.fallback_route, "generic_method_dispatch");
    assert_eq!(decision.source_plan_kind, "DirectExactHotCoreCallPlan");
    assert_eq!(
        decision.proof_ids,
        vec!["same_module", "static_exact", "scalar_return"]
    );
}

#[test]
fn direct_exact_required_region_marks_hotcore_route_decision_required() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.runOne/2".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .metadata
        .required_fastpath_regions
        .push(RequiredFastPathRegion {
            region_id: 0,
            source_kind: "diagnostic_mode",
            relevant_access_policy: "direct_exact_call",
            route_requirement: "direct_exact_required",
            bounds_requirement: "checked_allowed",
            fallback_policy: "fail_fast",
        });
    function
        .metadata
        .direct_exact_hotcore_call_plans
        .push(direct_exact_hotcore_call_plan());
    let mut module = MirModule::new("route-decision-direct-exact-required-test".to_string());
    module.add_function(function);

    refresh_module_hotcore_route_decisions(&mut module);

    let function = module.functions.get("Main.runOne/2").expect("function");
    assert_eq!(function.metadata.route_decisions.len(), 1);
    assert_eq!(
        function.metadata.route_decisions[0].fallback_policy,
        "require_direct_exact"
    );
}
