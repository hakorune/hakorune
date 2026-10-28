//! Whole Print qualification over the shared original expression plan.
//! No physical IDs, runtime tokens or independent call inventory are issued.
use super::super::{LocalCallObservationV1, OwnedExprSiteV1, SourceExprSiteV1};
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1,
    expression_root::StatementExpressionRoleV1,
};
use std::collections::BTreeSet;

/// Print purpose authority wraps the SAME qualified expression body. The body
/// alone neither selects a Print root nor installs a Home observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedPrintExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
}
impl CheckedPrintExpressionFrameV1 {
    /// A bare statement call whose owned result ends with the statement.
    pub(crate) fn is_discard(&self) -> bool {
        matches!(self.source().root(), ExpressionPlanRootV1::Statement {
            role: StatementExpressionRoleV1::DiscardValue, ..
        })
    }
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        self.evaluation.prior_homes()
    }
    pub(crate) fn ordered_nodes(&self) -> &[usize] {
        self.evaluation.ordered_nodes()
    }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] {
        self.evaluation.calls()
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] {
        self.evaluation.operations()
    }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] {
        self.evaluation.owned_result_nodes()
    }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 {
        self.evaluation.root_class()
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.evaluation.transfers_node(node)
    }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool {
        self.evaluation.node_is_identity(node)
    }
}

/// The caller first retains original actual facts at the pre-statement basis.
/// Call demands below are passive: none may publish Home call observations.
/// Finish every rejectable root/envelope/install check before the strict field
/// batch; after that final atomic batch succeeds, returning the frame cannot fail.
pub(crate) fn qualify_print_expression_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: &[LiveHomeV1],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<super::leaves::ExpressionCallSourceV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    let mut unique_homes = BTreeSet::new();
    if prior_homes
        .iter()
        .any(|home| home.owner() != source.owner() || !unique_homes.insert(home.binding()))
    {
        return Ok(None);
    }
    let ExpressionPlanRootV1::Statement {
        role: StatementExpressionRoleV1::PrintValue | StatementExpressionRoleV1::DiscardValue,
        value,
        ..
    } = source.root()
    else {
        return Ok(None);
    };
    let Some(root) = source.nodes().first() else {
        return Ok(None);
    };
    if root.site() != value {
        return Ok(None);
    }
    if source.nodes().iter().any(|node| {
        !matches!(node.children(), ExpressionChildrenV1::Known(children)
        if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
    }) {
        return Ok(None);
    }

    let Some(prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        super::leaves::ExpressionLeafProfileV1::Print,
        &std::collections::BTreeMap::new(),
        prior_homes,
        source_call,
    )?
    else {
        return Ok(None);
    };
    let Some((evaluation, fields)) = qualify_statement_evaluation_v1(
        prepared,
        Vec::new(),
        static_reference,
        residence_reference,
        loop_text_reference,
        scalar_reference,
        field_request,
    )?
    else {
        return Ok(None);
    };
    let frame = Rc::new(CheckedPrintExpressionFrameV1 { evaluation });
    if !frame.evaluation().reference_coverage_is_exact() {
        return Ok(None);
    }
    let candidate = HomeExpressionPlanV1 {
        source: Rc::clone(&source),
        scalar_operations: Vec::new(),
        source_fields: Vec::new(),
        initializer_homes: None,
        argument_frame: None,
        initializer_frame: None,
        rebind_frame: None,
        print_frame: Some(frame),
        condition_frame: None,
        loop_recipe: None,
        field_write: None,
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression { probe: false, plan: source },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}

/// One checked body for existing statement purposes; no terminal or Home grant.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn qualify_statement_evaluation_v1<E>(
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
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
) -> Result<Option<(CheckedExpressionEvaluationV1, Vec<LocalFieldReadRequestV1>)>, E> {
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    let source = Rc::clone(prepared.source());
    let prior_homes = prepared.prior_homes().to_vec();
    let Ok(order) = source.ordered_postorder_v1() else {
        return Ok(None);
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
            &mut |site| {
                scalar_reference(site).map(super::evaluation::CheckedScalarReferenceV1::Original)
            },
        )
    else {
        return Ok(None);
    };
    let Some(super::leaves::QualifiedExpressionLeavesV1 {
        classes,
        mut calls,
        cores,
        fields,
        transferred,
        references,
    }) = super::leaves::qualify_expression_leaves_with_calls_v1::<E>(
        prepared,
        &mut |_| None,
        &order,
        field_request,
    )?
    else {
        return Ok(None);
    };
    if !references.is_empty() {
        return Ok(None);
    }
    reference_proofs.retain(|node, _| !transferred.contains(node));
    if !matches!(classes[0], Leaf::Value(Class::String | Class::I64)) {
        return Ok(None);
    }
    let Some((operations, mut owned_results)) = super::operations::qualify_arithmetic_operations_v1(
        &source,
        &order,
        &classes,
        &transferred,
    ) else {
        return Ok(None);
    };
    owned_results.extend(order.iter().copied().filter(|node| !transferred.contains(node)
        && classes[*node] == Leaf::Value(Class::String)
        && matches!(source.nodes()[*node].source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }))));
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
        return Ok(None);
    }
    Ok(Some((evaluation, fields)))
}
