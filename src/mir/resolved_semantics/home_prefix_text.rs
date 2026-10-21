//! Current Text residences retain their original acquisition, never Object Home.
use super::*;
use std::rc::Rc;
use super::super::{CheckedInitializerExpressionFrameV1, CheckedBindingRebindFrameV1, BindingRebindDestinationV1};
use crate::mir::resolved_semantics::source_value_flow::{SourceValueRefV1, SourceValueOriginV1, VerifiedSourceValueFlowV1};
use std::sync::Arc;

/// One live source obligation. Text retains the exact acquisition, while
/// existing non-rebindable Homes keep their stable binding identity.
#[derive(Debug, Clone)]
pub(crate) enum LiveHomeV1 {
    Stable(BindingRefV1),
    Text(TextResidenceAcquisitionV1),
}
impl LiveHomeV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match self { Self::Stable(binding) => *binding, Self::Text(source) => source.binding() }
    }
    pub(crate) fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 { self.binding().owner() }
}
impl PartialEq for LiveHomeV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Stable(a), Self::Stable(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => a.same_source(b),
            _ => false,
        }
    }
}
impl Eq for LiveHomeV1 {}

/// Source-owned identity of one Text acquisition. Consumers borrow its origin;
/// they do not turn another purpose into an initializer or issue a second Home.
#[derive(Debug, Clone)]
pub(crate) struct TextResidenceAcquisitionV1 {
    origin: TextResidenceOriginV1,
}
#[derive(Debug, Clone)]
enum TextResidenceOriginV1 {
    Initializer(Rc<CheckedInitializerExpressionFrameV1>),
    Rebind(Rc<CheckedBindingRebindFrameV1>),
}
impl PartialEq for TextResidenceAcquisitionV1 {
    fn eq(&self, other: &Self) -> bool { self.same_source(other) }
}
impl Eq for TextResidenceAcquisitionV1 {}
impl TextResidenceAcquisitionV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn from_initializer(frame: Rc<CheckedInitializerExpressionFrameV1>) -> Self {
        assert!(frame.is_text_residence(), "Text installation requires an owned Text root");
        Self { origin: TextResidenceOriginV1::Initializer(frame) }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn from_rebind(frame: Rc<CheckedBindingRebindFrameV1>) -> Self {
        assert!(frame.is_text_residence(), "Text rebind requires an owned Text root");
        Self { origin: TextResidenceOriginV1::Rebind(frame) }
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match &self.origin { TextResidenceOriginV1::Initializer(frame) => frame.binding(),
            TextResidenceOriginV1::Rebind(frame) => frame.target() }
    }
    pub(crate) fn same_source(&self, other: &Self) -> bool {
        match (&self.origin, &other.origin) {
            (TextResidenceOriginV1::Initializer(a), TextResidenceOriginV1::Initializer(b)) => Rc::ptr_eq(a, b),
            (TextResidenceOriginV1::Rebind(a), TextResidenceOriginV1::Rebind(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
    pub(crate) fn initializer_origin_v1(&self) -> Option<&Rc<CheckedInitializerExpressionFrameV1>> {
        match &self.origin { TextResidenceOriginV1::Initializer(frame) => Some(frame), _ => None }
    }
    pub(crate) fn rebind_origin_v1(&self) -> Option<&Rc<CheckedBindingRebindFrameV1>> {
        match &self.origin { TextResidenceOriginV1::Rebind(frame) => Some(frame), _ => None }
    }
    /// Original source coordinate only; not an evaluation or ownership grant.
    /// Initializer and Rebind acquisitions refer to their original root expression.
    pub(crate) fn source(&self) -> &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1> {
        match &self.origin { TextResidenceOriginV1::Initializer(frame) => frame.source(), TextResidenceOriginV1::Rebind(frame) => frame.source() }
    }
    pub(crate) fn owns_root(&self) -> bool {
        match &self.origin { TextResidenceOriginV1::Initializer(frame) => frame.owns_root(), TextResidenceOriginV1::Rebind(frame) => frame.is_text_residence() }
    }
    pub(crate) fn is_text_residence(&self) -> bool {
        match &self.origin { TextResidenceOriginV1::Initializer(frame) => frame.is_text_residence(), TextResidenceOriginV1::Rebind(frame) => frame.is_text_residence() }
    }
}

/// The covered read of one original non-owning literal definition. Bytes alone
/// cannot stand in for the reaching definition or the current Prefix state.
#[derive(Debug, Clone)]
pub(crate) struct StaticTextReadV1 {
    read: SourceExprSiteV1, binding: BindingRefV1, value: SourceValueRefV1,
    origin: SourceValueRefV1, definition: SourceExprSiteV1, definition_binding: BindingRefV1, utf8: Box<str>,
    flow: Arc<VerifiedSourceValueFlowV1>,
}
impl PartialEq for StaticTextReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read && self.binding == other.binding && self.value == other.value
            && self.origin == other.origin && self.definition == other.definition && self.definition_binding == other.definition_binding
            && self.utf8 == other.utf8 && Arc::ptr_eq(&self.flow, &other.flow)
    }
}
impl Eq for StaticTextReadV1 {}
impl StaticTextReadV1 {
    pub(crate) fn read(&self) -> &SourceExprSiteV1 { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn definition(&self) -> &SourceExprSiteV1 { &self.definition }
    pub(crate) fn definition_binding(&self) -> BindingRefV1 { self.definition_binding }
    pub(crate) fn utf8(&self) -> &str { &self.utf8 }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn rebind_destination_v1(
        &self, binding: BindingRefV1, homes: &[LiveHomeV1],
    ) -> Option<BindingRebindDestinationV1> {
        if self.can_rebind_integer_v1(binding, homes) { return Some(BindingRebindDestinationV1::Integer); }
        if binding.owner() != self.input.owner() { return None; }
        match self.locals.get(&binding)? {
            StoredLocal::Trivial(Some(SourceScalarKind::Bool)) if !homes.iter().any(|home| home.binding() == binding) =>
                Some(BindingRebindDestinationV1::Bool),
            StoredLocal::StaticText { definition } if !homes.iter().any(|home| home.binding() == binding) =>
                Some(BindingRebindDestinationV1::TextStatic(definition.clone())),
            StoredLocal::Text { root, acquisition } if *root == binding && acquisition.binding() == binding
                && homes.iter().filter(|home| home.binding() == binding).eq(std::iter::once(&LiveHomeV1::Text(acquisition.clone()))) =>
                Some(BindingRebindDestinationV1::TextResidence(acquisition.clone())),
            _ => None,
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_text_rebind(&mut self, frame: Rc<CheckedBindingRebindFrameV1>) {
        let root = frame.target();
        self.store(root, StoredLocal::Text { root, acquisition: frame.text_acquisition_v1() });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_static_text(&mut self,
        binding: BindingRefV1, definition: SourceExprSiteV1) {
        self.store(binding, StoredLocal::StaticText { definition });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn static_text_reference(&self,
        site: &SourceExprSiteV1) -> Option<StaticTextReadV1> {
        let function = self.input.function();
        let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(site)? else { return None };
        let StoredLocal::StaticText { definition } = self.locals.get(&binding)? else { return None };
        let flow = function.expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let origin = flow.alias_origin(value, |site| function.variable_ref(site))?;
        let SourceValueOriginV1::Expression(original) = flow.origin(origin)? else { return None };
        if original != definition { return None; }
        let ResolvedLiteralSourceV1::String(utf8) = function.expression_source().literal(original)? else { return None };
        let mut definitions = function.expression_source().initializers().filter(|row| row.initializer_site() == Some(original));
        let definition_binding = definitions.next()?.binding();
        if definitions.next().is_some() || definition_binding.owner() != binding.owner() { return None; }
        Some(StaticTextReadV1 { read: site.clone(), binding, value, origin, definition: original.clone(), definition_binding,
            utf8: utf8.clone(), flow: Arc::clone(flow) })
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn retain_null_exclusion(&mut self,
        proof: Rc<super::super::SourceNullExclusionV1>) {
        self.null_exclusions.insert(proof.root_binding(), proof);
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn entry_null_exclusion(&self, site: &SourceExprSiteV1,
        binding: BindingRefV1, root: BindingRefV1) -> Option<Rc<super::super::SourceNullExclusionV1>> {
        let flow = self.input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        self.null_exclusions.get(&root).filter(|proof|
            proof.corroborates(flow, value, root, |site| self.input.function().variable_ref(site))).map(Rc::clone)
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn text_reference(&self, site: &SourceExprSiteV1)
        -> Option<(BindingRefV1, BindingRefV1, TextResidenceAcquisitionV1)> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else { return None };
        let StoredLocal::Text { root, acquisition } = self.locals.get(&binding)? else { return None };
        // An alias cannot silently follow the same binding's newer acquisition.
        let StoredLocal::Text { root: live, acquisition: current } = self.locals.get(root)? else { return None };
        (*live == *root && *root == acquisition.binding() && acquisition.same_source(current))
            .then(|| (binding, *root, acquisition.clone()))
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_text_initializer(&mut self, frame: Rc<CheckedInitializerExpressionFrameV1>) {
        let acquisition = TextResidenceAcquisitionV1::from_initializer(frame);
        let root = acquisition.binding();
        self.store(root, StoredLocal::Text { root, acquisition });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn install_text_alias(&mut self, binding: BindingRefV1, root: BindingRefV1,
        acquisition: TextResidenceAcquisitionV1) {
        self.store(binding, StoredLocal::Text { root, acquisition });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn signed_entry_reference(&self, site: &SourceExprSiteV1) -> Option<BindingRefV1> {
        let OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer)) = self.observe(site)? else { return None };
        let flow = self.input.function().expression_source().value_flow()?;
        let root = flow.alias_entry_origin(flow.read(site, binding)?, |site| self.input.function().variable_ref(site))?;
        self.signed_formals.contains(&root).then_some(binding)
    }
}

#[cfg(test)]
pub(crate) fn assert_identity_for_test(
    input: ResolvedFunctionLoweringInputV1<'_>, frame: Rc<CheckedInitializerExpressionFrameV1>,
    next: &CheckedInitializerExpressionFrameV1,
) {
    let read = &next.residence_reads()[0];
    let site = next.source().nodes()[read.node()].site();
    let mut current = PrefixLocalFlow::new(input);
    current.install_text_initializer(Rc::clone(&frame));
    let acquisition = TextResidenceAcquisitionV1::from_initializer(Rc::clone(&frame));
    current.install_text_alias(read.binding(), frame.binding(), acquisition.clone());
    assert!(current.text_reference(site).unwrap().2.same_source(&acquisition));
    assert!(current.observe(site).is_none(), "Text is not an Object/Scalar/BoundValue");
    let snapshot = current.clone();
    assert!(current.join_branch(&snapshot), "same acquisition survives a branch clone");
    current.install_uninitialized(frame.binding());
    assert!(current.text_reference(site).is_none(), "invalid root cannot retain an alias");
    assert!(snapshot.text_reference(site).unwrap().2.same_source(&acquisition));
    let replacement = Rc::new(frame.as_ref().clone());
    current.install_text_initializer(replacement);
    assert!(current.text_reference(site).is_none(), "alias cannot follow an equal-content acquisition");
    assert!(!current.join_branch(&snapshot), "different acquisitions cannot merge by binding name");
}
