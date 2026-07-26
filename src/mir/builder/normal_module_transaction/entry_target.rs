//! Canonical physical `main/0` identity, independent from Raw publication.

use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::canonical_physical_drain::CanonicalInsertedDispositionV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct CanonicalNormalMainEntryTargetV1 {
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    disposition: CanonicalInsertedDispositionV1,
    _seal: CanonicalNormalMainEntryTargetSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct CanonicalNormalMainEntryTargetSealV1;

impl CanonicalNormalMainEntryTargetV1 {
    pub(in crate::mir) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir) fn is_main(&self) -> bool {
        self.key == FunctionDraftKeyV1::Main
    }

    #[cfg(test)]
    pub(in crate::mir) fn from_unchecked_parts_for_test(
        symbol: impl Into<Box<str>>,
        arity: usize,
    ) -> Self {
        Self {
            key: FunctionDraftKeyV1::Main,
            symbol: symbol.into(),
            arity,
            disposition: CanonicalInsertedDispositionV1::from_canonical_source(),
            _seal: CanonicalNormalMainEntryTargetSealV1,
        }
    }
}

/// Sole producer for the canonical normal physical-entry identity.
pub(in crate::mir) fn canonical_normal_main_entry_target() -> CanonicalNormalMainEntryTargetV1 {
    CanonicalNormalMainEntryTargetV1 {
        key: FunctionDraftKeyV1::Main,
        symbol: "main".into(),
        arity: 0,
        disposition: CanonicalInsertedDispositionV1::from_canonical_source(),
        _seal: CanonicalNormalMainEntryTargetSealV1,
    }
}
