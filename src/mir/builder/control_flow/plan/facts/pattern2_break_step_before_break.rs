use super::pattern2_break_helpers::{extract_break_if_parts, extract_loop_var_for_plan_subset};
use super::pattern2_break_types::Pattern2BreakFacts;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;

/// Phase 29bq: Pattern2Break step-before-break subset (strict/dev + planner-required only)
///
/// Shape:
/// ```text
/// loop(i < N) {
///   i = i + 1
///   if break_cond { break }
///   carrier = carrier + 1
/// }
/// ```
///
/// Rationale:
/// - The step happens before the break check; moving it to a separate step_bb changes semantics.
/// - We keep this subset dev-only to avoid changing release routing/semantics.
pub(super) fn try_extract_pattern2_break_step_before_break_subset(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Option<Pattern2BreakFacts> {
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if !planner_required {
        return None;
    }

    let Some(loop_var) = extract_loop_var_for_plan_subset(condition) else {
        return None;
    };

    let counts = count_control_flow(body, ControlFlowDetector::default());
    if counts.break_count != 1 || counts.continue_count > 0 || counts.return_count > 0 {
        return None;
    }

    if body.len() != 3 {
        return None;
    }

    let loop_increment = match &body[0] {
        ASTNode::Assignment { target, value, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return None;
            };
            if name != &loop_var {
                return None;
            }

            let ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                right,
                ..
            } = value.as_ref()
            else {
                return None;
            };

            if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == &loop_var) {
                return None;
            }
            if !matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(_),
                    ..
                }
            ) {
                return None;
            }

            value.as_ref().clone()
        }
        _ => return None,
    };

    let (break_condition, carrier_update_in_break) =
        extract_break_if_parts(&body[1])?;

    let (carrier_var, carrier_update_in_body) = match &body[2] {
        ASTNode::Assignment { target, value, .. } => {
            let carrier_name = match target.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => return None,
            };
            if carrier_name == loop_var {
                return None;
            }
            (carrier_name, value.as_ref().clone())
        }
        _ => return None,
    };

    Some(Pattern2BreakFacts {
        loop_var,
        carrier_var,
        loop_condition: condition.clone(),
        break_condition,
        carrier_update_in_break,
        carrier_update_in_body,
        loop_increment,
        step_placement: LoopBreakStepPlacement::BeforeBreak,
    })
}
