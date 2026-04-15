use super::types::{PlannerFirstMode, RouterEnv};
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;

pub(crate) fn emit_planner_first(mode: PlannerFirstMode, env: &RouterEnv, rule: PlanRuleId) {
    let emit = match mode {
        PlannerFirstMode::Never => false,
        PlannerFirstMode::StrictOrDev => env.strict_or_dev,
        PlannerFirstMode::StrictOrDevPlannerRequired => env.strict_or_dev && env.planner_required,
    };
    if emit {
        let msg =
            crate::mir::builder::control_flow::plan::planner::tags::planner_first_tag_with_label(
                rule,
            );
        // Gate sentinel: in strict+planner_required mode, emit stable, prefix-free tags (stderr)
        // so hermetic smokes can validate routing without depending on `NYASH_RING0_LOG_LEVEL`.
        //
        // Outside of the gate, keep the old behavior (debug-only) to avoid surprising users with
        // extra stderr output.
        if crate::config::env::joinir_dev::strict_planner_required_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
        } else if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&msg);
        }
    }
}

pub(crate) fn loop_break_recipe_needs_flowbox_adopt_tag_in_strict(
    facts: &crate::mir::builder::control_flow::facts::LoopBreakFacts,
) -> bool {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

    let loop_condition_is_var_less_literal = match &facts.loop_condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left,
            right,
            ..
        } => {
            matches!(left.as_ref(), ASTNode::Variable { .. })
                && matches!(
                    right.as_ref(),
                    ASTNode::Literal {
                        value: LiteralValue::Integer(_),
                        ..
                    }
                )
        }
        _ => false,
    };

    if loop_condition_is_var_less_literal {
        return true;
    }

    let is_empty_string_literal = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::Literal {
                value: LiteralValue::String(s),
                ..
            } if s.is_empty()
        )
    };

    let is_substring_call = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::MethodCall { method, .. } if method == "substring"
        )
    };

    match &facts.break_condition {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left,
            right,
            ..
        } => {
            (is_substring_call(left.as_ref()) && is_empty_string_literal(right.as_ref()))
                || (is_substring_call(right.as_ref()) && is_empty_string_literal(left.as_ref()))
        }
        _ => false,
    }
}
