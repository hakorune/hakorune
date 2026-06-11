/*!
 * Planner-owned route decisions.
 *
 * RouteDecisionV0 is a report-only outcome view: planners may prefer fast
 * routes, but MIRBuilder must only preserve origins/spans/types and lowering
 * must consume the selected route instead of re-deciding policy.
 */

use crate::mir::direct_array_access_plan::{DirectArrayAccessOp, DirectArrayAccessPlan};
use crate::mir::direct_exact_hotcore_call_plan::DirectExactHotCoreCallPlan;
use crate::mir::function::{DirectStatePlan, TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::map_lookup_fusion_plan::MapLookupFusionRoute;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDecision {
    pub site_id: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub semantic_op: &'static str,
    pub access_kind: &'static str,
    pub preferred_route: &'static str,
    pub selected_route: &'static str,
    pub fallback_route: &'static str,
    pub fallback_policy: &'static str,
    pub proof_ids: Vec<&'static str>,
    pub miss_reason: Option<&'static str>,
    pub source_plan_kind: &'static str,
    pub selected_i64_const: Option<i64>,
    pub selected_bool_const: Option<bool>,
    pub selected_lowering_form: Option<&'static str>,
    pub selected_bridge_symbol: Option<&'static str>,
    pub selected_slot: Option<u32>,
    pub selected_storage: Option<&'static str>,
    pub receiver_box_name: Option<String>,
    pub field_id: Option<String>,
}

impl RouteDecision {
    fn from_direct_array_access_plan(
        plan: &DirectArrayAccessPlan,
        fallback_policy: &'static str,
    ) -> Self {
        let (semantic_op, access_kind, fallback_route) = match plan.op() {
            DirectArrayAccessOp::Load => (
                "ArrayGet",
                "direct_array_i64_load",
                "generic_array_get_helper",
            ),
            DirectArrayAccessOp::Store => (
                "ArraySet",
                "direct_array_i64_store",
                "generic_array_set_helper",
            ),
        };
        Self {
            site_id: format!("b{}.i{}", plan.block().as_u32(), plan.instruction_index()),
            block: plan.block(),
            instruction_index: plan.instruction_index(),
            semantic_op,
            access_kind,
            preferred_route: plan.route(),
            selected_route: plan.route(),
            fallback_route,
            fallback_policy,
            proof_ids: plan.proof_ids().to_vec(),
            miss_reason: None,
            source_plan_kind: "DirectArrayAccessPlan",
            selected_i64_const: None,
            selected_bool_const: None,
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }

    fn from_direct_exact_hotcore_call_plan(
        plan: &DirectExactHotCoreCallPlan,
        fallback_policy: &'static str,
    ) -> Self {
        let selected_route = if plan.lowering_consumer_enabled {
            "static_exact_call"
        } else {
            "generic_method_dispatch"
        };
        let proof_ids = if plan.lowering_consumer_enabled {
            vec!["same_module", "static_exact", "scalar_return"]
        } else {
            Vec::new()
        };
        Self {
            site_id: format!("b{}.i{}", plan.block.as_u32(), plan.instruction_index),
            block: plan.block,
            instruction_index: plan.instruction_index,
            semantic_op: "MethodCall",
            access_kind: "hotcore_call",
            preferred_route: "static_exact_call",
            selected_route,
            fallback_route: "generic_method_dispatch",
            fallback_policy,
            proof_ids,
            miss_reason: plan.failure_reason,
            source_plan_kind: "DirectExactHotCoreCallPlan",
            selected_i64_const: None,
            selected_bool_const: None,
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }

    fn from_map_lookup_fusion_route(
        plan: &MapLookupFusionRoute,
        fallback_policy: &'static str,
        access_kind: &'static str,
        fallback_route: &'static str,
        instruction_index: usize,
        semantic_op: &'static str,
    ) -> Self {
        let selected_route = if plan.stored_value_proof_tag() != "unknown_scalar" {
            "map_lookup_const_fold"
        } else {
            fallback_route
        };
        let mut proof_ids = vec![plan.proof_tag()];
        if plan.stored_value_proof_tag() != "unknown_scalar" {
            proof_ids.push(plan.stored_value_proof_tag());
        }
        Self {
            site_id: format!("b{}.i{}", plan.block().as_u32(), instruction_index),
            block: plan.block(),
            instruction_index,
            semantic_op,
            access_kind,
            preferred_route: "map_lookup_const_fold",
            selected_route,
            fallback_route,
            fallback_policy,
            proof_ids,
            miss_reason: if selected_route == "map_lookup_const_fold" {
                None
            } else {
                Some("stored_value_proof_missing")
            },
            source_plan_kind: "MapLookupFusionRoute",
            selected_i64_const: if selected_route == "map_lookup_const_fold"
                && semantic_op == "MapGet"
            {
                plan.stored_value_const()
            } else {
                None
            },
            selected_bool_const: if selected_route == "map_lookup_const_fold"
                && semantic_op == "MapHas"
            {
                Some(true)
            } else {
                None
            },
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }
}

pub fn refresh_module_typed_object_exact_slot_route_decisions(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        function
            .metadata
            .route_decisions
            .retain(|decision| decision.source_plan_kind != "TypedObjectExactSlotRoute");
        let decisions = collect_typed_object_exact_slot_route_decisions(
            function,
            &module.metadata.typed_object_plans,
            &module.metadata.direct_state_plans,
        );
        function.metadata.route_decisions.extend(decisions);
        function.metadata.route_decisions.sort_by_key(|decision| {
            (
                decision.block.as_u32(),
                decision.instruction_index,
                decision.source_plan_kind,
                decision.semantic_op,
                decision.access_kind,
            )
        });
    }
}

fn collect_typed_object_exact_slot_route_decisions(
    function: &MirFunction,
    typed_object_plans: &[TypedObjectPlan],
    direct_state_plans: &[DirectStatePlan],
) -> Vec<RouteDecision> {
    let def_map = build_value_def_map(function);
    let mut decisions = Vec::new();
    for block in function.blocks.values() {
        for (instruction_index, spanned_inst) in block.all_spanned_instructions_enumerated() {
            let decision = match spanned_inst.inst {
                MirInstruction::FieldGet {
                    base,
                    field,
                    declared_type,
                    ..
                } => typed_object_exact_slot_route_decision_for_field(
                    function,
                    &def_map,
                    typed_object_plans,
                    direct_state_plans,
                    block.id,
                    instruction_index,
                    "FieldGet",
                    *base,
                    field.as_str(),
                    declared_type.as_ref(),
                ),
                MirInstruction::FieldSet {
                    base,
                    field,
                    declared_type,
                    ..
                } => typed_object_exact_slot_route_decision_for_field(
                    function,
                    &def_map,
                    typed_object_plans,
                    direct_state_plans,
                    block.id,
                    instruction_index,
                    "FieldSet",
                    *base,
                    field.as_str(),
                    declared_type.as_ref(),
                ),
                _ => None,
            };
            if let Some(decision) = decision {
                decisions.push(decision);
            }
        }
    }
    decisions
}

fn typed_object_exact_slot_route_decision_for_field(
    function: &MirFunction,
    def_map: &ValueDefMap,
    typed_object_plans: &[TypedObjectPlan],
    direct_state_plans: &[DirectStatePlan],
    block: BasicBlockId,
    instruction_index: usize,
    semantic_op: &'static str,
    base: ValueId,
    field_name: &str,
    declared_type: Option<&MirType>,
) -> Option<RouteDecision> {
    let receiver_box_name = typed_object_value_box_name(function, def_map, base)?;
    let plan = typed_object_plans
        .iter()
        .find(|plan| plan.box_name == receiver_box_name)?;
    let field_plan = plan.fields.iter().find(|field| field.name == field_name)?;
    if field_plan.is_weak {
        return None;
    }
    let (selected_route, fallback_route, selected_bridge_symbol, selected_storage) =
        typed_object_exact_slot_route_parts(semantic_op, field_plan.storage)?;
    let selected_slot = field_plan.slot;
    let native_direct_ready = typed_object_exact_slot_native_direct_ready_for_field(
        direct_state_plans,
        &receiver_box_name,
        field_name,
        selected_slot,
        selected_storage,
    );
    let mut proof_ids = vec![
        "typed_object_plan",
        "field_decl_authority",
        "receiver_exact_type_id",
        "slot_in_bounds",
        "storage_exact_slot",
        "non_weak_field",
        "materialization_boundary_known",
        "exact_slot_bridge_available",
    ];
    if native_direct_ready {
        proof_ids.push("direct_state_plan_present");
        proof_ids.push("direct_state_plan_field_selected");
        proof_ids.push("direct_state_plan_materialization_boundary_known");
        proof_ids.push("direct_state_plan_positive_net_expected");
        proof_ids.push("native_direct_ready");
    }
    if declared_type.is_some() {
        proof_ids.push("declared_type_present");
    }
    Some(RouteDecision {
        site_id: format!("b{}.i{}", block.as_u32(), instruction_index),
        block,
        instruction_index,
        semantic_op,
        access_kind: typed_object_exact_slot_access_kind(semantic_op, selected_storage),
        preferred_route: selected_route,
        selected_route,
        fallback_route,
        fallback_policy: "fail_fast",
        proof_ids,
        miss_reason: None,
        source_plan_kind: "TypedObjectExactSlotRoute",
        selected_i64_const: None,
        selected_bool_const: None,
        selected_lowering_form: Some(if native_direct_ready {
            "native_direct"
        } else {
            "exact_helper_bridge"
        }),
        selected_bridge_symbol: if native_direct_ready {
            None
        } else {
            Some(selected_bridge_symbol)
        },
        selected_slot: Some(selected_slot),
        selected_storage: Some(selected_storage),
        receiver_box_name: Some(receiver_box_name),
        field_id: Some(field_name.to_string()),
    })
}

fn typed_object_exact_slot_native_direct_ready_for_field(
    direct_state_plans: &[DirectStatePlan],
    receiver_box_name: &str,
    field_name: &str,
    selected_slot: u32,
    selected_storage: &str,
) -> bool {
    direct_state_plans.iter().any(|plan| {
        plan.box_name == receiver_box_name
            && plan.field_decl_authority
            && plan.materialization_boundary_known
            && plan.positive_net_expected
            && plan.fields.iter().any(|field| {
                field.name == field_name
                    && field.slot == selected_slot
                    && typed_object_exact_slot_native_direct_storage_matches(
                        selected_storage,
                        field.storage,
                    )
            })
    })
}

fn typed_object_exact_slot_native_direct_storage_matches(
    selected_storage: &str,
    storage: TypedObjectFieldStorage,
) -> bool {
    match (selected_storage, storage) {
        ("i64", TypedObjectFieldStorage::I64 | TypedObjectFieldStorage::ISize) => true,
        ("u64", TypedObjectFieldStorage::U64 | TypedObjectFieldStorage::USize) => true,
        ("handle", TypedObjectFieldStorage::Handle) => true,
        _ => false,
    }
}

fn typed_object_exact_slot_route_parts(
    semantic_op: &str,
    storage: TypedObjectFieldStorage,
) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    match (semantic_op, storage) {
        ("FieldGet", TypedObjectFieldStorage::I64 | TypedObjectFieldStorage::ISize) => Some((
            "hako.typed_object.slot_load_i64",
            "nyash.object.field_get_i64_hii",
            "hako.object.exact_slot_get_i64_hii",
            "i64",
        )),
        ("FieldSet", TypedObjectFieldStorage::I64 | TypedObjectFieldStorage::ISize) => Some((
            "hako.typed_object.slot_store_i64",
            "nyash.object.field_set_i64_hii",
            "hako.object.exact_slot_set_i64_hii",
            "i64",
        )),
        ("FieldGet", TypedObjectFieldStorage::U64 | TypedObjectFieldStorage::USize) => Some((
            "hako.typed_object.slot_load_u64",
            "nyash.object.field_get_u64_hii",
            "hako.object.exact_slot_get_u64_hii",
            "u64",
        )),
        ("FieldSet", TypedObjectFieldStorage::U64 | TypedObjectFieldStorage::USize) => Some((
            "hako.typed_object.slot_store_u64",
            "nyash.object.field_set_u64_hiu",
            "hako.object.exact_slot_set_u64_hiu",
            "u64",
        )),
        ("FieldGet", TypedObjectFieldStorage::Handle) => Some((
            "hako.typed_object.slot_load_handle",
            "nyash.object.field_get_hii",
            "hako.object.exact_slot_get_handle_hii",
            "handle",
        )),
        ("FieldSet", TypedObjectFieldStorage::Handle) => Some((
            "hako.typed_object.slot_store_handle",
            "nyash.object.field_set_hii",
            "hako.object.exact_slot_set_handle_hii",
            "handle",
        )),
        _ => None,
    }
}

fn typed_object_exact_slot_access_kind(semantic_op: &str, selected_storage: &str) -> &'static str {
    match (semantic_op, selected_storage) {
        ("FieldGet", "i64") => "typed_object_exact_slot_get_i64",
        ("FieldSet", "i64") => "typed_object_exact_slot_set_i64",
        ("FieldGet", "u64") => "typed_object_exact_slot_get_u64",
        ("FieldSet", "u64") => "typed_object_exact_slot_set_u64",
        ("FieldGet", "handle") => "typed_object_exact_slot_get_handle",
        ("FieldSet", "handle") => "typed_object_exact_slot_set_handle",
        _ => "typed_object_exact_slot",
    }
}

fn typed_object_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    function
        .metadata
        .value_types
        .get(&origin)
        .and_then(box_name_from_mir_type)
        .map(str::to_string)
        .or_else(|| {
            def_map
                .get(&origin)
                .and_then(|(block_id, instruction_index)| {
                    let block = function.blocks.get(block_id)?;
                    match block.instructions.get(*instruction_index)? {
                        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
                        MirInstruction::Phi { type_hint, .. } => type_hint
                            .as_ref()
                            .and_then(box_name_from_mir_type)
                            .map(str::to_string),
                        _ => None,
                    }
                })
        })
}

fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

pub fn refresh_function_route_decisions(function: &mut MirFunction) {
    let fallback_policy = direct_memory_route_policy(function);

    let mut decisions = function
        .metadata
        .direct_array_access_plans
        .iter()
        .map(|plan| RouteDecision::from_direct_array_access_plan(plan, fallback_policy))
        .collect::<Vec<_>>();

    let map_lookup_decisions = function
        .metadata
        .map_lookup_fusion_routes
        .iter()
        .flat_map(|plan| {
            [
                RouteDecision::from_map_lookup_fusion_route(
                    plan,
                    "opportunistic",
                    "map_lookup_same_key_get",
                    "generic_map_get_helper",
                    plan.get_instruction_index(),
                    "MapGet",
                ),
                RouteDecision::from_map_lookup_fusion_route(
                    plan,
                    "opportunistic",
                    "map_lookup_same_key_has",
                    "generic_map_has_helper",
                    plan.has_instruction_index(),
                    "MapHas",
                ),
            ]
        });
    decisions.extend(map_lookup_decisions);
    decisions.sort_by_key(|decision| {
        (
            decision.block.as_u32(),
            decision.instruction_index,
            decision.source_plan_kind,
            decision.semantic_op,
            decision.access_kind,
        )
    });
    function.metadata.route_decisions = decisions;
}

pub fn refresh_module_hotcore_route_decisions(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        let fallback_policy = direct_exact_route_policy(function);
        function
            .metadata
            .route_decisions
            .retain(|decision| decision.source_plan_kind != "DirectExactHotCoreCallPlan");
        function.metadata.route_decisions.extend(
            function
                .metadata
                .direct_exact_hotcore_call_plans
                .iter()
                .map(|plan| {
                    RouteDecision::from_direct_exact_hotcore_call_plan(plan, fallback_policy)
                }),
        );
    }
}

fn direct_memory_route_policy(function: &MirFunction) -> &'static str {
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            region.relevant_access_policy == "direct_memory"
                && region.route_requirement == "fastpath_plan_required"
                && region.fallback_policy == "fail_fast"
        })
    {
        return "require_fastpath";
    }
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            region.relevant_access_policy == "direct_memory"
                && region.fallback_policy == "report_if_slow"
        })
    {
        return "report_if_slow";
    }
    "opportunistic"
}

fn direct_exact_route_policy(function: &MirFunction) -> &'static str {
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            is_direct_exact_region(region.relevant_access_policy)
                && is_direct_exact_requirement(region.route_requirement)
                && region.fallback_policy == "fail_fast"
        })
    {
        return "require_direct_exact";
    }
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            is_direct_exact_region(region.relevant_access_policy)
                && region.fallback_policy == "report_if_slow"
        })
    {
        return "report_if_slow";
    }
    "opportunistic"
}

fn is_direct_exact_region(relevant_access_policy: &str) -> bool {
    matches!(
        relevant_access_policy,
        "direct_exact" | "direct_exact_call" | "hotcore_call"
    )
}

fn is_direct_exact_requirement(route_requirement: &str) -> bool {
    matches!(
        route_requirement,
        "direct_exact_required" | "static_exact_call_required" | "fastpath_plan_required"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans;
    use crate::mir::function::{
        DirectStateFieldPlan, DirectStatePlan, RequiredFastPathRegion, TypedObjectFieldPlan,
        TypedObjectFieldStorage, TypedObjectPlan,
    };
    use crate::mir::generic_method_route_plan::refresh_function_generic_method_routes;
    use crate::mir::{
        BasicBlock, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType, ValueId,
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
        module.metadata.direct_state_plans.push(direct_state_ready_plan());
        module.add_function(function);

        refresh_module_typed_object_exact_slot_route_decisions(&mut module);

        let function = &module.functions["main"];
        assert_eq!(function.metadata.route_decisions.len(), 1);

        let decision = &function.metadata.route_decisions[0];
        assert_eq!(decision.site_id, "b0.i1");
        assert_eq!(decision.semantic_op, "FieldGet");
        assert_eq!(decision.source_plan_kind, "TypedObjectExactSlotRoute");
        assert_eq!(
            decision.preferred_route,
            "hako.typed_object.slot_load_u64"
        );
        assert_eq!(decision.selected_route, "hako.typed_object.slot_load_u64");
        assert_eq!(decision.selected_lowering_form, Some("native_direct"));
        assert_eq!(decision.selected_bridge_symbol, None);
        assert_eq!(decision.selected_slot, Some(0));
        assert_eq!(decision.selected_storage, Some("u64"));
        assert_eq!(decision.field_id.as_deref(), Some("capacity"));
        assert_eq!(decision.receiver_box_name.as_deref(), Some("Page"));
        assert_eq!(decision.fallback_policy, "fail_fast");
        assert!(
            decision
                .proof_ids
                .iter()
                .any(|proof| *proof == "native_direct_ready")
        );
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
}
