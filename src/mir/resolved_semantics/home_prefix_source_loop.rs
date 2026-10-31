//! Scoped inductive Header basis. This is source analysis, never a completed
//! Loop, Home After or physical Phi permission. Body frames retain only the
//! immutable basis, never a mutable exit collector or expression inventory.
use super::*;
use crate::mir::builder::{CallableLoopSourceBindingClassV1 as Role, RetainedCallableLoopSourceV1};
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::source_value_flow::{
    SourceValueEdgeV1 as Edge, SourceValueJoinKindV1 as Join, SourceValueOriginV1,
    SourceValueRefV1, VerifiedSourceValueFlowV1,
};
use std::{rc::Rc, sync::Arc};
#[path = "home_prefix_source_loop_control.rs"]
mod control;
#[path = "home_prefix_source_loop_ports.rs"]
mod ports;
pub(crate) use control::{
    CheckedSourceLoopBodyV1, CheckedSourceLoopStatementV1, SourceLoopReturnV1,
};
pub(crate) use ports::{SourceLoopPortKindV1, SourceLoopScanSummaryV1};
#[path = "home_prefix_source_loop_dynamic.rs"]
mod dynamic;
pub(crate) use dynamic::{LoopDynamicReadV1, SourceLoopDynamicProducerV1};
#[path = "home_prefix_source_loop_after.rs"]
mod after;

/// The original value producer is class-separated before any Recipe or Phi.
/// A Text branch can never lend an Integer read or ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceLoopValueProducerV1 {
    Scalar(CurrentScalarProducerV1),
    Text(SourceLoopTextProducerV1),
    Dynamic(SourceLoopDynamicProducerV1),
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
    flow: Arc<VerifiedSourceValueFlowV1>,
    value: SourceValueRefV1,
    binding: BindingRefV1,
    site: SourceStmtSiteV1,
    incoming: Box<[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)]>,
}
impl PartialEq for BranchNormalTextSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow) && self.value == other.value
            && self.binding == other.binding && self.site == other.site
    }
}
impl Eq for BranchNormalTextSourceV1 {}
impl BranchNormalTextSourceV1 {
    pub(crate) fn new(
        flow: Arc<VerifiedSourceValueFlowV1>, value: SourceValueRefV1,
        binding: BindingRefV1, site: SourceStmtSiteV1,
        incoming: Box<[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)]>,
    ) -> Option<Rc<Self>> {
        let source = Rc::new(Self { flow, value, binding, site, incoming });
        source.corroborates_original().then_some(source)
    }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn value(&self) -> SourceValueRefV1 { self.value }
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 { &self.site }
    pub(crate) fn matches_source(
        &self, flow: &Arc<VerifiedSourceValueFlowV1>, binding: BindingRefV1,
        value: SourceValueRefV1,
    ) -> bool {
        Arc::ptr_eq(&self.flow, flow) && self.binding == binding && self.value == value
            && self.corroborates_original()
    }
    pub(crate) fn incoming(&self) -> &[(Edge, SourceValueRefV1, SourceLoopTextProducerV1)] {
        &self.incoming
    }
    pub(crate) fn is_owned_residence_tree_v1(&self) -> bool {
        self.corroborates_original() && self.incoming.iter().all(|(_, _, producer)| {
            match producer {
                SourceLoopTextProducerV1::Residence(acquisition) =>
                    acquisition.is_text_residence() && acquisition.owns_root(),
                SourceLoopTextProducerV1::BranchNormal(source) => source.is_owned_residence_tree_v1(),
                _ => false,
            }
        })
    }
    pub(crate) fn is_loop_header_or_owned_residence_tree_v1(
        &self,
        basis: &Rc<LoopHeaderValueBasisV1>,
    ) -> bool {
        self.binding == basis.binding
            && self.corroborates_original()
            && self.incoming.iter().all(|(_, _, producer)| match producer {
                SourceLoopTextProducerV1::Header(actual) => Rc::ptr_eq(actual, basis),
                SourceLoopTextProducerV1::Residence(acquisition) => {
                    acquisition.binding() == basis.binding
                        && acquisition.is_text_residence()
                        && acquisition.owns_root()
                }
                SourceLoopTextProducerV1::BranchNormal(source) => {
                    source.is_loop_header_or_owned_residence_tree_v1(basis)
                }
                _ => false,
            })
            && self.contains_loop_header_v1(basis)
    }
    fn contains_loop_header_v1(&self, basis: &Rc<LoopHeaderValueBasisV1>) -> bool {
        self.incoming.iter().any(|(_, _, producer)| match producer {
            SourceLoopTextProducerV1::Header(actual) => Rc::ptr_eq(actual, basis),
            SourceLoopTextProducerV1::BranchNormal(source) => {
                source.contains_loop_header_v1(basis)
            }
            _ => false,
        })
    }
    pub(crate) fn corroborates_original(&self) -> bool {
        self.flow.is_complete() && self.flow.owner() == self.binding.owner()
            && matches!(self.flow.origin(self.value), Some(SourceValueOriginV1::Join {
                binding, site, kind: Join::BranchNormal, incoming
            }) if *binding == self.binding && site.node() == self.site.node()
                && incoming.len() == 2 && self.incoming.len() == 2
                && incoming.iter().zip(self.incoming.iter()).all(|((edge, value), (actual_edge, actual_value, producer))|
                    edge == actual_edge && value == actual_value
                        && Self::producer_matches(&self.flow, self.binding, *value, producer))
                && matches!((&incoming[0].0, &incoming[1].0), (Edge::ThenNormal, Edge::ElseNormal)))
    }
    fn producer_matches(
        flow: &Arc<VerifiedSourceValueFlowV1>, binding: BindingRefV1,
        value: SourceValueRefV1, producer: &SourceLoopTextProducerV1,
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
                    && source.value == value && source.corroborates_original()
            }
            _ => false,
        }
    }
}
impl SourceLoopValueProducerV1 {
    pub(crate) fn scalar(&self) -> Option<&CurrentScalarProducerV1> {
        match self {
            Self::Scalar(value) => Some(value),
            Self::Text(_) | Self::Dynamic(_) => None,
        }
    }
    pub(crate) fn class(
        &self,
    ) -> crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 {
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Class;
        match self {
            Self::Scalar(_) => Class::I64,
            Self::Text(_) => Class::String,
            Self::Dynamic(_) => Class::Dynamic,
        }
    }
}
impl From<CurrentScalarProducerV1> for SourceLoopValueProducerV1 {
    fn from(value: CurrentScalarProducerV1) -> Self {
        Self::Scalar(value)
    }
}

#[derive(Debug)]
pub(crate) struct LoopHeaderValueBasisV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    flow: Arc<VerifiedSourceValueFlowV1>,
    binding: BindingRefV1,
    header: SourceValueRefV1,
    entry: SourceValueRefV1,
    entry_origin: SourceValueRefV1,
    entry_producer: SourceLoopValueProducerV1,
}
/// Exact lexical read of the current Text Header. This is a descriptor read,
/// not a Text residence acquisition: the Header may carry Static or Owned
/// Text depending on the predecessor selected by the same loop product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopTextReadV1 {
    read: SourceExprSiteV1,
    value: SourceValueRefV1,
    producer: SourceLoopTextProducerV1,
}

impl LoopTextReadV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match &self.producer {
            SourceLoopTextProducerV1::Header(basis) => basis.binding(),
            SourceLoopTextProducerV1::After { binding, .. } => *binding,
            SourceLoopTextProducerV1::Static { binding, .. } => *binding,
            SourceLoopTextProducerV1::Residence(acquisition) => acquisition.binding(),
            SourceLoopTextProducerV1::BranchNormal(source) => source.binding(),
        }
    }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 {
        &self.read
    }
    pub(crate) fn value(&self) -> SourceValueRefV1 {
        self.value
    }
    pub(crate) fn producer(&self) -> &SourceLoopTextProducerV1 {
        &self.producer
    }
    pub(crate) fn matches_source_node(
        &self,
        source: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        node: usize,
    ) -> bool {
        let Some(original) = source.nodes().get(node) else {
            return false;
        };
        let binding = self.binding();
        if source.owner() != binding.owner()
            || original.site() != &self.read
            || !matches!(original.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
            ResolvedLexicalRefV1::Local(actual)) if *actual == binding)
        {
            return false;
        }
        match &self.producer {
            SourceLoopTextProducerV1::Header(basis) => {
                source.owner() == basis.binding().owner() && basis.binding() == binding
                    && basis.source.owner() == source.owner() && basis.flow.owner() == source.owner()
                    && basis.flow.read(&self.read, binding) == Some(self.value)
                    && basis.header == self.value
                    && basis.entry_producer().class()
                        == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String
            }
            SourceLoopTextProducerV1::After { product, binding: after_binding } => {
                if *after_binding != binding || product.source.source.owner() != source.owner() { return false; }
                let Some(basis) = product.source.headers.get(&binding) else { return false; };
                if !matches!(basis.entry_producer(), SourceLoopValueProducerV1::Text(_))
                    || basis.flow.read(&self.read, binding) != Some(self.value)
                    || basis.flow.loop_join(product.source.source.site(), binding, Join::LoopAfter)
                        .is_none_or(|(after, _)| after != self.value) {
                    return false;
                }
                product.source.incoming.as_ref().is_ok_and(|incoming| incoming.iter().filter(|row|
                    row.after == self.value && row.basis.binding() == binding && Rc::ptr_eq(&row.basis, basis)).count() == 1)
            }
            SourceLoopTextProducerV1::BranchNormal(source) => {
                source.binding() == binding && source.value() == self.value
                    && source.corroborates_original()
                    && source.flow.read(&self.read, binding) == Some(self.value)
            }
            SourceLoopTextProducerV1::Static { .. } | SourceLoopTextProducerV1::Residence(_) => false,
        }
    }
}
// The immutable, non-Clone basis is minted once per selected source scope.
impl PartialEq for LoopHeaderValueBasisV1 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for LoopHeaderValueBasisV1 {}
impl LoopHeaderValueBasisV1 {
    pub(crate) fn source(&self) -> &Rc<RetainedCallableLoopSourceV1> {
        &self.source
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn header(&self) -> SourceValueRefV1 {
        self.header
    }
    pub(crate) fn entry(&self) -> SourceValueRefV1 {
        self.entry
    }
    pub(crate) fn entry_origin(&self) -> SourceValueRefV1 {
        self.entry_origin
    }
    pub(crate) fn entry_producer(&self) -> &SourceLoopValueProducerV1 {
        &self.entry_producer
    }
}

pub(super) struct SourceLoopHeaderEnvironmentV1 {
    source: Rc<RetainedCallableLoopSourceV1>,
    headers: BTreeMap<BindingRefV1, Rc<LoopHeaderValueBasisV1>>,
    ports: std::cell::RefCell<Vec<ports::SourceLoopPortSnapshotV1>>,
    refused: std::cell::Cell<bool>,
    control: std::cell::RefCell<control::SourceLoopControlDraftV1>,
}
impl SourceLoopHeaderEnvironmentV1 {
    pub(super) fn header_for_value(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        value: SourceValueRefV1,
    ) -> Option<&Rc<LoopHeaderValueBasisV1>> {
        let mut matching = self.headers.values().filter(|basis| {
            basis.header == value
                && Arc::ptr_eq(&basis.flow, flow)
                && Rc::ptr_eq(&basis.source, &self.source)
        });
        let basis = matching.next()?;
        matching.next().is_none().then_some(basis)
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) struct SourceLoopHeaderCheckpointV1 {
    active: Rc<SourceLoopHeaderEnvironmentV1>,
    previous: Option<Rc<SourceLoopHeaderEnvironmentV1>>,
}

pub(super) fn same_context(
    a: &Option<Rc<SourceLoopHeaderEnvironmentV1>>,
    b: &Option<Rc<SourceLoopHeaderEnvironmentV1>>,
) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn source_loop_header_basis_v1(
        &self,
        binding: BindingRefV1,
    ) -> Option<&Rc<LoopHeaderValueBasisV1>> {
        self.source_loop.as_ref()?.headers.get(&binding)
    }

    /// Borrow the exact Text value at this active Loop Header. The header
    /// basis must come from the original value-flow join for this read.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn loop_header_text_reference_v1(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<LoopTextReadV1> {
        let function = self.input.function();
        let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(site)? else {
            return None;
        };
        let flow = function.expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let active = self.source_loop.as_ref()?;
        let prefix = active.source.site().node().segments();
        let segments = site.node().segments();
        if !segments.starts_with(prefix)
            || !matches!(
                segments.get(prefix.len()),
                Some(crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_))
            )
        {
            return None;
        }
        let basis = active.header_for_value(flow, value)?;
        if basis.binding() != binding
            || basis.entry_producer().class()
                != crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String {
            return None;
        }
        Some(LoopTextReadV1 {
            read: site.clone(),
            value,
            producer: SourceLoopTextProducerV1::Header(Rc::clone(basis)),
        })
    }

    /// Read a Text value at its exact source-loop point. Active loop-body
    /// reads use Header; source-continuation reads use the same product's After.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn loop_text_reference_v1(
        &self,
        site: &SourceExprSiteV1,
        homes: &[LiveHomeV1],
    ) -> Option<LoopTextReadV1> {
        if let Some(read) = self.loop_header_text_reference_v1(site) {
            return Some(read);
        }
        let function = self.input.function();
        let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(site)? else {
            return None;
        };
        let flow = function.expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let Some(active) = self.source_loop.as_ref() else {
            return self.loop_after_terminal_return_text_reference_v1(
                site, binding, value, flow, homes,
            );
        };
        let prefix = active.source.site().node().segments();
        let segments = site.node().segments();
        if !segments.starts_with(prefix)
            || !matches!(
                segments.get(prefix.len()),
                Some(crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_))
            )
        {
            return None;
        }
        if homes.iter().any(|home| home.binding() == binding) {
            return None;
        }
        if let Some(StoredLocal::TextBranch { source }) = self.locals.get(&binding) {
            if source.matches_source(flow, binding, value) {
                return Some(LoopTextReadV1 {
                    read: site.clone(),
                    value,
                    producer: SourceLoopTextProducerV1::BranchNormal(Rc::clone(source)),
                });
            }
        }
        let StoredLocal::LoopText { product } = self.locals.get(&binding)? else {
            return None;
        };
        let (_, producer) =
            self.source_loop_after_text_for_value_v1(binding, value, flow, product)?;
        let SourceLoopTextProducerV1::After {
            product: exact,
            binding: exact_binding,
        } = producer
        else {
            return None;
        };
        Some(LoopTextReadV1 {
            read: site.clone(),
            value,
            producer: SourceLoopTextProducerV1::After {
                product: exact,
                binding: exact_binding,
            },
        })
    }

    /// Resolve one exact Text incoming from the current source path. Static
    /// bytes are admitted only through the original definition in this flow;
    /// an owned value retains its existing acquisition and Home.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn text_producer_for_value_v1(
        &self,
        binding: BindingRefV1,
        value: SourceValueRefV1,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        homes: &[LiveHomeV1],
    ) -> Option<(SourceValueRefV1, SourceLoopTextProducerV1)> {
        if binding.owner() != self.input.owner() || flow.owner() != binding.owner() {
            return None;
        }
        let function = self.input.function();
        match self.locals.get(&binding)? {
            StoredLocal::StaticText { definition } => {
                if homes.iter().any(|home| home.binding() == binding) {
                    return None;
                }
                if let Some(basis) = self.source_loop_header_basis_v1(binding)
                    .filter(|basis| Arc::ptr_eq(&basis.flow, flow)
                        && basis.entry_producer().class()
                            == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)
                {
                    let (header, _) = flow.loop_join(basis.source.site(), binding, Join::LoopHeader)?;
                    if basis.header == value && header == value {
                        return Some((value, SourceLoopTextProducerV1::Header(Rc::clone(basis))));
                    }
                }
                let origin = flow.alias_origin_with_expression_boundary(
                    value,
                    |site| function.variable_ref(site),
                    |site| site == definition,
                )?;
                if !matches!(flow.origin(origin)?, SourceValueOriginV1::Expression(site) if site == definition)
                {
                    return None;
                }
                let ResolvedLiteralSourceV1::String(utf8) =
                    function.expression_source().literal(definition)?
                else {
                    return None;
                };
                let mut definitions = function.expression_source().initializers().filter(|row| {
                    row.binding() == binding && row.initializer_site() == Some(definition)
                });
                let initializer = definitions.next()?;
                if definitions.next().is_some() || initializer.binding().owner() != flow.owner() {
                    return None;
                }
                Some((
                    origin,
                    SourceLoopTextProducerV1::Static {
                        binding,
                        definition: definition.clone(),
                        utf8: utf8.clone(),
                    },
                ))
            }
            StoredLocal::Text { root, acquisition } => {
                let acquisition_home = homes
                    .iter()
                    .filter(|home| **home == LiveHomeV1::Text(acquisition.clone()))
                    .count()
                    == 1;
                let transferred_carrier = self.source_loop.as_ref().and_then(|active| active.headers.get(&binding))
                    .is_some_and(|basis| basis.entry_producer().class()
                        == crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)
                    && !homes.iter().any(|home| home.binding() == binding);
                if *root != binding
                    || acquisition.binding() != binding
                    || !acquisition.is_text_residence()
                    || (!acquisition_home && !transferred_carrier)
                {
                    return None;
                }
                let definition = acquisition.source().nodes().first()?.site();
                if acquisition.source().owner() != flow.owner() {
                    return None;
                }
                let origin = flow.alias_origin_with_expression_boundary(
                    value,
                    |site| function.variable_ref(site),
                    |site| site == definition,
                )?;
                if !matches!(flow.origin(origin)?, SourceValueOriginV1::Expression(site) if site == definition)
                {
                    return None;
                }
                Some((
                    origin,
                    SourceLoopTextProducerV1::Residence(acquisition.clone()),
                ))
            }
            StoredLocal::LoopText { product } => {
                if homes.iter().any(|home| home.binding() == binding) {
                    return None;
                }
                let (origin, producer) =
                    self.source_loop_after_text_for_value_v1(binding, value, flow, product)?;
                Some((origin, producer))
            }
            StoredLocal::TextBranch { source }
                if !homes.iter().any(|home| home.binding() == binding)
                    && source.matches_source(flow, binding, value) =>
            {
                Some((value, SourceLoopTextProducerV1::BranchNormal(Rc::clone(source))))
            }
            _ => None,
        }
    }

    fn source_loop_after_text_for_value_v1(
        &self,
        binding: BindingRefV1,
        value: SourceValueRefV1,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) -> Option<(SourceValueRefV1, SourceLoopTextProducerV1)> {
        let source = &product.source;
        let basis = source.headers.get(&binding)?;
        if binding.owner() != self.input.owner()
            || flow.owner() != binding.owner()
            || !Arc::ptr_eq(flow, &basis.flow)
            || !Rc::ptr_eq(&basis.source, &source.source)
            || basis.entry_producer().class()
                != crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String
        { return None; }
        let (after, _) = flow.loop_join(source.source.site(), binding, Join::LoopAfter)?;
        if after != value {
            return None;
        }
        let incoming = source.incoming.as_ref().ok()?;
        let mut rows = incoming.iter().filter(|row| {
            row.after == after && row.basis.binding() == binding && Rc::ptr_eq(&row.basis, basis)
        });
        rows.next()?;
        rows.next().is_none().then(|| {
            (
                after,
                SourceLoopTextProducerV1::After {
                    product: Rc::clone(product),
                    binding,
                },
            )
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn stage_source_loop_headers_v1(
        &mut self,
        source: Rc<RetainedCallableLoopSourceV1>,
        homes: &[LiveHomeV1],
        calls: &[super::super::LocalCallObservationV1],
    ) -> Option<SourceLoopHeaderCheckpointV1> {
        source.condition_source_v1(self.input)?;
        if source.bindings().owner() != self.input.owner()
            || source.bindings().loop_site() != source.site().node()
        {
            return None;
        }
        let flow = self.input.function().expression_source().value_flow()?;
        let mut headers = BTreeMap::new();
        for (binding, role) in source.bindings().bindings() {
            if role != Role::Carrier {
                continue;
            }
            let (header, incoming) = flow.loop_join(source.site(), binding, Join::LoopHeader)?;
            let mut entries = incoming
                .iter()
                .filter_map(|(edge, value)| (*edge == Edge::LoopEntry).then_some(*value));
            let entry = entries.next()?;
            if entries.next().is_some()
                || incoming.iter().any(|(edge, _)| {
                    !matches!(edge, Edge::LoopEntry | Edge::Backedge | Edge::Continue(_))
                })
            {
                return None;
            }
            let (entry_origin, entry_producer) = match self.locals.get(&binding)? {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
                    if self.can_rebind_integer_v1(binding, homes) =>
                {
                    let (origin, producer) =
                        self.scalar_producer_for_value_v1(binding, entry, calls)?;
                    (origin, SourceLoopValueProducerV1::Scalar(producer))
                }
                StoredLocal::StaticText { .. }
                | StoredLocal::Text { .. }
                | StoredLocal::LoopText { .. } => {
                    let (origin, producer) =
                        self.text_producer_for_value_v1(binding, entry, flow, homes)?;
                    (origin, SourceLoopValueProducerV1::Text(producer))
                }
                StoredLocal::TaggedDynamic | StoredLocal::DynamicInvocationResult { .. } => {
                    let (origin, producer) = self.dynamic_producer_for_value_v1(binding, entry, flow)?;
                    (origin, SourceLoopValueProducerV1::Dynamic(producer))
                }
                _ => return None,
            };
            let basis = Rc::new(LoopHeaderValueBasisV1 {
                source: Rc::clone(&source),
                flow: Arc::clone(flow),
                binding,
                header,
                entry,
                entry_origin,
                entry_producer,
            });
            if headers.insert(binding, basis).is_some() {
                return None;
            }
        }
        if headers.is_empty() {
            return None;
        }
        let active = Rc::new(SourceLoopHeaderEnvironmentV1 {
            source,
            headers,
            ports: Default::default(),
            refused: Default::default(),
            control: Default::default(),
        });
        let previous = self.source_loop.replace(Rc::clone(&active));
        Some(SourceLoopHeaderCheckpointV1 { active, previous })
    }

    /// Restore outer assumptions. The summary retains no expression inventory
    /// and grants no After; every source port still needs whole Recipe sealing.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn finish_source_loop_headers_v1(
        &mut self,
        checkpoint: SourceLoopHeaderCheckpointV1,
        condition: Rc<super::super::CheckedBoolConditionFrameV1>,
        calls: &[super::super::LocalCallObservationV1],
    ) -> Option<SourceLoopScanSummaryV1> {
        if !self
            .source_loop
            .as_ref()
            .is_some_and(|active| Rc::ptr_eq(active, &checkpoint.active))
        {
            return None;
        }
        if !Rc::ptr_eq(
            condition.source(),
            checkpoint.active.source.condition_source_v1(self.input)?,
        ) {
            return None;
        }
        let snapshots = std::mem::take(&mut *checkpoint.active.ports.borrow_mut());
        let incoming = ports::verify_incoming(
            self,
            &checkpoint.active.source,
            &checkpoint.active.headers,
            &snapshots,
            checkpoint.active.refused.get(),
            calls,
        );
        let prefix = checkpoint.active.source.site().node().segments();
        let body_expressions: Box<[_]> = self
            .expression_plans()
            .into_iter()
            .filter(|plan| {
                let segments = plan.source().root().statement().node().segments();
                // A child Loop's condition and body plans belong to that
                // child's own product, never to this Loop's execution roots.
                let body_expression = segments.starts_with(prefix)
                    && matches!(
                        segments.get(prefix.len()),
                        Some(crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_))
                    )
                    && !segments.iter().skip(prefix.len() + 1).any(|segment| matches!(segment,
                        crate::mir::resolved_semantics::SourcePathSegmentV1::LoopCondition
                            | crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBodyRoot
                            | crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_)))
                    && !self.input.source().exact_stmt(plan.source().root().statement())
                        .is_ok_and(|statement| matches!(statement.node(), crate::ast::ASTNode::Loop { .. }));
                let loop_condition_argument = plan.argument_frame().is_some_and(|frame| {
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                        call, ..
                    } = frame.source().root() else { return false; };
                    frame.source().root().statement() == condition.source().root().statement()
                        && condition.source().nodes().iter().any(|node| {
                            node.site() == call.site()
                                && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                        })
                });
                body_expression || loop_condition_argument
            })
            .collect();
        let bool_expressions = std::iter::once(&condition)
            .chain(
                body_expressions
                    .iter()
                    .filter_map(|plan| plan.condition_frame()),
            )
            .map(|frame| {
                crate::mir::loop_recipe_contract::project_source_loop_bool_v1(Rc::clone(frame))
            })
            .collect();
        let statement = self
            .input
            .source()
            .exact_stmt(checkpoint.active.source.site())
            .ok()?;
        let body = self
            .input
            .source()
            .child_body_from_stmt(
                &statement,
                crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
            )
            .ok()?;
        let control =
            std::mem::take(&mut *checkpoint.active.control.borrow_mut()).finish(body.site());
        self.source_loop = checkpoint.previous;
        Some(SourceLoopScanSummaryV1 {
            condition,
            refused: checkpoint.active.refused.get(),
            source: Rc::clone(&checkpoint.active.source),
            headers: checkpoint.active.headers.clone(),
            ports: snapshots,
            incoming,
            body_expressions,
            bool_expressions,
            control,
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn note_source_loop_availability_v1(
        &self,
        available: bool,
    ) {
        if !available {
            if let Some(context) = &self.source_loop {
                context.refused.set(true);
            }
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn in_source_loop_v1(&self) -> bool {
        self.source_loop.is_some()
    }
}

#[cfg(test)]
pub(crate) use ports::tests::assert_original_loop_source_walk_for_test;
