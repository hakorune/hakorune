//! Existing checked Add envelope construction shared by expression purposes.
use super::*;
use crate::mir::dynamic_operator_contract::{issue_dynamic_operator_execution_envelope_v1,
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorNormalResultV1, DynamicOperatorValueClassV1};
use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
    expression_projection::ExpressionLeafClassV1 as Leaf};
use std::collections::BTreeSet;
pub(super) fn qualify_add_operations_v1(
    source: &Rc<ExpressionSourcePlanV1>, order: &[usize], classes: &[Leaf], transferred: &BTreeSet<usize>,
) -> Option<(Vec<Rc<CheckedExpressionOperationLoanV1>>, Vec<usize>)> {
    let mut operations = Vec::new();
    let mut owned_results = Vec::new();
    let class_domain = |class| match class {
        Leaf::Value(Class::I64) => Some(DynamicOperatorValueClassV1::NormalInteger),
        Leaf::Value(Class::String) => Some(DynamicOperatorValueClassV1::String),
        _ => None,
    };
    for &index in order {
        if transferred.contains(&index) { continue; }
        let Some(binary) = source.nodes()[index].binary() else { continue };
        let family = match binary.operator() {
            ResolvedBinaryOperatorV1::Add => DynamicOperatorFamilyV1::Add,
            _ => return None,
        };
        let operand = |site| source.nodes().iter().position(|node| node.site() == site)
            .and_then(|index| class_domain(classes[index]));
        let (Some(lhs), Some(rhs)) = (operand(binary.lhs()), operand(binary.rhs())) else { return None };
        let Ok(envelope) = issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(family, lhs, rhs))
            else { return None };
        if envelope.normal_result() == DynamicOperatorNormalResultV1::OwnedStringValue { owned_results.push(index); }
        operations.push(Rc::new(CheckedExpressionOperationLoanV1 { plan: Rc::clone(&source), node: index, envelope }));
    }
    Some((operations, owned_results))
}
