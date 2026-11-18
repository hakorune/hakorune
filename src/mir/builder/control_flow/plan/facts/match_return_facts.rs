//! Legacy match-return diagnostics for strict JoinIR.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(in crate::mir::builder) fn check_match_return_source(
    expr: &ASTNode,
    strict: bool,
) -> Result<(), Freeze> {
    let ASTNode::MatchExpr {
        scrutinee,
        arms,
        else_expr,
        ..
    } = expr
    else {
        return Ok(());
    };

    if !matches!(scrutinee.as_ref(), ASTNode::Variable { .. } | ASTNode::Literal {
        value: LiteralValue::Integer(_), ..
    }) {
        log_reject(
            "match_return_facts",
            RejectReason::MatchReturnScrutineeNotSupported,
            handoff_tables::for_match_return_facts,
        );
        return reject_if_strict(
            strict,
            RejectReason::MatchReturnScrutineeNotSupported.as_freeze_message(),
        );
    }

    if arms.len() < 2 {
        log_reject(
            "match_return_facts",
            RejectReason::MatchReturnTooFewArms,
            handoff_tables::for_match_return_facts,
        );
        return reject_if_strict(
            strict,
            RejectReason::MatchReturnTooFewArms.as_freeze_message(),
        );
    }

    if !matches!(else_expr.as_ref(), ASTNode::Literal { .. }) {
        log_reject(
            "match_return_facts",
            RejectReason::MatchReturnElseNotLiteral,
            handoff_tables::for_match_return_facts,
        );
        return reject_if_strict(strict, RejectReason::MatchReturnElseNotLiteral.as_freeze_message());
    }

    if arms.iter().any(|(_, expr)| !matches!(expr, ASTNode::Literal { .. })) {
        log_reject(
            "match_return_facts",
            RejectReason::MatchReturnArmNotLiteral,
            handoff_tables::for_match_return_facts,
        );
        return reject_if_strict(strict, RejectReason::MatchReturnArmNotLiteral.as_freeze_message());
    }

    Ok(())
}

fn reject_if_strict(strict: bool, message: &str) -> Result<(), Freeze> {
    if strict {
        Err(Freeze::unsupported(message))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::check_match_return_source;
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn lit_int(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }
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
            ), (LiteralValue::Integer(2), lit_int(2))],
            else_expr: Box::new(lit_int(0)),
            span: Span::unknown(),
        };

        let err = check_match_return_source(&match_expr, true).unwrap_err();
        assert!(err.to_string().contains("match return arm must be literal"));
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

        check_match_return_source(&match_expr, false).expect("Ok");
    }
}
