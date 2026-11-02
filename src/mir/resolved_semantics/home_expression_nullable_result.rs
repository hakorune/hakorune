//! Nullable identity keeps its genuine source; it grants no payload view/Home.
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, MapFieldGetLocalV1};
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use std::rc::Rc;

/// A use of the SAME declared result, retaining exact alias-only source flow.
/// Current binding and original Call destination remain independent. This
/// lends provenance only; prefix liveness and physical Copy checks are owed.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedResultReadV1 {
    source: Box<BorrowedNullableLocalSourceLoanV1>,
    binding: BindingRefV1,
    read: OwnedExprSiteV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    origin: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    flow: std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
}
impl PartialEq for BorrowedResultReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.same_reference_v1(other) && self.binding == other.binding && self.read == other.read && self.value == other.value
    }
}
impl Eq for BorrowedResultReadV1 {}
impl std::ops::Deref for BorrowedResultReadV1 {
    type Target = BorrowedNullableLocalSourceLoanV1;
    fn deref(&self) -> &Self::Target { &self.source }
}
impl AsRef<BorrowedNullableLocalSourceLoanV1> for BorrowedResultReadV1 {
    fn as_ref(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.source }
}
impl BorrowedResultReadV1 {
    pub(crate) fn issue_v1(input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        source: &BorrowedNullableLocalSourceLoanV1, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Option<Self> {
        let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else { return None; };
        if !source.is_declared_read_v1() || binding.owner() != source.call().owner() || input.owner() != binding.owner() { return None; }
        let flow = input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let origin = flow.alias_origin_with_expression_boundary(value,
            |site| input.function().variable_ref(site), |site| site == source.call().site())?;
        if !matches!(flow.origin(origin), Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(original)) if original == source.call().site()) { return None; }
        let mut declarations = input.function().expression_source().initializers().filter(|row| row.binding() == source.destination());
        if declarations.next()?.initializer_site() != Some(source.call().site()) || declarations.next().is_some() { return None; }
        Some(Self { source: Box::new(source.clone()), binding, read: OwnedExprSiteV1::new(input.owner(), site.clone()), value, origin, flow: std::sync::Arc::clone(flow) })
    }
    pub(crate) fn source(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.source }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { &self.read }
    pub(crate) fn same_reference_v1(&self, other: &Self) -> bool {
        self.source == other.source && self.origin == other.origin && std::sync::Arc::ptr_eq(&self.flow, &other.flow)
    }
    pub(crate) fn corroborates_v1(&self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>) -> bool {
        input.owner() == self.read.owner() && input.function().variable_ref(self.read.site()) == Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(self.binding))
            && input.function().expression_source().value_flow().is_some_and(|flow| std::sync::Arc::ptr_eq(flow, &self.flow)
                && flow.read(self.read.site(), self.binding) == Some(self.value))
    }
}

#[derive(Debug, Clone)]
pub(crate) enum NullableResultReferenceV1 {
    StoredMapGet(Rc<MapFieldGetLocalV1>),
    DeclaredFrom(BorrowedResultReadV1),
}
impl PartialEq for NullableResultReferenceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StoredMapGet(a), Self::StoredMapGet(b)) => Rc::ptr_eq(a, b),
            (Self::DeclaredFrom(a), Self::DeclaredFrom(b)) => a.same_reference_v1(b),
            _ => false,
        }
    }
}
impl Eq for NullableResultReferenceV1 {}
impl NullableResultReferenceV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match self { Self::StoredMapGet(frame) => frame.binding(), Self::DeclaredFrom(view) => view.binding() }
    }
    pub(crate) fn original_binding_v1(&self) -> BindingRefV1 {
        match self { Self::StoredMapGet(frame) => frame.binding(), Self::DeclaredFrom(view) => view.source().destination() }
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match self { Self::StoredMapGet(frame) => frame.site(), Self::DeclaredFrom(view) => view.source().call() }
    }
    pub(crate) fn map_frame(&self) -> Option<&Rc<MapFieldGetLocalV1>> {
        match self { Self::StoredMapGet(frame) => Some(frame), _ => None }
    }
    pub(crate) fn matches_source(&self) -> bool {
        match self {
            Self::StoredMapGet(frame) => frame.source().get().owner() == frame.site().owner()
                && frame.source().get().site() == frame.site().site(),
            Self::DeclaredFrom(view) => view.source().is_declared_read_v1() && view.binding().owner() == view.source().call().owner(),
        }
    }
    pub(crate) fn accepts_prior_homes(&self, homes: &[LiveHomeV1]) -> bool {
        match self {
            Self::StoredMapGet(frame) => homes == frame.prior_homes(),
            Self::DeclaredFrom(view) => !homes.iter().any(|home| home.binding() == view.source().destination() || home.binding() == view.binding())
                && match view.source().owned_local_support_v1() {
                    Some((root, _, _)) => homes.iter().any(|home| home.binding() == root),
                    None => view.source().entry_support_v1().is_some(),
                },
        }
    }
}
