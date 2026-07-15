//! Source-unit callable header index for canonical direct calls.
//!
//! P0c-L0 exposes only a one-entry exact-i64 seal. The schema is cardinality
//! independent so CAT0 can later grow the source-unit catalog without changing
//! consumer identity, key, signature, or symbol vocabulary.

use std::collections::BTreeMap;

use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;

use super::{CallableHeaderSyntaxViewV1, FunctionOwnerIdV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CallableNamespaceV1 {
    FreeStatic,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalCallableKeyV1 {
    namespace: CallableNamespaceV1,
    name: Box<str>,
    arity: u32,
}

impl CanonicalCallableKeyV1 {
    fn free_static(name: &str, arity: u32) -> Self {
        Self {
            namespace: CallableNamespaceV1::FreeStatic,
            name: name.into(),
            arity,
        }
    }

    pub(crate) const fn namespace(&self) -> CallableNamespaceV1 {
        self.namespace
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ResolvedCallableRefV1 {
    owner: FunctionOwnerIdV1,
}

impl ResolvedCallableRefV1 {
    fn new(owner: FunctionOwnerIdV1) -> Self {
        Self { owner }
    }

    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalCallableSymbolV1(Box<str>);

impl CanonicalCallableSymbolV1 {
    fn from_key(key: &CanonicalCallableKeyV1) -> Self {
        Self(format!("{}/{}", key.name(), key.arity()).into_boxed_str())
    }

    pub(crate) fn as_mir_name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactTrivialCallableSignatureV1 {
    params: Box<[ExactTrivialScalarAbiV1]>,
    result: ExactTrivialScalarAbiV1,
}

impl ExactTrivialCallableSignatureV1 {
    fn exact_i64(arity: usize) -> Self {
        Self {
            params: vec![ExactTrivialScalarAbiV1::I64; arity].into_boxed_slice(),
            result: ExactTrivialScalarAbiV1::I64,
        }
    }

    pub(crate) fn params(&self) -> &[ExactTrivialScalarAbiV1] {
        &self.params
    }

    pub(crate) const fn result(&self) -> ExactTrivialScalarAbiV1 {
        self.result
    }

    pub(crate) fn arity(&self) -> usize {
        self.params.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableHeaderV1 {
    callable: ResolvedCallableRefV1,
    source_key: CanonicalCallableKeyV1,
    symbol: CanonicalCallableSymbolV1,
    signature: ExactTrivialCallableSignatureV1,
}

impl VerifiedCallableHeaderV1 {
    pub(crate) const fn callable(&self) -> ResolvedCallableRefV1 {
        self.callable
    }

    pub(crate) const fn source_key(&self) -> &CanonicalCallableKeyV1 {
        &self.source_key
    }

    pub(crate) const fn symbol(&self) -> &CanonicalCallableSymbolV1 {
        &self.symbol
    }

    pub(crate) const fn signature(&self) -> &ExactTrivialCallableSignatureV1 {
        &self.signature
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableIndexSealErrorV1 {
    StaticRequired,
    MainUnsupported,
    OverrideUnsupported,
    MetadataOutsideProfile,
    PhysicalSymbolSpellingInSource,
    ZeroParameters,
    ParameterDeclarationCardinality,
    ParameterNameMismatch { index: usize },
    ParameterTypeOutsideProfile { index: usize },
    ReturnTypeOutsideProfile,
    ArityOverflow,
    DuplicateSourceKey,
    DuplicateCallableIdentity,
    DuplicatePhysicalSymbol,
    IndexCardinality { actual: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableLookupErrorV1 {
    PhysicalSymbolSpellingInSource,
    MissingExactSourceKey,
    MissingCallableIdentity,
    MissingPhysicalSymbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CallableCatalogCardinalityErrorV1 {
    actual: usize,
}

impl CallableCatalogCardinalityErrorV1 {
    pub(crate) const fn actual(self) -> usize {
        self.actual
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableIndexV1 {
    headers_by_key: BTreeMap<CanonicalCallableKeyV1, VerifiedCallableHeaderV1>,
    key_by_callable: BTreeMap<ResolvedCallableRefV1, CanonicalCallableKeyV1>,
    key_by_symbol: BTreeMap<CanonicalCallableSymbolV1, CanonicalCallableKeyV1>,
}

impl VerifiedCallableIndexV1 {
    pub(crate) fn seal_one(
        owner: FunctionOwnerIdV1,
        view: CallableHeaderSyntaxViewV1<'_>,
    ) -> Result<Self, CallableIndexSealErrorV1> {
        let header = seal_exact_i64_header(owner, view)?;
        let mut draft = CallableIndexDraftV1::default();
        draft.insert(header)?;
        draft.seal_one()
    }

    pub(crate) fn lookup(&self, key: &CanonicalCallableKeyV1) -> Option<&VerifiedCallableHeaderV1> {
        self.headers_by_key.get(key)
    }

    pub(crate) fn resolve_free_static_source_call(
        &self,
        name: &str,
        arity: u32,
    ) -> Result<&VerifiedCallableHeaderV1, CallableLookupErrorV1> {
        if name.contains('/') {
            return Err(CallableLookupErrorV1::PhysicalSymbolSpellingInSource);
        }
        self.lookup(&CanonicalCallableKeyV1::free_static(name, arity))
            .ok_or(CallableLookupErrorV1::MissingExactSourceKey)
    }

    pub(crate) fn header_for_callable(
        &self,
        callable: ResolvedCallableRefV1,
    ) -> Result<&VerifiedCallableHeaderV1, CallableLookupErrorV1> {
        let key = self
            .key_by_callable
            .get(&callable)
            .ok_or(CallableLookupErrorV1::MissingCallableIdentity)?;
        self.headers_by_key
            .get(key)
            .ok_or(CallableLookupErrorV1::MissingCallableIdentity)
    }

    pub(crate) fn header_for_symbol(
        &self,
        symbol: &CanonicalCallableSymbolV1,
    ) -> Result<&VerifiedCallableHeaderV1, CallableLookupErrorV1> {
        let key = self
            .key_by_symbol
            .get(symbol)
            .ok_or(CallableLookupErrorV1::MissingPhysicalSymbol)?;
        self.headers_by_key
            .get(key)
            .ok_or(CallableLookupErrorV1::MissingPhysicalSymbol)
    }

    pub(crate) fn sole_header(
        &self,
    ) -> Result<&VerifiedCallableHeaderV1, CallableCatalogCardinalityErrorV1> {
        if self.headers_by_key.len() != 1 {
            return Err(CallableCatalogCardinalityErrorV1 {
                actual: self.headers_by_key.len(),
            });
        }
        match self.headers_by_key.first_key_value() {
            Some((_, header)) => Ok(header),
            None => Err(CallableCatalogCardinalityErrorV1 { actual: 0 }),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.headers_by_key.len()
    }
}

#[derive(Debug, Default)]
struct CallableIndexDraftV1 {
    by_source_key: BTreeMap<CanonicalCallableKeyV1, VerifiedCallableHeaderV1>,
}

impl CallableIndexDraftV1 {
    fn insert(&mut self, header: VerifiedCallableHeaderV1) -> Result<(), CallableIndexSealErrorV1> {
        if self
            .by_source_key
            .insert(header.source_key.clone(), header)
            .is_some()
        {
            return Err(CallableIndexSealErrorV1::DuplicateSourceKey);
        }
        Ok(())
    }

    fn seal_one(self) -> Result<VerifiedCallableIndexV1, CallableIndexSealErrorV1> {
        if self.by_source_key.len() != 1 {
            return Err(CallableIndexSealErrorV1::IndexCardinality {
                actual: self.by_source_key.len(),
            });
        }
        let mut key_by_callable = BTreeMap::new();
        let mut key_by_symbol = BTreeMap::new();
        for (key, header) in &self.by_source_key {
            if key_by_callable
                .insert(header.callable(), key.clone())
                .is_some()
            {
                return Err(CallableIndexSealErrorV1::DuplicateCallableIdentity);
            }
            if key_by_symbol
                .insert(header.symbol().clone(), key.clone())
                .is_some()
            {
                return Err(CallableIndexSealErrorV1::DuplicatePhysicalSymbol);
            }
        }
        Ok(VerifiedCallableIndexV1 {
            headers_by_key: self.by_source_key,
            key_by_callable,
            key_by_symbol,
        })
    }
}

fn seal_exact_i64_header(
    owner: FunctionOwnerIdV1,
    view: CallableHeaderSyntaxViewV1<'_>,
) -> Result<VerifiedCallableHeaderV1, CallableIndexSealErrorV1> {
    if !view.is_static() {
        return Err(CallableIndexSealErrorV1::StaticRequired);
    }
    if view.name() == "main" {
        return Err(CallableIndexSealErrorV1::MainUnsupported);
    }
    if view.is_override() {
        return Err(CallableIndexSealErrorV1::OverrideUnsupported);
    }
    if !view.metadata_is_empty() {
        return Err(CallableIndexSealErrorV1::MetadataOutsideProfile);
    }
    if view.name().contains('/') {
        return Err(CallableIndexSealErrorV1::PhysicalSymbolSpellingInSource);
    }
    if view.params().is_empty() {
        return Err(CallableIndexSealErrorV1::ZeroParameters);
    }
    if view.params().len() != view.param_decls().len() {
        return Err(CallableIndexSealErrorV1::ParameterDeclarationCardinality);
    }
    for (index, (name, declaration)) in view.params().iter().zip(view.param_decls()).enumerate() {
        if declaration.name != *name {
            return Err(CallableIndexSealErrorV1::ParameterNameMismatch { index });
        }
        if declaration
            .declared_type_name
            .as_deref()
            .and_then(ExactTrivialScalarAbiV1::classify)
            != Some(ExactTrivialScalarAbiV1::I64)
        {
            return Err(CallableIndexSealErrorV1::ParameterTypeOutsideProfile { index });
        }
    }
    if view
        .return_type_name()
        .and_then(ExactTrivialScalarAbiV1::classify)
        != Some(ExactTrivialScalarAbiV1::I64)
    {
        return Err(CallableIndexSealErrorV1::ReturnTypeOutsideProfile);
    }

    let arity =
        u32::try_from(view.params().len()).map_err(|_| CallableIndexSealErrorV1::ArityOverflow)?;
    let source_key = CanonicalCallableKeyV1::free_static(view.name(), arity);
    Ok(VerifiedCallableHeaderV1 {
        callable: ResolvedCallableRefV1::new(owner),
        symbol: CanonicalCallableSymbolV1::from_key(&source_key),
        signature: ExactTrivialCallableSignatureV1::exact_i64(view.params().len()),
        source_key,
    })
}

#[cfg(test)]
#[path = "callable_index_tests.rs"]
mod tests;
