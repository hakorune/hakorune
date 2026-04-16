//! Core extraction pipeline for loop_cond_break_continue facts.
//!
//! This module contains the main facts extraction function that orchestrates
//! the entire extraction process.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::is_true_literal;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_accept, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::{
    LoopCondBreakContinueItem, LoopCondBreakContinueRecipe,
};
use crate::mir::builder::control_flow::recipes::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;

use super::break_continue_accept::determine_accept_kind;
use super::break_continue_helpers::{
    collect_continue_branch_sigs, detect_handled_guard_break, matches_parse_string2_shape,
};
use super::break_continue_item::build_loop_cond_break_continue_recipe;
use super::break_continue_types::{LoopCondBreakAcceptKind, LoopCondBreakContinueFacts};
use super::break_continue_validator_exit::returns_only_in_exit_if;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem;
use crate::mir::policies::BodyLoweringPolicy;

/// Core extraction function for loop_cond_break_continue facts.
///
/// This function performs the full extraction pipeline:
/// 1. Validate condition and basic constraints
/// 2. Build the recipe for the loop body
/// 3. Determine the accept kind
/// 4. Construct the final facts
pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts_inner(
    condition: &ASTNode,
    body: &[ASTNode],
    allow_nested: bool,
    allow_extended: bool,
    debug: bool,
    max_nested_loops: usize,
    require_nested_loops: Option<usize>,
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    if is_true_literal(condition) {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::ConditionIsTrue,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }
    if !is_supported_bool_expr_with_canon(condition, allow_extended) {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::ConditionNotSupported,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }

    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    if counts.has_nested_loop {
        if !allow_nested {
            log_reject(
                "loop_cond_break_continue",
                RejectReason::NestedLoopNotAllowed,
                handoff_tables::for_loop_cond_break_continue,
            );
            return Ok(None);
        }
    }
    let body_exit_allowed_probe = if allow_extended && counts.return_count > 0 {
        try_build_exit_allowed_block_recipe(body, allow_extended)
    } else {
        None
    };
    if counts.return_count > 0 {
        let returns_shape_ok = returns_only_in_exit_if(body, allow_extended);
        let allow_return_via_exit_allowed = allow_extended && body_exit_allowed_probe.is_some();
        if !allow_extended || (!returns_shape_ok && !allow_return_via_exit_allowed) {
            log_reject(
                "loop_cond_break_continue",
                RejectReason::ReturnInBody,
                handoff_tables::for_loop_cond_break_continue,
            );
            return Ok(None);
        }
    }
    if counts.return_count > 0 {
        log_accept("loop_cond_break_continue", "return_in_exit_if");
    }
    let no_break_or_continue = counts.break_count == 0 && counts.continue_count == 0;
    if no_break_or_continue && counts.return_count > 0 {
        log_accept("loop_cond_break_continue", "return_only_body");
    }
    let continue_only = counts.continue_count > 0 && counts.break_count == 0;

    let mut exit_if_seen = 0usize;
    let mut conditional_update_seen = 0usize;
    let mut nested_seen = 0usize;
    let mut continue_if_seen = 0usize;
    let recipe = match build_loop_cond_break_continue_recipe(
        body,
        allow_nested,
        allow_extended,
        max_nested_loops,
        debug,
        &mut exit_if_seen,
        &mut continue_if_seen,
        &mut conditional_update_seen,
        &mut nested_seen,
        true,
    ) {
        Some(recipe) => recipe,
        None => {
            if let Some(recipe) = try_build_exit_free_if_stmt_recipe(
                body,
                allow_extended,
                &mut conditional_update_seen,
            ) {
                recipe
            } else {
                log_reject(
                    "loop_cond_break_continue",
                    RejectReason::UnsupportedStmt,
                    handoff_tables::for_loop_cond_break_continue,
                );
                return Ok(None);
            }
        }
    };

    if let Some(required) = require_nested_loops {
        if nested_seen != required {
            log_reject(
                "loop_cond_break_continue",
                RejectReason::NestedLoopCount,
                handoff_tables::for_loop_cond_break_continue,
            );
            return Ok(None);
        }
    }

    // ExitIf item with a stmt-only exit-allowed payload is not structurally safe in this route:
    // lowering can collapse branch-specific values onto a shared predecessor and violate dominance.
    // Let other loop routes handle these shapes.
    let has_stmt_only_exit_if_payload = recipe.items.iter().any(|item| {
        let LoopCondBreakContinueItem::ExitIf {
            block: Some(block), ..
        } = item
        else {
            return false;
        };
        block
            .block
            .items
            .iter()
            .all(|it| matches!(it, RecipeItem::Stmt(_)))
    });
    if has_stmt_only_exit_if_payload {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::UnsupportedStmt,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }

    let program_block_seen = recipe.items.iter().any(|item| {
        matches!(
            item,
            LoopCondBreakContinueItem::ProgramBlock { .. }
        )
    });
    let has_exit_signal = counts.break_count > 0
        || counts.continue_count > 0
        || counts.return_count > 0
        || exit_if_seen > 0
        || continue_if_seen > 0
        || conditional_update_seen > 0
        || (require_nested_loops.is_some()
            && (program_block_seen || nested_seen > 0 || counts.has_nested_loop));
    if !has_exit_signal {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::NoBreakOrContinue,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }
    let allow_cluster_without_exit = require_nested_loops.is_some()
        && (nested_seen > 0 || program_block_seen || counts.has_nested_loop);
    let exit_site_count = counts.break_count
        + counts.continue_count
        + counts.return_count
        + exit_if_seen
        + continue_if_seen;
    let break_kind = if exit_site_count <= 1 {
        "Single"
    } else {
        "Multi"
    };
    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[plan/trace:loopcond_flags] break={} continue={} return={} exit_if={} continue_if={} cond_update={} nested={} no_break_or_continue={} allow_cluster_without_exit={}",
            counts.break_count,
            counts.continue_count,
            counts.return_count,
            exit_if_seen,
            continue_if_seen,
            conditional_update_seen,
            nested_seen,
            no_break_or_continue,
            allow_cluster_without_exit
        ));
        ring0.log.debug(&format!(
            "[plan/trace:loopcond_break_kind] kind={} exit_sites={}",
            break_kind, exit_site_count
        ));
    }
    if no_break_or_continue && conditional_update_seen == 0 && !allow_cluster_without_exit {
        if !allow_extended || counts.return_count == 0 {
            log_reject(
                "loop_cond_break_continue",
                RejectReason::NoBreakOrContinue,
                handoff_tables::for_loop_cond_break_continue,
            );
            return Ok(None);
        }
    }

    if exit_if_seen == 0
        && conditional_update_seen == 0
        && continue_if_seen == 0
        && !allow_cluster_without_exit
    {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::NoExitIf,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }
    if program_block_seen && no_break_or_continue {
        log_accept("loop_cond_break_continue", "program_block_no_exit");
    }
    if continue_only && continue_if_seen == 0 {
        if exit_if_seen == 0 || nested_seen == 0 {
            log_reject(
                "loop_cond_break_continue",
                RejectReason::ContinueOnly,
                handoff_tables::for_loop_cond_break_continue,
            );
            return Ok(None);
        }
    }

    let (has_handled_guard_break, handled_var_name) = detect_handled_guard_break(body);
    let continue_branches = collect_continue_branch_sigs(body);
    let accept_kind = determine_accept_kind(
        &counts,
        exit_if_seen,
        continue_if_seen,
        conditional_update_seen,
        &recipe,
    )?;

    let has_then_only_break = recipe
        .items
        .iter()
        .any(|item| matches!(item, LoopCondBreakContinueItem::ThenOnlyBreakIf { .. }));
    let is_parse_string2 = matches_parse_string2_shape(body);
    let body_lowering_policy =
        if !allow_extended || has_then_only_break || is_parse_string2 || program_block_seen {
            // ProgramBlock items are lowered item-by-item via recipe path.
            // Forcing ExitAllowed here can reject valid recipes when the whole-body
            // exit_allowed block is unavailable (e.g. nested-if + break tail shapes).
            BodyLoweringPolicy::RecipeOnly
        } else {
            BodyLoweringPolicy::ExitAllowed {
                allow_join_if: false,
            }
        };

    let body_exit_allowed = match body_lowering_policy {
        BodyLoweringPolicy::ExitAllowed { .. } => body_exit_allowed_probe
            .clone()
            .or_else(|| try_build_exit_allowed_block_recipe(body, allow_extended)),
        BodyLoweringPolicy::RecipeOnly => None,
    };
    if matches!(body_lowering_policy, BodyLoweringPolicy::ExitAllowed { .. })
        && body_exit_allowed.is_none()
    {
        log_reject(
            "loop_cond_break_continue",
            RejectReason::ExitAllowedRecipeBuildFailed,
            handoff_tables::for_loop_cond_break_continue,
        );
        return Ok(None);
    }

    Ok(Some(LoopCondBreakContinueFacts {
        accept_kind,
        propagate_nested_carriers: accept_kind == LoopCondBreakAcceptKind::NestedLoopOnly,
        condition: condition.clone(),
        recipe,
        has_handled_guard_break,
        handled_var_name,
        continue_branches,
        body_lowering_policy,
        body_exit_allowed,
    }))
}

fn try_build_exit_free_if_stmt_recipe(
    body: &[ASTNode],
    allow_extended: bool,
    conditional_update_seen: &mut usize,
) -> Option<LoopCondBreakContinueRecipe> {
    let mut items = Vec::with_capacity(body.len());
    for (idx, stmt) in body.iter().enumerate() {
        match stmt {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if if_has_exit_signals(then_body, else_body.as_ref()) {
                    return None;
                }
                // Keep accept_kind non-empty: GeneralIf is counted via conditional_update_seen.
                *conditional_update_seen += 1;
                items.push(LoopCondBreakContinueItem::Stmt(StmtRef::new(idx)));
            }
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Loop { .. }
            | ASTNode::Program { .. }
            | ASTNode::ScopeBox { .. } => {
                items.push(LoopCondBreakContinueItem::Stmt(StmtRef::new(idx)));
            }
            ASTNode::Print { .. } if allow_extended => {
                items.push(LoopCondBreakContinueItem::Stmt(StmtRef::new(idx)));
            }
            _ => return None,
        }
    }
    Some(LoopCondRecipe::new(body.to_vec(), items))
}

fn if_has_exit_signals(then_body: &[ASTNode], else_body: Option<&Vec<ASTNode>>) -> bool {
    let then_counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(then_body);
    if then_counts.break_count > 0 || then_counts.continue_count > 0 || then_counts.return_count > 0
    {
        return true;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    let else_counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(else_body);
    else_counts.break_count > 0 || else_counts.continue_count > 0 || else_counts.return_count > 0
}

#[cfg(test)]
mod tests {
    use super::super::break_continue_types::MAX_NESTED_LOOPS;
    use super::try_extract_loop_cond_break_continue_facts_inner;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
    use crate::mir::policies::BodyLoweringPolicy;

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn cond_lt(var: &str, value: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(var)),
            right: Box::new(lit_int(value)),
            span: Span::unknown(),
        }
    }

    fn cond_eq_zero(var: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(v(var)),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        }
    }

    fn cond_ge_zero(var: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(v(var)),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        }
    }

    fn assign_inc(var: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(var)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(var)),
                right: Box::new(lit_int(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn local_int(name: &str, value: i64) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(lit_int(value)))],
            span: Span::unknown(),
        }
    }

    fn if_break_eq_zero(var: &str) -> ASTNode {
        ASTNode::If {
            condition: Box::new(cond_eq_zero(var)),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    #[test]
    fn policy_recipe_only_when_not_extended() {
        let condition = cond_lt("i", 1);
        let body = vec![
            ASTNode::If {
                condition: Box::new(cond_eq_zero("i")),
                then_body: vec![assign_inc("i")],
                else_body: None,
                span: Span::unknown(),
            },
            assign_inc("i"),
        ];

        let facts = try_extract_loop_cond_break_continue_facts_inner(
            &condition,
            &body,
            false,
            false,
            false,
            MAX_NESTED_LOOPS,
            None,
        )
        .expect("freeze")
        .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
    }

    #[test]
    fn policy_recipe_only_with_then_only_break() {
        let condition = cond_lt("i", 2);
        let body = vec![
            ASTNode::If {
                condition: Box::new(cond_eq_zero("i")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: Some(vec![assign_inc("i")]),
                span: Span::unknown(),
            },
            assign_inc("i"),
        ];

        let facts = try_extract_loop_cond_break_continue_facts_inner(
            &condition,
            &body,
            true,
            true,
            false,
            MAX_NESTED_LOOPS,
            None,
        )
        .expect("freeze")
        .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
    }

    #[test]
    fn accepts_nested_guard_break_if_as_program_block_recipe_only() {
        let condition = cond_lt("scan", 10);
        let body = vec![
            local_int("t_idx", 1),
            if_break_eq_zero("t_idx"),
            local_int("atype", 1),
            if_break_eq_zero("atype"),
            ASTNode::If {
                condition: Box::new(cond_eq_zero("atype")),
                then_body: vec![
                    local_int("v_idx", 1),
                    if_break_eq_zero("v_idx"),
                    local_int("x", 1),
                ],
                else_body: Some(vec![
                    local_int("n_idx", 2),
                    if_break_eq_zero("n_idx"),
                    local_int("y", 2),
                ]),
                span: Span::unknown(),
            },
            assign_inc("scan"),
        ];

        let facts = try_extract_loop_cond_break_continue_facts_inner(
            &condition,
            &body,
            true,
            true,
            false,
            MAX_NESTED_LOOPS,
            None,
        )
        .expect("freeze")
        .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(matches!(
            facts.recipe.items[4],
            LoopCondBreakContinueItem::ProgramBlock {
                stmt_only: None,
                ..
            }
        ));
    }

    #[test]
    fn accepts_nested_loop_if_as_program_block_recipe_only() {
        let condition = cond_lt("pos", 10);
        let nested_loop = ASTNode::Loop {
            condition: Box::new(cond_lt("j", 3)),
            body: vec![if_break_eq_zero("j"), assign_inc("j")],
            span: Span::unknown(),
        };
        let body = vec![
            local_int("name_idx", 1),
            if_break_eq_zero("name_idx"),
            local_int("params_idx", 1),
            ASTNode::If {
                condition: Box::new(cond_ge_zero("params_idx")),
                then_body: vec![local_int("j", 0), nested_loop],
                else_body: None,
                span: Span::unknown(),
            },
            assign_inc("pos"),
        ];

        let facts = try_extract_loop_cond_break_continue_facts_inner(
            &condition,
            &body,
            true,
            false,
            false,
            MAX_NESTED_LOOPS,
            None,
        )
        .expect("freeze")
        .expect("facts");

        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(matches!(
            facts.recipe.items[3],
            LoopCondBreakContinueItem::ProgramBlock {
                stmt_only: None,
                ..
            }
        ));
    }

    #[test]
    fn program_block_with_exit_signals_prefers_recipe_only() {
        let condition = cond_lt("j", 3);
        let body = vec![
            assign_inc("j"),
            ASTNode::If {
                condition: Box::new(cond_eq_zero("j")),
                then_body: vec![
                    assign_inc("x"),
                    ASTNode::Continue {
                        span: Span::unknown(),
                    },
                ],
                else_body: Some(vec![
                    ASTNode::If {
                        condition: Box::new(cond_ge_zero("x")),
                        then_body: vec![assign_inc("x")],
                        else_body: Some(vec![assign_inc("x")]),
                        span: Span::unknown(),
                    },
                    ASTNode::Break {
                        span: Span::unknown(),
                    },
                ]),
                span: Span::unknown(),
            },
            assign_inc("j"),
        ];

        let facts = try_extract_loop_cond_break_continue_facts_inner(
            &condition,
            &body,
            true,
            true,
            false,
            MAX_NESTED_LOOPS,
            None,
        )
        .expect("freeze")
        .expect("facts");

        assert!(matches!(
            facts.recipe.items[1],
            LoopCondBreakContinueItem::ProgramBlock { .. }
        ));
        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
    }
}
