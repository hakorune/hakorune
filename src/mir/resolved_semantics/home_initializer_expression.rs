//! Original Local initializer purpose over the shared checked expression body.
use super::super::{BindingRefV1, SourceExprSiteV1, SourceScalarKind};
use super::*;
use crate::mir::normal_callable_semantic_package::FiniteEntryReadV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::{
    expression_class::ExpressionValueClassV1 as Class, expression_plan::ExpressionPlanRootV1,
    expression_projection::ExpressionLeafClassV1 as Leaf,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(crate) struct TextResidenceReadV1 {
    node: usize,
    binding: BindingRefV1,
    root: BindingRefV1,
    acquisition: TextResidenceAcquisitionV1,
}
impl PartialEq for TextResidenceReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.binding == other.binding
            && self.root == other.root
            && self.acquisition.same_source(&other.acquisition)
    }
}
impl Eq for TextResidenceReadV1 {}
impl TextResidenceReadV1 {
    pub(super) fn new(
        node: usize,
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: TextResidenceAcquisitionV1,
    ) -> Self {
        Self {
            node,
            binding,
            root,
            acquisition,
        }
    }
    pub(crate) fn node(&self) -> usize {
        self.node
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn root(&self) -> BindingRefV1 {
        self.root
    }
    pub(crate) fn acquisition(&self) -> &TextResidenceAcquisitionV1 {
        &self.acquisition
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedInitializerExpressionFrameV1 {
    evaluation: CheckedExpressionEvaluationV1,
    signed_root_sum: Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>>,
}
impl CheckedInitializerExpressionFrameV1 {
    pub(crate) fn signed_root_sum(
        &self,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>> {
        self.signed_root_sum.as_ref()
    }
    pub(crate) fn text_acquisition_v1(self: &Rc<Self>) -> TextResidenceAcquisitionV1 {
        TextResidenceAcquisitionV1::from_initializer(Rc::clone(self))
    }
    pub(crate) fn evaluation(&self) -> &CheckedExpressionEvaluationV1 {
        &self.evaluation
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        self.evaluation.source()
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.source()
            .root()
            .initializer()
            .expect("original initializer")
            .binding()
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        self.evaluation.prior_homes()
    }
    pub(crate) fn entry_reads(&self) -> Vec<&FiniteEntryReadV1> {
        self.evaluation
            .references()
            .values()
            .filter_map(|read| match read {
                CheckedExpressionReferenceV1::FiniteEntry { read, .. } => Some(read),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn entry_refinements(
        &self,
    ) -> BTreeMap<usize, &Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1>>
    {
        self.evaluation
            .references()
            .iter()
            .filter_map(|(&node, read)| match read {
                CheckedExpressionReferenceV1::FiniteEntry {
                    exclusion: Some(proof),
                    ..
                } => Some((node, proof)),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn residence_reads(&self) -> Vec<&TextResidenceReadV1> {
        self.evaluation
            .references()
            .values()
            .filter_map(|read| match read {
                CheckedExpressionReferenceV1::Residence(read) => Some(read),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn scalar_reads(&self) -> BTreeMap<usize, (BindingRefV1, SourceScalarKind)> {
        self.evaluation
            .references()
            .iter()
            .filter_map(|(&node, read)| match read {
                CheckedExpressionReferenceV1::Scalar(read) => {
                    Some((node, (read.binding(), read.kind())))
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn is_i64(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::I64)
    }
    /// An exact Bool Local produced by a root comparison.
    pub(crate) fn is_bool(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::Bool)
    }
    pub(crate) fn is_text_residence(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::String)
            && (self.evaluation.owned_text_root_node_v1().is_some()
                || self.evaluation.text_snapshot_root_v1().is_some())
    }
    /// A Text-proven Array dynamic get is an owned carrier; the Local owns a Text snapshot of it.
    pub(crate) fn materializes_root(&self) -> bool {
        self.is_text_residence() && self.evaluation.owned_text_root_node_v1().is_none()
    }
    pub(crate) fn is_dynamic_invocation_result(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::Dynamic)
            && self.evaluation.calls().iter().any(|call| {
                call.site().site() == self.source().nodes()[0].site()
                    && call.dynamic_invocation_result_lifecycle_v1().is_some()
            })
    }
    /// A local alias borrows the original carrier. Its initializer frame keeps
    /// the reaching definition, while the original Stable Home owns cleanup.
    pub(crate) fn is_dynamic_alias(&self) -> bool {
        self.evaluation.root_class() == Leaf::Value(Class::Dynamic)
            && self.source().nodes().len() == 1
            && !self.owns_root()
            && matches!(self.evaluation.references().get(&0),
                Some(CheckedExpressionReferenceV1::DynamicInvocationResult(source))
                if source.dynamic_home_binding_v1().is_some_and(|owner|
                    self.prior_homes().contains(&LiveHomeV1::Stable(owner))))
    }
    pub(crate) fn is_dynamic_value(&self) -> bool {
        self.is_dynamic_invocation_result() || self.is_dynamic_alias()
    }
    pub(crate) fn dynamic_home_binding_v1(&self) -> Option<BindingRefV1> {
        if self.is_dynamic_invocation_result() {
            return Some(self.binding());
        }
        if self.evaluation.root_class() != Leaf::Value(Class::Dynamic)
            || self.source().nodes().len() != 1
            || self.owns_root()
        {
            return None;
        }
        let CheckedExpressionReferenceV1::DynamicInvocationResult(source) =
            self.evaluation.references().get(&0)?
        else {
            return None;
        };
        let owner = source.dynamic_home_binding_v1()?;
        self.prior_homes()
            .contains(&LiveHomeV1::Stable(owner))
            .then_some(owner)
    }
    pub(crate) fn owns_root(&self) -> bool {
        self.evaluation.owned_result_nodes().contains(&0)
    }
}

pub(crate) fn qualify_initializer_expression_v1<E>(
    prepared: super::leaves::PreparedExpressionCallsV1,
    entry_reads: &[FiniteEntryReadV1],
    dynamic_entry_reads: &[crate::mir::normal_callable_semantic_package::DynamicEntryReadV1],
    dynamic_local_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<Rc<CheckedInitializerExpressionFrameV1>>,
    signed_root_sum: Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>>,
    // A selected source Loop body evaluates even a scalar literal or plain
    // read through one complete frame; the caller proves that context.
    loop_scalar_frame: bool,
    entry_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<(BindingRefV1, BindingRefV1)>,
    entry_refinement: &mut impl FnMut(
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
    ) -> Option<
        Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1>,
    >,
    residence_reference: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<(
        BindingRefV1,
        BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    static_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    scalar_reference: &mut impl FnMut(&SourceExprSiteV1) -> Option<super::super::CurrentScalarReadV1>,
    field_request: &mut impl FnMut(&SourceExprSiteV1) -> Option<LocalFieldReadRequestV1>,
    can_install: &mut impl FnMut(&HomeExpressionPlanV1) -> bool,
    field_batch: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<bool, E>,
) -> Result<Option<HomeExpressionPlanV1>, E> {
    if !prepared.is_local_initializer() {
        return Ok(None);
    }
    let prior_homes = prepared.prior_homes().to_vec();
    let source = Rc::clone(prepared.source());
    let has_core = prepared.has_core();
    let ExpressionPlanRootV1::LocalInitializer { initializer, .. } = source.root() else {
        return Ok(None);
    };
    if initializer.binding().owner() != source.owner()
        || source.nodes().first().map(|node| node.site()) != initializer.initializer_site()
    {
        return Ok(None);
    }
    if !source.admits_prior_homes_v1(&prior_homes) {
        return Ok(None);
    }
    if !source.children_always_known_v1() {
        return Ok(None);
    }
    let mut initial = Vec::new();
    for (node, original) in source.nodes().iter().enumerate() {
        if matches!(original.source(), ExpressionNodeSourceV1::Reference(_)) {
            if let Some(frame) = dynamic_local_reference(original.site()) {
                initial.push((
                    node,
                    CheckedExpressionReferenceV1::DynamicInvocationResult(frame),
                ));
            }
        }
    }
    for read in entry_reads {
        let requirement = read.requirement();
        if entry_reference(requirement.read())
            != Some((requirement.binding(), requirement.formal().1))
        {
            return Ok(None);
        }
        let exclusion = entry_refinement(
            requirement.read(),
            requirement.binding(),
            requirement.formal().1,
        );
        initial.push((
            requirement.node(),
            CheckedExpressionReferenceV1::FiniteEntry {
                read: read.clone(),
                exclusion,
            },
        ));
    }
    for read in dynamic_entry_reads {
        let requirement = read.requirement();
        if entry_reference(requirement.read())
            != Some((requirement.binding(), requirement.formal().1))
            || !read.corroborates_original_read_v1()
        {
            return Ok(None);
        }
        initial.push((
            requirement.node(),
            CheckedExpressionReferenceV1::DynamicEntry(read.clone()),
        ));
    }
    let Ok((evaluation, fields)) = super::qualification::qualify_expression_evaluation_v1(
        prepared,
        initial,
        static_reference,
        residence_reference,
        &mut |_| None,
        &mut |site| {
            scalar_reference(site).map(super::evaluation::CheckedScalarReferenceV1::Original)
        },
        field_request,
    )?
    else {
        return Ok(None);
    };
    let classes = &evaluation.classes;
    let reference_proofs = evaluation.references();
    let root_call = evaluation
        .calls()
        .iter()
        .find(|call| call.site().site() == source.nodes()[0].site());
    let signed_current = signed_root_sum.as_ref().is_some_and(|sum| {
        sum.signed_only()
            && matches!(
                source.nodes()[0].source(),
                ExpressionNodeSourceV1::Reference(_)
            )
            && matches!(reference_proofs.get(&0), Some(CheckedExpressionReferenceV1::Scalar(read))
            if read.original_read().is_some_and(|read| read.read() == source.nodes()[0].site()))
    });
    if signed_root_sum.is_some() && (!signed_current || classes[0] != Leaf::Value(Class::I64)) {
        return Ok(None);
    }
    let dynamic_invocation_root = classes[0] == Leaf::Value(Class::Dynamic)
        && root_call.is_some_and(|call| {
            call.site().owner() == source.owner()
                && call.site().site() == source.nodes()[0].site()
                && call.dynamic_invocation_result_lifecycle_v1().is_some()
        });
    let dynamic_alias_root = classes[0] == Leaf::Value(Class::Dynamic)
        && source.nodes().len() == 1
        && matches!(reference_proofs.get(&0), Some(CheckedExpressionReferenceV1::DynamicInvocationResult(frame))
            if frame.dynamic_home_binding_v1().is_some_and(|owner| prior_homes.contains(&LiveHomeV1::Stable(owner))));
    // R16: a root comparison over exact operands is a Bool Local. The comparison table names
    // the family; the shared qualifier admits the operand classes.
    let bool_comparison = classes[0] == Leaf::Value(Class::Bool)
        && source.nodes()[0].binary().is_some_and(|binary| {
            crate::mir::resolved_value_profile::expression_class::comparison_operator_family_v1(
                binary.operator(),
            )
            .is_some()
        });
    let copy_root = source.copy_operand_node_v1(0).is_some();
    let bool_root = bool_comparison || (copy_root && classes[0] == Leaf::Value(Class::Bool));
    if evaluation.transfers_node(0)
        || (!matches!(classes[0], Leaf::Value(Class::String | Class::I64))
            && !dynamic_invocation_root
            && !dynamic_alias_root
            && !bool_root)
        || (classes[0] == Leaf::Value(Class::I64)
            && !has_core
            && root_call.is_none()
            && !entry_reads
                .iter()
                .any(|read| read.requirement().node() == 0 && read.signed_only())
            && !signed_current
            && !loop_scalar_frame
            && source.nodes()[0].binary().is_none()
            && !copy_root)
    {
        return Ok(None);
    }
    // Destination owns the installed Text root; child acquisitions stay in
    // the common ordered evaluation regardless of the destination's class.
    if classes[0] == Leaf::Value(Class::String) && !evaluation.owned_result_nodes().contains(&0) {
        return Ok(None);
    }
    let Some(candidate) = initializer_plan_v1(evaluation, signed_root_sum) else {
        return Ok(None);
    };
    let Some(candidate) = candidate.with_source_field_requirements(&fields) else {
        return Ok(None);
    };
    if !can_install(&candidate) {
        return Ok(None);
    }
    if !field_batch(
        &fields,
        super::super::LocalFieldReadDemandV1::SourceExpression {
            probe: false,
            plan: source,
        },
    )? {
        return Ok(None);
    }
    Ok(Some(candidate))
}

fn initializer_plan_v1(
    mut evaluation: CheckedExpressionEvaluationV1,
    signed_root_sum: Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>>,
) -> Option<HomeExpressionPlanV1> {
    if !evaluation.retain_core_text_entry_reads_v1() {
        return None;
    }
    let source = Rc::clone(evaluation.source());
    let frame = Rc::new(CheckedInitializerExpressionFrameV1 {
        signed_root_sum,
        evaluation,
    });
    if !frame.evaluation().reference_coverage_is_exact() {
        return None;
    }
    let candidate = HomeExpressionPlanV1 {
        initializer_frame: Some(super::InitializerFrameV1::Evaluation(frame)),
        ..HomeExpressionPlanV1::for_source(Rc::clone(&source))
    };
    candidate.is_consistent().then_some(candidate)
}

/// The existing alias/null law lends its same non-owning Bool to the ordinary
/// evaluator; Eq/Ne permission does not come from the general Dynamic domain.
pub(crate) fn qualify_alias_null_initializer_v1(
    source: Rc<ExpressionSourcePlanV1>,
    binding: BindingRefV1,
    root: BindingRefV1,
    homes: &[LiveHomeV1],
) -> Option<HomeExpressionPlanV1> {
    source.root().initializer()?;
    let binary = source.nodes().first()?.binary()?;
    let index = |site| source.nodes().iter().position(|node| node.site() == site);
    let (left, right) = (index(binary.lhs())?, index(binary.rhs())?);
    let (reference, null) = if matches!(
        source.nodes()[left].source(),
        ExpressionNodeSourceV1::Literal(ResolvedLiteralSourceV1::Null)
    ) {
        (right, left)
    } else {
        (left, right)
    };
    let loan = Rc::new(CheckedExpressionOperationLoanV1 {
        plan: Rc::clone(&source),
        node: 0,
        proof: CheckedExpressionOperationProofV1::NullableAliasNull {
            reference,
            null,
            binding,
            root,
        },
    });
    loan.nullable_alias_null_v1()?;
    let order = source.ordered_postorder_v1().ok()?;
    let mut classes = vec![Leaf::Unresolved; source.nodes().len()];
    classes[0] = Leaf::Value(Class::Bool);
    classes[reference] = Leaf::Value(Class::Opaque);
    classes[null] = Leaf::Value(Class::Null);
    if order.iter().any(|node| classes[*node] == Leaf::Unresolved) {
        return None;
    }
    let evaluation = CheckedExpressionEvaluationV1 {
        source,
        prior_homes: homes.into(),
        order: order.into(),
        classes: classes.into(),
        calls: Box::new([]),
        cores: BTreeMap::new(),
        references: BTreeMap::from([(
            reference,
            CheckedExpressionReferenceV1::Alias(Rc::clone(&loan)),
        )]),
        operations: vec![loan].into(),
        owned_results: Box::new([]),
        transferred_inputs: Default::default(),
    };
    initializer_plan_v1(evaluation, None)
}
