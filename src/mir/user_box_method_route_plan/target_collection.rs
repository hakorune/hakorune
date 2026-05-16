use std::collections::{BTreeMap, BTreeSet};

use super::origin_inference::{box_name_from_type, route_result_box_name, value_box_name};
use super::return_shape::{
    infer_user_box_method_return, UserBoxFieldReturnHints, UserBoxMethodInferredReturn,
};
use crate::mir::same_module_body_shape::same_module_body_supported;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct UserBoxMethodTargetFacts {
    pub(super) box_name: String,
    pub(super) arity: usize,
    pub(super) return_type: MirType,
    pub(super) inferred_return: Option<UserBoxMethodInferredReturn>,
    pub(super) result_box_name: Option<String>,
    pub(super) body_supported: bool,
}

pub(super) fn collect_method_targets(
    module: &MirModule,
    typed_plan_type_ids: &BTreeMap<String, u32>,
    field_return_hints: &UserBoxFieldReturnHints,
) -> BTreeMap<String, UserBoxMethodTargetFacts> {
    module
        .functions
        .iter()
        .filter_map(|(name, function)| {
            let (box_name, _method, arity) = parse_method_symbol(name)?;
            if !module.metadata.user_box_decls.contains_key(box_name)
                && !module.metadata.user_box_field_decls.contains_key(box_name)
            {
                return None;
            }
            let target_arity = if function.params.is_empty() {
                function.signature.params.len()
            } else {
                function.params.len()
            };
            // Method symbols encode only explicit arguments; receiver is an
            // extra uniform ABI parameter at index 0.
            if target_arity != arity + 1 {
                return None;
            }
            Some((
                name.clone(),
                UserBoxMethodTargetFacts {
                    box_name: box_name.to_string(),
                    arity: target_arity,
                    return_type: function.signature.return_type.clone(),
                    inferred_return: infer_user_box_method_return(function, field_return_hints),
                    result_box_name: infer_user_box_method_result_box_name(function),
                    body_supported: same_module_body_supported(function, typed_plan_type_ids),
                },
            ))
        })
        .collect()
}

pub(super) fn parse_method_symbol(name: &str) -> Option<(&str, &str, usize)> {
    let (owner_and_method, arity_s) = name.rsplit_once('/')?;
    let (box_name, method) = owner_and_method.rsplit_once('.')?;
    let arity = arity_s.parse::<usize>().ok()?;
    Some((box_name, method, arity))
}

pub(super) fn method_target_symbol(box_name: &str, method: &str, arity: usize) -> String {
    format!("{box_name}.{method}/{arity}")
}

fn infer_user_box_method_result_box_name(function: &MirFunction) -> Option<String> {
    let def_map = build_value_def_map(function);
    let mut result = None;
    let mut saw_box = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Return { value: Some(value) } = instruction else {
                continue;
            };
            let mut visiting = BTreeSet::new();
            let box_name = return_value_box_name(function, &def_map, *value, &mut visiting)?;
            let Some(box_name) = box_name else {
                continue;
            };
            saw_box = true;
            match &result {
                None => result = Some(box_name),
                Some(existing) if existing == &box_name => {}
                Some(_) => return None,
            }
        }
    }
    if saw_box {
        result
    } else {
        None
    }
}

fn return_value_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<Option<String>> {
    let origin = resolve_value_origin(function, def_map, value);
    if !visiting.insert(origin) {
        return Some(None);
    }
    let result = return_value_box_name_inner(function, def_map, origin, visiting);
    visiting.remove(&origin);
    result
}

fn return_value_box_name_inner(
    function: &MirFunction,
    def_map: &ValueDefMap,
    origin: ValueId,
    visiting: &mut BTreeSet<ValueId>,
) -> Option<Option<String>> {
    if let Some(box_name) = value_box_name(function, origin) {
        return Some(Some(box_name.to_string()));
    }
    if let Some(box_name) = route_result_box_name(function, origin) {
        return Some(Some(box_name.to_string()));
    }
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return None;
    };
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::Const {
            value: ConstValue::Null | ConstValue::Void,
            ..
        } => Some(None),
        MirInstruction::NewBox { box_type, .. } => Some(Some(box_type.clone())),
        MirInstruction::Phi {
            inputs, type_hint, ..
        } => {
            if let Some(box_name) = type_hint.as_ref().and_then(box_name_from_type) {
                return Some(Some(box_name.to_string()));
            }
            let mut result = None;
            for (_incoming_block, incoming_value) in inputs {
                let box_name = return_value_box_name(function, def_map, *incoming_value, visiting)?;
                let Some(box_name) = box_name else {
                    continue;
                };
                match &result {
                    None => result = Some(box_name),
                    Some(existing) if existing == &box_name => {}
                    Some(_) => return None,
                }
            }
            Some(result)
        }
        MirInstruction::Select {
            then_val, else_val, ..
        } => merge_return_value_box_names(
            return_value_box_name(function, def_map, *then_val, visiting)?,
            return_value_box_name(function, def_map, *else_val, visiting)?,
        ),
        _ => None,
    }
}

fn merge_return_value_box_names(
    left: Option<String>,
    right: Option<String>,
) -> Option<Option<String>> {
    match (left, right) {
        (Some(left), Some(right)) if left == right => Some(Some(left)),
        (Some(_), Some(_)) => None,
        (Some(name), None) | (None, Some(name)) => Some(Some(name)),
        (None, None) => Some(None),
    }
}
