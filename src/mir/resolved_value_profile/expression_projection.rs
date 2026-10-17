//! Source-class projection of ONE retained original plan. Every structural
//! child remains an obligation; a known parent class cannot erase an unknown
//! argument, receiver or opaque prelude. This is not an execution proof.
use super::expression_children::{binary_children_v1, ExpressionChildrenV1};
use super::expression_class::{binary_expression_class_v1, ExpressionValueClassV1};
use super::expression_plan::{ExpressionNodeSourceV1, ExpressionSourcePlanV1};
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionLeafClassV1 {
    Value(ExpressionValueClassV1),
    /// Source identity/qualification, never an Integer operand.
    Identity,
    Unresolved,
}

pub(crate) fn project_expression_class_v1(
    plan: &ExpressionSourcePlanV1,
    leaf: impl FnMut(usize) -> ExpressionLeafClassV1,
) -> Option<ExpressionValueClassV1> {
    match project_expression_nodes_v1(plan, &std::collections::BTreeSet::new(), leaf)?.first()? {
        ExpressionLeafClassV1::Value(value) => Some(*value),
        _ => None,
    }
}

/// A transferred subtree is qualified by the caller's retained whole-input
/// loan. This projection grants no such loan: its consumer must prove exact
/// original call/argument membership. Opaque/missing children still refuse.
pub(crate) fn project_expression_nodes_v1(
    plan: &ExpressionSourcePlanV1,
    transferred: &std::collections::BTreeSet<usize>,
    mut leaf: impl FnMut(usize) -> ExpressionLeafClassV1,
) -> Option<Vec<ExpressionLeafClassV1>> {
    use ExpressionLeafClassV1 as Class;
    if transferred.iter().any(|index| *index >= plan.nodes().len()) { return None; }
    let mut nodes = vec![Class::Unresolved; plan.nodes().len()];
    let mut values = BTreeMap::new();
    for index in plan.ordered_postorder_v1().ok()? {
        let node = &plan.nodes()[index];
        let ExpressionChildrenV1::Known(children) = node.children() else {
            return None;
        };
        for child in children {
            if values
                .get(&child.site)
                .is_none_or(|value| *value == Class::Unresolved)
            {
                return None;
            }
        }
        let value = if transferred.contains(&index) { Class::Identity } else { match node.source() {
            ExpressionNodeSourceV1::Literal(literal) => Class::Value(match literal {
                ResolvedLiteralSourceV1::Integer(_)
                | ResolvedLiteralSourceV1::TypedInteger { .. } => ExpressionValueClassV1::I64,
                ResolvedLiteralSourceV1::String(_) => ExpressionValueClassV1::String,
                ResolvedLiteralSourceV1::Bool(_) => ExpressionValueClassV1::Bool,
                ResolvedLiteralSourceV1::Null => ExpressionValueClassV1::Null,
                ResolvedLiteralSourceV1::Float | ResolvedLiteralSourceV1::Void => return None,
            }),
            ExpressionNodeSourceV1::Binary(binary) => {
                let [lhs, rhs] = binary_children_v1(node.site(), binary)?;
                let (Some(Class::Value(lhs)), Some(Class::Value(rhs))) =
                    (values.get(lhs), values.get(rhs))
                else {
                    return None;
                };
                Class::Value(binary_expression_class_v1(binary.operator(), *lhs, *rhs).ok()?)
            }
            // Their source operation/tail obligations are not closed by merely
            // lending a class to an enclosing operator.
            ExpressionNodeSourceV1::Unary | ExpressionNodeSourceV1::Conditional => return None,
            _ => leaf(index),
        } };
        nodes[index] = value;
        if value == Class::Unresolved || values.insert(node.site().clone(), value).is_some() {
            return None;
        }
    }
    Some(nodes)
}
