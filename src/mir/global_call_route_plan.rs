/*!
 * MIR-owned route plans for unsupported global user calls.
 *
 * This module does not make global calls lowerable. It records the typed
 * owner boundary in MIR metadata so backend shims can fail-fast from a plan
 * instead of rediscovering unsupported `Global(...)` names from raw MIR.
 */

use super::{BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirModule, MirType};
use std::collections::BTreeMap;

mod box_type_inspector_describe_body;
mod builder_registry_dispatch_body;
mod generic_i64_body;
mod generic_string_abi;
mod generic_string_body;
mod generic_string_corridor;
mod generic_string_facts;
mod generic_string_guards;
mod generic_string_reject;
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

use box_type_inspector_describe_body::{
    box_type_inspector_describe_body_reject_reason, is_box_type_inspector_describe_body_candidate,
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
pub use model::{
    GlobalCallRoute, GlobalCallRouteSite, GlobalCallTargetFacts, GlobalCallTargetShape,
};
use model::{GlobalCallShapeBlocker, GlobalCallTargetClassification, GlobalCallTargetShapeReason};
use parser_program_json_body::is_parser_program_json_body_function;
use pattern_util_local_value_probe_body::is_pattern_util_local_value_probe_body_function;
use program_json_emit_body::is_program_json_emit_body_function;
use static_string_array_body::is_static_string_array_body_function;

fn supported_backend_global(name: &str) -> bool {
    matches!(name, "print")
}

fn string_or_void_sentinel_return_type_candidate(return_type: &MirType) -> bool {
    matches!(
        return_type,
        MirType::Integer | MirType::Void | MirType::Unknown | MirType::String
    ) || matches!(return_type, MirType::Box(name) if name == "StringBox")
}

pub fn refresh_module_global_call_routes(module: &mut MirModule) {
    let targets = collect_global_call_targets(module);
    for function in module.functions.values_mut() {
        refresh_function_global_call_routes_with_targets(function, &targets);
    }
}

pub fn refresh_function_global_call_routes(function: &mut MirFunction) {
    refresh_function_global_call_routes_with_targets(function, &BTreeMap::new());
}

fn collect_global_call_targets(module: &MirModule) -> BTreeMap<String, GlobalCallTargetFacts> {
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
            let classification = classify_global_call_target_shape(function, &targets);
            if current.shape() != classification.shape
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
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::PatternUtilLocalValueProbeBody,
        );
    }
    if is_parser_program_json_body_function(function) {
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::ParserProgramJsonBody,
        );
    }
    if is_program_json_emit_body_function(function) {
        return GlobalCallTargetClassification::direct(GlobalCallTargetShape::ProgramJsonEmitBody);
    }
    if is_jsonfrag_instruction_array_normalizer_body_function(function) {
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody,
        );
    }
    if is_static_string_array_body_function(function) {
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::StaticStringArrayBody,
        );
    }
    if is_box_type_inspector_describe_body_candidate(function) {
        if let Some(reject) = box_type_inspector_describe_body_reject_reason(function) {
            return GlobalCallTargetClassification::unknown(reject.reason);
        }
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::BoxTypeInspectorDescribeBody,
        );
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
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::MirSchemaMapConstructorBody,
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
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::BuilderRegistryDispatchBody,
        );
    }
    if string_or_void_sentinel_return_type_candidate(&function.signature.return_type) {
        if let Some(reject) = generic_string_void_sentinel_body_reject_reason(function, targets) {
            if reject.reason
                == GlobalCallTargetShapeReason::GenericStringReturnVoidSentinelCandidate
                && reject.blocker.is_none()
            {
                return GlobalCallTargetClassification::direct(
                    GlobalCallTargetShape::GenericStringOrVoidSentinelBody,
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
        return GlobalCallTargetClassification::direct(
            GlobalCallTargetShape::GenericStringVoidLoggingBody,
        );
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
            routes.push(GlobalCallRoute::new(
                GlobalCallRouteSite::new(block_id, instruction_index),
                name,
                args.len(),
                *dst,
                lookup_global_call_target(name, targets)
                    .cloned()
                    .unwrap_or_else(GlobalCallTargetFacts::missing),
            ));
        }
    }

    function.metadata.global_call_routes = routes;
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
