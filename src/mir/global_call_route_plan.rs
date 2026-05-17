/*!
 * MIR-owned route plans for unsupported global user calls.
 *
 * This module does not make global calls lowerable. It records the typed
 * owner boundary in MIR metadata so backend shims can fail-fast from a plan
 * instead of rediscovering unsupported `Global(...)` names from raw MIR.
 */

use super::{
    BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::{BTreeMap, BTreeSet};

mod box_type_inspector_describe_body;
mod builder_registry_dispatch_body;
mod generic_i64_body;
mod generic_string_abi;
mod generic_string_body;
mod generic_string_body_analysis;
mod generic_string_corridor;
mod generic_string_facts;
mod generic_string_guards;
mod generic_string_reject;
mod generic_string_route_value_class;
mod generic_string_surface;
mod jsonfrag_normalizer_body;
mod mir_schema_map_constructor_body;
mod model;
mod parser_program_json_body;
mod pattern_util_local_value_probe_body;
mod program_json_emit_body;
mod shape_blocker;
mod static_string_array_body;
mod string_return_profile;
mod type_label;
mod value_type_publish;
mod void_side_effect_body;

use box_type_inspector_describe_body::{
    box_type_inspector_describe_body_reject_reason, box_type_inspector_describe_classification,
    is_box_type_inspector_describe_body_candidate,
};
use builder_registry_dispatch_body::{
    builder_registry_dispatch_body_reject_reason, is_builder_registry_dispatch_body_candidate,
};
use generic_i64_body::is_generic_i64_body_function;
use generic_string_body::{
    generic_pure_string_body_reject_reason, generic_string_void_logging_body_reject_reason,
    generic_string_void_sentinel_body_reject_reason,
};
use jsonfrag_normalizer_body::is_jsonfrag_instruction_array_normalizer_body_function;
use mir_schema_map_constructor_body::{
    is_mir_schema_map_constructor_body_candidate, mir_schema_map_constructor_body_reject_reason,
};
use model::{
    GlobalCallLoweringOverride, GlobalCallProof, GlobalCallReturnContract, GlobalCallShapeBlocker,
    GlobalCallTargetClassification, GlobalCallTargetShapeReason,
};
pub use model::{
    GlobalCallRoute, GlobalCallRouteSite, GlobalCallTargetFacts, GlobalCallTargetShape,
};
use parser_program_json_body::is_parser_program_json_body_function;
use pattern_util_local_value_probe_body::{
    is_pattern_util_local_value_probe_body_function, pattern_util_probe_body_classification,
};
use program_json_emit_body::is_program_json_emit_body_function;
use static_string_array_body::is_static_string_array_body_function;
use value_type_publish::{
    propagate_global_call_box_value_types, publish_global_call_route_param_value_types,
};
use void_side_effect_body::is_void_side_effect_body_function;

use crate::mir::same_module_body_shape::{same_module_body_supported, supported_backend_global};

fn string_or_void_sentinel_return_type_candidate(return_type: &MirType) -> bool {
    matches!(
        return_type,
        MirType::Integer | MirType::Void | MirType::Unknown | MirType::String
    ) || matches!(return_type, MirType::Box(name) if name == "StringBox")
}

pub fn refresh_module_global_call_routes(module: &mut MirModule) {
    let typed_plan_type_ids = module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| (plan.box_name.clone(), plan.type_id))
        .collect::<BTreeMap<_, _>>();
    for _ in 0..module.functions.len().saturating_mul(4).max(8) {
        let before = module
            .functions
            .iter()
            .map(|(name, function)| (name.clone(), function.metadata.global_call_routes.clone()))
            .collect::<BTreeMap<_, _>>();
        let targets = collect_global_call_targets(module, &typed_plan_type_ids);
        for function in module.functions.values_mut() {
            refresh_function_global_call_routes_with_targets(function, &targets);
        }
        let param_value_type_changed = publish_global_call_route_param_value_types(module);
        let propagated_value_type_changed = propagate_global_call_box_value_types(module);
        let route_changed = module.functions.iter().any(|(name, function)| {
            before.get(name).map_or(true, |routes| {
                routes != &function.metadata.global_call_routes
            })
        });
        if !(route_changed || param_value_type_changed || propagated_value_type_changed) {
            break;
        }
    }
}

pub fn refresh_function_global_call_routes(function: &mut MirFunction) {
    refresh_function_global_call_routes_with_targets(function, &BTreeMap::new());
}

fn collect_global_call_targets(
    module: &MirModule,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> BTreeMap<String, GlobalCallTargetFacts> {
    let mut targets = module
        .functions
        .iter()
        .map(|(name, function)| {
            let arity = if function.params.is_empty() {
                function.signature.params.len()
            } else {
                function.params.len()
            };
            (
                name.clone(),
                GlobalCallTargetFacts::present_with_symbol_and_return_type(
                    name.clone(),
                    arity,
                    function.signature.return_type.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut function_names = module.functions.keys().collect::<Vec<_>>();
    function_names.sort();
    for _ in 0..module.functions.len() {
        let mut changed = false;
        for name in &function_names {
            let name = *name;
            let Some(function) = module.functions.get(name) else {
                continue;
            };
            let Some(current) = targets.get(name).cloned() else {
                continue;
            };
            let classification =
                classify_global_call_target_shape(function, &targets, typed_plan_type_ids);
            if current.shape() != classification.shape
                || current.return_contract() != classification.return_contract
                || current.proof() != classification.proof
                || current.shape_reason() != classification.reason
                || current.shape_blocker != classification.blocker
            {
                targets.insert(name.clone(), current.with_classification(classification));
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    targets
}

fn classify_global_call_target_shape(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> GlobalCallTargetClassification {
    if function.params.len() != function.signature.params.len() {
        return GlobalCallTargetClassification::unknown(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        );
    }
    if function
        .signature
        .params
        .iter()
        .all(|ty| *ty == MirType::Integer)
        && function.signature.return_type == MirType::Integer
        && is_numeric_i64_leaf_function(function)
    {
        return GlobalCallTargetClassification::direct(GlobalCallTargetShape::NumericI64Leaf);
    }
    if is_generic_i64_body_function(function, targets) {
        return GlobalCallTargetClassification::direct(GlobalCallTargetShape::GenericI64Body);
    }
    if is_pattern_util_local_value_probe_body_function(function, targets) {
        return pattern_util_probe_body_classification();
    }
    if is_parser_program_json_body_function(function) {
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::ParserProgramJson,
            GlobalCallReturnContract::StringHandle,
        );
    }
    if is_program_json_emit_body_function(function) {
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::GenericPureStringBody,
        );
    }
    if is_jsonfrag_instruction_array_normalizer_body_function(function) {
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::GenericPureStringBody,
        );
    }
    if is_static_string_array_body_function(function) {
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::StaticStringArray,
            GlobalCallReturnContract::ArrayHandle,
        );
    }
    if is_box_type_inspector_describe_body_candidate(function) {
        if let Some(reject) = box_type_inspector_describe_body_reject_reason(function) {
            return GlobalCallTargetClassification::unknown(reject.reason);
        }
        return box_type_inspector_describe_classification();
    }
    if is_mir_schema_map_constructor_body_candidate(function, targets) {
        if let Some(reject) = mir_schema_map_constructor_body_reject_reason(function, targets) {
            return if let Some(blocker) = reject.blocker {
                GlobalCallTargetClassification::unknown_with_blocker(
                    reject.reason,
                    blocker.symbol,
                    blocker.reason,
                )
            } else {
                GlobalCallTargetClassification::unknown(
                    GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
                )
            };
        }
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::MirSchemaMapConstructor,
            GlobalCallReturnContract::MapHandle,
        );
    }
    if is_builder_registry_dispatch_body_candidate(function) {
        if let Some(reject) = builder_registry_dispatch_body_reject_reason(function, targets) {
            return if let Some(blocker) = reject.blocker {
                GlobalCallTargetClassification::unknown_with_blocker(
                    reject.reason,
                    blocker.symbol,
                    blocker.reason,
                )
            } else {
                GlobalCallTargetClassification::unknown(reject.reason)
            };
        }
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::GenericStringOrVoidSentinel,
            GlobalCallReturnContract::StringHandleOrNull,
        );
    }
    if string_or_void_sentinel_return_type_candidate(&function.signature.return_type) {
        if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets) {
            if reject.reason
                == GlobalCallTargetShapeReason::GenericStringReturnVoidSentinelCandidate
                && reject.blocker.is_none()
            {
                return GlobalCallTargetClassification::direct_contract(
                    GlobalCallProof::GenericStringOrVoidSentinel,
                    GlobalCallReturnContract::StringHandleOrNull,
                );
            }
            return if let Some(blocker) = reject.blocker {
                GlobalCallTargetClassification::unknown_with_blocker(
                    reject.reason,
                    blocker.symbol,
                    blocker.reason,
                )
            } else {
                GlobalCallTargetClassification::unknown(reject.reason)
            };
        }
    }
    if function.signature.return_type == MirType::Void
        && generic_string_void_logging_body_reject_reason(function, targets).is_none()
    {
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::GenericStringVoidLogging,
            GlobalCallReturnContract::VoidSentinelI64Zero,
        );
    }
    if is_void_side_effect_body_function(function, targets) {
        return GlobalCallTargetClassification::direct_contract(
            GlobalCallProof::VoidSideEffect,
            GlobalCallReturnContract::VoidSentinelI64Zero,
        );
    }
    if same_module_body_supported(function, typed_plan_type_ids) {
        if let Some((proof, return_contract)) =
            infer_same_module_static_helper_return_contract(function, typed_plan_type_ids)
        {
            if same_module_static_helper_contract_allowed(
                function,
                return_contract,
                typed_plan_type_ids,
            ) {
                return GlobalCallTargetClassification::direct_contract(proof, return_contract);
            }
        }
    }
    if let Some(reject) = generic_pure_string_body_reject_reason(function, targets) {
        if let Some(blocker) = reject.blocker {
            GlobalCallTargetClassification::unknown_with_blocker(
                reject.reason,
                blocker.symbol,
                blocker.reason,
            )
        } else {
            GlobalCallTargetClassification::unknown(reject.reason)
        }
    } else {
        GlobalCallTargetClassification::direct(GlobalCallTargetShape::GenericPureStringBody)
    }
}

fn infer_same_module_static_helper_return_contract(
    function: &MirFunction,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> Option<(GlobalCallProof, GlobalCallReturnContract)> {
    let mut inferred = same_module_static_helper_return_type_contract(
        &function.signature.return_type,
        typed_plan_type_ids,
    );
    let mut copy_sources = BTreeMap::new();
    let mut result_contracts = BTreeMap::new();

    for route in &function.metadata.user_box_method_routes {
        if route.reason().is_none() {
            if let Some(value) = route.result_value() {
                if let Some(contract) = same_module_static_helper_route_return_contract(
                    route.return_shape(),
                    route.target_result_box_name(),
                ) {
                    result_contracts.insert(value, contract);
                }
            }
        }
    }

    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Copy { dst, src } => {
                    copy_sources.insert(*dst, *src);
                }
                MirInstruction::Const { dst, value } => {
                    if let Some(contract) = same_module_static_helper_const_return_contract(value) {
                        result_contracts.insert(*dst, contract);
                    }
                }
                MirInstruction::Return { value } => {
                    let contract = match value {
                        Some(value) => same_module_static_helper_value_contract(
                            *value,
                            typed_plan_type_ids,
                            &copy_sources,
                            &result_contracts,
                            &function.metadata.value_types,
                        ),
                        None => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
                    };
                    inferred = merge_same_module_static_helper_contract(inferred, contract)?;
                }
                _ => {}
            }
        }
    }

    inferred.map(|contract| (same_module_static_helper_contract_proof(contract), contract))
}

fn same_module_static_helper_return_type_contract(
    return_type: &MirType,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> Option<GlobalCallReturnContract> {
    match return_type {
        MirType::Integer | MirType::Bool => Some(GlobalCallReturnContract::ScalarI64),
        MirType::Void => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        MirType::Box(name) if typed_plan_type_ids.contains_key(name) => {
            Some(GlobalCallReturnContract::ObjectHandle)
        }
        _ => None,
    }
}

fn same_module_static_helper_route_return_contract(
    return_shape: Option<&str>,
    target_result_box_name: Option<&str>,
) -> Option<GlobalCallReturnContract> {
    match return_shape {
        Some("scalar_i64") => Some(GlobalCallReturnContract::ScalarI64),
        Some("void_sentinel_i64_zero") => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        Some("object_handle") if target_result_box_name.is_some() => {
            Some(GlobalCallReturnContract::ObjectHandle)
        }
        _ => None,
    }
}

fn same_module_static_helper_const_return_contract(
    value: &ConstValue,
) -> Option<GlobalCallReturnContract> {
    match value {
        ConstValue::Integer(_) | ConstValue::Bool(_) => Some(GlobalCallReturnContract::ScalarI64),
        ConstValue::Void => Some(GlobalCallReturnContract::VoidSentinelI64Zero),
        _ => None,
    }
}

fn same_module_static_helper_value_contract(
    value: ValueId,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    copy_sources: &BTreeMap<ValueId, ValueId>,
    result_contracts: &BTreeMap<ValueId, GlobalCallReturnContract>,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Option<GlobalCallReturnContract> {
    let mut current = value;
    for _ in 0..32 {
        if let Some(contract) = result_contracts.get(&current) {
            return Some(*contract);
        }
        if let Some(contract) = value_types
            .get(&current)
            .and_then(|ty| same_module_static_helper_return_type_contract(ty, typed_plan_type_ids))
        {
            return Some(contract);
        }
        let Some(next) = copy_sources.get(&current).copied() else {
            return None;
        };
        if next == current {
            return None;
        }
        current = next;
    }
    None
}

fn merge_same_module_static_helper_contract(
    current: Option<GlobalCallReturnContract>,
    next: Option<GlobalCallReturnContract>,
) -> Option<Option<GlobalCallReturnContract>> {
    match (current, next) {
        (None, Some(next)) => Some(Some(next)),
        (Some(current), Some(next)) if current == next => Some(Some(current)),
        (Some(current), None) => Some(Some(current)),
        (None, None) => Some(None),
        (Some(_), Some(_)) => None,
    }
}

fn same_module_static_helper_contract_proof(contract: GlobalCallReturnContract) -> GlobalCallProof {
    match contract {
        GlobalCallReturnContract::ScalarI64 => GlobalCallProof::SameModuleScalarI64,
        GlobalCallReturnContract::VoidSentinelI64Zero => GlobalCallProof::SameModuleVoidSentinel,
        GlobalCallReturnContract::ObjectHandle => GlobalCallProof::SameModuleObjectHandle,
        _ => GlobalCallProof::ContractMissing,
    }
}

fn same_module_static_helper_contract_allowed(
    function: &MirFunction,
    contract: GlobalCallReturnContract,
    typed_plan_type_ids: &BTreeMap<String, u32>,
) -> bool {
    match contract {
        GlobalCallReturnContract::ObjectHandle => matches!(
            function.signature.return_type,
            MirType::Box(ref name) if typed_plan_type_ids.contains_key(name)
        ),
        GlobalCallReturnContract::ScalarI64 | GlobalCallReturnContract::VoidSentinelI64Zero => {
            same_module_body_has_known_user_defined_method_call(function)
        }
        _ => false,
    }
}

fn same_module_body_has_known_user_defined_method_call(function: &MirFunction) -> bool {
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(known_user_defined_method_instruction)
    })
}

fn known_user_defined_method_instruction(instruction: &MirInstruction) -> bool {
    matches!(
        instruction,
        MirInstruction::Call {
            callee: Some(Callee::Method {
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
                ..
            }),
            ..
        }
    )
}

fn is_numeric_i64_leaf_function(function: &MirFunction) -> bool {
    if function.blocks.len() != 1 {
        return false;
    }
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return false;
    };
    matches!(
        block.terminator,
        Some(MirInstruction::Return { value: Some(_) })
    ) && block
        .instructions
        .iter()
        .all(is_numeric_i64_leaf_instruction)
}

fn is_numeric_i64_leaf_instruction(instruction: &MirInstruction) -> bool {
    match instruction {
        MirInstruction::Const {
            value: ConstValue::Integer(_),
            ..
        } => true,
        MirInstruction::Copy { .. } => true,
        MirInstruction::BinOp { op, .. } => matches!(
            op,
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        ),
        _ => false,
    }
}

fn refresh_function_global_call_routes_with_targets(
    function: &mut MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) {
    let mut routes = Vec::new();
    let const_null_sentinels = collect_const_null_sentinels(function);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            if supported_backend_global(name) {
                continue;
            }
            routes.push(
                GlobalCallRoute::new(
                    GlobalCallRouteSite::new(block_id, instruction_index),
                    name,
                    args.len(),
                    *dst,
                    lookup_global_call_target(name, targets)
                        .cloned()
                        .unwrap_or_else(GlobalCallTargetFacts::missing),
                )
                .with_optional_lowering_override(
                    classify_global_call_lowering_override(name, args, &const_null_sentinels),
                ),
            );
        }
    }

    function.metadata.global_call_routes = routes;
}

fn collect_const_null_sentinels(function: &MirFunction) -> BTreeSet<ValueId> {
    let mut nulls = BTreeSet::new();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if let MirInstruction::Const {
                dst,
                value: ConstValue::Null | ConstValue::Void,
            } = instruction
            {
                nulls.insert(*dst);
            }
        }
    }
    nulls
}

fn classify_global_call_lowering_override(
    name: &str,
    args: &[ValueId],
    const_null_sentinels: &BTreeSet<ValueId>,
) -> Option<GlobalCallLoweringOverride> {
    match name {
        "BuildBox.emit_program_json_v0/2"
            if args.len() == 2 && const_null_sentinels.contains(&args[1]) =>
        {
            Some(GlobalCallLoweringOverride::Stage1EmitProgramJson)
        }
        _ => None,
    }
}

fn lookup_global_call_target<'a>(
    name: &str,
    targets: &'a BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<&'a GlobalCallTargetFacts> {
    if let Some(target) = targets.get(name) {
        return Some(target);
    }
    let canonical = crate::mir::naming::normalize_static_global_name(name);
    if canonical == name {
        return None;
    }
    targets.get(&canonical)
}

#[cfg(test)]
mod tests;
