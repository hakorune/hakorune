//! Declared result reads share one passive source relation; loans own use law.
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1,
    ResolvedLexicalRefV1, source_value_flow::{SourceValueRefV1, SourceValueOriginV1, VerifiedSourceValueFlowV1}};
use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, MapFieldGetLocalV1};
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use std::{rc::Rc, sync::Arc};

/// Exact alias-only use of an original initializer Call. No type, Home,
/// nonnull or read permission is inferred by this source relationship.
#[derive(Debug, Clone)]
pub(crate) struct SourceCallResultReadV1 {
    producer: OwnedExprSiteV1,
    destination: BindingRefV1,
    binding: BindingRefV1,
    read: OwnedExprSiteV1,
    value: SourceValueRefV1,
    origin: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
}
impl SourceCallResultReadV1 {
    pub(crate) fn issue_v1(input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceExprSiteV1) -> Option<Self> {
        let ResolvedLexicalRefV1::Local(binding) = input.function().variable_ref(site)? else { return None; };
        if binding.owner() != input.owner() { return None; }
        let flow = input.function().expression_source().value_flow()?;
        let value = flow.read(site, binding)?;
        let origin = flow.alias_origin(value, |at| input.function().variable_ref(at))?;
        let SourceValueOriginV1::Expression(producer) = flow.origin(origin)? else { return None; };
        if site.node().segments().starts_with(producer.node().segments())
            || input.function().method_call_source_v1(producer).is_none() { return None; }
        let mut declarations = input.function().expression_source().initializers().filter(|row| row.initializer_site() == Some(producer));
        let destination = declarations.next()?.binding();
        if declarations.next().is_some() { return None; }
        let mut original = input.function().expression_source().initializers().filter(|row| row.binding() == destination);
        if original.next()?.initializer_site() != Some(producer) || original.next().is_some() { return None; }
        Some(Self { producer: OwnedExprSiteV1::new(input.owner(), producer.clone()), destination, binding,
            read: OwnedExprSiteV1::new(input.owner(), site.clone()), value, origin, flow: Arc::clone(flow) })
    }
    pub(crate) fn producer(&self) -> &OwnedExprSiteV1 { &self.producer }
    pub(crate) fn destination(&self) -> BindingRefV1 { self.destination }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { &self.read }
    fn same_result_v1(&self, other: &Self) -> bool {
        self.producer == other.producer && self.destination == other.destination && self.origin == other.origin && Arc::ptr_eq(&self.flow, &other.flow)
    }
    pub(crate) fn corroborates_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        input.owner() == self.read.owner() && input.function().variable_ref(self.read.site()) == Some(ResolvedLexicalRefV1::Local(self.binding))
            && input.function().expression_source().value_flow().is_some_and(|flow| Arc::ptr_eq(flow, &self.flow)
                && flow.read(self.read.site(), self.binding) == Some(self.value))
    }
}
impl PartialEq for SourceCallResultReadV1 {
    fn eq(&self, other: &Self) -> bool { self.same_result_v1(other) && self.binding == other.binding && self.read == other.read && self.value == other.value }
}
impl Eq for SourceCallResultReadV1 {}

/// Original From signature and its exact source read. This grants only the
/// declared nominal class plus Null, never completed roots or live support.
#[derive(Debug, Clone)]
pub(crate) struct DeclaredCallReadSourceV1 {
    read: SourceCallResultReadV1,
    declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1>,
}
impl DeclaredCallReadSourceV1 {
    pub(crate) fn issue_v1(read: SourceCallResultReadV1, declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1>) -> Self { Self { read, declaration } }
    pub(crate) fn source_read(&self) -> &SourceCallResultReadV1 { &self.read }
    pub(crate) fn declared_type(&self) -> &str { self.declaration.declared_type() }
}
impl PartialEq for DeclaredCallReadSourceV1 {
    fn eq(&self, other: &Self) -> bool { self.read == other.read && ResolvedCallableBorrowedResultDeclarationV1::same_source(Some(&self.declaration), Some(&other.declaration)) }
}
impl Eq for DeclaredCallReadSourceV1 {}

/// Completed original loan plus its current use. Prefix liveness and physical
/// Copy checks remain independent; source type classification cannot issue it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedResultReadV1 {
    source: Box<BorrowedNullableLocalSourceLoanV1>,
    declared: DeclaredCallReadSourceV1,
}
impl std::ops::Deref for BorrowedResultReadV1 {
    type Target = BorrowedNullableLocalSourceLoanV1;
    fn deref(&self) -> &Self::Target { &self.source }
}
impl AsRef<BorrowedNullableLocalSourceLoanV1> for BorrowedResultReadV1 {
    fn as_ref(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.source }
}
impl BorrowedResultReadV1 {
    pub(crate) fn issue_v1(input: ResolvedFunctionLoweringInputV1<'_>, source: &BorrowedNullableLocalSourceLoanV1, site: &SourceExprSiteV1) -> Option<Self> {
        let read = SourceCallResultReadV1::issue_v1(input, site)?;
        let declaration = Arc::clone(source.declared_read_result_v1()?);
        Self::from_declared_v1(source, DeclaredCallReadSourceV1::issue_v1(read, declaration))
    }
    pub(crate) fn from_declared_v1(source: &BorrowedNullableLocalSourceLoanV1, declared: DeclaredCallReadSourceV1) -> Option<Self> {
        (source.is_declared_read_v1() && declared.read.producer() == source.call() && declared.read.destination() == source.destination()
            && declared.declaration.owner() == source.callee_owner() && source.declared_result_matches_v1(Some(&declared.declaration)))
            .then(|| Self { source: Box::new(source.clone()), declared })
    }
    /// The issuer chooses the SAME canonical producer after checking both
    /// original source loans; only the retained graph, never the read, changes.
    pub(crate) fn with_original_source_v1(&self, source: &BorrowedNullableLocalSourceLoanV1) -> Option<Self> {
        Self::from_declared_v1(source, self.declared.clone())
    }
    pub(crate) fn source(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.source }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.declared.read.binding() }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { self.declared.read.read() }
    pub(crate) fn same_reference_v1(&self, other: &Self) -> bool { self.source == other.source && self.declared.read.same_result_v1(&other.declared.read) }
    pub(crate) fn corroborates_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool { self.declared.read.corroborates_v1(input) }
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
