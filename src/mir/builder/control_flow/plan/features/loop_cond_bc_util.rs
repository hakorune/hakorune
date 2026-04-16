//! Utility functions for item/statement lowering.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::BTreeMap;

use super::loop_cond_bc::LOOP_COND_ERR;
use super::loop_cond_bc_item_stmt::lower_loop_cond_stmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DirectExitRejectReason {
    BlockContainsDirectExit,
    ExitMustBeInsideIf,
    BreakMustBeLast,
    ReturnMustBeLast,
}

const DIRECT_EXIT_REJECT_TAG: &str = "[loop_cond/direct_exit]";

fn direct_exit_reason_text(reason: DirectExitRejectReason) -> &'static str {
    match reason {
        DirectExitRejectReason::BlockContainsDirectExit => "block contains direct exit",
        DirectExitRejectReason::ExitMustBeInsideIf => "exit must be inside if",
        DirectExitRejectReason::BreakMustBeLast => "break must be last",
        DirectExitRejectReason::ReturnMustBeLast => "return must be last",
    }
}

pub(in crate::mir::builder) fn direct_exit_reject(
    error_prefix: &str,
    reason: DirectExitRejectReason,
) -> String {
    format!(
        "{error_prefix}: {DIRECT_EXIT_REJECT_TAG} {}",
        direct_exit_reason_text(reason)
    )
}

pub(in crate::mir::builder) fn is_direct_exit_reject(err: &str) -> bool {
    if err.contains(DIRECT_EXIT_REJECT_TAG) {
        return true;
    }
    // Backward-compatible fallback for legacy message shapes.
    err.contains(": block contains direct exit")
        || err.contains(": exit must be inside if")
        || err.contains(": break must be last")
        || err.contains(": return must be last")
}

pub(super) fn get_stmt<'a>(body: &'a RecipeBody, stmt_ref: StmtRef) -> Result<&'a ASTNode, String> {
    body.get_ref(stmt_ref)
        .ok_or_else(|| format!("{LOOP_COND_ERR}: missing stmt idx={}", stmt_ref.index()))
}

pub(super) fn lower_simple_effect_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if value_has_blockexpr_prelude_loop(value) {
                    let (value_id, plans) = lower_value_stmt_with_blockexpr_loop_prelude(
                        builder,
                        current_bindings,
                        carrier_phis,
                        carrier_step_phis,
                        break_phi_dsts,
                        carrier_updates,
                        value,
                        error_prefix,
                    )?;
                    if carrier_phis.contains_key(name) {
                        carrier_updates.insert(name.clone(), value_id);
                    }
                    if carrier_phis.contains_key(name) || current_bindings.contains_key(name) {
                        current_bindings.insert(name.clone(), value_id);
                    }
                    builder
                        .variable_ctx
                        .variable_map
                        .insert(name.clone(), value_id);
                    return Ok(Some(plans));
                }
            }

            let effects = lower_assignment_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            if variables.len() != initial_values.len() {
                return Err(format!("{error_prefix}: local init arity mismatch"));
            }
            if initial_values
                .iter()
                .flatten()
                .any(|value| value_has_blockexpr_prelude_loop(value))
            {
                let mut plans = Vec::new();
                for (name, init) in variables.iter().zip(initial_values.iter()) {
                    let init_node = loop_body_lowering::local_init_node_or_null(init.as_ref());
                    let (value_id, mut init_plans) = lower_value_stmt_with_blockexpr_loop_prelude(
                        builder,
                        current_bindings,
                        carrier_phis,
                        carrier_step_phis,
                        break_phi_dsts,
                        carrier_updates,
                        init_node.as_ref(),
                        error_prefix,
                    )?;
                    plans.append(&mut init_plans);
                    current_bindings.insert(name.clone(), value_id);
                    builder
                        .variable_ctx
                        .variable_map
                        .insert(name.clone(), value_id);
                }
                return Ok(Some(plans));
            }

            let effects = lower_local_init_stmt(
                builder,
                current_bindings,
                variables,
                initial_values,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Call { .. } => {
            let (_value_id, effects) =
                PlanNormalizer::lower_value_ast(stmt, builder, current_bindings)?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Print { expression, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_ast(expression, builder, current_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            Ok(Some(effects_to_plans(effects)))
        }
        _ => Ok(None),
    }
}

fn lower_value_stmt_with_blockexpr_loop_prelude(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<(crate::mir::ValueId, Vec<LoweredRecipe>), String> {
    let ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr,
        ..
    } = value
    else {
        let (value_id, effects) =
            PlanNormalizer::lower_value_ast(value, builder, current_bindings)?;
        return Ok((value_id, effects_to_plans(effects)));
    };

    if !prelude_stmts
        .iter()
        .any(|stmt| stmt_has_loop_stmt_recursive(stmt))
    {
        let (value_id, effects) =
            PlanNormalizer::lower_value_ast(value, builder, current_bindings)?;
        return Ok((value_id, effects_to_plans(effects)));
    }

    for stmt in prelude_stmts {
        if stmt.contains_non_local_exit_outside_loops() {
            return Err(format!(
                "[freeze:contract][blockexpr] {error_prefix}: exit stmt is forbidden in BlockExpr prelude"
            ));
        }
    }

    let mut block_bindings = current_bindings.clone();
    let mut plans = Vec::new();
    for stmt in prelude_stmts {
        let mut stmt_plans = lower_loop_cond_stmt(
            builder,
            &mut block_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            false,
            stmt,
        )?;
        plans.append(&mut stmt_plans);
    }

    let (tail_id, tail_effects) =
        PlanNormalizer::lower_value_ast(tail_expr.as_ref(), builder, &block_bindings)?;
    plans.extend(effects_to_plans(tail_effects));
    Ok((tail_id, plans))
}

fn value_has_blockexpr_prelude_loop(value: &ASTNode) -> bool {
    let ASTNode::BlockExpr { prelude_stmts, .. } = value else {
        return false;
    };
    prelude_stmts.iter().any(stmt_has_loop_stmt_recursive)
}

fn stmt_has_loop_stmt_recursive(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { .. } | ASTNode::While { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_loop_stmt_recursive)
                || else_body
                    .as_ref()
                    .is_some_and(|body| body.iter().any(stmt_has_loop_stmt_recursive))
        }
        ASTNode::Program { statements, .. } => statements.iter().any(stmt_has_loop_stmt_recursive),
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_loop_stmt_recursive),
        _ => false,
    }
}

pub(super) fn lower_stmt_list_no_exit(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    use crate::mir::builder::control_flow::plan::CoreEffectPlan;

    let mut block_plans = Vec::new();
    for (idx, stmt) in stmts.iter().enumerate() {
        let is_last = idx + 1 == stmts.len();
        let mut plans = lower_loop_cond_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            is_last,
            stmt,
        )?;
        if plans.iter().any(|plan| {
            matches!(plan, CorePlan::Exit(_))
                || matches!(plan, CorePlan::Effect(CoreEffectPlan::ExitIf { .. }))
        }) {
            return Err(format!("{error_prefix}: block contains exit"));
        }
        block_plans.append(&mut plans);
    }
    Ok(block_plans)
}

pub(super) fn lower_stmt_list_no_direct_exit(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut block_plans = Vec::new();
    for (idx, stmt) in stmts.iter().enumerate() {
        let is_last = idx + 1 == stmts.len();
        let mut plans = lower_loop_cond_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            is_last,
            stmt,
        )?;
        if plans.iter().any(|plan| matches!(plan, CorePlan::Exit(_))) {
            return Err(direct_exit_reject(
                error_prefix,
                DirectExitRejectReason::BlockContainsDirectExit,
            ));
        }
        block_plans.append(&mut plans);
    }
    Ok(block_plans)
}

#[cfg(test)]
mod tests {
    use super::{direct_exit_reject, is_direct_exit_reject, DirectExitRejectReason};

    #[test]
    fn direct_exit_reject_is_tagged_and_detectable() {
        let err = direct_exit_reject(
            "[normalizer] loop_cond_break_continue",
            DirectExitRejectReason::ExitMustBeInsideIf,
        );
        assert!(is_direct_exit_reject(&err));
        assert!(err.contains("[loop_cond/direct_exit]"));
    }

    #[test]
    fn direct_exit_reject_accepts_legacy_strings() {
        let legacy = "[normalizer] loop_cond_break_continue: break must be last";
        assert!(is_direct_exit_reject(legacy));
    }
}
