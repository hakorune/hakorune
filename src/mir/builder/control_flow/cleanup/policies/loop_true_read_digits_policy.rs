//! loop(true) + break-only digits (read_digits_from family) policy
//!
//! Goal: keep loop_break core lowering structural
//! by moving shape recognition + routing
//! for loop(true) read-digits family into a dedicated policy box.
//!
//! This policy is intentionally narrow:
//! - It only triggers when the body matches the read-digits(loop(true)) detector.
//! - It returns a normalized "break when true" condition AST:
//!     `break_when_true := (ch == "") || !(digit_cond)`
//! - It provides the single allowed body-local variable name (`ch`) for condition lowering.

use super::PolicyDecision;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};

#[derive(Debug, Clone)]
pub(crate) struct LoopTrueReadDigitsPolicyResult {
    pub break_condition_node: ASTNode,
    pub allowed_ch_var: String,
}

pub(crate) fn classify_loop_true_read_digits(
    condition: &ASTNode,
    body: &[ASTNode],
) -> PolicyDecision<LoopTrueReadDigitsPolicyResult> {
    use crate::mir::builder::control_flow::cleanup::policies::read_digits_break_condition_box::ReadDigitsBreakConditionBox;
    use crate::mir::builder::control_flow::facts::route_shape_recognizers::parse_number::detect_read_digits_loop_true_shape;

    if !matches!(
        condition,
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) {
        return PolicyDecision::None;
    }
    if detect_read_digits_loop_true_shape(body).is_none() {
        return PolicyDecision::None;
    }

    let (ch_var, eos_cond, digit_cond) =
        match ReadDigitsBreakConditionBox::extract_eos_and_digit_condition(body) {
            Ok(v) => v,
            Err(e) => return PolicyDecision::Reject(e),
        };

    let break_on_not_digit = ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand: Box::new(digit_cond),
        span: Span::unknown(),
    };

    let break_when_true = ASTNode::BinaryOp {
        operator: BinaryOperator::Or,
        left: Box::new(eos_cond),
        right: Box::new(break_on_not_digit),
        span: Span::unknown(),
    };

    PolicyDecision::Use(LoopTrueReadDigitsPolicyResult {
        break_condition_node: break_when_true,
        allowed_ch_var: ch_var,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span, UnaryOperator};

    fn span() -> Span {
        Span::unknown()
    }

    fn bool_lit(v: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(v),
            span: span(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn str_lit(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: span(),
        }
    }

    fn int_lit(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: span(),
        }
    }

    fn eq(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn add(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn or(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn not(node: ASTNode) -> ASTNode {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(node),
            span: span(),
        }
    }

    fn digit_chain(var_name: &str, digits: &[&str]) -> ASTNode {
        let mut it = digits.iter();
        let first = it.next().expect("digits must be non-empty");
        let mut acc = eq(var(var_name), str_lit(first));
        for d in it {
            acc = or(acc, eq(var(var_name), str_lit(d)));
        }
        acc
    }

    #[test]
    fn read_digits_loop_true_policy_returns_break_when_true_and_allowlist() {
        // loop(true) {
        //   if ch == "" { break }
        //   if is_digit(ch) { out = out + ch; i = i + 1 } else { break }
        // }
        let condition = bool_lit(true);

        let eos_cond = eq(var("ch"), str_lit(""));
        let digit_cond = digit_chain("ch", &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);

        let body = vec![
            ASTNode::If {
                condition: Box::new(eos_cond.clone()),
                then_body: vec![ASTNode::Break { span: span() }],
                else_body: None,
                span: span(),
            },
            ASTNode::If {
                condition: Box::new(digit_cond.clone()),
                then_body: vec![
                    ASTNode::Assignment {
                        target: Box::new(var("out")),
                        value: Box::new(add(var("out"), var("ch"))),
                        span: span(),
                    },
                    ASTNode::Assignment {
                        target: Box::new(var("i")),
                        value: Box::new(add(var("i"), int_lit(1))),
                        span: span(),
                    },
                ],
                else_body: Some(vec![ASTNode::Break { span: span() }]),
                span: span(),
            },
        ];

        let decision = classify_loop_true_read_digits(&condition, &body);
        let result = match decision {
            PolicyDecision::Use(v) => v,
            other => panic!("expected PolicyDecision::Use, got {:?}", other),
        };

        assert_eq!(result.allowed_ch_var, "ch");
        assert_eq!(result.break_condition_node, or(eos_cond, not(digit_cond)));
    }
}
