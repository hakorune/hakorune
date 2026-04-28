//! Statement-level lowering for continue-only pattern.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::features::exit_if_map::lower_if_exit_stmt;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::parts::conditional_update::try_lower_general_if_recipe_authority;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::ContinueOnlyStmtRecipe;
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::BTreeMap;

use super::loop_cond_co_continue_if::{lower_continue_if_group_prelude, lower_continue_if_no_else};
use super::loop_cond_co_group_if::{lower_continue_if_nested_loop, lower_group_if};
use super::loop_cond_co_helpers::{get_body_stmt, sync_carrier_bindings};

const LOOP_COND_CONTINUE_ONLY_ERR: &str = "[normalizer] loop_cond_continue_only";

/// Lower a single continue-only statement recipe to CorePlans.
pub(super) fn lower_continue_only_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &BodyView<'_>,
    stmt: &ContinueOnlyStmtRecipe,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ContinueOnlyStmtRecipe::Stmt(node) => {
            let stmt = get_body_stmt(body, *node, LOOP_COND_CONTINUE_ONLY_ERR)?;
            lower_stmt_ast(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                stmt,
            )
        }
        ContinueOnlyStmtRecipe::ContinueIf {
            if_stmt,
            prelude_span,
        } => {
            let stmt = get_body_stmt(body, *if_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_ONLY_ERR}: recipe if mismatch (ContinueIf)"
                ));
            };
            if else_body.is_some() {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_ONLY_ERR}: continue-if must not have else branch"
                ));
            }
            let then_view = BodyView::Slice(then_body);
            lower_continue_if_no_else(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                &then_view,
                *prelude_span,
            )
        }
        ContinueOnlyStmtRecipe::ContinueIfGroupPrelude {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let stmt = get_body_stmt(body, *if_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
            let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_ONLY_ERR}: recipe if mismatch (ContinueIfGroupPrelude)"
                ));
            };
            if else_body.is_some() {
                return Err(format!(
                    "{LOOP_COND_CONTINUE_ONLY_ERR}: continue-if must not have else branch"
                ));
            }
            let then_view = BodyView::Slice(then_body);
            lower_continue_if_group_prelude(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                &then_view,
                *prelude_span,
                prelude_items,
            )
        }
        ContinueOnlyStmtRecipe::GroupIf {
            then_body,
            else_body,
            if_stmt,
        } => lower_group_if(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            if_stmt,
            body,
            then_body,
            else_body.as_ref(),
        ),
        ContinueOnlyStmtRecipe::ContinueIfNestedLoop {
            inner_loop_prelude_span,
            inner_loop_prelude_items,
            inner_loop_body,
            inner_loop_stmt,
            inner_loop_postlude_span,
            inner_loop_postlude_items,
            if_stmt,
        } => lower_continue_if_nested_loop(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            if_stmt,
            body,
            *inner_loop_prelude_span,
            inner_loop_prelude_items,
            inner_loop_body,
            inner_loop_stmt,
            *inner_loop_postlude_span,
            inner_loop_postlude_items,
        ),
    }
}

/// Lower a single AST statement to CorePlans.
pub(super) fn lower_stmt_ast(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
) -> Result<Vec<LoweredRecipe>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let effects = lower_assignment_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
                LOOP_COND_CONTINUE_ONLY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let effects = lower_local_init_stmt(
                builder,
                current_bindings,
                variables,
                initial_values,
                LOOP_COND_CONTINUE_ONLY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_CONTINUE_ONLY_ERR,
            )?;
            Ok(effects_to_plans(effects))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                LOOP_COND_CONTINUE_ONLY_ERR,
            )?;
            Ok(effects_to_plans(effects))
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
            Ok(effects_to_plans(effects))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            if let Some(plans) = parts::entry::lower_conditional_update_if(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                condition,
                then_body,
                else_body.as_ref(),
                LOOP_COND_CONTINUE_ONLY_ERR,
            )? {
                return Ok(plans);
            }
            if let Some(plans) = try_lower_general_if_recipe_authority(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                condition,
                then_body,
                else_body.as_ref(),
                LOOP_COND_CONTINUE_ONLY_ERR,
                |builder, bindings, carrier_phis, carrier_step_phis, body| {
                    // General-if body inside this box must not contain exits.
                    let mut carrier_updates = BTreeMap::new();
                    let mut out = Vec::new();
                    for stmt in body {
                        let mut plans = lower_stmt_ast(
                            builder,
                            bindings,
                            carrier_phis,
                            carrier_step_phis,
                            &mut carrier_updates,
                            stmt,
                        )?;
                        if plans.iter().any(|plan| matches!(plan, CorePlan::Exit(_))) {
                            return Err(format!(
                                "{LOOP_COND_CONTINUE_ONLY_ERR}: general-if body contains exit"
                            ));
                        }
                        out.append(&mut plans);
                    }
                    Ok(out)
                },
            )? {
                return Ok(plans);
            }
            lower_if_exit_stmt(
                builder,
                current_bindings,
                carrier_step_phis,
                condition,
                then_body,
                else_body.as_ref(),
                LOOP_COND_CONTINUE_ONLY_ERR,
            )
        }
        ASTNode::Continue { .. } => Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: continue must be inside if"
        )),
        ASTNode::Break { .. } | ASTNode::Return { .. } => Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: break/return out-of-scope"
        )),
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => {
            // Phase 29bq: Allow nested loops in group-if body (e.g., hex-parsing loop in _decode_escapes)
            let plan = lower_nested_loop_depth1_any(
                builder,
                condition,
                body,
                LOOP_COND_CONTINUE_ONLY_ERR,
            )?;
            // Sync bindings after nested loop (loop may modify carrier variables)
            sync_carrier_bindings(builder, current_bindings, carrier_phis);
            Ok(vec![plan])
        }
        ASTNode::ForRange { .. } => {
            // ForRange has a different structure; delegate to nested_loop_depth1 via a match.
            // For now, reject as unsupported; can be expanded if needed.
            Err(format!(
                "{LOOP_COND_CONTINUE_ONLY_ERR}: nested ForRange out-of-scope"
            ))
        }
        _ => Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: unsupported stmt {:?}",
            stmt
        )),
    }
}
