//! Retained source-read obligations and seeding of the same checked evaluation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
/// Existing purpose proofs live once, at the exact active source node. These
/// obligations are not physical operands; entry/residence fulfillment is later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedExpressionReferenceV1 {
    NullableResult(NullableResultReferenceV1),
    Alias(Rc<CheckedExpressionOperationLoanV1>),
    /// A local Dynamic result borrowed from its exact initializer call frame.
    /// The frame grants neither a concrete payload type nor ownership transfer.
    DynamicInvocationResult(Rc<super::super::CheckedInitializerExpressionFrameV1>),
    StaticText(super::super::StaticTextReadV1),
    Residence(super::initializer_expression::TextResidenceReadV1),
    LoopText(super::super::LoopTextReadV1),
    ArgumentEntry(crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1),
    DynamicEntry(crate::mir::normal_callable_semantic_package::DynamicEntryReadV1),
    LoopDynamic(super::super::LoopDynamicReadV1),
    FiniteEntry {
        read: crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
        exclusion: Option<Rc<super::super::SourceNullExclusionV1>>,
    },
    Scalar(CheckedScalarReferenceV1),
}
impl CheckedExpressionReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self {
            Self::Alias(loan) => {
                let CheckedExpressionOperationProofV1::NullableAliasNull { binding, .. } =
                    loan.proof
                else {
                    unreachable!("qualified alias reference")
                };
                binding
            }
            Self::NullableResult(frame) => frame.binding(),
            Self::DynamicInvocationResult(frame) => frame.binding(),
            Self::StaticText(read) => read.binding(),
            Self::Residence(read) => read.binding(),
            Self::LoopText(read) => read.binding(),
            Self::ArgumentEntry(read) => read.binding(),
            Self::DynamicEntry(read) => read.requirement().binding(),
            Self::LoopDynamic(read) => read.binding(),
            Self::FiniteEntry { read, .. } => read.requirement().binding(),
            Self::Scalar(read) => read.binding(),
        }
    }
    pub(crate) fn matches_node(&self, source: &Rc<ExpressionSourcePlanV1>, node: usize) -> bool {
        let Some(original) = source.nodes().get(node) else {
            return false;
        };
        if self.binding().owner() != source.owner()
            || !matches!(original.source(),
            ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if *binding == self.binding())
        {
            return false;
        }
        match self {
            Self::Alias(loan) => {
                Rc::ptr_eq(loan.source(), source)
                    && loan
                        .nullable_alias_null_v1()
                        .is_some_and(|(reference, _)| reference == node)
            }
            Self::NullableResult(frame) => {
                frame.site().owner() == source.owner() && frame.matches_source()
            }
            Self::DynamicInvocationResult(frame) => {
                frame.is_dynamic_value()
                    && frame.source().owner() == source.owner()
                    && frame.binding() == self.binding()
            }
            Self::StaticText(read) => read.read() == original.site(),
            Self::Residence(read) => {
                read.node() == node
                    && read.root() == read.acquisition().binding()
                    && read.acquisition().source().owner() == source.owner()
                    && read.acquisition().is_text_residence()
            }
            Self::LoopText(read) => read.matches_source_node(source, node),
            Self::ArgumentEntry(read) => {
                Rc::ptr_eq(read.source_plan(), source)
                    && read.node() == node
                    && read.read() == original.site()
            }
            Self::DynamicEntry(read) => {
                Rc::ptr_eq(read.requirement().source_plan(), source)
                    && read.requirement().node() == node
                    && read.requirement().read() == original.site()
                    && read.corroborates_original_read_v1()
            }
            Self::LoopDynamic(read) => read.matches_source_node(source, node),
            Self::FiniteEntry { read, exclusion } => {
                Rc::ptr_eq(read.requirement().source_plan(), source)
                    && read.requirement().node() == node
                    && read.requirement().read() == original.site()
                    && read.leaf_class(exclusion.as_deref()).is_some()
            }
            Self::Scalar(read) => read
                .original_read()
                .is_none_or(|read| read.read() == original.site()),
        }
    }
}

/// Existing class obligations and original current reads share one node slot.
/// Only Original may later be fulfilled as a physical operand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CheckedScalarReferenceV1 {
    Obligation {
        binding: super::super::BindingRefV1,
        kind: super::super::SourceScalarKind,
    },
    Original(super::super::CurrentScalarReadV1),
}
impl CheckedScalarReferenceV1 {
    pub(crate) fn binding(&self) -> super::super::BindingRefV1 {
        match self {
            Self::Obligation { binding, .. } => *binding,
            Self::Original(read) => read.binding(),
        }
    }
    pub(crate) fn kind(&self) -> super::super::SourceScalarKind {
        match self {
            Self::Obligation { kind, .. } => *kind,
            Self::Original(read) => read.kind(),
        }
    }
    pub(crate) fn original_read(&self) -> Option<&super::super::CurrentScalarReadV1> {
        match self {
            Self::Original(read) => Some(read),
            Self::Obligation { .. } => None,
        }
    }
}

/// Seed original references once before leaf projection. Candidate selection is
/// purpose-owned; a missing candidate remains unresolved, while a malformed
/// candidate rejects the whole expression. No runtime owner is acquired here.
pub(super) fn seed_expression_references_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    order: &[usize],
    candidate: &mut impl FnMut(
        usize,
        &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<Option<CheckedExpressionReferenceV1>, ()>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let source = Rc::clone(prepared.source());
    let mut proofs = std::collections::BTreeMap::new();
    // Initial loans are a list: duplicate source reads must reject, even after
    // the first loan has resolved its node and made it inactive.
    for (node, proof) in initial {
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    for &node in order {
        if !prepared.is_active_reference(node) {
            continue;
        }
        let original = source.nodes().get(node)?;
        let Some(proof) = candidate(node, original.site()).ok()? else {
            continue;
        };
        seed_reference_v1(prepared, &source, node, proof, &mut proofs)?;
    }
    Some(proofs)
}

fn seed_reference_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    source: &Rc<ExpressionSourcePlanV1>,
    node: usize,
    proof: CheckedExpressionReferenceV1,
    proofs: &mut std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>,
) -> Option<()> {
    use crate::mir::resolved_value_profile::{
        expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf,
    };
    if proofs.contains_key(&node) || !proof.matches_node(source, node) {
        return None;
    }
    let class = match &proof {
        CheckedExpressionReferenceV1::Alias(_) => return None,
        CheckedExpressionReferenceV1::NullableResult(frame) => {
            if !prepared.accepts_nullable_reference_v1(frame)
                || !frame.accepts_prior_homes(prepared.prior_homes())
            {
                return None;
            }
            Leaf::Value(
                if matches!(frame, NullableResultReferenceV1::DeclaredFrom(_)) {
                    Class::BoxReference
                } else {
                    Class::Opaque
                },
            )
        }
        CheckedExpressionReferenceV1::DynamicInvocationResult(frame) => {
            if !frame.is_dynamic_value()
                || frame.dynamic_home_binding_v1().is_none_or(|owner| {
                    !prepared.prior_homes().contains(&LiveHomeV1::Stable(owner))
                })
            {
                return None;
            }
            Leaf::Value(Class::Dynamic)
        }
        CheckedExpressionReferenceV1::Scalar(read) => Leaf::Value(match read.kind() {
            super::super::SourceScalarKind::Integer => Class::I64,
            super::super::SourceScalarKind::Bool => Class::Bool,
        }),
        CheckedExpressionReferenceV1::FiniteEntry { read, exclusion } => {
            read.leaf_class(exclusion.as_deref())?
        }
        CheckedExpressionReferenceV1::Residence(read) => {
            if !read.acquisition().is_text_residence()
                || !prepared
                    .prior_homes()
                    .contains(&LiveHomeV1::Text(read.acquisition().clone()))
            {
                return None;
            }
            Leaf::Value(Class::String)
        }
        CheckedExpressionReferenceV1::DynamicEntry(read) => read.leaf_class(),
        CheckedExpressionReferenceV1::LoopDynamic(_) => Leaf::Value(Class::Dynamic),
        CheckedExpressionReferenceV1::StaticText(_)
        | CheckedExpressionReferenceV1::ArgumentEntry(_)
        | CheckedExpressionReferenceV1::LoopText(_) => Leaf::Value(Class::String),
    };
    if !prepared.seed_reference(node, class) {
        return None;
    }
    proofs.insert(node, proof);
    Some(())
}

/// Current Home references use the same original proof selection for every
/// participating purpose. Purpose-specific entry loans remain with that issuer.
pub(super) fn seed_current_expression_references_with_loop_text_v1(
    prepared: &mut super::leaves::PreparedExpressionCallsV1,
    initial: Vec<(usize, CheckedExpressionReferenceV1)>,
    order: &[usize],
    static_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<super::super::StaticTextReadV1>,
    residence_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<(
        super::super::BindingRefV1,
        super::super::BindingRefV1,
        TextResidenceAcquisitionV1,
    )>,
    loop_text_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<super::super::LoopTextReadV1>,
    scalar_reference: &mut impl FnMut(
        &super::super::SourceExprSiteV1,
    ) -> Option<CheckedScalarReferenceV1>,
) -> Option<std::collections::BTreeMap<usize, CheckedExpressionReferenceV1>> {
    let bool_condition = prepared.is_bool_condition();
    seed_expression_references_v1(prepared, initial, order, &mut |node, site| {
        let proof = if let Some(read) = loop_text_reference(site) {
            CheckedExpressionReferenceV1::LoopText(read)
        } else if let Some(read) = static_reference(site) {
            CheckedExpressionReferenceV1::StaticText(read)
        } else if let Some((binding, root, acquisition)) = residence_reference(site) {
            CheckedExpressionReferenceV1::Residence(
                super::initializer_expression::TextResidenceReadV1::new(
                    node,
                    binding,
                    root,
                    acquisition,
                ),
            )
        } else if let Some(read) = scalar_reference(site) {
            // An exact Bool Local is a first-class operand (Text display, Bool
            // equality); no other non-Integer scalar exists.
            if !matches!(
                read.kind(),
                super::super::SourceScalarKind::Integer | super::super::SourceScalarKind::Bool
            ) && !bool_condition
            {
                return Err(());
            }
            CheckedExpressionReferenceV1::Scalar(read)
        } else {
            return Ok(None);
        };
        Ok(Some(proof))
    })
}
