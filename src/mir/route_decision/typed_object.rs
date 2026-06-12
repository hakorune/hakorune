//! Typed-object exact-slot route decision planning.

use super::RouteDecision;
use crate::mir::function::{DirectStatePlan, TypedObjectFieldStorage, TypedObjectPlan};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};

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
