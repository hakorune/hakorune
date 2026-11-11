//! Checked expression laws shared by value and control destinations.
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

/// One operation walk for all expression destinations. The immutable coverage
/// snapshot can only refuse a source domain; exact loans own every accepted law.
pub(super) fn qualify_expression_operations_v1(
    source: &Rc<ExpressionSourcePlanV1>,
    order: &[usize],
    classes: &[Leaf],
    transferred: &BTreeSet<usize>,
    reference_proofs: &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    fields: &[LocalFieldReadRequestV1],
    coverage: super::leaves::ExpressionLeafProfileV1,
) -> Option<(
    Vec<Rc<CheckedExpressionOperationLoanV1>>,
    Vec<usize>,
    Vec<usize>,
)> {
    use crate::mir::resolved_value_profile::expression_children::{
        ExpressionChildReachV1, ExpressionChildrenV1,
    };
    use ResolvedBinaryOperatorV1 as Op;
    let references = reference_proofs;
    let mut operations = Vec::new();
    let mut owned_results = Vec::new();
    let mut logical = Vec::new();
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
        let node = &source.nodes()[index];
        let ExpressionChildrenV1::Known(children) = node.children() else {
            return None;
        };
        if source.copy_operand_node_v1(index).is_some() {
            operations.push(qualify_copy_operation_v1(source, index, classes)?);
            continue;
        }
        let Some(binary) = node.binary() else {
            if children
                .iter()
                .any(|child| child.reach != ExpressionChildReachV1::Always)
            {
                return None;
            }
            continue;
        };
        let [lhs_site, rhs_site] =
            crate::mir::resolved_value_profile::expression_children::binary_children_v1(
                node.site(),
                binary,
            )?;
        if children.len() != 2
            || children[0].site != *lhs_site
            || children[1].site != *rhs_site
            || children[0].reach != ExpressionChildReachV1::Always
        {
            return None;
        }
        if matches!(binary.operator(), Op::And | Op::Or) {
            if !coverage.covers_short_circuit_v1()
                || children[1].reach
                    != (ExpressionChildReachV1::When {
                        condition: lhs_site.clone(),
                        value: binary.operator() == Op::And,
                    })
                || classes[index] != Leaf::Value(Class::Bool)
            {
                return None;
            }
            logical.push(index);
            continue;
        }
        if children[1].reach != ExpressionChildReachV1::Always {
            return None;
        }
        let child = |site| source.nodes().iter().position(|node| node.site() == site);
        let (left, right) = (child(lhs_site)?, child(rhs_site)?);
        if let Some(family) =
            crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(
                binary.operator(),
            )
        {
            if !coverage.covers_comparison_node_v1(index) {
                return None;
            }
            if coverage.covers_nullable_comparison_v1() {
                if binary.operator() == Op::Equal {
                    let nullable_field =
                        [(left, right), (right, left)]
                            .into_iter()
                            .find_map(|(reference, null)| {
                                let request = fields.iter().find(|request| {
                                    request.node_in_plan(source) == Some(reference)
                                })?;
                                (classes[reference] == Leaf::Value(Class::Opaque)
                                    && classes[null] == Leaf::Value(Class::Null)
                                    && request.is_nullable_object_comparison_v1(source))
                                .then(|| (reference, null, request.clone()))
                            });
                    if let Some((reference, null, request)) = nullable_field {
                        let loan = CheckedExpressionOperationLoanV1 {
                            plan: Rc::clone(source),
                            node: index,
                            proof: CheckedExpressionOperationProofV1::NullableFieldNull {
                                reference,
                                null,
                                request,
                            },
                        };
                        if loan.nullable_field_null_v1().is_none() {
                            return None;
                        }
                        operations.push(Rc::new(loan));
                        continue;
                    }
                    let nullable =
                        [(left, right), (right, left)]
                            .into_iter()
                            .find_map(|(reference, null)| {
                                let Some(CheckedExpressionReferenceV1::NullableResult(frame)) =
                                    reference_proofs.get(&reference)
                                else {
                                    return None;
                                };
                                (classes[reference] == Leaf::Value(Class::Opaque)
                                    && classes[null] == Leaf::Value(Class::Null))
                                .then(|| (reference, null, frame.clone()))
                            });
                    if let Some((reference, null, frame)) = nullable {
                        let loan = CheckedExpressionOperationLoanV1 {
                            plan: Rc::clone(source),
                            node: index,
                            proof: CheckedExpressionOperationProofV1::NullableNull {
                                reference,
                                null,
                                frame,
                            },
                        };
                        if !loan.matches_nullable_null_reference_v1(&reference_proofs) {
                            return None;
                        }
                        operations.push(Rc::new(loan));
                        continue;
                    }
                }
                // A finite entry guard or an invocation result can lend its original
                // carrier tag for equality with the exact null literal.
                if binary.operator() == Op::Equal
                    && [(left, right), (right, left)]
                        .into_iter()
                        .any(|(reference, null)| {
                            if !matches!(
                                source.nodes()[null].source(),
                                ExpressionNodeSourceV1::Literal(
                                    super::super::ResolvedLiteralSourceV1::Null
                                )
                            ) {
                                return false;
                            }
                            match (reference_proofs.get(&reference), classes[reference]) {
                                (
                                    Some(CheckedExpressionReferenceV1::FiniteEntry {
                                        read,
                                        exclusion: None,
                                    }),
                                    Leaf::Value(Class::Opaque),
                                ) => {
                                    read.null_compare_v1().is_some_and(|original| {
                                        original.owner() == source.owner()
                                            && original.site() == node.site()
                                    }) && Rc::ptr_eq(read.requirement().source_plan(), source)
                                }
                                (
                                    Some(CheckedExpressionReferenceV1::DynamicInvocationResult(_)),
                                    Leaf::Value(Class::Dynamic | Class::Opaque),
                                ) => true,
                                _ => false,
                            }
                        })
                {
                    let Ok(envelope) =
                        issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
                            family,
                            DynamicOperatorValueClassV1::Dynamic,
                            DynamicOperatorValueClassV1::Null,
                        ))
                    else {
                        return None;
                    };
                    operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
                        plan: Rc::clone(source),
                        node: index,
                        proof: CheckedExpressionOperationProofV1::Dynamic(Box::new([envelope])),
                    }));
                    continue;
                }
            }
            operations.push(qualify_comparison_operation_v1(source, index, classes)?);
            continue;
        }
        let (lhs, rhs) = (
            classes[left].value_classes()?,
            classes[right].value_classes()?,
        );
        if !coverage.covers_arithmetic_domains_v1(
            classes[left] == Leaf::Value(Class::I64) && classes[right] == Leaf::Value(Class::I64),
        ) {
            return None;
        }
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
        ) && !source_dynamic_sub
        {
            let needs_divisor = matches!(
                binary.operator(),
                ResolvedBinaryOperatorV1::Divide | ResolvedBinaryOperatorV1::Modulo
            );
            if !coverage.covers_scalar_arithmetic_v1(
                classes.first() == Some(&Leaf::Value(Class::I64)),
                classes[index] == Leaf::Value(Class::I64),
                needs_divisor,
            ) || [binary.lhs(), binary.rhs()].iter().any(|site| {
                source
                    .nodes()
                    .iter()
                    .position(|node| node.site() == *site)
                    .is_none_or(|index| classes[index] != Leaf::Value(Class::I64))
            }) {
                return None;
            }
            let divisor_read = source
                .nodes()
                .iter()
                .position(|node| node.site() == binary.rhs())
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
                        (node.site() == binary.rhs())
                            .then(|| match node.source() {
                                ExpressionNodeSourceV1::Literal(
                                    ResolvedLiteralSourceV1::Integer(value),
                                ) => Some(*value),
                                _ => None,
                            })
                            .flatten()
                    }),
                }
            } else {
                None
            };
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
            ResolvedBinaryOperatorV1::Subtract if source_dynamic_sub => {
                DynamicOperatorFamilyV1::Sub
            }
            _ => return None,
        };
        let mut envelopes = Vec::new();
        for lhs in lhs.iter() {
            for rhs in rhs.iter() {
                let dynamic = lhs == Class::Dynamic || rhs == Class::Dynamic;
                let envelope =
                    issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
                        family,
                        if dynamic {
                            tagged_domain(lhs)?
                        } else {
                            class_domain(lhs)?
                        },
                        if dynamic {
                            tagged_domain(rhs)?
                        } else {
                            class_domain(rhs)?
                        },
                    ))
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
    Some((operations, owned_results, logical))
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
