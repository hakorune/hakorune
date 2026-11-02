//! Declared result reads share one passive source relation; loans own use law.
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, SourceExprSiteV1,
    ResolvedLexicalRefV1, source_value_flow::{SourceValueRefV1, SourceValueOriginV1, VerifiedSourceValueFlowV1}};
use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, MapFieldGetLocalV1};
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use std::{rc::Rc, sync::Arc};
use crate::mir::resolved_semantics::{SourceStmtSiteV1, source_value_flow::{SourceValueEdgeV1, SourceValueJoinKindV1}};

/// Exact alias or unanimous Normal-branch use of an original initializer Call. No type, Home,
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
        let origin = flow.unanimous_alias_origin(value, |at| input.function().variable_ref(at))?;
        let (producer, destination) = original_initializer_call_v1(input, flow, origin)?;
        if site.node().segments().starts_with(producer.node().segments()) { return None; }
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
fn original_initializer_call_v1<'a>(
    input: ResolvedFunctionLoweringInputV1<'_>, flow: &'a VerifiedSourceValueFlowV1,
    origin: SourceValueRefV1,
) -> Option<(&'a SourceExprSiteV1, BindingRefV1)> {
    let SourceValueOriginV1::Expression(producer) = flow.origin(origin)? else { return None; };
    input.function().method_call_source_v1(producer)?;
    let mut declarations = input.function().expression_source().initializers()
        .filter(|row| row.initializer_site() == Some(producer));
    let destination = declarations.next()?.binding();
    if declarations.next().is_some() { return None; }
    let mut original = input.function().expression_source().initializers().filter(|row| row.binding() == destination);
    if original.next()?.initializer_site() != Some(producer) || original.next().is_some() { return None; }
    Some((producer, destination))
}

/// Passive original branch census and its unanimous Call identity. The
/// physical writer and current prefix must supply their own evidence.
#[derive(Debug, Clone)]
pub(crate) struct BranchCallResultSourceV1 {
    producer: OwnedExprSiteV1,
    destination: BindingRefV1,
    binding: BindingRefV1,
    statement: SourceStmtSiteV1,
    value: SourceValueRefV1,
    origin: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
}
impl PartialEq for BranchCallResultSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.flow, &other.flow) && self.value == other.value
            && self.binding == other.binding && self.statement == other.statement
            && self.producer == other.producer && self.destination == other.destination && self.origin == other.origin
    }
}
impl Eq for BranchCallResultSourceV1 {}
impl BranchCallResultSourceV1 {
    pub(crate) fn issue_v1(input: ResolvedFunctionLoweringInputV1<'_>,
        statement: &SourceStmtSiteV1, binding: BindingRefV1,
    ) -> Option<Self> {
        let flow = input.function().expression_source().value_flow()?;
        let (value, _) = flow.branch_join(statement, binding)?;
        let origin = flow.unanimous_alias_origin(value, |site| input.function().variable_ref(site))?;
        let (producer, destination) = original_initializer_call_v1(input, flow, origin)?;
        Some(Self { producer: OwnedExprSiteV1::new(input.owner(), producer.clone()), destination,
            binding, statement: statement.clone(), value, origin, flow: Arc::clone(flow) })
    }
    pub(crate) fn producer(&self) -> &OwnedExprSiteV1 { &self.producer }
    pub(crate) fn destination(&self) -> BindingRefV1 { self.destination }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn incoming(&self) -> Option<&[(SourceValueEdgeV1, SourceValueRefV1)]> {
        match self.flow.origin(self.value)? {
            SourceValueOriginV1::Join { binding, site, kind: SourceValueJoinKindV1::BranchNormal, incoming }
                if *binding == self.binding && site.node() == self.statement.node()
                    && matches!(incoming.as_ref(), [(SourceValueEdgeV1::ThenNormal, _), (SourceValueEdgeV1::ElseNormal, _)]) => Some(incoming),
            _ => None,
        }
    }
    pub(crate) fn matches_read_v1(&self, read: &BorrowedResultReadV1) -> bool {
        let source = &read.read;
        Arc::ptr_eq(&self.flow, &source.flow) && self.origin == source.origin
            && self.producer == source.producer && self.destination == source.destination
            && self.flow.alias_terminal_with_expression_boundary(source.value,
                |site| self.flow.read_at(site).map(|(binding, _)| ResolvedLexicalRefV1::Local(binding)),
                |_| false) == Some(self.value)
    }
    pub(crate) fn incoming_expression_v1(&self, index: usize) -> Option<&SourceExprSiteV1> {
        match self.flow.origin(self.incoming()?.get(index)?.1)? {
            SourceValueOriginV1::Expression(site) => Some(site), _ => None,
        }
    }
}

impl PartialEq for SourceCallResultReadV1 {
    fn eq(&self, other: &Self) -> bool { self.same_result_v1(other) && self.binding == other.binding && self.read == other.read && self.value == other.value }
}
impl Eq for SourceCallResultReadV1 {}

/// Original From signature and its exact source read. This grants only the
/// declared nominal class plus Null, never completed roots or live support.
#[derive(Debug, Clone)]
pub(crate) struct DeclaredCallReadSourceV1 { kind: DeclaredReadSourceKindV1 }
#[derive(Debug, Clone)]
enum DeclaredReadSourceKindV1 {
    Single { read: SourceCallResultReadV1, declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1> },
    Flow { read: OwnedExprSiteV1, binding: BindingRefV1, value: SourceValueRefV1,
        flow: Arc<VerifiedSourceValueFlowV1>, terminals: Vec<DeclaredReadTerminalV1>, class: Box<str> },
}
#[derive(Debug, Clone)]
struct DeclaredReadTerminalV1 {
    origin: SourceValueRefV1,
    // None means an original literal Null, never a nullable Call.
    call: Option<(OwnedExprSiteV1, BindingRefV1, Arc<ResolvedCallableBorrowedResultDeclarationV1>)>,
}
impl DeclaredCallReadSourceV1 {
    pub(crate) fn issue_v1(read: SourceCallResultReadV1, declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1>) -> Self { Self { kind: DeclaredReadSourceKindV1::Single { read, declaration } } }
    /// Only source signatures are folded here. Each terminal keeps its original
    /// declaration; live loans and caller support must be checked separately.
    pub(crate) fn issue_source_v1(
        input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceExprSiteV1,
        mut resolve: impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<Option<Arc<ResolvedCallableBorrowedResultDeclarationV1>>, &'static str>,
    ) -> Result<Option<Self>, &'static str> {
        if let Some(read) = SourceCallResultReadV1::issue_v1(input, site) {
            return Ok(resolve(read.producer(), read.destination())?.map(|declaration| Self::issue_v1(read, declaration)));
        }
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(site) else { return Ok(None); };
        if binding.owner() != input.owner() { return Ok(None); }
        let Some(flow) = input.function().expression_source().value_flow() else { return Ok(None); };
        let Some(value) = flow.read(site, binding) else { return Ok(None); };
        let Some(origins) = flow.alias_origins_v1(value, |at| input.function().variable_ref(at)) else { return Ok(None); };
        let mut terminals = Vec::with_capacity(origins.len());
        let mut class: Option<Arc<ResolvedCallableBorrowedResultDeclarationV1>> = None;
        for origin in origins {
            let call = if let Some((producer, destination)) = original_initializer_call_v1(input, flow, origin) {
                if site.node().segments().starts_with(producer.node().segments()) { return Ok(None); }
                let producer = OwnedExprSiteV1::new(input.owner(), producer.clone());
                let Some(declaration) = resolve(&producer, destination)? else { return Ok(None); };
                if class.as_ref().is_some_and(|original| original.declared_type() != declaration.declared_type()) { return Ok(None); }
                class.get_or_insert_with(|| Arc::clone(&declaration));
                Some((producer, destination, declaration))
            } else if matches!(flow.origin(origin), Some(SourceValueOriginV1::Expression(at))
                if input.function().expression_source().literal(at) == Some(&crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null)) {
                None
            } else { return Ok(None); };
            terminals.push(DeclaredReadTerminalV1 { origin, call });
        }
        // A set of Null literals supplies neither a nominal class nor support.
        let Some(class) = class else { return Ok(None); };
        Ok(Some(Self { kind: DeclaredReadSourceKindV1::Flow { read: OwnedExprSiteV1::new(input.owner(), site.clone()), binding, value,
            flow: Arc::clone(flow), terminals, class: class.declared_type().into() } }))
    }
    pub(crate) fn source_read(&self) -> Option<&SourceCallResultReadV1> {
        match &self.kind { DeclaredReadSourceKindV1::Single { read, .. } => Some(read), DeclaredReadSourceKindV1::Flow { .. } => None }
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match &self.kind { DeclaredReadSourceKindV1::Single { read, .. } => read.binding(), DeclaredReadSourceKindV1::Flow { binding, .. } => *binding }
    }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 {
        match &self.kind { DeclaredReadSourceKindV1::Single { read, .. } => read.read(), DeclaredReadSourceKindV1::Flow { read, .. } => read }
    }
    pub(crate) fn declared_type(&self) -> &str {
        match &self.kind {
            DeclaredReadSourceKindV1::Single { declaration, .. } => declaration.declared_type(),
            DeclaredReadSourceKindV1::Flow { class, .. } => class,
        }
    }
}
impl PartialEq for DeclaredReadTerminalV1 {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && match (&self.call, &other.call) {
            (None, None) => true,
            (Some((a, b, c)), Some((x, y, z))) => a == x && b == y
                && ResolvedCallableBorrowedResultDeclarationV1::same_source(Some(c), Some(z)),
            _ => false,
        }
    }
}
impl PartialEq for DeclaredCallReadSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (DeclaredReadSourceKindV1::Single { read: a, declaration: b }, DeclaredReadSourceKindV1::Single { read: x, declaration: y }) => a == x
                && ResolvedCallableBorrowedResultDeclarationV1::same_source(Some(b), Some(y)),
            (DeclaredReadSourceKindV1::Flow { read: a, binding: b, value: c, flow: d, terminals: e, class: f },
             DeclaredReadSourceKindV1::Flow { read: x, binding: y, value: z, flow: w, terminals: v, class: u }) =>
                a == x && b == y && c == z && Arc::ptr_eq(d, w) && e == v && f == u,
            _ => false,
        }
    }
}
impl Eq for DeclaredCallReadSourceV1 {}

/// Completed original loan plus its current use. Prefix liveness and physical
/// Copy checks remain independent; source type classification cannot issue it.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedResultReadV1 {
    source: Box<BorrowedNullableLocalSourceLoanV1>,
    read: SourceCallResultReadV1,
    declaration: Arc<ResolvedCallableBorrowedResultDeclarationV1>,
}
impl PartialEq for BorrowedResultReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.read == other.read
            && ResolvedCallableBorrowedResultDeclarationV1::same_source(Some(&self.declaration), Some(&other.declaration))
    }
}
impl Eq for BorrowedResultReadV1 {}
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
        let DeclaredReadSourceKindV1::Single { read, declaration } = declared.kind else { return None; };
        (source.is_declared_read_v1() && read.producer() == source.call() && read.destination() == source.destination()
            && declaration.owner() == source.callee_owner() && source.declared_result_matches_v1(Some(&declaration)))
            .then(|| Self { source: Box::new(source.clone()), read, declaration })
    }
    /// The issuer chooses the SAME canonical producer after checking both
    /// original source loans; only the retained graph, never the read, changes.
    pub(crate) fn with_original_source_v1(&self, source: &BorrowedNullableLocalSourceLoanV1) -> Option<Self> {
        Self::from_declared_v1(source, DeclaredCallReadSourceV1::issue_v1(self.read.clone(), Arc::clone(&self.declaration)))
    }
    pub(crate) fn source(&self) -> &BorrowedNullableLocalSourceLoanV1 { &self.source }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.read.binding() }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { self.read.read() }
    pub(crate) fn same_reference_v1(&self, other: &Self) -> bool { self.source == other.source && self.read.same_result_v1(&other.read) }
    pub(crate) fn corroborates_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool { self.read.corroborates_v1(input) }
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
