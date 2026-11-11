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
/// Copy authority comes from the exact source class, never the wire carrier.
pub(super) fn qualify_copy_operation_v1(
    source: &Rc<ExpressionSourcePlanV1>, index: usize, classes: &[Leaf],
) -> Option<Rc<CheckedExpressionOperationLoanV1>> {
    let operand = source.copy_operand_node_v1(index)?;
    let kind = match classes.get(operand)? {
        Leaf::Value(Class::I64) => super::super::SourceScalarKind::Integer,
        Leaf::Value(Class::Bool) => super::super::SourceScalarKind::Bool,
        _ => return None,
    };
    Some(Rc::new(CheckedExpressionOperationLoanV1 {
        plan: Rc::clone(source), node: index, proof: CheckedExpressionOperationProofV1::ScalarCopy(kind),
    }))
}

/// One comparison of exact Text and/or opaque Dynamic operands, of two exact
/// Bools, or of two exact i64 values, producing a trivial Bool. The shared
/// operator contract owns the law.
pub(super) fn qualify_comparison_operation_v1(
    source: &Rc<ExpressionSourcePlanV1>,
    index: usize,
    classes: &[Leaf],
) -> Option<Rc<CheckedExpressionOperationLoanV1>> {
    let binary = source.nodes()[index].binary()?;
    let family = crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(binary.operator())?;
    let class = |site| {
        let position = source.nodes().iter().position(|node| node.site() == site)?;
        match classes[position] {
            Leaf::Value(Class::I64) => Some(DynamicOperatorValueClassV1::I64),
            Leaf::Value(Class::String) => Some(DynamicOperatorValueClassV1::String),
            Leaf::Value(Class::Dynamic) => Some(DynamicOperatorValueClassV1::Dynamic),
            Leaf::Value(Class::Bool) => Some(DynamicOperatorValueClassV1::Bool),
            _ => None,
        }
    };
    let (lhs, rhs) = (class(binary.lhs())?, class(binary.rhs())?);
    if lhs == DynamicOperatorValueClassV1::Dynamic && rhs == DynamicOperatorValueClassV1::Dynamic {
        return None;
    }
    let envelope = issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(family, lhs, rhs)).ok()?;
    if envelope.normal_result() != DynamicOperatorNormalResultV1::TrivialBool {
        return None;
    }
    Some(Rc::new(CheckedExpressionOperationLoanV1 {
        plan: Rc::clone(source),
        node: index,
        proof: CheckedExpressionOperationProofV1::Dynamic(vec![envelope].into_boxed_slice()),
    }))
}

pub(super) fn qualify_arithmetic_operations_v1(
    source: &Rc<ExpressionSourcePlanV1>,
    order: &[usize],
    classes: &[Leaf],
    transferred: &BTreeSet<usize>,
    references: &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
) -> Option<(Vec<Rc<CheckedExpressionOperationLoanV1>>, Vec<usize>)> {
    let mut operations = Vec::new();
    let mut owned_results = Vec::new();
    let class_domain = |class| match class {
        Class::I64 => Some(DynamicOperatorValueClassV1::NormalInteger),
        Class::Bool => Some(DynamicOperatorValueClassV1::Bool),
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
        if source.copy_operand_node_v1(index).is_some() {
            operations.push(qualify_copy_operation_v1(source, index, classes)?);
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
            if classes.first() != Some(&Leaf::Value(Class::I64))
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
            let divisor_read = source.nodes().iter().position(|node| node.site() == binary.rhs())
                .and_then(|node| references.get(&node))
                .and_then(|reference| match reference {
                    CheckedExpressionReferenceV1::Scalar(read) => read.original_read(),
                    _ => None,
                })
                .filter(|read| read.exact_integer_value_v1().is_some())
                .cloned();
            let divisor = if matches!(
                binary.operator(),
                ResolvedBinaryOperatorV1::Divide | ResolvedBinaryOperatorV1::Modulo
            ) {
                match divisor_read.as_ref() {
                    Some(read) => read.exact_integer_value_v1(),
                    None => source.nodes().iter().find_map(|node| {
                        (node.site() == binary.rhs()).then(|| match node.source() {
                            ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Integer(value)) => Some(*value),
                            _ => None,
                        }).flatten()
                    }),
                }
            } else { None };
            let scalar = ExpressionScalarOperationLoanV1::issue_node_v1(
                Rc::clone(source),
                index,
                binary,
                divisor,
                divisor.and(divisor_read),
            )?;
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

/// A plain field read classified String materializes an owned copy: it is one more acquisition of the SAME ordered evaluation.
pub(super) fn owned_text_field_nodes_v1<'a>(
    source: &'a crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    order: &'a [usize],
    classes: &'a [Leaf],
    transferred: &'a BTreeSet<usize>,
) -> impl Iterator<Item = usize> + 'a {
    order.iter().copied().filter(move |node| !transferred.contains(node)
        && classes[*node] == Leaf::Value(Class::String)
        && matches!(source.nodes()[*node].source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })))
}
