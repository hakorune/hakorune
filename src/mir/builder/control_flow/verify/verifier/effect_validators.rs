//! Phase 29bq+: Effect structure validators
//!
//! # Responsibilities
//!
//! - Validate Effect plan structure and ValueIds (V6)
//! - Validate IfEffect and ExitIf special cases (V12)
//! - Check effect leaf constraints in loop bodies
//!
//! # Invariants
//!
//! - V6: Effect ValueId validity (all ValueIds pre-generated)
//! - V12: Loop body effect-only (IfEffect/ExitIf allowed with restrictions)

use super::{position_validators, primitives};
use crate::mir::builder::control_flow::lower::{CoreEffectPlan, CoreExitPlan};
use crate::mir::builder::control_flow::verify::is_leaf_effect_plan;

/// V6: Effect ValueId validity
pub(super) fn verify_effect(
    effect: &CoreEffectPlan,
    depth: usize,
    loop_depth: usize,
) -> Result<(), String> {
    match effect {
        CoreEffectPlan::MethodCall {
            dst,
            object,
            method,
            args,
            effects: _,
        } => {
            // P2: dst is now Option<ValueId>
            if let Some(dst_val) = dst {
                primitives::verify_value_id_basic(*dst_val, depth, "MethodCall.dst")?;
            }
            primitives::verify_value_id_basic(*object, depth, "MethodCall.object")?;
            if method.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "method_call_empty_name",
                    format!("MethodCall at depth {} has empty method name", depth),
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                primitives::verify_value_id_basic(*arg, depth, &format!("MethodCall.args[{}]", i))?;
            }
        }
        CoreEffectPlan::GlobalCall { dst, func, args } => {
            if let Some(dst_val) = dst {
                primitives::verify_value_id_basic(*dst_val, depth, "GlobalCall.dst")?;
            }
            if func.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "global_call_empty_func",
                    format!("GlobalCall at depth {} has empty func", depth),
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                primitives::verify_value_id_basic(*arg, depth, &format!("GlobalCall.args[{}]", i))?;
            }
        }
        CoreEffectPlan::ValueCall { dst, callee, args } => {
            if let Some(dst_val) = dst {
                primitives::verify_value_id_basic(*dst_val, depth, "ValueCall.dst")?;
            }
            primitives::verify_value_id_basic(*callee, depth, "ValueCall.callee")?;
            for (i, arg) in args.iter().enumerate() {
                primitives::verify_value_id_basic(*arg, depth, &format!("ValueCall.args[{}]", i))?;
            }
        }
        CoreEffectPlan::ExternCall {
            dst,
            iface_name,
            method_name,
            args,
            effects: _,
        } => {
            if let Some(dst_val) = dst {
                primitives::verify_value_id_basic(*dst_val, depth, "ExternCall.dst")?;
            }
            if iface_name.is_empty() || method_name.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "extern_call_empty_name",
                    format!("ExternCall at depth {} has empty iface/method", depth),
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                primitives::verify_value_id_basic(*arg, depth, &format!("ExternCall.args[{}]", i))?;
            }
        }
        CoreEffectPlan::NewBox {
            dst,
            box_type,
            args,
        } => {
            primitives::verify_value_id_basic(*dst, depth, "NewBox.dst")?;
            if box_type.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "newbox_empty_type",
                    format!("NewBox at depth {} has empty box_type", depth),
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                primitives::verify_value_id_basic(*arg, depth, &format!("NewBox.args[{}]", i))?;
            }
        }
        CoreEffectPlan::FieldGet {
            dst,
            base,
            field,
            declared_type: _,
        } => {
            primitives::verify_value_id_basic(*dst, depth, "FieldGet.dst")?;
            primitives::verify_value_id_basic(*base, depth, "FieldGet.base")?;
            if field.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "field_get_empty_name",
                    format!("FieldGet at depth {} has empty field name", depth),
                ));
            }
        }
        CoreEffectPlan::FieldSet {
            base,
            field,
            value,
            declared_type: _,
        } => {
            primitives::verify_value_id_basic(*base, depth, "FieldSet.base")?;
            primitives::verify_value_id_basic(*value, depth, "FieldSet.value")?;
            if field.is_empty() {
                return Err(primitives::err(
                    "V6",
                    "field_set_empty_name",
                    format!("FieldSet at depth {} has empty field name", depth),
                ));
            }
        }
        CoreEffectPlan::BinOp {
            dst,
            lhs,
            op: _,
            rhs,
        } => {
            primitives::verify_value_id_basic(*dst, depth, "BinOp.dst")?;
            primitives::verify_value_id_basic(*lhs, depth, "BinOp.lhs")?;
            primitives::verify_value_id_basic(*rhs, depth, "BinOp.rhs")?;
        }
        CoreEffectPlan::Compare {
            dst,
            lhs,
            op: _,
            rhs,
        } => {
            primitives::verify_value_id_basic(*dst, depth, "Compare.dst")?;
            primitives::verify_value_id_basic(*lhs, depth, "Compare.lhs")?;
            primitives::verify_value_id_basic(*rhs, depth, "Compare.rhs")?;
        }
        CoreEffectPlan::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            primitives::verify_value_id_basic(*dst, depth, "Select.dst")?;
            primitives::verify_value_id_basic(*cond, depth, "Select.cond")?;
            primitives::verify_value_id_basic(*then_val, depth, "Select.then_val")?;
            primitives::verify_value_id_basic(*else_val, depth, "Select.else_val")?;
        }
        CoreEffectPlan::Const { dst, value: _ } => {
            primitives::verify_value_id_basic(*dst, depth, "Const.dst")?;
        }
        CoreEffectPlan::Copy { dst, src } => {
            primitives::verify_value_id_basic(*dst, depth, "Copy.dst")?;
            primitives::verify_value_id_basic(*src, depth, "Copy.src")?;
        }
        CoreEffectPlan::ExitIf { cond, exit } => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V12",
                    "exit_if_outside_loop",
                    format!("ExitIf outside loop body at depth {}", depth),
                ));
            }
            primitives::verify_value_id_basic(*cond, depth, "ExitIf.cond")?;
            match exit {
                CoreExitPlan::Return(Some(value)) => {
                    primitives::verify_value_id_basic(*value, depth, "ExitIf.return_value")?;
                }
                CoreExitPlan::Return(None) => {
                    return Err(primitives::err(
                        "V12",
                        "exit_if_return_no_payload",
                        format!("ExitIf(Return) without payload at depth {}", depth),
                    ));
                }
                CoreExitPlan::Break(exit_depth) | CoreExitPlan::Continue(exit_depth) => {
                    position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                }
                CoreExitPlan::BreakWithPhiArgs {
                    depth: exit_depth,
                    phi_args,
                } => {
                    position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                    if phi_args.is_empty() {
                        return Err(primitives::err(
                            "V12",
                            "exit_if_break_phi_args_empty",
                            format!(
                                "ExitIf(BreakWithPhiArgs) has empty phi_args at depth {}",
                                depth
                            ),
                        ));
                    }
                    for (dst, src) in phi_args {
                        primitives::verify_value_id_basic(*dst, depth, "ExitIf.break.phi_dst")?;
                        primitives::verify_value_id_basic(*src, depth, "ExitIf.break.phi_src")?;
                    }
                }
                CoreExitPlan::ContinueWithPhiArgs {
                    depth: exit_depth,
                    phi_args,
                } => {
                    position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                    if phi_args.is_empty() {
                        return Err(primitives::err(
                            "V12",
                            "exit_if_continue_phi_args_empty",
                            format!(
                                "ExitIf(ContinueWithPhiArgs) has empty phi_args at depth {}",
                                depth
                            ),
                        ));
                    }
                    for (dst, src) in phi_args {
                        primitives::verify_value_id_basic(*dst, depth, "ExitIf.continue.phi_dst")?;
                        primitives::verify_value_id_basic(*src, depth, "ExitIf.continue.phi_src")?;
                    }
                }
            }
        }
        CoreEffectPlan::IfEffect {
            cond,
            then_effects,
            else_effects,
        } => {
            if loop_depth == 0 {
                return Err(primitives::err(
                    "V12",
                    "if_effect_outside_loop",
                    format!("IfEffect outside loop body at depth {}", depth),
                ));
            }
            primitives::verify_value_id_basic(*cond, depth, "IfEffect.cond")?;
            verify_if_effect_branch(then_effects, depth, loop_depth, "then_effects")?;
            if let Some(else_effects) = else_effects {
                verify_if_effect_branch(else_effects, depth, loop_depth, "else_effects")?;
            }
        }
    }
    Ok(())
}

/// Validate IfEffect branch (V12)
pub(super) fn verify_if_effect_branch(
    effects: &[CoreEffectPlan],
    depth: usize,
    loop_depth: usize,
    label: &str,
) -> Result<(), String> {
    if effects.is_empty() {
        return Err(primitives::err(
            "V12",
            "if_effect_empty",
            format!("IfEffect at depth {} has empty {}", depth, label),
        ));
    }
    for (idx, effect) in effects.iter().enumerate() {
        let is_last = idx + 1 == effects.len();
        match effect {
            CoreEffectPlan::ExitIf {
                cond: exit_cond,
                exit: CoreExitPlan::Continue(exit_depth),
            } if is_last => {
                primitives::verify_value_id_basic(*exit_cond, depth, "IfEffect.continue.cond")?;
                position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
            }
            CoreEffectPlan::ExitIf {
                cond: exit_cond,
                exit:
                    CoreExitPlan::ContinueWithPhiArgs {
                        depth: exit_depth,
                        phi_args,
                    },
            } if is_last => {
                primitives::verify_value_id_basic(*exit_cond, depth, "IfEffect.continue.cond")?;
                position_validators::verify_exit_depth(*exit_depth, loop_depth, depth)?;
                if phi_args.is_empty() {
                    return Err(primitives::err(
                        "V12",
                        "if_effect_continue_phi_args_empty",
                        format!(
                            "IfEffect at depth {} has ContinueWithPhiArgs with empty phi_args",
                            depth
                        ),
                    ));
                }
                for (dst, src) in phi_args {
                    primitives::verify_value_id_basic(*dst, depth, "IfEffect.continue.phi_dst")?;
                    primitives::verify_value_id_basic(*src, depth, "IfEffect.continue.phi_src")?;
                }
            }
            CoreEffectPlan::ExitIf { .. } => {
                return Err(primitives::err(
                    "V12",
                    "if_effect_exit_not_allowed",
                    format!(
                        "IfEffect.{}[{}] has forbidden exit at depth {}",
                        label, idx, depth
                    ),
                ));
            }
            CoreEffectPlan::IfEffect { .. } => {
                verify_effect(effect, depth + 1, loop_depth).map_err(|e| {
                    primitives::err(
                        "V12",
                        "if_effect_invalid",
                        format!("IfEffect.{}[{}] invalid: {}", label, idx, e),
                    )
                })?;
            }
            _ => {
                if !is_leaf_effect_plan(effect) {
                    return Err(primitives::err(
                        "V12",
                        "if_effect_non_leaf",
                        format!(
                            "IfEffect.{}[{}] has non-leaf effect at depth {}",
                            label, idx, depth
                        ),
                    ));
                }
                verify_effect(effect, depth, loop_depth).map_err(|e| {
                    primitives::err(
                        "V12",
                        "if_effect_invalid",
                        format!("IfEffect.{}[{}] invalid: {}", label, idx, e),
                    )
                })?;
            }
        }
    }
    Ok(())
}
