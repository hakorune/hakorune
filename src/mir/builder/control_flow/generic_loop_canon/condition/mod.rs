//! Generic-loop condition canon owner.
//!
//! This module flattens the old deep condition-canon path while keeping
//! candidate collection and bound extraction separate.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::expr_bool::is_supported_bool_expr_with_canon;
use crate::mir::builder::control_flow::generic_loop_canon::ConditionCanon;
use crate::mir::policies::{CondParam, CondProfile, CondSkeleton};

pub(in crate::mir::builder) mod bound;
pub(in crate::mir::builder) mod candidates;

pub(crate) fn canon_condition_for_generic_loop_v0(
    condition: &ASTNode,
    allow_extended: bool,
) -> Option<ConditionCanon> {
    let candidates = if allow_extended {
        let mut candidates = Vec::new();
        if !candidates::collect_candidates_from_condition(condition, &mut candidates) {
            return None;
        }
        candidates
    } else {
        candidates::collect_candidates_from_top_level_comparison(condition)?
    };
    if candidates.is_empty() && !allow_extended {
        return None;
    }
    if candidates.is_empty() && !is_supported_bool_expr_with_canon(condition, true) {
        return None;
    }
    let cond_profile = build_cond_profile(condition, &candidates);
    Some(ConditionCanon {
        loop_var_candidates: candidates,
        cond_profile,
    })
}

fn build_cond_profile(condition: &ASTNode, candidates: &[String]) -> CondProfile {
    let mut params = Vec::new();
    for candidate in candidates {
        params.push(CondParam::LoopVar(candidate.clone()));
    }
    if let Some(bound) = bound::extract_bound_from_condition(condition, candidates) {
        params.push(CondParam::Bound(bound));
    }
    CondProfile::new(CondSkeleton::LoopCond, params)
}

#[cfg(test)]
mod tests {
    use super::canon_condition_for_generic_loop_v0;
    use crate::ast::{ASTNode, BinaryOperator, Span, UnaryOperator};

    fn span() -> Span {
        Span::unknown()
    }

    fn me_call(method: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(ASTNode::Me { span: span() }),
            method: method.to_string(),
            arguments: Vec::new(),
            span: span(),
        }
    }

    #[test]
    fn extended_supported_bool_condition_may_have_no_loop_var_candidates() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(me_call("is_eof")),
                span: span(),
            }),
            right: Box::new(me_call("is_ready")),
            span: span(),
        };

        let canon = canon_condition_for_generic_loop_v0(&condition, true)
            .expect("extended supported bool condition should build canon");
        assert!(canon.loop_var_candidates.is_empty());
    }

    #[test]
    fn non_extended_condition_still_requires_loop_var_candidates() {
        let condition = me_call("is_ready");
        assert!(canon_condition_for_generic_loop_v0(&condition, false).is_none());
    }
}
