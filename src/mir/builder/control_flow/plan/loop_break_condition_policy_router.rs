//! Loop-break condition routing (policy → break_cond + allow-list + flags)
//!
//! This box exists to keep the loop-break lowering orchestrator focused on wiring.
//! It applies loop-break-specific policies to derive:
//! - a normalized `break when true` condition node
//! - an allow-list of body-local variables permitted in conditions (Phase 92 P3)
//! - flags that affect later lowering (e.g. schedule and promotion heuristics)
//!
//! Fail-Fast: Policy rejections are returned as an error string and must not
//! silently fall back to an unrelated route.

use crate::ast::ASTNode;
use crate::ast::LiteralValue;
use crate::ast::UnaryOperator;
use crate::mir::loop_route_detection::break_condition_analyzer::BreakConditionAnalyzer;

use crate::mir::builder::control_flow::cleanup::policies::loop_true_read_digits_policy;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;

#[derive(Debug, Clone)]
pub(crate) struct LoopBreakConditionRouting {
    pub break_condition_node: ASTNode,
    pub allowed_body_locals_for_conditions: Vec<String>,
    pub is_loop_true_read_digits: bool,
}

pub(crate) struct LoopBreakConditionPolicyRouterBox;

impl LoopBreakConditionPolicyRouterBox {
    fn negate_condition(condition: &ASTNode) -> ASTNode {
        match condition {
            ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } => operand.as_ref().clone(),
            other => ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(other.clone()),
                span: other.span(),
            },
        }
    }

    pub(crate) fn route(
        condition: &ASTNode,
        body: &[ASTNode],
    ) -> Result<LoopBreakConditionRouting, String> {
        // loop(true) read-digits family:
        // - multiple breaks exist; normalize as:
        //   `break_when_true := (ch == "") || !(is_digit(ch))`
        match loop_true_read_digits_policy::classify_loop_true_read_digits(condition, body) {
            PolicyDecision::Use(result) => Ok(LoopBreakConditionRouting {
                break_condition_node: result.break_condition_node,
                allowed_body_locals_for_conditions: vec![result.allowed_ch_var],
                is_loop_true_read_digits: true,
            }),
            PolicyDecision::Reject(reason) => Err(format!("[cf_loop/loop_break] {}", reason)),
            PolicyDecision::None => Ok(LoopBreakConditionRouting {
                // Phase 260 P0.1: If the loop has an explicit header condition and the body
                // does not contain a top-level break-guard shape, the exit condition is
                // structurally derived as `!(loop_condition)`.
                break_condition_node: BreakConditionAnalyzer::extract_break_condition_node(body)
                    .or_else(|_| {
                        if matches!(
                            condition,
                            ASTNode::Literal {
                                value: LiteralValue::Bool(true),
                                ..
                            }
                        ) {
                            Err(
                                "[cf_loop/loop_break] loop(true) requires a break-guard shape"
                                    .to_string(),
                            )
                        } else {
                            Ok(Self::negate_condition(condition))
                        }
                    })
                    .map_err(|_| {
                        "[cf_loop/loop_break] Failed to extract break condition from loop body"
                            .to_string()
                    })?,
                allowed_body_locals_for_conditions: Vec::new(),
                is_loop_true_read_digits: false,
            }),
        }
    }
}
