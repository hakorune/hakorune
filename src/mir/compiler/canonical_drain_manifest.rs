//! ROOT0-DRAIN0-MANIFEST0: exact canonical drain inventory.
//!
//! This module owns only the source-derived expected rows for a future drain.
//! It deliberately does not inspect a collector, receipt, `MirModule`, or
//! current Builder state.  Those are physical evidence and belong to the
//! later PHYSICAL0 row.

use crate::mir::canonical_physical_drain::{
    CanonicalPhysicalCallableRowV1, CanonicalPhysicalDrainManifestV1, CanonicalPhysicalSingleRowV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::module_invocation_policy::ModuleInvocationPolicyV1;
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};

/// The only publication disposition admitted by a canonical drain manifest.
///
/// This is intentionally a distinct zero-sized seal rather than a caller
/// supplied `DraftPublicationPolicyV1`: canonical rows are always inserted
/// under reject-duplicate admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanonicalInsertedSealV1 {
    _private: (),
}

impl CanonicalInsertedSealV1 {
    const fn new() -> Self {
        Self { _private: () }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CanonicalDrainIdentityV1 {
    ResolvedOwner(FunctionOwnerIdV1),
    Callable(CanonicalCallableKeyV1),
}

/// One exact source row expected to be present in the future physical shell.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct CanonicalDrainRowV1 {
    identity: CanonicalDrainIdentityV1,
    symbol: Box<str>,
    arity: usize,
    inserted: CanonicalInsertedSealV1,
}

impl CanonicalDrainRowV1 {
    pub(super) fn new(identity: CanonicalDrainIdentityV1, symbol: Box<str>, arity: usize) -> Self {
        Self {
            identity,
            symbol,
            arity,
            inserted: CanonicalInsertedSealV1::new(),
        }
    }

    pub(super) const fn identity(&self) -> &CanonicalDrainIdentityV1 {
        &self.identity
    }

    pub(super) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(super) const fn arity(&self) -> usize {
        self.arity
    }

    pub(super) const fn inserted(&self) -> CanonicalInsertedSealV1 {
        self.inserted
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum CanonicalDrainManifestV1 {
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
        Self::Single { brand, policy, row }
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

    pub(super) const fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { brand, .. } | Self::Callable { brand, .. } => *brand,
        }
    }

    pub(super) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.policy().family()
    }

    pub(super) const fn policy(&self) -> ModuleInvocationPolicyV1 {
        match self {
            Self::Single { policy, .. } | Self::Callable { policy, .. } => *policy,
        }
    }

    pub(super) fn rows(&self) -> &[CanonicalDrainRowV1] {
        match self {
            Self::Single { row, .. } => std::slice::from_ref(row),
            Self::Callable { rows, .. } => rows,
        }
    }
}

impl CanonicalDrainManifestV1 {
    /// Consume the compiler-owned source manifest at the compiler/Builder
    /// boundary.  Only the narrow physical vocabulary crosses this seam;
    /// generic policy and compiler source types remain private here.
    pub(super) fn into_physical(self) -> CanonicalPhysicalDrainManifestV1 {
        match self {
            Self::Single { brand, policy, row } => {
                let CanonicalDrainRowV1 {
                    identity,
                    symbol,
                    arity,
                    inserted: _,
                } = row;
                let owner = match identity {
                    CanonicalDrainIdentityV1::ResolvedOwner(owner) => owner,
                    CanonicalDrainIdentityV1::Callable(_) => {
                        unreachable!("single canonical drain row must be a resolved owner")
                    }
                };
                CanonicalPhysicalDrainManifestV1::single(
                    brand,
                    policy.family(),
                    CanonicalPhysicalSingleRowV1::new(
                        owner,
                        symbol,
                        arity,
                        crate::mir::canonical_physical_drain::CanonicalInsertedDispositionV1::from_canonical_source(),
                    ),
                )
            }
            Self::Callable {
                brand,
                policy,
                rows,
            } => {
                let physical_rows = rows
                    .into_vec()
                    .into_iter()
                    .map(|row| {
                        let CanonicalDrainRowV1 {
                            identity,
                            symbol,
                            arity,
                            inserted: _,
                        } = row;
                        let key = match identity {
                            CanonicalDrainIdentityV1::Callable(key) => key,
                            CanonicalDrainIdentityV1::ResolvedOwner(_) => {
                                unreachable!("callable canonical drain row must have a callable key")
                            }
                        };
                        CanonicalPhysicalCallableRowV1::new(
                            key,
                            symbol,
                            arity,
                            crate::mir::canonical_physical_drain::CanonicalInsertedDispositionV1::from_canonical_source(),
                        )
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                CanonicalPhysicalDrainManifestV1::callable(brand, policy.family(), physical_rows)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CanonicalDrainManifestErrorV1 {
    MissingCallableHeader(CanonicalCallableKeyV1),
}
