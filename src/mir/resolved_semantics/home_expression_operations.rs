//! Checked arithmetic laws shared by expression purposes.
use super::*;
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1, DynamicOperatorFamilyV1,
    DynamicOperatorNormalResultV1, DynamicOperatorValueClassV1,
};
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1 as Class,
    expression_projection::ExpressionLeafClassV1 as Leaf,
};
use std::collections::BTreeSet;
pub(super) fn qualify_arithmetic_operations_v1(
    source: &Rc<ExpressionSourcePlanV1>,
    order: &[usize],
    classes: &[Leaf],
    transferred: &BTreeSet<usize>,
) -> Option<(Vec<Rc<CheckedExpressionOperationLoanV1>>, Vec<usize>)> {
    let mut operations = Vec::new();
    let mut owned_results = Vec::new();
    let class_domain = |class| match class {
        Class::I64 => Some(DynamicOperatorValueClassV1::NormalInteger),
        Class::String => Some(DynamicOperatorValueClassV1::String),
        Class::Dynamic => Some(DynamicOperatorValueClassV1::Dynamic),
        _ => None,
    };
    let tagged_domain = |class| match class {
        Class::I64 => Some(DynamicOperatorValueClassV1::I64),
        Class::String => Some(DynamicOperatorValueClassV1::String),
        Class::Dynamic => Some(DynamicOperatorValueClassV1::Dynamic),
        _ => None,
    };
    for &index in order {
        if transferred.contains(&index) {
            continue;
        }
        let Some(binary) = source.nodes()[index].binary() else {
            continue;
        };
        let operand = |site| {
            source
                .nodes()
                .iter()
                .position(|node| node.site() == site)
                .and_then(|index| classes[index].value_classes())
        };
        let (Some(lhs), Some(rhs)) = (operand(binary.lhs()), operand(binary.rhs())) else {
            return None;
        };
        let source_dynamic_sub = binary.operator() == ResolvedBinaryOperatorV1::Subtract
            && index == 0
            && classes.first() == Some(&Leaf::Value(Class::Dynamic))
            && lhs.iter().all(|class| class == Class::Dynamic)
            && rhs.iter().all(|class| class == Class::I64);
        if matches!(
            binary.operator(),
            ResolvedBinaryOperatorV1::Subtract
                | ResolvedBinaryOperatorV1::Divide
                | ResolvedBinaryOperatorV1::Modulo
        ) && !source_dynamic_sub {
            if index != 0
                || classes.first() != Some(&Leaf::Value(Class::I64))
                || [binary.lhs(), binary.rhs()].iter().any(|site| {
                    source
                        .nodes()
                        .iter()
                        .position(|node| node.site() == *site)
                        .is_none_or(|index| classes[index] != Leaf::Value(Class::I64))
                })
            {
                return None;
            }
            let divisor = if matches!(
                binary.operator(),
                ResolvedBinaryOperatorV1::Divide | ResolvedBinaryOperatorV1::Modulo
            ) {
                source.nodes().iter().find_map(|node| {
                    (node.site() == binary.rhs())
                        .then(|| match node.source() {
                            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Integer(
                                value,
                            )) => Some(*value),
                            _ => None,
                        })
                        .flatten()
                })
            } else {
                None
            };
            let scalar =
                ExpressionScalarOperationLoanV1::issue_root(Rc::clone(source), binary, divisor)?;
            operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
                plan: Rc::clone(source),
                node: index,
                proof: CheckedExpressionOperationProofV1::Scalar(scalar),
            }));
            continue;
        }
        let family = match binary.operator() {
            ResolvedBinaryOperatorV1::Add => DynamicOperatorFamilyV1::Add,
            ResolvedBinaryOperatorV1::Multiply => DynamicOperatorFamilyV1::Mul,
            ResolvedBinaryOperatorV1::Subtract if source_dynamic_sub => DynamicOperatorFamilyV1::Sub,
            _ => return None,
        };
        let mut envelopes = Vec::new();
        for lhs in lhs.iter() {
            for rhs in rhs.iter() {
                let dynamic = lhs == Class::Dynamic || rhs == Class::Dynamic;
                let envelope = issue_dynamic_operator_execution_envelope_v1(
                    DynamicOperatorDomainV1::new(
                        family,
                        if dynamic { tagged_domain(lhs)? } else { class_domain(lhs)? },
                        if dynamic { tagged_domain(rhs)? } else { class_domain(rhs)? },
                    ),
                )
                .ok()?;
                if envelopes.iter().any(|previous: &&crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1|
                    previous.normal_result() != envelope.normal_result()) { return None; }
                envelopes.push(envelope);
            }
        }
        let result = envelopes.first()?.normal_result();
        if result == DynamicOperatorNormalResultV1::OwnedStringValue
            || family == DynamicOperatorFamilyV1::Sub
        {
            owned_results.push(index);
        }
        operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
            plan: Rc::clone(source),
            node: index,
            proof: CheckedExpressionOperationProofV1::Dynamic(envelopes.into_boxed_slice()),
        }));
    }
    Some((operations, owned_results))
}
