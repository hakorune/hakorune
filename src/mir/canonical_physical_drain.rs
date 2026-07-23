//! PHYSICAL0 neutral source-to-Builder drain vocabulary.
//!
//! This module intentionally contains no compiler source manifest, Builder
//! collector, or generic publication-policy types.  It is the narrow value
//! contract that crosses the compiler/Builder boundary after source
//! projection has completed.

use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};

/// The only publication disposition admitted by the canonical physical seam.
///
/// The field is private and the constructor is crate-internal so callers can
/// carry the seal but cannot select a legacy replacement policy at this
/// boundary.  The compiler source manifest is the sole producer today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalInsertedDispositionV1 {
    _private: (),
}

impl CanonicalInsertedDispositionV1 {
    pub(crate) const fn from_canonical_source() -> Self {
        Self { _private: () }
    }
}

/// A canonical physical row for a single resolved owner.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalPhysicalSingleRowV1 {
    owner: FunctionOwnerIdV1,
    symbol: Box<str>,
    arity: usize,
    disposition: CanonicalInsertedDispositionV1,
}

impl CanonicalPhysicalSingleRowV1 {
    pub(in crate::mir) fn new(
        owner: FunctionOwnerIdV1,
        symbol: Box<str>,
        arity: usize,
        disposition: CanonicalInsertedDispositionV1,
    ) -> Self {
        Self {
            owner,
            symbol,
            arity,
            disposition,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) const fn disposition(&self) -> CanonicalInsertedDispositionV1 {
        self.disposition
    }
}

/// A canonical physical row for one callable catalog entry.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalPhysicalCallableRowV1 {
    key: CanonicalCallableKeyV1,
    symbol: Box<str>,
    arity: usize,
    disposition: CanonicalInsertedDispositionV1,
}

impl CanonicalPhysicalCallableRowV1 {
    pub(in crate::mir) fn new(
        key: CanonicalCallableKeyV1,
        symbol: Box<str>,
        arity: usize,
        disposition: CanonicalInsertedDispositionV1,
    ) -> Self {
        Self {
            key,
            symbol,
            arity,
            disposition,
        }
    }

    pub(crate) const fn key(&self) -> &CanonicalCallableKeyV1 {
        &self.key
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) const fn disposition(&self) -> CanonicalInsertedDispositionV1 {
        self.disposition
    }
}

/// Source-derived physical inventory, split by route shape so impossible
/// single/callable identity combinations cannot be assembled by callers.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CanonicalPhysicalDrainManifestV1 {
    Single {
        brand: ModuleInvocationBrandV1,
        family: ModuleInvocationFamilyV1,
        row: CanonicalPhysicalSingleRowV1,
    },
    Callable {
        brand: ModuleInvocationBrandV1,
        family: ModuleInvocationFamilyV1,
        rows: Box<[CanonicalPhysicalCallableRowV1]>,
    },
}

impl CanonicalPhysicalDrainManifestV1 {
    pub(in crate::mir) fn single(
        brand: ModuleInvocationBrandV1,
        family: ModuleInvocationFamilyV1,
        row: CanonicalPhysicalSingleRowV1,
    ) -> Self {
        Self::Single { brand, family, row }
    }

    pub(in crate::mir) fn callable(
        brand: ModuleInvocationBrandV1,
        family: ModuleInvocationFamilyV1,
        rows: Box<[CanonicalPhysicalCallableRowV1]>,
    ) -> Self {
        Self::Callable {
            brand,
            family,
            rows,
        }
    }

    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single { brand, .. } | Self::Callable { brand, .. } => *brand,
        }
    }

    pub(crate) const fn family(&self) -> ModuleInvocationFamilyV1 {
        match self {
            Self::Single { family, .. } | Self::Callable { family, .. } => *family,
        }
    }

    pub(crate) fn rows_len(&self) -> usize {
        match self {
            Self::Single { .. } => 1,
            Self::Callable { rows, .. } => rows.len(),
        }
    }

    pub(crate) fn single_row(&self) -> Option<&CanonicalPhysicalSingleRowV1> {
        match self {
            Self::Single { row, .. } => Some(row),
            Self::Callable { .. } => None,
        }
    }

    pub(crate) fn callable_rows(&self) -> Option<&[CanonicalPhysicalCallableRowV1]> {
        match self {
            Self::Single { .. } => None,
            Self::Callable { rows, .. } => Some(rows),
        }
    }
}
