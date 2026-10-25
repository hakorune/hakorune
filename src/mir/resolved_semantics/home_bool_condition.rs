//! Whole Bool condition qualification on the same immutable expression source.
//! Conditional reach is retained; this owner never emits or eagerly runs RHS.
use super::super::{
    BindingRefV1, LocalCallObservationV1, LocalCallResultClassV1, OwnedExprSiteV1,
    SourceExprSiteV1, SourceScalarKind,
};
use super::leaves::{ExpressionLeafProfileV1, QualifiedExpressionLeavesV1};
use super::*;
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1, DynamicOperatorFamilyV1,
    DynamicOperatorValueClassV1,
};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
    expression_class::ExpressionValueClassV1,
    expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedBoolConditionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    logical: Box<[usize]>,
}
impl CheckedBoolConditionFrameV1 {
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
    pub(crate) fn logical_nodes(&self) -> &[usize] {
        &self.logical
    }
    pub(crate) fn references(&self) -> &BTreeMap<usize, CheckedExpressionReferenceV1> {
        self.evaluation.references()
    }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> {
        self.evaluation.classes.get(node).copied()
    }
    pub(crate) fn transfers_node(&self, node: usize) -> bool {
        self.evaluation.transfers_node(node)
    }
    /// These original leaves are observations owned by one checked comparison,
    /// not scalar-producing nodes or separate Local acquisitions.
    pub(crate) fn comparison_input_children_v1(&self, node: usize) -> Option<[usize; 2]> {
        let loan = self.operations().iter().find(|loan| loan.node() == node)?;
        let envelope = loan.single_envelope()?;
        let domain = envelope.domain();
        let operands_supported = self.evaluation.carrier_integer_comparison_nodes_v1(node).is_some() || matches!(
            (domain.left(), domain.right()),
            (DynamicOperatorValueClassV1::String, DynamicOperatorValueClassV1::String)
                | (DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::String)
                | (DynamicOperatorValueClassV1::String, DynamicOperatorValueClassV1::Dynamic)
        );
        if !operands_supported || envelope.normal_result()
            != crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::TrivialBool {
            return None;
        }
        let binary = self.source().nodes().get(node)?.binary()?;
        let index = |site| {
            self.source()
                .nodes()
                .iter()
                .position(|node| node.site() == site)
        };
        Some([index(binary.lhs())?, index(binary.rhs())?])
    }
    pub(crate) fn scalar_invocation_input_nodes_v1(&self) -> BTreeSet<usize> {
        self.ordered_nodes()
            .iter()
            .filter_map(|node| self.evaluation.scalar_invoke_input_order_v1(*node))
            .flatten()
            .collect()
    }
}

/// Every source/operator/call/refusal check precedes the last strict field
/// batch. Only a complete candidate may enter the existing Home inventory.
pub(crate) fn qualify_bool_condition_v1<E>(
    source: Rc<ExpressionSourcePlanV1>,
    prior_homes: &[LiveHomeV1],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, SourceScalarKind)>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    source_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallObservationV1>, E>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !matches!(source.root(), ExpressionPlanRootV1::IfCondition { .. }) {
        return Ok(None);
    }
    let Some(prepared) = super::leaves::prepare_expression_calls_v1(
        &source,
        ExpressionLeafProfileV1::BoolCondition,
        &BTreeMap::new(),
        prior_homes,
        &mut |site| {
            source_call(site).map(|call| call.map(super::leaves::ExpressionCallSourceV1::Ordinary))
        },
    )?
    else {
        return Ok(None);
    };
    qualify_condition_expression_v1(
        prepared,
        &[],
        static_reference,
        residence_reference,
        &mut |site| {
            scalar_reference(site)
                .map(|(binding, kind)| CheckedScalarReferenceV1::Obligation { binding, kind })
        },
        field_request,
        can_install,
        field_batch,
    )
}

/// Preserve the genuine If/Loop purpose and exact original scalar proofs.
/// This shares qualification only; it grants no outer control-flow completion.
pub(crate) fn qualify_condition_expression_v1<E>(
    mut prepared: super::leaves::PreparedExpressionCallsV1,
    initial_references: &[(usize, CheckedExpressionReferenceV1)],
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<CheckedScalarReferenceV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !prepared.is_bool_condition() {
        return Ok(None);
    }
    let source = Rc::clone(prepared.source());
    let prior_homes = prepared.prior_homes().to_vec();
    use ExpressionLeafClassV1 as Leaf;
    use ExpressionValueClassV1 as Class;
    use ResolvedBinaryOperatorV1 as Op;
    let mut homes = BTreeSet::new();
    if prior_homes
        .iter()
        .any(|home| home.owner() != source.owner() || !homes.insert(home.binding()))
    {
        return Ok(None);
    }
    let condition = match source.root() {
        ExpressionPlanRootV1::IfCondition { condition, .. }
        | ExpressionPlanRootV1::LoopCondition { condition, .. } => condition,
        _ => return Ok(None),
    };
    if source
        .nodes()
        .first()
        .is_none_or(|node| node.site() != condition)
    {
        return Ok(None);
    }
    let Ok(order) = source.ordered_postorder_v1() else {
        return Ok(None);
    };
    let initial = initial_references.to_vec();
    let Some(reference_proofs) = super::evaluation::seed_current_expression_references_v1(
        &mut prepared,
        initial,
        &order,
        static_reference,
        residence_reference,
        scalar_reference,
    ) else {
        return Ok(None);
    };
    let Some(QualifiedExpressionLeavesV1 {
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
    if classes[0] != Leaf::Value(Class::Bool) {
        return Ok(None);
    }
    let mut operations = Vec::new();
    let mut logical = Vec::new();
    for &index in &order {
        if transferred.contains(&index) {
            continue;
        }
        let node = &source.nodes()[index];
        let ExpressionChildrenV1::Known(children) = node.children() else {
            return Ok(None);
        };
        let Some(binary) = node.binary() else {
            if children
                .iter()
                .any(|child| child.reach != ExpressionChildReachV1::Always)
            {
                return Ok(None);
            }
            continue;
        };
        let Some([lhs, rhs]) =
            crate::mir::resolved_value_profile::expression_children::binary_children_v1(
                node.site(),
                binary,
            )
        else {
            return Ok(None);
        };
        if children.len() != 2
            || children[0].site != *lhs
            || children[1].site != *rhs
            || children[0].reach != ExpressionChildReachV1::Always
        {
            return Ok(None);
        }
        if matches!(binary.operator(), Op::And | Op::Or) {
            if children[1].reach
                != (ExpressionChildReachV1::When {
                    condition: lhs.clone(),
                    value: binary.operator() == Op::And,
                })
                || classes[index] != Leaf::Value(Class::Bool)
            {
                return Ok(None);
            }
            logical.push(index);
            continue;
        }
        if children[1].reach != ExpressionChildReachV1::Always {
            return Ok(None);
        }
        let child = |site| source.nodes().iter().position(|node| node.site() == site);
        let (Some(left), Some(right)) = (child(lhs), child(rhs)) else {
            return Ok(None);
        };
        if binary.operator() == Op::Subtract {
            if [index, left, right].into_iter().any(|node| classes[node] != Leaf::Value(Class::I64)) {
                return Ok(None);
            }
            let Some(scalar) = ExpressionScalarOperationLoanV1::issue_node_v1(
                Rc::clone(&source), index, binary, None,
            ) else {
                return Ok(None);
            };
            operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
                plan: Rc::clone(&source),
                node: index,
                proof: CheckedExpressionOperationProofV1::Scalar(scalar),
            }));
            continue;
        }
        let family = match binary.operator() {
            Op::Equal => DynamicOperatorFamilyV1::Equal,
            Op::NotEqual => DynamicOperatorFamilyV1::NotEqual,
            Op::Less => DynamicOperatorFamilyV1::Less,
            Op::Greater => DynamicOperatorFamilyV1::Greater,
            Op::LessEqual => DynamicOperatorFamilyV1::LessEqual,
            Op::GreaterEqual => DynamicOperatorFamilyV1::GreaterEqual,
            Op::Add => DynamicOperatorFamilyV1::Add,
            _ => return Ok(None),
        };
        if binary.operator() == Op::Equal {
            let nullable_field = [(left, right), (right, left)].into_iter().find_map(|(reference, null)| {
                let request = fields.iter().find(|request| request.node_in_plan(&source) == Some(reference))?;
                (classes[reference] == Leaf::Value(Class::Opaque)
                    && classes[null] == Leaf::Value(Class::Null)
                    && request.is_nullable_object_comparison_v1(&source)).then(|| (reference, null, request.clone()))
            });
            if let Some((reference, null, request)) = nullable_field {
                let loan = CheckedExpressionOperationLoanV1 { plan: Rc::clone(&source), node: index,
                    proof: CheckedExpressionOperationProofV1::NullableFieldNull { reference, null, request } };
                if loan.nullable_field_null_v1().is_none() { return Ok(None); }
                operations.push(Rc::new(loan));
                continue;
            }
            let nullable =
                [(left, right), (right, left)]
                    .into_iter()
                    .find_map(|(reference, null)| {
                        let Some(CheckedExpressionReferenceV1::StoredMapGet(frame)) =
                            reference_proofs.get(&reference)
                        else {
                            return None;
                        };
                        (classes[reference] == Leaf::Value(Class::Opaque)
                            && classes[null] == Leaf::Value(Class::Null))
                        .then(|| (reference, null, Rc::clone(frame)))
                    });
            if let Some((reference, null, frame)) = nullable {
                let loan = CheckedExpressionOperationLoanV1 {
                    plan: Rc::clone(&source),
                    node: index,
                    proof: CheckedExpressionOperationProofV1::NullableNull {
                        reference,
                        null,
                        frame,
                    },
                };
                if !loan.matches_nullable_null_reference_v1(&reference_proofs) {
                    return Ok(None);
                }
                operations.push(Rc::new(loan));
                continue;
            }
        }
        // Only the original entry guard loan admits raw tagged equality. The
        // sibling remains the exact source null; no payload view is issued.
        if binary.operator() == Op::Equal {
            let entry_null = [(left, right), (right, left)]
                .into_iter()
                .any(|(reference, null)| {
                    matches!(reference_proofs.get(&reference),
                    Some(CheckedExpressionReferenceV1::FiniteEntry { read, exclusion: None })
                    if read.null_compare_v1().is_some_and(|original|
                        original.owner() == source.owner() && original.site() == node.site())
                        && Rc::ptr_eq(read.requirement().source_plan(), &source))
                        && classes[reference] == Leaf::Value(Class::Opaque)
                        && matches!(
                            source.nodes()[null].source(),
                            ExpressionNodeSourceV1::Literal(
                                super::super::ResolvedLiteralSourceV1::Null
                            )
                        )
                });
            if entry_null {
                let Ok(envelope) =
                    issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
                        family,
                        DynamicOperatorValueClassV1::Dynamic,
                        DynamicOperatorValueClassV1::Null,
                    ))
                else {
                    return Ok(None);
                };
                operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
                    plan: Rc::clone(&source),
                    node: index,
                    proof: CheckedExpressionOperationProofV1::Dynamic(Box::new([envelope])),
                }));
                continue;
            }
        }
        if matches!(
            binary.operator(),
            Op::Equal | Op::NotEqual | Op::Less | Op::LessEqual | Op::Greater | Op::GreaterEqual
        ) {
            let mixed_string_dynamic = match (classes[left], classes[right]) {
                (Leaf::Value(Class::Dynamic), Leaf::Value(Class::String)) => Some((
                    DynamicOperatorValueClassV1::Dynamic,
                    DynamicOperatorValueClassV1::String,
                )),
                (Leaf::Value(Class::String), Leaf::Value(Class::Dynamic)) => Some((
                    DynamicOperatorValueClassV1::String,
                    DynamicOperatorValueClassV1::Dynamic,
                )),
                _ => None,
            };
            if let Some((left_class, right_class)) = mixed_string_dynamic {
                let Ok(envelope) = issue_dynamic_operator_execution_envelope_v1(
                    DynamicOperatorDomainV1::new(family, left_class, right_class),
                ) else {
                    return Ok(None);
                };
                operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
                    plan: Rc::clone(&source),
                    node: index,
                    proof: CheckedExpressionOperationProofV1::Dynamic(Box::new([envelope])),
                }));
                continue;
            }
        }
        let (left_class, right_class) = match (classes[left], classes[right]) {
            (Leaf::Value(Class::I64), Leaf::Value(Class::I64)) => (
                DynamicOperatorValueClassV1::NormalInteger,
                DynamicOperatorValueClassV1::NormalInteger,
            ),
            (Leaf::Value(Class::Dynamic), Leaf::Value(Class::I64))
                if matches!(binary.operator(), Op::Less | Op::Greater) => (
                    DynamicOperatorValueClassV1::Dynamic,
                    DynamicOperatorValueClassV1::I64,
                ),
            (Leaf::Value(Class::I64), Leaf::Value(Class::Dynamic))
                if binary.operator() == Op::Less => (
                    DynamicOperatorValueClassV1::I64,
                    DynamicOperatorValueClassV1::Dynamic,
                ),
            (Leaf::Value(Class::Bool), Leaf::Value(Class::Bool))
                if matches!(binary.operator(), Op::Equal | Op::NotEqual) =>
            {
                (DynamicOperatorValueClassV1::Bool, DynamicOperatorValueClassV1::Bool)
            }
            (Leaf::Value(Class::String), Leaf::Value(Class::String))
                if binary.operator() != Op::Add =>
            {
                // The complete original input subtree is checked below, after
                // all comparison loans and Core acquisitions are retained.
                (DynamicOperatorValueClassV1::String, DynamicOperatorValueClassV1::String)
            }
            _ => return Ok(None),
        };
        let Ok(envelope) = issue_dynamic_operator_execution_envelope_v1(
            DynamicOperatorDomainV1::new(family, left_class, right_class),
        ) else { return Ok(None); };
        operations.push(Rc::new(CheckedExpressionOperationLoanV1 {
            plan: Rc::clone(&source),
            node: index,
            proof: CheckedExpressionOperationProofV1::Dynamic(Box::new([envelope])),
        }));
    }
    // Dynamic calls are admitted only as owned operands of a checked comparison;
    // the exact comparison consumer is checked below through input_owners.
    if calls.values().any(|call| {
        !matches!(
            call.result(),
            LocalCallResultClassV1::I64
                | LocalCallResultClassV1::Bool
                | LocalCallResultClassV1::Dynamic
        )
    }) {
        return Ok(None);
    }
    let owned_results = super::evaluation::owned_result_nodes(&order, &[], &cores, &calls);
    let mut evaluation = CheckedExpressionEvaluationV1 {
        source: Rc::clone(&source),
        prior_homes: prior_homes.into(),
        order: order.clone().into_boxed_slice(),
        classes: classes.into_boxed_slice(),
        calls: order.iter().filter_map(|node| calls.remove(node)).collect(),
        operations: operations.into_boxed_slice(),
        references: reference_proofs,
        cores,
        owned_results,
        transferred_inputs: transferred,
    };
    if !evaluation.retain_core_text_entry_reads_v1() {
        return Ok(None);
    }
    let frame = Rc::new(CheckedBoolConditionFrameV1 {
        evaluation,
        logical: logical.into_boxed_slice(),
    });
    if !frame.evaluation().reference_coverage_is_exact() {
        return Ok(None);
    }
    let mut input_owners = BTreeMap::new();
    for &node in frame.ordered_nodes() {
        let required_carrier_comparison = frame.operations().iter().find(|loan| loan.node() == node)
            .and_then(|loan| loan.single_envelope()).is_some_and(|envelope|
                envelope.domain().family() == DynamicOperatorFamilyV1::Greater
                    && envelope.domain().left() == DynamicOperatorValueClassV1::Dynamic
                    && envelope.domain().right() == DynamicOperatorValueClassV1::I64);
        if required_carrier_comparison && frame.evaluation().carrier_integer_comparison_nodes_v1(node).is_none() {
            return Ok(None);
        }
        let scalar_core = frame.evaluation().cores().get(&node).is_some_and(|core| {
            core.field_map().is_some() && core.source().target().checked_map_presence_law_v1()
        });
        if frame.comparison_input_children_v1(node).is_none() && !scalar_core {
            continue;
        }
        let Some(inputs) = frame.evaluation().scalar_invoke_input_order_v1(node) else {
            return Ok(None);
        };
        for input in inputs {
            if input_owners.insert(input, node).is_some() {
                return Ok(None);
            }
        }
    }
    if frame.evaluation().cores().iter().any(|(node, core)| {
        !input_owners.contains_key(node)
            && !(core.field_map().is_some()
                && core.source().target().checked_map_presence_law_v1()
                && frame
                    .evaluation()
                    .scalar_invoke_input_order_v1(*node)
                    .is_some())
    }) || frame
        .evaluation()
        .owned_result_nodes()
        .iter()
        .any(|node| !input_owners.contains_key(node))
    {
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
        print_frame: None,
        condition_frame: Some(frame),
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
        super::super::LocalFieldReadDemandV1::SourceExpression { plan: source },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}
