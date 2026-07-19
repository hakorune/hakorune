//! Associated-input tail owner for If-plan conditions.
//!
//! Raw `CondBlockView` prelude policy remains in `cond_lowering_if_plan`; this
//! core owns only the exact tail expression and existing CorePlan semantics.

use super::cond_lowering_freshen::clone_plans_with_fresh_loops;
use super::cond_lowering_if_plan::{
    clone_branch_plans_for_shortcircuit, debug_log_cond_if_lit3_origin, merge_value_maps,
    normalize_empty_branches, remap_joins_with_map,
};
use super::cond_lowering_value_expr::lower_cond_value_input;
use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreIfJoin, CoreIfPlan, CorePlan, LoopPlanExpressionPortV1, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_cond_expr_to_if_plans_input<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let (then_plans, else_plans) = normalize_empty_branches(then_plans, else_plans);
    let else_plans = Some(else_plans.unwrap_or_else(|| vec![CorePlan::Seq(Vec::new())]));

    match port.expr_syntax(&input) {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            ..
        } => {
            let operand = child(port, &input, ExprChildRoleV1::UnaryOperand)?;
            let new_then = else_plans.unwrap_or_default();
            lower_cond_expr_to_if_plans_input(
                port,
                operand,
                builder,
                phi_bindings,
                new_then,
                Some(then_plans),
                joins,
                error_prefix,
            )
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            ..
        } => {
            if joins.is_empty() {
                let else_base = else_plans.unwrap_or_else(|| vec![CorePlan::Seq(Vec::new())]);
                return lower_joinless_leaf_chain(
                    port,
                    input,
                    BinaryOperator::And,
                    builder,
                    phi_bindings,
                    then_plans,
                    else_base,
                    error_prefix,
                );
            }
            let left = child(port, &input, ExprChildRoleV1::BinaryLeft)?;
            let right = child(port, &input, ExprChildRoleV1::BinaryRight)?;
            let (else_for_right, joins_for_right) = match else_plans.as_ref() {
                Some(plans) => {
                    let fresh = clone_plans_with_fresh_loops(builder, plans)?;
                    (
                        Some(fresh.plans),
                        remap_joins_with_map(&joins, &fresh.value_map),
                    )
                }
                None => (None, joins.clone()),
            };
            let (intermediate, right_joins) =
                allocate_intermediate_joins(builder, &joins_for_right);
            let inner = lower_cond_expr_to_if_plans_input(
                port,
                right,
                builder,
                phi_bindings,
                then_plans,
                else_for_right,
                right_joins,
                error_prefix,
            )?;
            let outer_joins = joins
                .iter()
                .zip(intermediate.iter())
                .map(|(join, value)| CoreIfJoin {
                    name: join.name.clone(),
                    dst: join.dst,
                    pre_val: join.pre_val,
                    then_val: *value,
                    else_val: join.else_val,
                })
                .collect();
            lower_cond_expr_to_if_plans_input(
                port,
                left,
                builder,
                phi_bindings,
                inner,
                else_plans,
                outer_joins,
                error_prefix,
            )
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            ..
        } => {
            if joins.is_empty() {
                let else_base = else_plans.unwrap_or_else(|| vec![CorePlan::Seq(Vec::new())]);
                return lower_joinless_leaf_chain(
                    port,
                    input,
                    BinaryOperator::Or,
                    builder,
                    phi_bindings,
                    then_plans,
                    else_base,
                    error_prefix,
                );
            }
            let left = child(port, &input, ExprChildRoleV1::BinaryLeft)?;
            let right = child(port, &input, ExprChildRoleV1::BinaryRight)?;
            let then_fresh = clone_plans_with_fresh_loops(builder, &then_plans)?;
            let else_fresh = match else_plans.as_ref() {
                Some(plans) => Some(clone_plans_with_fresh_loops(builder, plans)?),
                None => None,
            };
            let value_map = merge_value_maps(
                builder,
                &then_fresh.value_map,
                else_fresh.as_ref().map(|fresh| &fresh.value_map),
            )?;
            let joins_for_right = remap_joins_with_map(&joins, &value_map);
            let (intermediate, right_joins) =
                allocate_intermediate_joins(builder, &joins_for_right);
            let inner = lower_cond_expr_to_if_plans_input(
                port,
                right,
                builder,
                phi_bindings,
                then_fresh.plans,
                else_fresh.map(|fresh| fresh.plans),
                right_joins,
                error_prefix,
            )?;
            let outer_joins = joins
                .iter()
                .zip(intermediate.iter())
                .map(|(join, value)| CoreIfJoin {
                    name: join.name.clone(),
                    dst: join.dst,
                    pre_val: join.pre_val,
                    then_val: join.then_val,
                    else_val: *value,
                })
                .collect();
            lower_cond_expr_to_if_plans_input(
                port,
                left,
                builder,
                phi_bindings,
                then_plans,
                Some(inner),
                outer_joins,
                error_prefix,
            )
        }
        _ => lower_leaf(
            port,
            input,
            builder,
            phi_bindings,
            then_plans,
            else_plans,
            joins,
            error_prefix,
        ),
    }
}

fn child<'input, P>(
    port: &P,
    input: &P::ExprInput<'input>,
    role: ExprChildRoleV1,
) -> Result<P::ExprInput<'input>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    port.child_expr(input, role).map_err(|error| error.render())
}

fn allocate_intermediate_joins(
    builder: &mut MirBuilder,
    joins: &[CoreIfJoin],
) -> (Vec<ValueId>, Vec<CoreIfJoin>) {
    let mut values = Vec::with_capacity(joins.len());
    let mut rows = Vec::with_capacity(joins.len());
    for join in joins {
        let ty = builder
            .function_state
            .type_ctx
            .get_type(join.then_val)
            .or_else(|| builder.function_state.type_ctx.get_type(join.else_val))
            .cloned()
            .unwrap_or(MirType::Unknown);
        let value = builder.alloc_typed(ty);
        values.push(value);
        rows.push(CoreIfJoin {
            name: join.name.clone(),
            dst: value,
            pre_val: None,
            then_val: join.then_val,
            else_val: join.else_val,
        });
    }
    (values, rows)
}

fn lower_joinless_leaf_chain<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    operator: BinaryOperator,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Vec<LoweredRecipe>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let mut terms = Vec::new();
    let is_or = matches!(operator, BinaryOperator::Or);
    collect_terms(port, input, is_or, &mut terms)?;
    let tag = if is_or {
        "cond_or_leaf"
    } else {
        "cond_and_leaf"
    };
    if terms.is_empty() {
        return Err(format!(
            "[freeze:contract][{tag}] leaf lowering got empty term list"
        ));
    }

    let (root, mut current) = if is_or {
        (then_plans, else_plans)
    } else {
        (else_plans, then_plans)
    };
    let mut root_branch = Some(root);
    for (index, term) in terms.into_iter().enumerate().rev() {
        let (condition, effects) =
            lower_cond_value_input(port, term, builder, phi_bindings, error_prefix)?;
        debug_log_cond_if_lit3_origin(builder, &effects);
        let branch = if index == 0 {
            root_branch
                .take()
                .ok_or_else(|| format!("[freeze:contract][{tag}] missing root branch"))?
        } else {
            clone_branch_plans_for_shortcircuit(
                builder,
                root_branch
                    .as_ref()
                    .ok_or_else(|| format!("[freeze:contract][{tag}] missing template"))?,
            )?
        };
        let (then_branch, else_branch) = if is_or {
            (branch, current)
        } else {
            (current, branch)
        };
        let mut node = effects_to_plans(effects);
        node.push(CorePlan::If(CoreIfPlan {
            condition,
            then_plans: then_branch,
            else_plans: Some(else_branch),
            joins: Vec::new(),
        }));
        current = node;
    }
    Ok(current)
}

fn collect_terms<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    is_or: bool,
    out: &mut Vec<P::ExprInput<'input>>,
) -> Result<(), String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let same_operator = matches!(
        port.expr_syntax(&input),
        ASTNode::BinaryOp { operator: BinaryOperator::Or, .. } if is_or
    ) || matches!(
        port.expr_syntax(&input),
        ASTNode::BinaryOp { operator: BinaryOperator::And, .. } if !is_or
    );
    if !same_operator {
        out.push(input);
        return Ok(());
    }
    let left = child(port, &input, ExprChildRoleV1::BinaryLeft)?;
    let right = child(port, &input, ExprChildRoleV1::BinaryRight)?;
    collect_terms(port, left, is_or, out)?;
    collect_terms(port, right, is_or, out)
}

fn lower_leaf<'input, P>(
    port: &P,
    input: P::ExprInput<'input>,
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    then_plans: Vec<LoweredRecipe>,
    else_plans: Option<Vec<LoweredRecipe>>,
    joins: Vec<CoreIfJoin>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let (condition, effects) =
        lower_cond_value_input(port, input, builder, phi_bindings, error_prefix)?;
    debug_log_cond_if_lit3_origin(builder, &effects);
    let mut plans = effects_to_plans(effects);
    plans.push(CorePlan::If(CoreIfPlan {
        condition,
        then_plans,
        else_plans,
        joins,
    }));
    Ok(plans)
}
