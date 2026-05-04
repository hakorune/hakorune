use super::super::super::ast::{EnumMatchArmV0, ExprV0};
use super::super::merge::new_block;
use super::super::BridgeEnv;
use super::{lower_args_with_scope, lower_expr_with_vars, VarScope};
use crate::ast::Span;
use crate::mir::{
    BasicBlockId, CompareOp, ConstValue, MirFunction, MirInstruction, MirType, ValueId,
};
use crate::semantics::option_contract::{nullish_payload_error, requires_non_nullish_payload};
use std::collections::{BTreeMap, BTreeSet};

struct ResolvedEnumVariant {
    tag: u32,
    payload_type_name: Option<String>,
}

pub(super) fn lower_variant_ctor_expr_with_scope<S: VarScope>(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    enum_name: &str,
    variant_name: &str,
    payload_type_name: Option<&str>,
    args: &[ExprV0],
    vars: &mut S,
) -> Result<(ValueId, BasicBlockId), String> {
    let resolved = resolve_enum_variant(env, enum_name, variant_name, payload_type_name)?;
    let expected_arity = usize::from(resolved.payload_type_name.is_some());
    if args.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][json_v0][enum_ctor] {}::{} expects {} arg(s), got {}",
            enum_name,
            variant_name,
            expected_arity,
            args.len()
        ));
    }
    if requires_non_nullish_payload(enum_name, variant_name)
        && args.iter().any(expr_is_statically_nullish)
    {
        return Err(nullish_payload_error("json_v0/enum_ctor"));
    }

    let (arg_ids, cur2) = lower_args_with_scope(env, f, cur_bb, args, vars)?;
    let payload = match arg_ids.as_slice() {
        [] => None,
        [payload] => Some(*payload),
        _ => {
            return Err(format!(
            "[freeze:contract][json_v0][enum_ctor] multi-payload variants are outside MVP: {}::{}",
            enum_name, variant_name
        ))
        }
    };

    let dst = f.next_value_id();
    let payload_type = payload_type_hint(resolved.payload_type_name.as_deref());
    if let Some(bb) = f.get_block_mut(cur2) {
        bb.add_instruction(MirInstruction::VariantMake {
            dst,
            enum_name: enum_name.to_string(),
            variant: variant_name.to_string(),
            tag: resolved.tag,
            payload,
            payload_type: payload_type.clone(),
        });
    }
    f.metadata.value_types.insert(dst, MirType::Unknown);
    Ok((dst, cur2))
}

pub(super) fn lower_variant_match_expr_with_vars(
    env: &BridgeEnv,
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    enum_name: &str,
    scrutinee: &ExprV0,
    arms: &[EnumMatchArmV0],
    else_expr: Option<&ExprV0>,
    vars: &mut BTreeMap<String, ValueId>,
) -> Result<(ValueId, BasicBlockId), String> {
    if arms.is_empty() {
        return Err(format!(
            "[freeze:contract][json_v0][enum_match] no arms for `{}`",
            enum_name
        ));
    }

    let enum_decl = env.enum_decls.get(enum_name).ok_or_else(|| {
        format!(
            "[freeze:contract][json_v0][enum_match] missing enum inventory for `{}`",
            enum_name
        )
    })?;

    let exhaustive_without_else = else_expr.is_none() && enum_match_is_exhaustive(enum_decl, arms);
    if else_expr.is_none() && !exhaustive_without_else {
        return Err(format!(
            "[freeze:contract][json_v0][enum_match] non-exhaustive lowering for `{}` without else arm",
            enum_name
        ));
    }

    let (scr_val, start_bb) = lower_expr_with_vars(env, f, cur_bb, scrutinee, vars)?;
    let dispatch_bb = new_block(f);
    if let Some(bb) = f.get_block_mut(start_bb) {
        if !bb.is_terminated() {
            crate::mir::ssot::cf_common::set_jump(f, start_bb, dispatch_bb);
        }
    }

    let tag_val = f.next_value_id();
    if let Some(bb) = f.get_block_mut(dispatch_bb) {
        bb.add_instruction(MirInstruction::VariantTag {
            dst: tag_val,
            value: scr_val,
            enum_name: enum_name.to_string(),
        });
    }
    f.metadata.value_types.insert(tag_val, MirType::Integer);

    let merge_bb = new_block(f);
    let else_bb = else_expr.map(|_| new_block(f));
    let compare_arms = if exhaustive_without_else {
        arms.len().saturating_sub(1)
    } else {
        arms.len()
    };

    let mut cur_dispatch = dispatch_bb;
    let mut phi_inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();

    for (index, arm) in arms.iter().enumerate() {
        let resolved =
            resolve_enum_variant(env, enum_name, &arm.variant, arm.payload_type.as_deref())?;
        let then_bb = new_block(f);
        let fall_bb = if exhaustive_without_else && index + 1 == arms.len() {
            None
        } else if index + 1 < compare_arms {
            Some(new_block(f))
        } else {
            else_bb
        };

        if let Some(fall_bb) = fall_bb {
            let tag_const = f.next_value_id();
            let cond = f.next_value_id();
            if let Some(bb) = f.get_block_mut(cur_dispatch) {
                bb.add_instruction(MirInstruction::Const {
                    dst: tag_const,
                    value: ConstValue::Integer(resolved.tag as i64),
                });
            }
            crate::mir::ssot::cf_common::emit_compare_func(
                f,
                cur_dispatch,
                cond,
                CompareOp::Eq,
                tag_val,
                tag_const,
            );
            crate::mir::ssot::cf_common::set_branch(f, cur_dispatch, cond, then_bb, fall_bb);
            cur_dispatch = fall_bb;
        } else if let Some(bb) = f.get_block_mut(cur_dispatch) {
            if !bb.is_terminated() {
                crate::mir::ssot::cf_common::set_jump(f, cur_dispatch, then_bb);
            }
        }

        let mut arm_vars = vars.clone();
        if let Some(bind_name) = &arm.bind {
            let payload_type_name = resolved.payload_type_name.as_deref().ok_or_else(|| {
                format!(
                    "[freeze:contract][json_v0][enum_match] unit variant `{}` cannot bind payload",
                    arm.variant
                )
            })?;
            let projected = f.next_value_id();
            let payload_type = payload_type_hint(Some(payload_type_name));
            if let Some(bb) = f.get_block_mut(then_bb) {
                bb.add_instruction(MirInstruction::VariantProject {
                    dst: projected,
                    value: scr_val,
                    enum_name: enum_name.to_string(),
                    variant: arm.variant.clone(),
                    tag: resolved.tag,
                    payload_type: payload_type.clone(),
                });
            }
            if let Some(payload_type) = payload_type {
                f.metadata.value_types.insert(projected, payload_type);
            }
            arm_vars.insert(bind_name.clone(), projected);
        }

        let (arm_value, arm_end) = lower_expr_with_vars(env, f, then_bb, &arm.expr, &mut arm_vars)?;
        if let Some(bb) = f.get_block_mut(arm_end) {
            if !bb.is_terminated() {
                crate::mir::ssot::cf_common::set_jump(f, arm_end, merge_bb);
            }
        }
        phi_inputs.push((arm_end, arm_value));
    }

    if let Some(else_expr) = else_expr {
        let else_bb = else_bb.expect("else block must exist when else expr is present");
        let mut else_vars = vars.clone();
        let (else_value, else_end) =
            lower_expr_with_vars(env, f, else_bb, else_expr, &mut else_vars)?;
        if let Some(bb) = f.get_block_mut(else_end) {
            if !bb.is_terminated() {
                crate::mir::ssot::cf_common::set_jump(f, else_end, merge_bb);
            }
        }
        phi_inputs.push((else_end, else_value));
    }

    let out = f.next_value_id();
    crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
        f,
        merge_bb,
        out,
        phi_inputs,
        Span::unknown(),
    )?;
    Ok((out, merge_bb))
}

fn resolve_enum_variant(
    env: &BridgeEnv,
    enum_name: &str,
    variant_name: &str,
    payload_type_name: Option<&str>,
) -> Result<ResolvedEnumVariant, String> {
    let enum_decl = env.enum_decls.get(enum_name).ok_or_else(|| {
        format!(
            "[freeze:contract][json_v0][enum] missing enum inventory for `{}`",
            enum_name
        )
    })?;

    let Some((tag, variant_decl)) = enum_decl
        .variants
        .iter()
        .enumerate()
        .find(|(_, variant)| variant.name == variant_name)
    else {
        return Err(format!(
            "[freeze:contract][json_v0][enum] unknown variant `{}::{}`",
            enum_name, variant_name
        ));
    };

    let declared_payload_type = variant_decl.payload_type.clone();
    if payload_type_name.is_some() && payload_type_name != declared_payload_type.as_deref() {
        return Err(format!(
            "[freeze:contract][json_v0][enum] payload type mismatch for {}::{}: declared={:?} json={:?}",
            enum_name,
            variant_name,
            declared_payload_type,
            payload_type_name
        ));
    }

    Ok(ResolvedEnumVariant {
        tag: tag as u32,
        payload_type_name: declared_payload_type,
    })
}

fn enum_match_is_exhaustive(
    enum_decl: &super::super::super::ast::EnumDeclV0,
    arms: &[EnumMatchArmV0],
) -> bool {
    let expected: BTreeSet<_> = enum_decl
        .variants
        .iter()
        .map(|variant| variant.name.as_str())
        .collect();
    let actual: BTreeSet<_> = arms.iter().map(|arm| arm.variant.as_str()).collect();
    expected == actual
}

fn payload_type_hint(raw: Option<&str>) -> Option<MirType> {
    let raw = raw?;
    let lower = raw.to_ascii_lowercase();
    match lower.as_str() {
        "integer" | "int" | "i64" | "integerbox" => Some(MirType::Integer),
        "float" | "f64" | "floatbox" => Some(MirType::Float),
        "bool" | "boolean" | "boolbox" => Some(MirType::Bool),
        "string" | "str" | "stringbox" => Some(MirType::String),
        "void" | "null" | "voidbox" | "nullbox" => Some(MirType::Void),
        _ if looks_like_generic_type_param(raw) => None,
        _ => Some(MirType::Box(raw.to_string())),
    }
}

fn looks_like_generic_type_param(raw: &str) -> bool {
    !raw.is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn expr_is_statically_nullish(expr: &ExprV0) -> bool {
    match expr {
        ExprV0::Null => true,
        ExprV0::BlockExpr { tail, .. } => block_expr_tail_is_statically_nullish(tail),
        _ => false,
    }
}

fn block_expr_tail_is_statically_nullish(tail: &serde_json::Value) -> bool {
    let Some(kind) = tail.get("type").and_then(|value| value.as_str()) else {
        return false;
    };
    if kind == "Null" {
        return true;
    }
    if kind != "Expr" {
        return false;
    }
    tail.get("expr")
        .map(expr_json_is_statically_nullish)
        .unwrap_or(false)
}

fn expr_json_is_statically_nullish(value: &serde_json::Value) -> bool {
    match serde_json::from_value::<ExprV0>(value.clone()) {
        Ok(expr) => expr_is_statically_nullish(&expr),
        Err(_) => false,
    }
}
