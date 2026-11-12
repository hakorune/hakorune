//! One query-local fold of the original Text branch witness DAG.
use super::*;
use crate::mir::resolved_semantics::source_value_flow::{
    with_completed_source_node_v1, SourceBranchIdentityV1,
};
#[derive(Clone, Copy)]
struct TextBranchSummaryV1 {
    owned: bool,
    header_owned: bool,
    contains_header: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLoopTextProducerV1 {
    Static {
        binding: BindingRefV1,
        definition: SourceExprSiteV1,
        utf8: Box<str>,
    },
    Residence(TextResidenceAcquisitionV1),
    Header(Rc<LoopHeaderValueBasisV1>),
    After {
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        binding: BindingRefV1,
    },
    BranchNormal(Rc<BranchNormalTextSourceV1>),
}

/// Exact, ordered normal inputs of one source Text branch join. This carries
/// source ownership only; it does not choose an arm or issue a physical value.
#[derive(Debug, Clone)]
pub(crate) struct BranchNormalTextSourceV1 {
    pub(super) flow: Arc<VerifiedSourceValueFlowV1>,
    pub(super) value: SourceValueRefV1,
    pub(super) binding: BindingRefV1,
    pub(super) site: SourceStmtSiteV1,
    pub(super) incoming: Box<[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)]>,
}
impl PartialEq for BranchNormalTextSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow)
            && self.value == other.value
            && self.binding == other.binding
            && self.site == other.site
    }
}
impl Eq for BranchNormalTextSourceV1 {}
impl BranchNormalTextSourceV1 {
    pub(crate) fn new(
        flow: Arc<VerifiedSourceValueFlowV1>,
        value: SourceValueRefV1,
        binding: BindingRefV1,
        site: SourceStmtSiteV1,
        incoming: Box<[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)]>,
    ) -> Option<Rc<Self>> {
        let source = Rc::new(Self {
            flow,
            value,
            binding,
            site,
            incoming,
        });
        source.corroborates_original().then_some(source)
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn value(&self) -> SourceValueRefV1 {
        self.value
    }
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(crate) fn matches_source(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        binding: BindingRefV1,
        value: SourceValueRefV1,
    ) -> bool {
        Arc::ptr_eq(&self.flow, flow)
            && self.binding == binding
            && self.value == value
            && self.corroborates_original()
    }
    pub(crate) fn incoming(&self) -> &[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)] {
        &self.incoming
    }
    pub(crate) fn identity_v1(&self) -> SourceBranchIdentityV1 {
        (
            Arc::as_ptr(&self.flow) as usize,
            self.value,
            self.binding,
            self.site.clone(),
        )
    }
    pub(crate) fn is_owned_residence_tree_v1(&self) -> bool {
        self.summary(None).is_ok_and(|summary| summary.owned)
    }
    pub(crate) fn is_loop_header_or_owned_residence_tree_v1(
        &self,
        basis: &Rc<LoopHeaderValueBasisV1>,
    ) -> bool {
        self.summary(Some(basis))
            .is_ok_and(|summary| summary.header_owned)
    }
    pub(crate) fn corroborates_original(&self) -> bool {
        self.summary(None).is_ok()
    }
    fn summary(
        &self,
        basis: Option<&Rc<LoopHeaderValueBasisV1>>,
    ) -> Result<TextBranchSummaryV1, ()> {
        self.summarize(basis, &mut BTreeSet::new(), &mut BTreeMap::new())
    }
    fn summarize(
        &self,
        basis: Option<&Rc<LoopHeaderValueBasisV1>>,
        active: &mut BTreeSet<(SourceBranchIdentityV1, usize)>,
        completed: &mut BTreeMap<(SourceBranchIdentityV1, usize), TextBranchSummaryV1>,
    ) -> Result<TextBranchSummaryV1, ()> {
        with_completed_source_node_v1(
            (self.identity_v1(), self as *const _ as usize),
            active,
            completed,
            || (),
            |active, completed| {
                if !self.flow.is_complete() || self.flow.owner() != self.binding.owner() {
                    return Err(());
                }
                let Some(SourceValueOriginV1::Join {
                    binding,
                    site,
                    kind: Join::BranchNormal,
                    incoming,
                }) = self.flow.origin(self.value)
                else {
                    return Err(());
                };
                if *binding != self.binding
                    || site.node() != self.site.node()
                    || incoming.len() != 2
                    || self.incoming.len() != 2
                    || !matches!(
                        (&incoming[0].0, &incoming[1].0),
                        (Edge::ThenNormal, Edge::ElseNormal)
                    )
                {
                    return Err(());
                }
                let mut summary = TextBranchSummaryV1 {
                    owned: true,
                    header_owned: basis.is_some_and(|basis| self.binding == basis.binding),
                    contains_header: false,
                };
                for ((edge, value), (actual_edge, actual_value, producer)) in
                    incoming.iter().zip(self.incoming.iter())
                {
                    if edge != actual_edge
                        || value != actual_value
                        || !Self::producer_matches(&self.flow, self.binding, *value, producer)
                    {
                        return Err(());
                    }
                    let child = match producer {
                        SourceLoopTextProducerV1::BranchNormal(source) => {
                            source.summarize(basis, active, completed)?
                        }
                        SourceLoopTextProducerV1::Residence(acquisition) => {
                            let owned = acquisition.is_text_residence() && acquisition.owns_root();
                            TextBranchSummaryV1 {
                                owned,
                                header_owned: owned
                                    && basis.is_some_and(|basis| {
                                        acquisition.binding() == basis.binding
                                    }),
                                contains_header: false,
                            }
                        }
                        SourceLoopTextProducerV1::Header(actual) => {
                            let matches = basis.is_some_and(|basis| Rc::ptr_eq(actual, basis));
                            TextBranchSummaryV1 {
                                owned: false,
                                header_owned: matches,
                                contains_header: matches,
                            }
                        }
                        _ => TextBranchSummaryV1 {
                            owned: false,
                            header_owned: false,
                            contains_header: false,
                        },
                    };
                    summary.owned &= child.owned;
                    summary.header_owned &= child.header_owned;
                    summary.contains_header |= child.contains_header;
                }
                // Every child independently owes Header presence, as before.
                summary.header_owned &= summary.contains_header;
                Ok(summary)
            },
        )
    }
    fn producer_matches(
        flow: &Arc<VerifiedSourceValueFlowV1>,
        binding: BindingRefV1,
        value: SourceValueRefV1,
        producer: &SourceLoopTextProducerV1,
    ) -> bool {
        match producer {
            SourceLoopTextProducerV1::Header(basis) => {
                Arc::ptr_eq(&basis.flow, flow) && basis.binding == binding
                    && basis.header == value
                    && basis.entry_producer().class()
                        == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String
                    && flow.loop_join(basis.source.site(), binding, Join::LoopHeader)
                        .is_some_and(|(header, _)| header == value)
            }
            SourceLoopTextProducerV1::Static { binding: source_binding, definition, .. } => {
                *source_binding == binding
                    && matches!(flow.origin(value), Some(SourceValueOriginV1::Expression(site)) if site == definition)
            }
            SourceLoopTextProducerV1::Residence(acquisition) => {
                if acquisition.binding() != binding || !acquisition.is_text_residence()
                    || acquisition.source().owner() != flow.owner() { return false; }
                let Some(definition) = acquisition.source().nodes().first().map(|node| node.site()) else { return false; };
                matches!(flow.origin(value), Some(SourceValueOriginV1::Expression(site)) if site == definition)
            }
            SourceLoopTextProducerV1::BranchNormal(source) => {
                Arc::ptr_eq(&source.flow, flow) && source.binding == binding
                    && source.value == value
            }
            _ => false,
        }
    }
}
