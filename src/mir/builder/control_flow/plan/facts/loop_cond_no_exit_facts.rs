//! loop_cond_no_exit facts extractor.
//!
//! S0: MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0
//!
//! Owns conditional loops whose body carries zero exit-signal items:
//! no break / continue / return, no exit-if, no conditional-update item,
//! and no nested loop. Produces `LoopCondBreakContinueFacts` with
//! `LoopCondBreakAcceptKind::NoExitBody` so the canonical issuer
//! (CallableGenericLoopSourceFactsIssuerV1), the LoopCond route match,
//! and the located-source physical lowerer stay the single owner chain.
//!
//! Disjointness: `loop_builder` runs this extractor only when
//! loop_cond_break_continue (cluster + base) already rejected the same
//! (condition, body) pair. Exit-signal bodies always belong to that
//! sibling; everything else outside the bounded vocabulary returns
//! `Ok(None)` so `FactsAbsent` remains the honest terminal.
//!
//! The accumulator-over-induction recurrence family (exact two-assignment
//! body `x = x ⊕ <reads v>` then `v = v ⊕ …` on a condition-referenced `v`)
//! is owned exclusively by the callable-seam VariableAccumRecurrence lane
//! (M10b-I0-R0-VAR). Claiming it here mints a competing LoopCond fact and
//! trips the designed `VariableAccumRecurrenceOverlap` freeze, so this
//! extractor defers the family instead (S10).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::is_true_literal;
use crate::mir::builder::control_flow::facts::loop_cond_break_continue::{
    LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
};
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_helpers::{
    collect_vars_from_expr, matches_parse_string2_shape,
};
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_item::build_loop_cond_break_continue_recipe;
use crate::mir::builder::control_flow::plan::loop_cond::loop_cond_unified_helpers::count_control_flow_with_returns;
use crate::mir::builder::control_flow::plan::loop_cond::MAX_NESTED_LOOPS;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::policies::BodyLoweringPolicy;

use super::reject_reason::{handoff_tables, log_reject, RejectReason};

const BOX_NAME: &str = "loop_cond_no_exit";

/// Extract facts for `loop(condition) { body }` when the body carries no
/// exit signals. Bounded slice: only `Stmt` / `ProgramBlock` / `GeneralIf`
/// recipe items (the vocabulary the located-source parts driver lowers).
/// `ConditionalUpdateIf` is F2's boundary and is rejected here.
pub(in crate::mir::builder) fn try_extract_loop_cond_no_exit_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    if is_true_literal(condition) {
        log_reject(
            BOX_NAME,
            RejectReason::ConditionIsTrue,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    let allow_extended = planner_required
        || (strict_or_dev && matches_parse_string2_shape(body))
        || body.iter().any(ASTNode::contains_return_stmt);
    if !is_supported_bool_expr_with_canon(condition, allow_extended) {
        log_reject(
            BOX_NAME,
            RejectReason::ConditionNotSupported,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }

    let counts = count_control_flow_with_returns(body);
    if counts.has_nested_loop {
        log_reject(
            BOX_NAME,
            RejectReason::NestedLoopNotAllowed,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }
    if counts.break_count > 0 || counts.continue_count > 0 || counts.return_count > 0 {
        log_reject(
            BOX_NAME,
            RejectReason::ExitSignalPresent,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }
    if claims_variable_accum_family(condition, body) {
        log_reject(
            BOX_NAME,
            RejectReason::VariableAccumFamily,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }

    let debug = crate::config::env::is_joinir_debug();
    let mut exit_if_seen = 0usize;
    let mut continue_if_seen = 0usize;
    let mut conditional_update_seen = 0usize;
    let mut nested_seen = 0usize;
    let recipe = match build_loop_cond_break_continue_recipe(
        body,
        false,
        allow_extended,
        MAX_NESTED_LOOPS,
        debug,
        &mut exit_if_seen,
        &mut continue_if_seen,
        &mut conditional_update_seen,
        &mut nested_seen,
        false,
    ) {
        Some(recipe) => recipe,
        None => {
            log_reject(
                BOX_NAME,
                RejectReason::UnsupportedStmt,
                handoff_tables::for_loop_cond_no_exit,
            );
            return Ok(None);
        }
    };

    if exit_if_seen > 0 || continue_if_seen > 0 || nested_seen > 0 {
        log_reject(
            BOX_NAME,
            RejectReason::ExitSignalPresent,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }
    let all_supported = recipe.items.iter().all(|item| {
        matches!(
            item,
            LoopCondBreakContinueItem::Stmt(_)
                | LoopCondBreakContinueItem::ProgramBlock { .. }
                | LoopCondBreakContinueItem::GeneralIf(_)
        )
    });
    if !all_supported {
        log_reject(
            BOX_NAME,
            RejectReason::UnsupportedStmt,
            handoff_tables::for_loop_cond_no_exit,
        );
        return Ok(None);
    }

    Ok(Some(LoopCondBreakContinueFacts {
        accept_kind: LoopCondBreakAcceptKind::NoExitBody,
        condition: condition.clone(),
        recipe,
        has_handled_guard_break: false,
        handled_var_name: None,
        continue_branches: Vec::new(),
        body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
        body_exit_allowed: None,
    }))
}

/// `v = v <binop> …` — an assignment whose binary-op value reads its own
/// target. Returns the target name and the value expression.
fn self_recurrence_assign(stmt: &ASTNode) -> Option<(&String, &ASTNode)> {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    let ASTNode::BinaryOp { left, right, .. } = value.as_ref() else {
        return None;
    };
    let mut vars = std::collections::BTreeSet::new();
    if !collect_vars_from_expr(left, &mut vars) || !collect_vars_from_expr(right, &mut vars) {
        return None;
    }
    vars.contains(name).then_some((name, value.as_ref()))
}

/// The VariableAccumRecurrence family profile: exactly two statements —
/// `accum = accum ⊕ <reads induction>` then `induction = induction ⊕ …` —
/// on distinct targets where the loop condition references `induction`.
/// Narrow by construction: single-statement steps, method-call bodies, and
/// same-target repeats stay in this extractor's own family.
fn claims_variable_accum_family(condition: &ASTNode, body: &[ASTNode]) -> bool {
    let [update, step] = body else {
        return false;
    };
    let Some((accumulator, update_value)) = self_recurrence_assign(update) else {
        return false;
    };
    let Some((induction, _)) = self_recurrence_assign(step) else {
        return false;
    };
    if accumulator == induction {
        return false;
    }
    let mut update_vars = std::collections::BTreeSet::new();
    if !collect_vars_from_expr(update_value, &mut update_vars)
        || !update_vars.contains(induction)
    {
        return false;
    }
    let mut condition_vars = std::collections::BTreeSet::new();
    collect_vars_from_expr(condition, &mut condition_vars) && condition_vars.contains(induction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;

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

    fn cond_lt_var(var: &str, bound: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(var)),
            right: Box::new(v(bound)),
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

    fn method_push(object: &str, arg: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(v(object)),
            method: "push".to_string(),
            arguments: vec![v(arg)],
            span: Span::unknown(),
        }
    }

    fn assign_accum(target: &str, arg: &str) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(v(target)),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v(target)),
                right: Box::new(v(arg)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn accepts_variable_bound_step_only_body() {
        // loop(i < n) { i = i + 1 }: variable bound, zero exit signals.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_inc("i")];

        let facts = try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");

        assert!(matches!(
            facts.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
        assert!(matches!(
            facts.body_lowering_policy,
            BodyLoweringPolicy::RecipeOnly
        ));
        assert!(facts.continue_branches.is_empty());
        assert!(facts.body_exit_allowed.is_none());
        assert!(facts
            .recipe
            .items
            .iter()
            .all(|item| matches!(item, LoopCondBreakContinueItem::Stmt(_))));
    }

    #[test]
    fn accepts_carrier_method_call_body() {
        // loop(i < capacity) { free_stack.push(i); i = i + 1 }
        let condition = cond_lt_var("i", "capacity");
        let body = vec![
            method_push("free_stack", "i"),
            method_push("block_used", "i"),
            assign_inc("i"),
        ];

        let facts = try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");

        assert!(matches!(
            facts.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
        assert_eq!(facts.recipe.items.len(), 3);
    }

    #[test]
    fn defers_variable_accum_recurrence_family() {
        // loop(i < 4) { acc = acc + i; i = i + 1 }: the exact
        // accumulator-over-induction profile is owned by the callable-seam
        // VariableAccumRecurrence lane; claiming it here produces the
        // designed overlap freeze.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_accum("acc", "i"), assign_inc("i")];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn keeps_same_target_self_recurrence() {
        // loop(i < n) { i = i + 1; i = i + 1 }: same-target repeats are not
        // the distinct-binding recurrence family.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_inc("i"), assign_inc("i")];

        let facts = try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");

        assert!(matches!(
            facts.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
    }

    #[test]
    fn keeps_induction_free_accum_update() {
        // loop(i < n) { acc = acc + k; i = i + 1 }: the accumulator update
        // does not read the induction variable, so this is not the VAR
        // family profile.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_accum("acc", "k"), assign_inc("i")];

        let facts = try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");

        assert!(matches!(
            facts.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
    }

    #[test]
    fn rejects_break_body() {
        // Exit-bearing bodies belong to loop_cond_break_continue.
        let condition = cond_lt_var("i", "n");
        let body = vec![
            ASTNode::If {
                condition: Box::new(cond_lt_var("i", "i")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign_inc("i"),
        ];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn rejects_conditional_update_if() {
        // `if cond { carrier = carrier op x }` (no exit) is F2's boundary,
        // not this extractor's vocabulary.
        let condition = cond_lt_var("i", "n");
        let body = vec![
            ASTNode::If {
                condition: Box::new(cond_lt_var("i", "n")),
                then_body: vec![assign_inc("out")],
                else_body: None,
                span: Span::unknown(),
            },
            assign_inc("i"),
        ];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn rejects_true_condition() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body = vec![assign_inc("i")];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn rejects_nested_loop() {
        let condition = cond_lt_var("i", "n");
        let nested = ASTNode::Loop {
            condition: Box::new(cond_lt_var("j", "m")),
            body: vec![assign_inc("j")],
            span: Span::unknown(),
        };
        let body = vec![nested, assign_inc("i")];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn rejects_return_in_body() {
        let condition = cond_lt_var("i", "n");
        let body = vec![
            assign_inc("i"),
            ASTNode::Return {
                value: Some(Box::new(v("i"))),
                span: Span::unknown(),
            },
        ];

        assert!(try_extract_loop_cond_no_exit_facts(&condition, &body)
            .expect("freeze")
            .is_none());
    }

    #[test]
    fn pipeline_pins_no_exit_body_kind() {
        // End-to-end through try_build_loop_facts_inner: the disjoint
        // sibling path must surface NoExitBody on the shared facts field.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_inc("i")];

        let facts = super::super::loop_builder::try_build_loop_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");
        let loop_cond = facts.loop_cond_break_continue.expect("loop_cond facts");
        assert!(matches!(
            loop_cond.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
    }

    #[test]
    fn pipeline_defers_variable_accum_family() {
        // Disjointness pin: the accumulator-over-induction recurrence is
        // owned by the callable-seam VAR lane; the canonical facts must not
        // mint a LoopCondBreakContinue fact for it.
        let condition = cond_lt_var("i", "n");
        let body = vec![assign_accum("acc", "i"), assign_inc("i")];

        let facts = super::super::loop_builder::try_build_loop_facts(&condition, &body)
            .expect("freeze");
        if let Some(facts) = &facts {
            assert!(facts.loop_cond_break_continue().is_none());
        }
    }

    #[test]
    fn pipeline_keeps_exit_bearing_body_on_sibling() {
        // Disjointness pin: an exit-bearing body is still claimed by
        // loop_cond_break_continue, never by the no-exit extractor.
        let condition = cond_lt_var("i", "n");
        let body = vec![
            ASTNode::If {
                condition: Box::new(cond_lt_var("i", "n")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
            assign_inc("i"),
        ];

        let facts = super::super::loop_builder::try_build_loop_facts(&condition, &body)
            .expect("freeze")
            .expect("facts");
        let loop_cond = facts.loop_cond_break_continue.expect("loop_cond facts");
        assert!(!matches!(
            loop_cond.accept_kind,
            LoopCondBreakAcceptKind::NoExitBody
        ));
    }
}

