//! RHS/self-reference classification for LoopUpdateSummary.

use super::UpdateKind;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Classify one RHS expression after validating it references the assigned carrier.
///
/// This intentionally recognizes only self-referential addition. Other update
/// forms should become explicit analyzer cards before being accepted here.
fn classify_update_kind_from_rhs(var_name: &str, rhs: &ASTNode) -> UpdateKind {
    match rhs {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            if matches!(operator, BinaryOperator::Add) {
                let is_self_reference =
                    matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == var_name);
                if !is_self_reference {
                    return UpdateKind::Other;
                }

                if let ASTNode::Literal { value, .. } = right.as_ref() {
                    if let LiteralValue::Integer(n) = value {
                        if *n == 1 {
                            return UpdateKind::CounterLike;
                        } else {
                            return UpdateKind::AccumulationLike;
                        }
                    }
                } else {
                    return UpdateKind::AccumulationLike;
                }
            }
            UpdateKind::Other
        }
        _ => UpdateKind::Other,
    }
}

/// Name tie-break for the proven `x = x + 1` shape only.
fn is_likely_loop_index(name: &str) -> bool {
    matches!(name, "i" | "j" | "k" | "e" | "idx" | "index" | "pos" | "n")
}

fn disambiguate_update_kind(var_name: &str, kind: UpdateKind) -> UpdateKind {
    match kind {
        UpdateKind::CounterLike if is_likely_loop_index(var_name) => UpdateKind::CounterLike,
        UpdateKind::CounterLike => UpdateKind::AccumulationLike,
        other => other,
    }
}

/// Classify all RHS candidates. Multiple candidates must agree after name tie-break.
pub(super) fn classify_update_kind_from_rhses(
    var_name: &str,
    rhses: &[&ASTNode],
) -> Option<UpdateKind> {
    let mut agreed = None;

    for rhs in rhses {
        let kind = disambiguate_update_kind(var_name, classify_update_kind_from_rhs(var_name, rhs));

        if kind == UpdateKind::Other {
            return Some(UpdateKind::Other);
        }

        match agreed {
            None => agreed = Some(kind),
            Some(previous) if previous == kind => {}
            Some(_) => return Some(UpdateKind::Other),
        }
    }

    agreed
}
