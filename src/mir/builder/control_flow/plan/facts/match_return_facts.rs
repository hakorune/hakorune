//! Phase 29at P3: MatchReturnFacts (Return-only match subset)

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum MatchReturnScrutinee {
    Var(String),
    Int(i64),
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct MatchReturnArmFacts {
    pub label: LiteralValue,
    pub return_value: LiteralValue,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct MatchReturnFacts {
    pub scrutinee: MatchReturnScrutinee,
    pub arms: Vec<MatchReturnArmFacts>,
    pub else_value: LiteralValue,
}

pub(in crate::mir::builder) fn try_extract_match_return_facts(
    expr: &ASTNode,
    strict: bool,
) -> Result<Option<MatchReturnFacts>, Freeze> {
    let ASTNode::MatchExpr {
        scrutinee,
        arms,
        else_expr,
        ..
    } = expr
    else {
        return Ok(None);
    };

    let scrutinee = match scrutinee.as_ref() {
        ASTNode::Variable { name, .. } => MatchReturnScrutinee::Var(name.clone()),
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            ..
        } => MatchReturnScrutinee::Int(*n),
        _ => {
            log_reject(
                "match_return_facts",
                RejectReason::MatchReturnScrutineeNotSupported,
                handoff_tables::for_match_return_facts,
            );
            return reject_or_none(strict, RejectReason::MatchReturnScrutineeNotSupported.as_freeze_message());
        }
    };

    if arms.len() < 2 {
        log_reject(
            "match_return_facts",
            RejectReason::MatchReturnTooFewArms,
            handoff_tables::for_match_return_facts,
        );
        return reject_or_none(strict, RejectReason::MatchReturnTooFewArms.as_freeze_message());
    }

    let else_value = match literal_from_expr(else_expr.as_ref()) {
        Some(value) if is_allowed_return_literal(&value) => value,
        _ => {
            log_reject(
                "match_return_facts",
                RejectReason::MatchReturnElseNotLiteral,
                handoff_tables::for_match_return_facts,
            );
            return reject_or_none(strict, RejectReason::MatchReturnElseNotLiteral.as_freeze_message());
        }
    };

    let mut arm_facts = Vec::with_capacity(arms.len());
    for (label, arm_expr) in arms {
        if !is_allowed_label_literal(label) {
            log_reject(
                "match_return_facts",
                RejectReason::MatchReturnArmLabelNotSupported,
                handoff_tables::for_match_return_facts,
            );
            return reject_or_none(strict, RejectReason::MatchReturnArmLabelNotSupported.as_freeze_message());
        }
        let Some(return_value) = literal_from_expr(arm_expr) else {
            log_reject(
                "match_return_facts",
                RejectReason::MatchReturnArmNotLiteral,
                handoff_tables::for_match_return_facts,
            );
            return reject_or_none(strict, RejectReason::MatchReturnArmNotLiteral.as_freeze_message());
        };
        if !is_allowed_return_literal(&return_value) {
            log_reject(
                "match_return_facts",
                RejectReason::MatchReturnArmLiteralTypeUnsupported,
                handoff_tables::for_match_return_facts,
            );
            return reject_or_none(strict, RejectReason::MatchReturnArmLiteralTypeUnsupported.as_freeze_message());
        }
        arm_facts.push(MatchReturnArmFacts {
            label: label.clone(),
            return_value,
        });
    }

    Ok(Some(MatchReturnFacts {
        scrutinee,
        arms: arm_facts,
        else_value,
    }))
}

fn literal_from_expr(expr: &ASTNode) -> Option<LiteralValue> {
    if let ASTNode::Literal { value, .. } = expr {
        return Some(value.clone());
    }
    None
}

fn is_allowed_label_literal(value: &LiteralValue) -> bool {
    matches!(value, LiteralValue::Integer(_) | LiteralValue::Bool(_))
}

fn is_allowed_return_literal(value: &LiteralValue) -> bool {
    matches!(value, LiteralValue::Integer(_) | LiteralValue::Bool(_))
}

fn reject_or_none(
    strict: bool,
    message: &str,
) -> Result<Option<MatchReturnFacts>, Freeze> {
    if strict {
        Err(Freeze::unsupported(message))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{try_extract_match_return_facts, MatchReturnScrutinee};
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn lit_int(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }
    }

    #[test]
    fn match_return_facts_accepts_min_subset() {
        let match_expr = ASTNode::MatchExpr {
            scrutinee: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            arms: vec![
                (LiteralValue::Integer(1), lit_int(10)),
                (LiteralValue::Integer(2), lit_int(20)),
            ],
            else_expr: Box::new(lit_int(30)),
            span: Span::unknown(),
        };

        let facts = try_extract_match_return_facts(&match_expr, true)
            .expect("Ok")
            .expect("Some");
        assert!(matches!(facts.scrutinee, MatchReturnScrutinee::Var(_)));
        assert_eq!(facts.arms.len(), 2);
    }

    #[test]
    fn match_return_facts_strict_rejects_non_literal_arm() {
        let match_expr = ASTNode::MatchExpr {
            scrutinee: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            arms: vec![(
                LiteralValue::Integer(1),
                ASTNode::Variable {
                    name: "y".to_string(),
                    span: Span::unknown(),
                },
            )],
            else_expr: Box::new(lit_int(0)),
            span: Span::unknown(),
        };

        let err = try_extract_match_return_facts(&match_expr, true).unwrap_err();
        assert!(err.to_string().contains("match return"));
    }

    #[test]
    fn match_return_facts_release_skips_invalid_shape() {
        let match_expr = ASTNode::MatchExpr {
            scrutinee: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            arms: vec![(
                LiteralValue::Integer(1),
                ASTNode::Variable {
                    name: "y".to_string(),
                    span: Span::unknown(),
                },
            )],
            else_expr: Box::new(lit_int(0)),
            span: Span::unknown(),
        };

        let facts = try_extract_match_return_facts(&match_expr, false).expect("Ok");
        assert!(facts.is_none());
    }
}
