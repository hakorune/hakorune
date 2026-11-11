//! Qualify one original expression body independently of its destination.
//! Source proofs, ordered operations and acquisitions are closed exactly once.
use super::super::SourceExprSiteV1;
use super::*;
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1, expression_projection::ExpressionLeafClassV1,
};

/// Existing statement values admit Text, Integer, or a comparison Bool.
/// This is destination eligibility, not a second expression qualification.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn admits_statement_value_v1(
    evaluation: &CheckedExpressionEvaluationV1,
) -> bool {
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    matches!(
        evaluation.root_class(),
        Leaf::Value(Class::String | Class::I64)
    ) || evaluation.root_class() == Leaf::Value(Class::Bool)
        && evaluation.source().nodes()[0]
            .binary()
            .is_some_and(|binary| {
                crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(
                    binary.operator(),
                )
                .is_some()
            })
}

/// One checked body for value purposes; no destination or Home grant.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn qualify_expression_evaluation_v1<E>(
    mut prepared: super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<super::evaluation::CheckedScalarReferenceV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
) -> Result<Result<(CheckedExpressionEvaluationV1, Vec<LocalFieldReadRequestV1>), &'static str>, E>
{
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    let coverage = prepared.operation_coverage_v1();
    let materializes_text_fields = coverage.materializes_text_fields_v1();
    let source = Rc::clone(prepared.source());
    let prior_homes = prepared.prior_homes().to_vec();
    let Ok(order) = source.ordered_postorder_v1() else {
        return Ok(Err("postorder-unavailable"));
    };
    let reference_order: Vec<_> = (0..source.nodes().len()).collect();
    let Some(mut reference_proofs) =
        super::evaluation::seed_current_expression_references_with_loop_text_v1(
            &mut prepared,
            initial,
            &reference_order,
            static_reference,
            residence_reference,
            loop_text_reference,
            scalar_reference,
        )
    else {
        return Ok(Err("reference-seeds-unproved"));
    };
    let super::leaves::QualifiedExpressionLeavesV1 {
        classes,
        mut calls,
        cores,
        fields,
        transferred,
        references,
    } = match super::leaves::qualify_expression_leaves_with_calls_v1::<E>(
        prepared,
        &mut |_| None,
        &order,
        field_request,
    )? {
        Ok(leaves) => leaves,
        Err(reason) => return Ok(Err(reason)),
    };
    if !references.is_empty() {
        return Ok(Err("scalar-references-unsupported"));
    }
    reference_proofs.retain(|node, _| !transferred.contains(node));
    // R16: a root comparison is a Bool value; its operand subtrees keep their own law.
    let root_comparison = (coverage.covers_comparison_node_v1(0)
        && classes[0] == Leaf::Value(Class::Bool))
    .then(|| super::operations::qualify_comparison_operation_v1(&source, 0, &classes))
    .flatten();
    let Some((operations, mut owned_results)) = (if let Some(root) = root_comparison {
        let inner_order: Vec<usize> = order.iter().copied().filter(|node| *node != 0).collect();
        super::operations::qualify_arithmetic_operations_v1(
            &source,
            &inner_order,
            &classes,
            &transferred,
            &reference_proofs,
        )
        .map(|(mut operations, owned)| {
            operations.push(root);
            (operations, owned)
        })
    } else {
        super::operations::qualify_arithmetic_operations_v1(
            &source,
            &order,
            &classes,
            &transferred,
            &reference_proofs,
        )
    }) else {
        return Ok(Err("arithmetic-operations-unproved"));
    };
    if materializes_text_fields {
        owned_results.extend(super::operations::owned_text_field_nodes_v1(
            &source,
            &order,
            &classes,
            &transferred,
        ));
    }
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[statement-evaluation] owner={:?} order={:?} owned_candidates={:?} transferred={:?} classes={:?} nodes={:?}",
            source.owner(), order, owned_results, transferred, classes, source.nodes().iter().map(|node| format!("{:?}", node.source())).collect::<Vec<_>>());
    }
    let owned_results =
        super::evaluation::owned_result_nodes(&order, &owned_results, &cores, &calls);
    let mut evaluation = CheckedExpressionEvaluationV1 {
        source,
        prior_homes: prior_homes.into(),
        order: order.clone().into_boxed_slice(),
        classes: classes.into_boxed_slice(),
        references: reference_proofs,
        calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
        operations: operations.into_boxed_slice(),
        owned_results,
        cores,
        transferred_inputs: transferred,
    };
    if !evaluation.retain_core_text_entry_reads_v1() || !evaluation.reference_coverage_is_exact() {
        return Ok(Err("core-text-entry-or-reference-coverage"));
    }
    Ok(Ok((evaluation, fields)))
}
