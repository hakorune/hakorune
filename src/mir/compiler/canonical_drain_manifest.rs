//! ROOT0-DRAIN0-MANIFEST0: exact canonical drain inventory.
//!
//! This module owns only the source-derived expected rows for a future drain.
//! It deliberately does not inspect a collector, receipt, `MirModule`, or
//! current Builder state.  Those are physical evidence and belong to the
//! later PHYSICAL0 row.

use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::module_invocation_policy::ModuleInvocationPolicyV1;
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};

/// The only publication disposition admitted by a canonical drain manifest.
///
/// This is intentionally a distinct zero-sized seal rather than a caller
/// supplied `DraftPublicationPolicyV1`: canonical rows are always inserted
/// under reject-duplicate admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalInsertedSealV1 {
    _private: (),
}

impl CanonicalInsertedSealV1 {
    const fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CanonicalDrainIdentityV1 {
    ResolvedOwner(FunctionOwnerIdV1),
    Callable(CanonicalCallableKeyV1),
}

/// One exact source row expected to be present in the future physical shell.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalDrainRowV1 {
    identity: CanonicalDrainIdentityV1,
    symbol: Box<str>,
    arity: usize,
    inserted: CanonicalInsertedSealV1,
}

impl CanonicalDrainRowV1 {
    pub(super) fn new(
        identity: CanonicalDrainIdentityV1,
        symbol: Box<str>,
        arity: usize,
    ) -> Self {
        Self {
            identity,
            symbol,
            arity,
            inserted: CanonicalInsertedSealV1::new(),
        }
    }

    pub(crate) const fn identity(&self) -> &CanonicalDrainIdentityV1 {
        &self.identity
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) const fn inserted(&self) -> CanonicalInsertedSealV1 {
        self.inserted
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CanonicalDrainManifestV1 {
    Single {
        brand: ModuleInvocationBrandV1,
        policy: ModuleInvocationPolicyV1,
        row: CanonicalDrainRowV1,
    },
    Callable {
        brand: ModuleInvocationBrandV1,
        policy: ModuleInvocationPolicyV1,
        rows: Box<[CanonicalDrainRowV1]>,
    },
}

impl CanonicalDrainManifestV1 {
    pub(super) fn single(
        brand: ModuleInvocationBrandV1,
        policy: ModuleInvocationPolicyV1,
        row: CanonicalDrainRowV1,
    ) -> Self {
        Self::Single {
            brand,
            policy,
            row,
        }
    }

    pub(super) fn callable(
        brand: ModuleInvocationBrandV1,
        policy: ModuleInvocationPolicyV1,
        rows: Vec<CanonicalDrainRowV1>,
    ) -> Self {
        Self::Callable {
            brand,
            policy,
            rows: rows.into_boxed_slice(),
        }
    }

    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { brand, .. } | Self::Callable { brand, .. } => *brand,
        }
    }

    pub(crate) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.policy().family()
    }

    pub(crate) const fn policy(&self) -> ModuleInvocationPolicyV1 {
        match self {
            Self::Single { policy, .. } | Self::Callable { policy, .. } => *policy,
        }
    }

    pub(crate) fn rows(&self) -> &[CanonicalDrainRowV1] {
        match self {
            Self::Single { row, .. } => std::slice::from_ref(row),
            Self::Callable { rows, .. } => rows,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CanonicalDrainManifestErrorV1 {
    MissingCallableHeader(CanonicalCallableKeyV1),
}
