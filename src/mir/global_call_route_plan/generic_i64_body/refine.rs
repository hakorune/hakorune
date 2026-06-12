use super::super::generic_string_surface::generic_pure_compare_proves_i64;
use super::super::model::GlobalCallReturnContract;
use super::super::{lookup_global_call_target, GlobalCallTargetFacts, GlobalCallTargetShape};
use super::route::{
    generic_i64_accepts_backend_global_call, generic_i64_global_call_result_class,
    generic_i64_route_value_class,
};
use super::string_methods::{
    generic_i64_accepts_length_method, generic_i64_contains_args_ready,
    generic_i64_indexof_args_ready, generic_i64_substring_args_ready,
};
use super::value_class::{
    generic_i64_select_value_class, generic_i64_value_class, generic_i64_value_class_from_type,
    set_generic_i64_string_handle_value_class, set_generic_i64_value_class, GenericI64ValueClass,
};
use crate::mir::extern_call_route_plan::{classify_extern_call_route, ExternCallRouteKind};
use crate::mir::same_module_body_shape::supported_backend_global;
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, UnaryOp, ValueId,
};
use std::collections::BTreeMap;

pub(super) fn generic_i64_body_refine_instruction(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericI64ValueClass>,
    changed: &mut bool,
) -> bool {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::Integer(_) => GenericI64ValueClass::I64,
                ConstValue::Bool(_) => GenericI64ValueClass::Bool,
                ConstValue::String(_) => GenericI64ValueClass::String,
                ConstValue::Null | ConstValue::Void => GenericI64ValueClass::VoidSentinel,
                _ => return false,
            };
            set_generic_i64_value_class(values, *dst, class, changed)
        }
        MirInstruction::Copy { dst, src } => {
            let class = generic_i64_value_class(values, *src);
            if class != GenericI64ValueClass::Unknown {
                set_generic_i64_value_class(values, *dst, class, changed)
            } else {
                let dst_class = generic_i64_value_class(values, *dst);
                if dst_class != GenericI64ValueClass::Unknown {
                    set_generic_i64_value_class(values, *src, dst_class, changed)
                } else {
                    true
                }
            }
        }
        MirInstruction::FieldGet { dst, base, .. } => {
            let base_class = generic_i64_value_class(values, *base);
            if base_class == GenericI64ValueClass::Unknown {
                return true;
            }
            if base_class != GenericI64ValueClass::Object {
                return false;
            }
            let Some(field_class) = function
                .metadata
                .value_types
                .get(dst)
                .and_then(generic_i64_value_class_from_type)
            else {
                return false;
            };
            set_generic_i64_value_class(values, *dst, field_class, changed)
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            if !matches!(
                op,
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
            ) {
                return false;
            }
            let lhs_class = generic_i64_value_class(values, *lhs);
            let rhs_class = generic_i64_value_class(values, *rhs);
            if *op == BinaryOp::Add
                && (lhs_class == GenericI64ValueClass::String
                    || rhs_class == GenericI64ValueClass::String)
            {
                return set_generic_i64_string_handle_value_class(values, *dst, changed);
            }
            if lhs_class == GenericI64ValueClass::Unknown
                || rhs_class == GenericI64ValueClass::Unknown
            {
                return true;
            }
            if lhs_class == GenericI64ValueClass::I64 && rhs_class == GenericI64ValueClass::I64 {
                set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
            } else {
                false
            }
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = generic_i64_value_class(values, *lhs);
            let rhs_class = generic_i64_value_class(values, *rhs);
            if generic_pure_compare_proves_i64(*op) {
                if lhs_class == GenericI64ValueClass::Unknown
                    && rhs_class == GenericI64ValueClass::I64
                {
                    return set_generic_i64_value_class(
                        values,
                        *lhs,
                        GenericI64ValueClass::I64,
                        changed,
                    );
                }
                if rhs_class == GenericI64ValueClass::Unknown
                    && lhs_class == GenericI64ValueClass::I64
                {
                    return set_generic_i64_value_class(
                        values,
                        *rhs,
                        GenericI64ValueClass::I64,
                        changed,
                    );
                }
            }
            let eq_ne = matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne);
            let string_ordered =
                matches!(op, crate::mir::CompareOp::Lt | crate::mir::CompareOp::Gt);
            if eq_ne || string_ordered {
                if lhs_class == GenericI64ValueClass::Unknown
                    && rhs_class == GenericI64ValueClass::String
                {
                    return set_generic_i64_value_class(
                        values,
                        *lhs,
                        GenericI64ValueClass::String,
                        changed,
                    );
                }
                if rhs_class == GenericI64ValueClass::Unknown
                    && lhs_class == GenericI64ValueClass::String
                {
                    return set_generic_i64_value_class(
                        values,
                        *rhs,
                        GenericI64ValueClass::String,
                        changed,
                    );
                }
            }
            if lhs_class == GenericI64ValueClass::Unknown
                || rhs_class == GenericI64ValueClass::Unknown
            {
                return true;
            }
            let comparable = match (lhs_class, rhs_class) {
                (GenericI64ValueClass::String, GenericI64ValueClass::String) => {
                    eq_ne || string_ordered
                }
                (GenericI64ValueClass::String, GenericI64ValueClass::VoidSentinel)
                | (GenericI64ValueClass::VoidSentinel, GenericI64ValueClass::String)
                | (GenericI64ValueClass::StringOrVoid, GenericI64ValueClass::VoidSentinel)
                | (GenericI64ValueClass::VoidSentinel, GenericI64ValueClass::StringOrVoid) => eq_ne,
                (GenericI64ValueClass::I64, GenericI64ValueClass::I64) => true,
                (GenericI64ValueClass::Object, GenericI64ValueClass::I64)
                | (GenericI64ValueClass::I64, GenericI64ValueClass::Object) => {
                    generic_pure_compare_proves_i64(*op)
                }
                (GenericI64ValueClass::Bool, GenericI64ValueClass::Bool) => eq_ne,
                _ => false,
            };
            if !comparable {
                return false;
            }
            set_generic_i64_value_class(values, *dst, GenericI64ValueClass::Bool, changed)
        }
        MirInstruction::UnaryOp {
            dst,
            op: UnaryOp::Not,
            operand,
        } => {
            let operand_class = generic_i64_value_class(values, *operand);
            if operand_class == GenericI64ValueClass::Unknown {
                return true;
            }
            if !matches!(
                operand_class,
                GenericI64ValueClass::Bool | GenericI64ValueClass::I64
            ) {
                return false;
            }
            set_generic_i64_value_class(values, *dst, GenericI64ValueClass::Bool, changed)
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            if inputs.is_empty() {
                return false;
            }
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_i64_value_class_from_type);
            let dst_class = generic_i64_value_class(values, *dst);
            let mut merged = dst_class;
            for (_, value) in inputs {
                let class = generic_i64_value_class(values, *value);
                if class == GenericI64ValueClass::Unknown {
                    if dst_class == GenericI64ValueClass::Unknown
                        && matches!(
                            type_hint_class,
                            Some(GenericI64ValueClass::I64 | GenericI64ValueClass::Bool)
                        )
                    {
                        return set_generic_i64_value_class(
                            values,
                            *dst,
                            type_hint_class.unwrap(),
                            changed,
                        );
                    }
                    if dst_class != GenericI64ValueClass::Unknown
                        && !set_generic_i64_value_class(values, *value, dst_class, changed)
                    {
                        return false;
                    }
                    return true;
                }
                if merged == GenericI64ValueClass::Unknown {
                    merged = class;
                } else if merged != class
                    && !(merged == GenericI64ValueClass::Bool && class == GenericI64ValueClass::I64)
                {
                    return false;
                }
            }
            set_generic_i64_value_class(values, *dst, merged, changed)
        }
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_class = generic_i64_value_class(values, *cond);
            if cond_class == GenericI64ValueClass::Unknown {
                return *changed;
            }
            if !matches!(
                cond_class,
                GenericI64ValueClass::Bool | GenericI64ValueClass::I64
            ) {
                return false;
            }

            let then_class = generic_i64_value_class(values, *then_val);
            let else_class = generic_i64_value_class(values, *else_val);
            if then_class == GenericI64ValueClass::Unknown
                && else_class == GenericI64ValueClass::Unknown
            {
                return *changed;
            }
            if then_class == GenericI64ValueClass::Unknown {
                return set_generic_i64_value_class(values, *then_val, else_class, changed);
            }
            if else_class == GenericI64ValueClass::Unknown {
                return set_generic_i64_value_class(values, *else_val, then_class, changed);
            }
            let Some(selected_class) = generic_i64_select_value_class(then_class, else_class)
            else {
                return false;
            };
            set_generic_i64_value_class(values, *dst, selected_class, changed)
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } => match classify_extern_call_route(name, args.len()) {
            Some(ExternCallRouteKind::EnvGet) => {
                if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::String, changed)
                } else {
                    false
                }
            }
            Some(
                ExternCallRouteKind::AnyHandleLive
                | ExternCallRouteKind::ArraySlotAppendAny
                | ExternCallRouteKind::ArraySlotLenI64
                | ExternCallRouteKind::ArraySlotLoadI64
                | ExternCallRouteKind::ArraySlotStoreI64
                | ExternCallRouteKind::HakoAtomicSlotCasI64
                | ExternCallRouteKind::HakoAtomicSlotFetchAddI64
                | ExternCallRouteKind::HakoAtomicSlotLoadI64
                | ExternCallRouteKind::HakoAtomicSlotStoreI64
                | ExternCallRouteKind::HakoAtomicPtrCasOrdered
                | ExternCallRouteKind::HakoAtomicPtrLoadOrdered
                | ExternCallRouteKind::HakoAtomicPtrStoreOrdered
                | ExternCallRouteKind::HakoMemAlloc
                | ExternCallRouteKind::HakoMemFree
                | ExternCallRouteKind::HakoOsvmReserveBytesI64
                | ExternCallRouteKind::HakoOsvmCommitBytesI64
                | ExternCallRouteKind::HakoOsvmDecommitBytesI64
                | ExternCallRouteKind::HakoOsvmUnreserveBytesI64
                | ExternCallRouteKind::HakoTlsCacheSlotGetI64
                | ExternCallRouteKind::HakoTlsCacheSlotSetI64
                | ExternCallRouteKind::HakoWorkerCurrentIdI64,
            ) => {
                if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
                } else {
                    classify_extern_call_route(name, args.len())
                        .is_some_and(ExternCallRouteKind::accepts_void_result)
                }
            }
            _ => false,
        },
        MirInstruction::Call {
            callee: Some(Callee::Method { receiver: None, .. }),
            ..
        } => false,
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
            if let Some(class) = generic_i64_route_value_class(function, block, instruction_index) {
                return if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, class, changed)
                } else {
                    false
                };
            }
            let receiver_class = generic_i64_value_class(values, *receiver);
            if receiver_class == GenericI64ValueClass::Unknown {
                if !set_generic_i64_value_class(
                    values,
                    *receiver,
                    GenericI64ValueClass::String,
                    changed,
                ) {
                    return false;
                }
                return true;
            }
            if generic_i64_accepts_length_method(box_name, method, args, receiver_class) {
                if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
                } else {
                    false
                }
            } else if let Some(ready) = generic_i64_indexof_args_ready(
                box_name,
                method,
                args,
                receiver_class,
                values,
                changed,
            ) {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::I64, changed)
                } else {
                    false
                }
            } else if let Some(ready) = generic_i64_contains_args_ready(
                box_name,
                method,
                args,
                receiver_class,
                values,
                changed,
            ) {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::Bool, changed)
                } else {
                    false
                }
            } else if let Some(ready) =
                generic_i64_substring_args_ready(box_name, method, args, receiver_class, values)
            {
                if !ready {
                    true
                } else if let Some(dst) = dst {
                    set_generic_i64_value_class(values, *dst, GenericI64ValueClass::String, changed)
                } else {
                    false
                }
            } else {
                false
            }
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if supported_backend_global(name) => {
            generic_i64_accepts_backend_global_call(function, name, dst, args)
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            ..
        } => {
            let Some(target) = lookup_global_call_target(name, targets) else {
                return false;
            };
            let Some(contract) = target.return_contract() else {
                return false;
            };
            let class = match contract {
                GlobalCallReturnContract::StringHandle => GenericI64ValueClass::String,
                GlobalCallReturnContract::ArrayHandle
                | GlobalCallReturnContract::MapHandle
                | GlobalCallReturnContract::ObjectHandle => GenericI64ValueClass::Unknown,
                GlobalCallReturnContract::StringHandleOrNull
                | GlobalCallReturnContract::MixedRuntimeI64OrHandle => {
                    GenericI64ValueClass::StringOrVoid
                }
                GlobalCallReturnContract::ScalarI64
                    if target.shape() == GlobalCallTargetShape::GenericI64Body =>
                {
                    generic_i64_global_call_result_class(values, dst)
                }
                GlobalCallReturnContract::ScalarI64 => GenericI64ValueClass::I64,
                GlobalCallReturnContract::VoidSentinelI64Zero => GenericI64ValueClass::VoidSentinel,
            };
            if let Some(dst) = dst {
                set_generic_i64_value_class(values, *dst, class, changed)
            } else {
                false
            }
        }
        MirInstruction::Call { .. } => false,
        MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::ReleaseStrong { .. } => true,
        _ => false,
    }
}
