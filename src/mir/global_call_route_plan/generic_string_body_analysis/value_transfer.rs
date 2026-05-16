use crate::mir::{BinaryOp, ConstValue, MirFunction, MirInstruction, MirType, UnaryOp, ValueId};

use super::GenericPureStringAnalysisContext;
use crate::mir::global_call_route_plan::generic_string_facts::{
    generic_pure_value_class_from_type, generic_pure_value_class_is_void_like,
    generic_pure_void_sentinel_compare_is_supported, set_guarded_non_void_array_value_class,
    set_guarded_non_void_map_value_class, set_guarded_non_void_scalar_value_class,
    set_guarded_non_void_string_value_class, set_proven_flow_value_class,
    set_string_handle_value_class, set_value_class, value_class, GenericPureValueClass,
};
use crate::mir::global_call_route_plan::generic_string_reject::GenericPureStringReject;
use crate::mir::global_call_route_plan::generic_string_route_value_class::generic_pure_select_value_class;
use crate::mir::global_call_route_plan::generic_string_surface::{
    generic_pure_compare_proves_i64, generic_pure_string_accepts_string_compare,
    generic_pure_string_compare_can_infer_string,
};
use crate::mir::global_call_route_plan::model::GlobalCallTargetShapeReason;

pub(super) fn generic_pure_string_value_instruction_reject_reason(
    ctx: &mut GenericPureStringAnalysisContext<'_>,
    instruction: &MirInstruction,
) -> Option<GenericPureStringReject> {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => {
                    *ctx.has_string_surface = true;
                    GenericPureValueClass::String
                }
                ConstValue::Integer(_) => GenericPureValueClass::I64,
                ConstValue::Bool(_) => GenericPureValueClass::Bool,
                ConstValue::Null | ConstValue::Void => {
                    *ctx.has_void_sentinel_const = true;
                    GenericPureValueClass::VoidSentinel
                }
                _ => GenericPureValueClass::Unknown,
            };
            set_value_class(ctx.values, *dst, class, ctx.changed);
            if class != GenericPureValueClass::Unknown {
                return None;
            }
            Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
            ))
        }
        MirInstruction::Copy { dst, src } => {
            let class = value_class(ctx.values, *src);
            if class != GenericPureValueClass::Unknown {
                set_proven_flow_value_class(ctx.values, *dst, class, ctx.changed);
            } else {
                let dst_class = value_class(ctx.values, *dst);
                if dst_class != GenericPureValueClass::Unknown {
                    set_value_class(ctx.values, *src, dst_class, ctx.changed);
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
            set_value_class(ctx.values, *dst, class, ctx.changed);
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
            let lhs_class = value_class(ctx.values, *lhs);
            let rhs_class = value_class(ctx.values, *rhs);
            if *op == BinaryOp::Add
                && (lhs_class == GenericPureValueClass::String
                    || rhs_class == GenericPureValueClass::String)
            {
                *ctx.has_string_surface = true;
                if lhs_class == GenericPureValueClass::Unknown
                    && generic_pure_string_operand_allows_string_inference(ctx.function, *lhs)
                {
                    set_string_handle_value_class(ctx.values, *lhs, ctx.changed);
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && generic_pure_string_operand_allows_string_inference(ctx.function, *rhs)
                {
                    set_string_handle_value_class(ctx.values, *rhs, ctx.changed);
                }
                set_string_handle_value_class(ctx.values, *dst, ctx.changed);
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
            set_value_class(ctx.values, *dst, class, ctx.changed);
            None
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = value_class(ctx.values, *lhs);
            let rhs_class = value_class(ctx.values, *rhs);
            if generic_pure_compare_proves_i64(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::I64
                {
                    set_value_class(ctx.values, *lhs, GenericPureValueClass::I64, ctx.changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::I64
                {
                    set_value_class(ctx.values, *rhs, GenericPureValueClass::I64, ctx.changed);
                    return None;
                }
            }
            if generic_pure_string_compare_can_infer_string(*op) {
                if lhs_class == GenericPureValueClass::Unknown
                    && rhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(ctx.values, *lhs, ctx.changed);
                    return None;
                }
                if rhs_class == GenericPureValueClass::Unknown
                    && lhs_class == GenericPureValueClass::String
                {
                    set_string_handle_value_class(ctx.values, *rhs, ctx.changed);
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
                set_value_class(ctx.values, *dst, GenericPureValueClass::Bool, ctx.changed);
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
                *ctx.has_string_surface = true;
            }
            set_value_class(ctx.values, *dst, GenericPureValueClass::Bool, ctx.changed);
            None
        }
        MirInstruction::UnaryOp {
            dst,
            op: UnaryOp::Not,
            operand,
        } => {
            let operand_class = value_class(ctx.values, *operand);
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
            set_value_class(ctx.values, *dst, GenericPureValueClass::Bool, ctx.changed);
            None
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let dst_class = value_class(ctx.values, *dst);
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
                let class = value_class(ctx.values, *value);
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
                        let class = value_class(ctx.values, *value);
                        class == GenericPureValueClass::Unknown || class == dst_class
                    })
                {
                    for (_, value) in inputs {
                        if value_class(ctx.values, *value) == GenericPureValueClass::Unknown {
                            set_proven_flow_value_class(ctx.values, *value, dst_class, ctx.changed);
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
                    set_proven_flow_value_class(
                        ctx.values,
                        *dst,
                        type_hint_class.unwrap(),
                        ctx.changed,
                    );
                }
                return None;
            } else if ctx.non_void_string_values.contains(dst)
                && saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_void_sentinel
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                set_guarded_non_void_scalar_value_class(ctx.values, *dst, ctx.changed);
            } else if ctx.non_void_string_values.contains(dst)
                && saw_string_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_array
                && !saw_map
                && !saw_array_or_void
                && !saw_map_or_void
            {
                *ctx.has_string_surface = true;
                set_guarded_non_void_string_value_class(ctx.values, *dst, ctx.changed);
            } else if ctx.non_void_string_values.contains(dst)
                && saw_array_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_guarded_non_void_array_value_class(ctx.values, *dst, ctx.changed);
            } else if ctx.non_void_string_values.contains(dst)
                && saw_map_or_void
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_array
                && !saw_array_or_void
            {
                set_guarded_non_void_map_value_class(ctx.values, *dst, ctx.changed);
            } else if all_string {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::String,
                    ctx.changed,
                );
            } else if all_array {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::Array,
                    ctx.changed,
                );
            } else if all_map {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::Map,
                    ctx.changed,
                );
            } else if (saw_array_or_void || (saw_void_sentinel && saw_array))
                && !saw_scalar
                && !saw_scalar_or_void
                && !saw_string
                && !saw_string_or_void
                && !saw_map
                && !saw_map_or_void
            {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::ArrayOrVoid,
                    ctx.changed,
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
                    ctx.values,
                    *dst,
                    GenericPureValueClass::MapOrVoid,
                    ctx.changed,
                );
            } else if saw_string_or_void && !saw_scalar {
                *ctx.has_string_surface = true;
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    ctx.changed,
                );
            } else if saw_void_sentinel && !saw_scalar && (saw_string || saw_string_or_void) {
                *ctx.has_string_surface = true;
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::StringOrVoid,
                    ctx.changed,
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
                    ctx.values,
                    *dst,
                    GenericPureValueClass::ScalarOrVoid,
                    ctx.changed,
                );
            } else if saw_void_sentinel && !saw_scalar {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::VoidSentinel,
                    ctx.changed,
                );
            } else if saw_string || saw_array || saw_map || saw_array_or_void || saw_map_or_void {
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            } else {
                set_proven_flow_value_class(
                    ctx.values,
                    *dst,
                    GenericPureValueClass::I64,
                    ctx.changed,
                );
            }
            None
        }
        MirInstruction::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            let cond_class = value_class(ctx.values, *cond);
            if cond_class == GenericPureValueClass::Unknown {
                if *ctx.changed {
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

            let then_class = value_class(ctx.values, *then_val);
            let else_class = value_class(ctx.values, *else_val);
            if then_class == GenericPureValueClass::Unknown
                && else_class == GenericPureValueClass::Unknown
            {
                if *ctx.changed {
                    return None;
                }
                return Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                ));
            }
            if then_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(ctx.values, *then_val, else_class, ctx.changed);
                return None;
            }
            if else_class == GenericPureValueClass::Unknown {
                set_proven_flow_value_class(ctx.values, *else_val, then_class, ctx.changed);
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
                *ctx.has_string_surface = true;
            }
            set_proven_flow_value_class(ctx.values, *dst, selected_class, ctx.changed);
            None
        }
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
