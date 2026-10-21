//! Checked arithmetic laws shared by expression purposes.
use super::*;
use crate::mir::dynamic_operator_contract::{issue_dynamic_operator_execution_envelope_v1,
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorNormalResultV1, DynamicOperatorValueClassV1};
use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
    expression_projection::ExpressionLeafClassV1 as Leaf};
use std::collections::BTreeSet;
pub(super) fn qualify_arithmetic_operations_v1(
    source: &Rc<ExpressionSourcePlanV1>, order: &[usize], classes: &[Leaf], transferred: &BTreeSet<usize>,
) -> Option<(Vec<Rc<CheckedExpressionOperationLoanV1>>, Vec<usize>)> {
    let mut operations = Vec::new();
    let mut owned_results = Vec::new();
    let class_domain = |class| match class {
        Class::I64 => Some(DynamicOperatorValueClassV1::NormalInteger),
        Class::String => Some(DynamicOperatorValueClassV1::String),
        _ => None,
    };
    for &index in order {
        if transferred.contains(&index) { continue; }
        let Some(binary) = source.nodes()[index].binary() else { continue };
        let operand = |site| source.nodes().iter().position(|node| node.site() == site)
            .and_then(|index| classes[index].value_classes());
        let (Some(lhs), Some(rhs)) = (operand(binary.lhs()), operand(binary.rhs())) else { return None };
        if binary.operator() == ResolvedBinaryOperatorV1::Subtract {
            if index != 0 || classes.first() != Some(&Leaf::Value(Class::I64))
                || [binary.lhs(), binary.rhs()].iter().any(|site| source.nodes().iter().position(|node| node.site() == *site)
                    .is_none_or(|index| classes[index] != Leaf::Value(Class::I64))) { return None; }
            let scalar = ExpressionScalarOperationLoanV1::issue_root(Rc::clone(source), binary, None)?;
            operations.push(Rc::new(CheckedExpressionOperationLoanV1 { plan: Rc::clone(source), node: index,
                proof: CheckedExpressionOperationProofV1::Scalar(scalar) }));
            continue;
        }
        let family = match binary.operator() {
            ResolvedBinaryOperatorV1::Add => DynamicOperatorFamilyV1::Add,
            ResolvedBinaryOperatorV1::Multiply => DynamicOperatorFamilyV1::Mul,
            _ => return None,
        };
        let mut envelopes = Vec::new();
        for lhs in lhs.iter() {
            for rhs in rhs.iter() {
                let envelope = issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
                    family, class_domain(lhs)?, class_domain(rhs)?)).ok()?;
                if envelopes.iter().any(|previous: &&crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1|
                    previous.normal_result() != envelope.normal_result()) { return None; }
                envelopes.push(envelope);
            }
        }
        let result = envelopes.first()?.normal_result();
        if result == DynamicOperatorNormalResultV1::OwnedStringValue { owned_results.push(index); }
        operations.push(Rc::new(CheckedExpressionOperationLoanV1 { plan: Rc::clone(source), node: index,
            proof: CheckedExpressionOperationProofV1::Dynamic(envelopes.into_boxed_slice()) }));
    }
    Some((operations, owned_results))
}
