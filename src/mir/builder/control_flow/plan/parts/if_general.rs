//! General if lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.
//! SSOT for try_lower_general_if.

use super::super::steps::build_join_payload;
use super::join_scope::{collect_branch_local_vars_from_body, filter_branch_locals_from_maps};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::normalizer::lower_cond_branch;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

/// View-first general if lowering (SSOT).
pub(in crate::mir::builder) fn try_lower_general_if_view<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    try_lower_general_if_view_impl(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        cond_view,
        then_body,
        else_body,
        error_prefix,
        false,
        lower_block,
    )
}

/// Recipe-authority general if lowering.
///
/// Use this only after router/recipe selection already made the release path
/// authoritative. It keeps the same lowering SSOT, but does not require the
/// strict/dev gate to be ON.
pub(in crate::mir::builder) fn try_lower_general_if_view_recipe_authority<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    try_lower_general_if_view_impl(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        cond_view,
        then_body,
        else_body,
        error_prefix,
        true,
        lower_block,
    )
}

fn try_lower_general_if_view_impl<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    allow_release_recipe_authority: bool,
    mut lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    let release_recipe_authority = allow_release_recipe_authority && !strict_or_dev;
    if !planner_required && !release_recipe_authority {
        return Ok(None);
    }
    if then_body.is_empty() {
        return Ok(None);
    }
    if body_has_exit(then_body) || else_body.map_or(false, |body| body_has_exit(body)) {
        return Ok(None);
    }

    let mut pre_if_map = builder.variable_ctx.variable_map.clone();
    for (name, value_id) in current_bindings.iter() {
        pre_if_map.insert(name.clone(), *value_id);
    }
    let pre_bindings = current_bindings.clone();

    let then_plans = lower_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        then_body,
    )?;
    let then_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let else_plans = match else_body {
        Some(body) => Some(lower_block(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            body,
        )?),
        None => None,
    };
    let else_map = builder.variable_ctx.variable_map.clone();

    builder.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings;

    let branch_locals =
        collect_branch_local_vars_from_body(then_body, else_body.map(|body| body.as_slice()));
    let (then_map, else_map) =
        filter_branch_locals_from_maps(&pre_if_map, &then_map, &else_map, &branch_locals);

    // Phase 8: use step for join payload generation (3-map diff)
    let joins = build_join_payload(builder, &pre_if_map, &then_map, &else_map)?;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        joins.clone(),
        error_prefix,
    )?;

    for join in &joins {
        builder
            .variable_ctx
            .variable_map
            .insert(join.name.clone(), join.dst);
        if carrier_phis.contains_key(&join.name) || current_bindings.contains_key(&join.name) {
            current_bindings.insert(join.name.clone(), join.dst);
        }
    }

    Ok(Some(plans))
}

/// ASTNode-based wrapper (delegates to view-first SSOT).
pub(in crate::mir::builder) fn try_lower_general_if<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let cond_view = CondBlockView::from_expr(condition);
    try_lower_general_if_view(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        &cond_view,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )
}

/// ASTNode-based wrapper for recipe-authority callers that are valid in release.
pub(in crate::mir::builder) fn try_lower_general_if_recipe_authority<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let cond_view = CondBlockView::from_expr(condition);
    try_lower_general_if_view_recipe_authority(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        &cond_view,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )
}

fn body_has_exit(body: &[ASTNode]) -> bool {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    counts.break_count > 0 || counts.continue_count > 0 || counts.return_count > 0
}

#[cfg(test)]
mod tests {
    use super::{try_lower_general_if, try_lower_general_if_recipe_authority};
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::LoweredRecipe;
    use crate::mir::builder::stmts::variable_stmt::build_local_statement;
    use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
    use crate::mir::builder::MirBuilder;
    use std::collections::BTreeMap;

    fn span() -> Span {
        Span::unknown()
    }

    fn lit_bool(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: span(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn assign(name: &str, expr: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(expr),
            span: span(),
        }
    }

    fn lower_simple_stmt_block(
        builder: &mut MirBuilder,
        bindings: &mut BTreeMap<String, crate::mir::ValueId>,
        _carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
        carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
        stmts: &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String> {
        let mut plans = Vec::new();
        for stmt in stmts {
            let mut stmt_plans = super::super::stmt::lower_return_prelude_stmt(
                builder,
                bindings,
                carrier_step_phis,
                None,
                stmt,
                "if_general_test",
            )?;
            plans.append(&mut stmt_plans);
        }
        Ok(plans)
    }

    #[test]
    fn recipe_authority_allows_general_if_lowering_in_release() {
        crate::tests::helpers::joinir_env::with_joinir_env_lock(|| {
            let saved = [
                (
                    "HAKO_JOINIR_STRICT",
                    std::env::var("HAKO_JOINIR_STRICT").ok(),
                ),
                (
                    "NYASH_JOINIR_STRICT",
                    std::env::var("NYASH_JOINIR_STRICT").ok(),
                ),
                (
                    "HAKO_JOINIR_PLANNER_REQUIRED",
                    std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").ok(),
                ),
                ("NYASH_JOINIR_DEV", std::env::var("NYASH_JOINIR_DEV").ok()),
                ("HAKO_JOINIR_DEBUG", std::env::var("HAKO_JOINIR_DEBUG").ok()),
                (
                    "NYASH_JOINIR_DEBUG",
                    std::env::var("NYASH_JOINIR_DEBUG").ok(),
                ),
            ];
            for (key, _) in &saved {
                std::env::remove_var(key);
            }

            let mut builder = MirBuilder::new();
            builder.enter_function_for_test("if_general_recipe_authority_release".to_string());
            let _scope = LexicalScopeGuard::new(&mut builder);
            build_local_statement(
                &mut builder,
                vec!["x".to_string()],
                vec![Some(Box::new(lit_int(0)))],
            )
            .expect("declare x");

            let then_body = vec![assign("x", lit_int(1))];
            let mut default_bindings = builder.variable_ctx.variable_map.clone();
            let empty = BTreeMap::new();
            let default_plans = try_lower_general_if(
                &mut builder,
                &mut default_bindings,
                &empty,
                &empty,
                &lit_bool(true),
                &then_body,
                None,
                "if_general_test",
                lower_simple_stmt_block,
            )
            .expect("default lower should not error");
            assert!(
                default_plans.is_none(),
                "default release wrapper should stay planner-only"
            );

            let mut release_bindings = builder.variable_ctx.variable_map.clone();
            let release_plans = try_lower_general_if_recipe_authority(
                &mut builder,
                &mut release_bindings,
                &empty,
                &empty,
                &lit_bool(true),
                &then_body,
                None,
                "if_general_test",
                lower_simple_stmt_block,
            )
            .expect("recipe-authority lower should not error");
            assert!(
                release_plans.is_some(),
                "recipe-authority wrapper should lower the same general-if in release"
            );
            assert_ne!(default_bindings.get("x"), release_bindings.get("x"));

            builder.exit_function_for_test();

            for (key, value) in saved {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        });
    }
}
