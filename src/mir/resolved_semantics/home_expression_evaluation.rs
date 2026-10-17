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
