use std::collections::{BTreeMap, BTreeSet};

use crate::mir::extern_call_route_plan::{
    classify_extern_call_route, is_hostbridge_extern_invoke_symbol, ExternCallRouteKind,
};
use crate::mir::same_module_body_shape::supported_backend_global;
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, UnaryOp,
    ValueId,
};

use super::generic_string_facts::{
    generic_pure_value_class_from_type, generic_pure_value_class_is_void_like,
    generic_pure_void_sentinel_compare_is_supported, set_guarded_non_void_array_value_class,
    set_guarded_non_void_map_value_class, set_guarded_non_void_scalar_value_class,
    set_guarded_non_void_string_value_class, set_proven_flow_value_class,
    set_string_handle_value_class, set_value_class, update_generic_pure_string_return_param_values,
    value_class, GenericPureValueClass,
};
use super::generic_string_reject::GenericPureStringReject;
use super::generic_string_route_value_class::{
    generic_pure_select_value_class, generic_pure_string_generic_i64_target_value_class,
    generic_pure_string_route_value_class,
};
use super::generic_string_surface::{
    generic_pure_compare_proves_i64, generic_pure_string_accepts_array_len_method,
    generic_pure_string_accepts_array_push_method,
    generic_pure_string_accepts_collection_birth_method,
    generic_pure_string_accepts_contains_method, generic_pure_string_accepts_env_set,
    generic_pure_string_accepts_indexof_method, generic_pure_string_accepts_lastindexof_method,
    generic_pure_string_accepts_length_method, generic_pure_string_accepts_map_set_method,
    generic_pure_string_accepts_string_compare, generic_pure_string_accepts_substring_method,
    generic_pure_string_compare_can_infer_string, generic_pure_string_global_name_is_self,
};
use super::model::{
    GlobalCallReturnContract, GlobalCallTargetFacts, GlobalCallTargetShape,
    GlobalCallTargetShapeReason,
};
use super::shape_blocker::propagated_unknown_global_target_blocker;

pub(super) fn generic_pure_string_instruction_reject_reason(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericPureValueClass>,
    return_param_values: &mut BTreeSet<ValueId>,
    has_string_surface: &mut bool,
    has_void_sentinel_const: &mut bool,
    non_void_string_values: &BTreeSet<ValueId>,
    changed: &mut bool,
) -> Option<GenericPureStringReject> {
    let current_function_name = function.signature.name.as_str();
    update_generic_pure_string_return_param_values(instruction, return_param_values, changed);
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => {
                    *has_string_surface = true;
                    GenericPureValueClass::String
                }
                ConstValue::Integer(_) => GenericPureValueClass::I64,
                ConstValue::Bool(_) => GenericPureValueClass::Bool,
                ConstValue::Null | ConstValue::Void => {
                    *has_void_sentinel_const = true;
                    GenericPureValueClass::VoidSentinel
                }
                _ => GenericPureValueClass::Unknown,
            };
            set_value_class(values, *dst, class, changed);
            if class != GenericPureValueClass::Unknown {
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
            ))
        }
        MirInstruction::Copy { dst, src } => {
            let class = value_class(values, *src);
            if class != GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *dst, class, changed);
            } else {
                let dst_class = value_class(values, *dst);
                if dst_class != GenericPureValueClass::Unknown {
                    set_value_class(values, *src, dst_class, changed);
                }
            }
            None
        }
        MirInstruction::NewBox {
            dst,
            box_type,
            args,
        } => {
            if !args.is_empty() {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            let class = match box_type.as_str() {
                "ArrayBox" => GenericPureValueClass::Array,
                "MapBox" => GenericPureValueClass::Map,
                _ => {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ));
                }
            };
            set_value_class(values, *dst, class, changed);
            None
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            if *op != BinaryOp::Add
                && *op != BinaryOp::Sub
                && *op != BinaryOp::Mul
                && *op != BinaryOp::Div
                && *op != BinaryOp::Mod
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if *op == BinaryOp::Add
                && (lhs_class == GenericPureValueClass::String
                    || rhs_class == GenericPureValueClass::String)
            {
                *has_string_surface = true;
                if lhs_class == GenericPureValueClass::Unknown
                    && generic_pure_string_operand_allows_string_inference(function, *lhs)
                {
                    set_string_handle_value_class(values, *lhs, changed);
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && generic_pure_string_operand_allows_string_inference(function, *rhs)
                {
                    set_string_handle_value_class(values, *rhs, changed);
                }
                set_string_handle_value_class(values, *dst, changed);
                return None;
            }
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return None;
            }
            if lhs_class == GenericPureValueClass::VoidSentinel
                || rhs_class == GenericPureValueClass::VoidSentinel
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                ));
            }
            let class = if *op == BinaryOp::Add {
                GenericPureValueClass::I64
            } else if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                GenericPureValueClass::I64
            };
            set_value_class(values, *dst, class, changed);
            None
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = value_class(values, *lhs);
            let rhs_class = value_class(values, *rhs);
            if generic_pure_compare_proves_i64(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::I64
                {
                    set_value_class(values, *lhs, GenericPureValueClass::I64, changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::I64
                {
                    set_value_class(values, *rhs, GenericPureValueClass::I64, changed);
                    return None;
                }
            }
            if generic_pure_string_compare_can_infer_string(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(values, *lhs, changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(values, *rhs, changed);
                    return None;
                }
            }
            if lhs_class == GenericPureValueClass::Unknown
                || rhs_class == GenericPureValueClass::Unknown
            {
                return None;
            }
            let has_void_sentinel = generic_pure_value_class_is_void_like(lhs_class)
                || generic_pure_value_class_is_void_like(rhs_class);
            if has_void_sentinel {
                let comparable =
                    matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
                        && generic_pure_void_sentinel_compare_is_supported(lhs_class, rhs_class);
                if !comparable {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedVoidSentinelConst,
                    ));
                }
                set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
                return None;
            }
            if lhs_class == GenericPureValueClass::String
                || rhs_class == GenericPureValueClass::String
            {
                if !generic_pure_string_accepts_string_compare(*op, lhs_class, rhs_class) {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ));
                }
                *has_string_surface = true;
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            None
        }
        MirInstruction::UnaryOp {
            dst,
            op: UnaryOp::Not,
            operand,
        } => {
            let operand_class = value_class(values, *operand);
            if operand_class == GenericPureValueClass::Unknown {
                return None;
            }
            if !matches!(
                operand_class,
                GenericPureValueClass::Bool | GenericPureValueClass::I64
            ) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
            None
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let dst_class = value_class(values, *dst);
            let mut saw_scalar_or_void = false;
            let mut saw_string = false;
            let mut saw_string_or_void = false;
            let mut saw_void_sentinel = false;
            let mut saw_scalar = false;
            let mut saw_array = false;
            let mut saw_map = false;
            let mut saw_array_or_void = false;
            let mut saw_map_or_void = false;
            let mut all_string = !inputs.is_empty();
            let mut all_array = !inputs.is_empty();
            let mut all_map = !inputs.is_empty();
            let mut saw_unknown = false;
            for (_, value) in inputs {
                let class = value_class(values, *value);
                saw_unknown |= class == GenericPureValueClass::Unknown;
                saw_scalar_or_void |= class == GenericPureValueClass::ScalarOrVoid;
                saw_string |= class == GenericPureValueClass::String;
                saw_string_or_void |= class == GenericPureValueClass::StringOrVoid;
                saw_void_sentinel |= class == GenericPureValueClass::VoidSentinel;
                saw_array |= class == GenericPureValueClass::Array;
                saw_map |= class == GenericPureValueClass::Map;
                saw_array_or_void |= class == GenericPureValueClass::ArrayOrVoid;
                saw_map_or_void |= class == GenericPureValueClass::MapOrVoid;
                saw_scalar |= matches!(
                    class,
                    GenericPureValueClass::I64 | GenericPureValueClass::Bool
                );
                all_string &= class == GenericPureValueClass::String;
                all_array &= class == GenericPureValueClass::Array;
                all_map &= class == GenericPureValueClass::Map;
            }
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_pure_value_class_from_type);
            if saw_unknown {
                if dst_class != GenericPureValueClass::Unknown
                    && inputs.iter().all(|(_, value)| {
                        let class = value_class(values, *value);
                        class == GenericPureValueClass::Unknown || class == dst_class
                    })
                {
                    for (_, value) in inputs {
                        if value_class(values, *value) == GenericPureValueClass::Unknown {
                            set_proven_flow_value_class(values, *value, dst_class, changed);
                        }
                    }
                }
                if matches!(
                    type_hint_class,
                    Some(GenericPureValueClass::I64 | GenericPureValueClass::Bool)
                ) && !saw_string
                    && !saw_scalar_or_void
                    && !saw_string_or_void
                    && !saw_void_sentinel
                    && !saw_array
                    && !saw_map
                    && !saw_array_or_void
                    && !saw_map_or_void
                {
                    set_proven_flow_value_class(values, *dst, type_hint_class.unwrap(), changed);
                }
                return None;
            } else if non_void_string_values.contains(dst)
                && saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_void_sentinel
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                set_guarded_non_void_scalar_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_string_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                *has_string_surface = true;
                set_guarded_non_void_string_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_array_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_guarded_non_void_array_value_class(values, *dst, changed);
            } else if non_void_string_values.contains(dst)
                && saw_map_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_array_or_void
            {
                set_guarded_non_void_map_value_class(values, *dst, changed);
            } else if all_string {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::String, changed);
            } else if all_array {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Array, changed);
            } else if all_map {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::Map, changed);
            } else if (saw_array_or_void || (saw_void_sentinel && saw_array))
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::ArrayOrVoid,
                    changed,
                );
            } else if (saw_map_or_void || (saw_void_sentinel && saw_map))
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_array_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::MapOrVoid,
                    changed,
                );
            } else if saw_string_or_void && !saw_scalar {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
            } else if saw_void_sentinel && !saw_scalar && (saw_string || saw_string_or_void) {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
            } else if (saw_scalar_or_void || (saw_void_sentinel && saw_scalar))
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::ScalarOrVoid,
                    changed,
                );
            } else if saw_void_sentinel && !saw_scalar {
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::VoidSentinel,
                    changed,
                );
            } else if saw_string || saw_array || saw_map || saw_array_or_void || saw_map_or_void {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                set_proven_flow_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            None
        }
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_class = value_class(values, *cond);
            if cond_class == GenericPureValueClass::Unknown {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            if !matches!(
                cond_class,
                GenericPureValueClass::Bool | GenericPureValueClass::I64
            ) {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }

            let then_class = value_class(values, *then_val);
            let else_class = value_class(values, *else_val);
            if then_class == GenericPureValueClass::Unknown
                && else_class == GenericPureValueClass::Unknown
            {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            if then_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *then_val, else_class, changed);
                return None;
            }
            if else_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(values, *else_val, then_class, changed);
                return None;
            }

            let Some(selected_class) = generic_pure_select_value_class(then_class, else_class)
            else {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            };
            if matches!(
                selected_class,
                GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
            ) {
                *has_string_surface = true;
            }
            set_proven_flow_value_class(values, *dst, selected_class, changed);
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if matches!(
            classify_extern_call_route(name, args.len()),
            Some(
                ExternCallRouteKind::EnvGet
                    | ExternCallRouteKind::Stage1EmitProgramJson
                    | ExternCallRouteKind::Stage1EmitMirFromSource
                    | ExternCallRouteKind::Stage1EmitMirFromProgramJson
            )
        ) =>
        {
            if let Some(dst) = dst {
                *has_string_surface = true;
                set_string_handle_value_class(values, *dst, changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if generic_pure_string_accepts_env_set(name, args, values) => {
            if let Some(dst) = dst {
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
            }
            None
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if classify_extern_call_route(name, args.len())
            == Some(ExternCallRouteKind::HostBridgeExternInvoke) =>
        {
            if let Some(dst) = dst {
                *has_string_surface = true;
                set_proven_flow_value_class(
                    values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    changed,
                );
            }
            None
        }
        MirInstruction::Call {
            callee: Some(Callee::Extern(_)),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedExternCall,
        )),
        MirInstruction::Call {
            dst,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } => {
            if let Some(class) =
                generic_pure_string_route_value_class(function, block, instruction_index)
            {
                if matches!(
                    class,
                    GenericPureValueClass::String | GenericPureValueClass::StringOrVoid
                ) {
                    *has_string_surface = true;
                }
                if let Some(dst) = dst {
                    set_proven_flow_value_class(values, *dst, class, changed);
                }
                return None;
            }
            let receiver_class = value_class(values, *receiver);
            if generic_pure_string_accepts_collection_birth_method(
                box_name,
                method,
                args,
                receiver_class,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
            if generic_pure_string_accepts_array_push_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
            if generic_pure_string_accepts_map_set_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                if let Some(dst) = dst {
                    set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                }
                return None;
            }
            if receiver_class == GenericPureValueClass::Unknown
                || args
                    .iter()
                    .any(|arg| value_class(values, *arg) == GenericPureValueClass::Unknown)
            {
                if *changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                ));
            }
            if generic_pure_string_accepts_length_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_array_len_method(box_name, method, args, receiver_class)
            {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_indexof_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_lastindexof_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::I64, changed);
                return None;
            }
            if generic_pure_string_accepts_contains_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                set_value_class(values, *dst, GenericPureValueClass::Bool, changed);
                return None;
            }
            if generic_pure_string_accepts_substring_method(
                box_name,
                method,
                args,
                receiver_class,
                values,
            ) {
                let Some(dst) = dst else {
                    return Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ));
                };
                *has_string_surface = true;
                set_value_class(values, *dst, GenericPureValueClass::String, changed);
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
            ))
        }
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
        )),
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if supported_backend_global(name) => None,
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if !supported_backend_global(name) => {
            if is_hostbridge_extern_invoke_symbol(name, args.len()) {
                if let Some(dst) = dst {
                    *has_string_surface = true;
                    set_proven_flow_value_class(
                        values,
                        *dst,
                        GenericPureValueClass::StringOrVoid,
                        changed,
                    );
                }
                return None;
            }
            if generic_pure_string_global_name_is_self(name, current_function_name) {
                if let Some(dst) = dst {
                    *has_string_surface = true;
                    set_proven_flow_value_class(
                        values,
                        *dst,
                        GenericPureValueClass::String,
                        changed,
                    );
                }
                return None;
            }
            let Some(target) = super::lookup_global_call_target(name, targets) else {
                return Some(GenericPureStringReject::with_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing,
                    crate::mir::naming::normalize_static_global_name(name),
                    None,
                ));
            };
            let Some(contract) = target.return_contract() else {
                return Some(GenericPureStringReject::with_shape_blocker(
                    GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                    propagated_unknown_global_target_blocker(name, target),
                ));
            };
            match contract {
                GlobalCallReturnContract::StringHandle
                | GlobalCallReturnContract::StringHandleOrNull => {
                    if let Some(dst) = dst {
                        *has_string_surface = true;
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::String,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ArrayHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::Array,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::MapHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::Map,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ObjectHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::Unknown,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::MixedRuntimeI64OrHandle => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::ScalarOrVoid,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::VoidSentinelI64Zero => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::VoidSentinel,
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ScalarI64
                    if target.shape() == GlobalCallTargetShape::GenericI64Body =>
                {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            generic_pure_string_generic_i64_target_value_class(target),
                            changed,
                        );
                    }
                    None
                }
                GlobalCallReturnContract::ScalarI64 => {
                    if let Some(dst) = dst {
                        set_proven_flow_value_class(
                            values,
                            *dst,
                            GenericPureValueClass::I64,
                            changed,
                        );
                    }
                    None
                }
            }
        }
        MirInstruction::Call { .. } => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedCall,
        )),
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => None,
        _ => Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
        )),
    }
}

fn generic_pure_string_operand_allows_string_inference(
    function: &MirFunction,
    value: ValueId,
) -> bool {
    !matches!(
        function.metadata.value_types.get(&value),
        Some(MirType::Integer | MirType::Bool)
    )
}
