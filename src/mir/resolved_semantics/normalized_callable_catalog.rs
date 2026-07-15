//! Invocation-independent normalized view of one verified callable catalog.
//!
//! Function origins, declaration sites, owner slots, and compilation brands
//! are deliberately excluded. CAT0-G0 uses this product only for deterministic
//! parity and guard evidence; it is not call resolution authority.

use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;

use super::{CallableNamespaceV1, VerifiedCallableCatalogV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedCallableCatalogRowV1 {
    namespace: CallableNamespaceV1,
    name: Box<str>,
    arity: u32,
    params: Box<[ExactTrivialScalarAbiV1]>,
    result: ExactTrivialScalarAbiV1,
    symbol: Box<str>,
}

impl NormalizedCallableCatalogRowV1 {
    pub(crate) const fn namespace(&self) -> CallableNamespaceV1 {
        self.namespace
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) fn params(&self) -> &[ExactTrivialScalarAbiV1] {
        &self.params
    }

    pub(crate) const fn result(&self) -> ExactTrivialScalarAbiV1 {
        self.result
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedCallableCatalogV1 {
    rows: Box<[NormalizedCallableCatalogRowV1]>,
}

impl NormalizedCallableCatalogV1 {
    pub(crate) fn from_catalog(catalog: &VerifiedCallableCatalogV1) -> Self {
        let rows = catalog
            .index()
            .headers()
            .map(|header| NormalizedCallableCatalogRowV1 {
                namespace: header.source_key().namespace(),
                name: header.source_key().name().into(),
                arity: header.source_key().arity(),
                params: header.signature().params().into(),
                result: header.signature().result(),
                symbol: header.symbol().as_mir_name().into(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { rows }
    }

    pub(crate) fn rows(&self) -> &[NormalizedCallableCatalogRowV1] {
        &self.rows
    }
}
