//! Checked body of one original expression, borrowed by its purpose wrapper.
//! Acquisitions are source templates; only the physical Normal edge owns a value.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
use std::collections::BTreeSet;

/// Templates for source-node acquisitions, not already acquired runtime owners.
/// Only the matching checked operation Normal may acquire an owned String.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedExpressionEvaluationV1 {
    pub(super) source: Rc<ExpressionSourcePlanV1>,
    pub(super) prior_homes: Box<[crate::mir::resolved_semantics::BindingRefV1]>,
    pub(super) order: Box<[usize]>,
    pub(super) classes: Box<[ExpressionLeafClassV1]>,
    pub(super) calls: Box<[LocalCallObservationV1]>,
    pub(super) cores: std::collections::BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>,
    pub(super) references: std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
    pub(super) operations: Box<[Rc<CheckedExpressionOperationLoanV1>]>,
    pub(super) owned_results: Box<[usize]>,
    pub(super) transferred_inputs: BTreeSet<usize>,
}
impl CheckedExpressionEvaluationV1 {
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> { &self.source }
    pub(crate) fn prior_homes(&self) -> &[crate::mir::resolved_semantics::BindingRefV1] { &self.prior_homes }
    pub(crate) fn ordered_nodes(&self) -> &[usize] { &self.order }
    pub(crate) fn calls(&self) -> &[LocalCallObservationV1] { &self.calls }
    pub(crate) fn cores(&self) -> &std::collections::BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1> { &self.cores }
    pub(crate) fn references(&self) -> &std::collections::BTreeMap<usize, CheckedExpressionReferenceV1> { &self.references }
    /// Every active value reference has exactly one retained original proof.
    /// Receiver identities and transferred ordinary-call inputs keep their own
    /// owners; neither becomes a reference permission in this evaluation.
    pub(crate) fn reference_coverage_is_exact(&self) -> bool {
        let expected: BTreeSet<_> = self.order.iter().copied().filter(|&node|
            !self.transfers_node(node) && !self.node_is_identity(node)
                && matches!(self.source.nodes()[node].source(), ExpressionNodeSourceV1::Reference(_))).collect();
        expected.len() == self.references.len() && self.references.iter().all(|(&node, proof)|
            expected.contains(&node) && proof.matches_node(&self.source, node))
    }
    pub(crate) fn operations(&self) -> &[Rc<CheckedExpressionOperationLoanV1>] { &self.operations }
    pub(crate) fn owned_result_nodes(&self) -> &[usize] { &self.owned_results }
    pub(crate) fn class(&self, node: usize) -> Option<ExpressionLeafClassV1> { self.classes.get(node).copied() }
    pub(crate) fn root_class(&self) -> ExpressionLeafClassV1 { self.classes[0] }
    pub(crate) fn transfers_node(&self, node: usize) -> bool { self.transferred_inputs.contains(&node) }
    pub(crate) fn node_is_identity(&self, node: usize) -> bool {
        self.classes.get(node) == Some(&ExpressionLeafClassV1::Identity)
    }
}



pub(super) fn owned_result_nodes(
    order: &[usize], add: &[usize],
    cores: &std::collections::BTreeMap<usize, crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>,
) -> Box<[usize]> {
    order.iter().copied().filter(|node| add.contains(node) || cores.get(node).is_some_and(|core| core.owns_result())).collect()
}

/// Existing purpose proofs live once, at the exact active source node. These
/// obligations are not physical operands; entry/residence fulfillment is later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedExpressionReferenceV1 {
    StaticText(super::super::StaticTextReadV1),
    Residence(super::initializer_expression::TextResidenceReadV1),
    ArgumentEntry(crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1),
    InitializerEntry {
        read: crate::mir::normal_callable_semantic_package::InitializerTextEntryReadV1,
        exclusion: Option<Rc<super::super::EntryNullExclusionV1>>,
    },
    Scalar(CheckedScalarReferenceV1),
}
impl CheckedExpressionReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self {
            Self::StaticText(read) => read.binding(),
            Self::Residence(read) => read.binding(),
            Self::ArgumentEntry(read) => read.binding(),
            Self::InitializerEntry { read, .. } => read.requirement().binding(),
            Self::Scalar(read) => read.binding(),
        }
    }
    pub(crate) fn matches_node(&self, source: &Rc<ExpressionSourcePlanV1>, node: usize) -> bool {
        let Some(original) = source.nodes().get(node) else { return false };
        if self.binding().owner() != source.owner() || !matches!(original.source(),
            ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if *binding == self.binding()) { return false; }
        match self {
            Self::StaticText(read) => read.read() == original.site(),
            Self::Residence(read) => read.node() == node && read.root() == read.acquisition().binding()
                && read.acquisition().source().owner() == source.owner() && read.acquisition().is_text_residence(),
            Self::ArgumentEntry(read) => Rc::ptr_eq(read.source_plan(), source) && read.node() == node && read.read() == original.site(),
            Self::InitializerEntry { read, exclusion } => Rc::ptr_eq(read.requirement().source_plan(), source)
                && read.requirement().node() == node && read.requirement().read() == original.site()
                && read.leaf_class(exclusion.as_deref()).is_some(),
            Self::Scalar(read) => read.original_read().is_none_or(|read| read.read() == original.site()),
        }
    }
}

/// Existing class obligations and original current reads share one node slot.
/// Only Original may later be fulfilled as a physical operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedScalarReferenceV1 {
    Obligation { binding: super::super::BindingRefV1, kind: super::super::SourceScalarKind },
    Original(super::super::CurrentScalarReadV1),
}
impl CheckedScalarReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self { Self::Obligation { binding, .. } => *binding, Self::Original(read) => read.binding() }
    }
    pub(crate) fn kind(&self) -> super::super::SourceScalarKind {
        match self { Self::Obligation { kind, .. } => *kind, Self::Original(read) => read.kind() }
    }
    pub(crate) fn original_read(&self) -> Option<&super::super::CurrentScalarReadV1> {
        match self { Self::Original(read) => Some(read), Self::Obligation { .. } => None }
    }
}

/// Seed original references once before leaf projection. Candidate selection is
/// purpose-owned; a missing candidate remains unresolved, while a malformed
/// candidate rejects the whole expression. No runtime owner is acquired here.
pub(super) fn seed_expression_references_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>, order: &[usize],
    candidate: &mut impl FnMut(usize, &crate::mir::resolved_semantics::SourceExprSiteV1)
        -> Result<Option<CheckedExpressionReferenceV1>, ()>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let source = Rc::clone(prepared.source());
    let mut proofs = std::collections::BTreeMap::new();
    // Initial loans are a list: duplicate source reads must reject, even after
    // the first loan has resolved its node and made it inactive.
    for (node, proof) in initial {
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    for &node in order {
        if !prepared.is_active_reference(node) { continue; }
        let original = source.nodes().get(node)?;
        let Some(proof) = candidate(node, original.site()).ok()? else { continue };
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    Some(proofs)
}

fn seed_reference_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    source: &Rc<ExpressionSourcePlanV1>, node: usize, proof: CheckedExpressionReferenceV1,
    proofs: &mut std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
) -> Option<()> {
    use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf};
    if proofs.contains_key(&node) || !proof.matches_node(source, node) { return None; }
    let class = match &proof {
        CheckedExpressionReferenceV1::Scalar(read) => Leaf::Value(match read.kind() {
            super::super::SourceScalarKind::Integer => Class::I64,
            super::super::SourceScalarKind::Bool => Class::Bool,
        }),
        CheckedExpressionReferenceV1::InitializerEntry { read, exclusion } => read.leaf_class(exclusion.as_deref())?,
        CheckedExpressionReferenceV1::Residence(read) => {
            if !read.acquisition().is_text_residence() || !prepared.prior_homes().contains(&read.root()) { return None; }
            Leaf::Value(Class::String)
        },
        CheckedExpressionReferenceV1::StaticText(_) | CheckedExpressionReferenceV1::ArgumentEntry(_) => Leaf::Value(Class::String),
    };
    if !prepared.seed_reference(node, class) { return None; }
    proofs.insert(node, proof);
    Some(())
}

/// Current Home references use the same original proof selection for every
/// participating purpose. Purpose-specific entry loans remain with that issuer.
pub(super) fn seed_current_expression_references_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>, order: &[usize],
    static_reference: &mut impl FnMut(&super::super::SourceExprSiteV1) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(&super::super::SourceExprSiteV1)
        -> Option<(super::super::BindingRefV1, super::super::BindingRefV1, TextResidenceAcquisitionV1)>,
    scalar_reference: &mut impl FnMut(&super::super::SourceExprSiteV1) -> Option<CheckedScalarReferenceV1>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let bool_condition = prepared.is_bool_condition();
    seed_expression_references_v1(prepared, initial, order, &mut |node, site| {
        let proof = if let Some(read) = static_reference(site) {
            CheckedExpressionReferenceV1::StaticText(read)
        } else if let Some((binding, root, acquisition)) = residence_reference(site) {
            CheckedExpressionReferenceV1::Residence(super::initializer_expression::TextResidenceReadV1::new(node, binding, root, acquisition))
        } else if let Some(read) = scalar_reference(site) {
            if read.kind() != super::super::SourceScalarKind::Integer && !bool_condition { return Err(()); }
            CheckedExpressionReferenceV1::Scalar(read)
        } else { return Ok(None) };
        Ok(Some(proof))
    })
}
