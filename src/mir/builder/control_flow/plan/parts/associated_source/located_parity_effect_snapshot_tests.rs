//! Exact effect snapshots for the located/raw Parts parity proof.
use super::*;

pub(super) fn normalize_effect(effect: &CoreEffectPlan) -> Result<NormalizedEffectV1, &'static str> {
    Ok(match effect {
        CoreEffectPlan::MapLiteralEntryWrite { receiver, key, value } =>
            NormalizedEffectV1::MapLiteralEntryWrite {
                receiver: *receiver, key: *key, value: *value,
            },
        CoreEffectPlan::MethodCall {
            dst,
            object,
            method,
            args,
            effects,
            source: _,
        } => NormalizedEffectV1::MethodCall {
            dst: *dst,
            object: *object,
            method: method.clone(),
            args: args.clone(),
            effects: *effects,
        },
        CoreEffectPlan::GlobalCall {
            dst,
            func,
            args,
            source: _,
        } => NormalizedEffectV1::GlobalCall {
            dst: *dst,
            func: func.clone(),
            args: args.clone(),
        },
        CoreEffectPlan::ValueCall {
            dst,
            callee,
            args,
            source: _,
        } => NormalizedEffectV1::ValueCall {
            dst: *dst,
            callee: *callee,
            args: args.clone(),
        },
        CoreEffectPlan::ExternCall {
            dst,
            iface_name,
            method_name,
            args,
            effects,
            source: _,
        } => NormalizedEffectV1::ExternCall {
            dst: *dst,
            iface_name: iface_name.clone(),
            method_name: method_name.clone(),
            args: args.clone(),
            effects: *effects,
        },
        CoreEffectPlan::NewBox { dst, target, args } => NormalizedEffectV1::NewBox {
            dst: *dst,
            target: target.clone(),
            args: args.clone(),
        },
        CoreEffectPlan::VariantMake {
            dst,
            enum_name,
            variant,
            tag,
            payload,
            payload_type,
        } => NormalizedEffectV1::VariantMake {
            dst: *dst,
            enum_name: enum_name.clone(),
            variant: variant.clone(),
            tag: *tag,
            payload: *payload,
            payload_type: payload_type.clone(),
        },
        CoreEffectPlan::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } => NormalizedEffectV1::FieldGet {
            dst: *dst,
            base: *base,
            field: field.clone(),
            declared_type: declared_type.clone(),
        },
        CoreEffectPlan::FieldSet {
            base,
            field,
            value,
            declared_type,
        } => NormalizedEffectV1::FieldSet {
            base: *base,
            field: field.clone(),
            value: *value,
            declared_type: declared_type.clone(),
        },
        CoreEffectPlan::BinOp { dst, lhs, op, rhs } => NormalizedEffectV1::BinOp {
            dst: *dst,
            lhs: *lhs,
            op: *op,
            rhs: *rhs,
        },
        CoreEffectPlan::Compare { dst, lhs, op, rhs } => NormalizedEffectV1::Compare {
            dst: *dst,
            lhs: *lhs,
            op: *op,
            rhs: *rhs,
        },
        CoreEffectPlan::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => NormalizedEffectV1::Select {
            dst: *dst,
            cond: *cond,
            then_val: *then_val,
            else_val: *else_val,
        },
        CoreEffectPlan::ExitIf { cond, exit } => NormalizedEffectV1::ExitIf {
            cond: *cond,
            exit: normalize_exit(exit),
        },
        CoreEffectPlan::IfEffect {
            cond,
            then_effects,
            else_effects,
        } => NormalizedEffectV1::IfEffect {
            cond: *cond,
            then_effects: then_effects
                .iter()
                .map(normalize_effect)
                .collect::<Result<_, _>>()?,
            else_effects: else_effects
                .as_ref()
                .map(|effects| effects.iter().map(normalize_effect).collect())
                .transpose()?,
        },
        CoreEffectPlan::Const { dst, value } => NormalizedEffectV1::Const {
            dst: *dst,
            value: value.clone(),
        },
        CoreEffectPlan::Copy { dst, src } => NormalizedEffectV1::Copy {
            dst: *dst,
            src: *src,
        },
        CoreEffectPlan::LocalContractWrite {
            dst,
            src,
            local_slot_id,
            write_kind,
        } => NormalizedEffectV1::LocalContractWrite {
            dst: *dst,
            src: *src,
            local_slot_id: *local_slot_id,
            write_kind: *write_kind,
        },
    })
}

