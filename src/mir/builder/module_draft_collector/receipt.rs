//! WIRING-I0-ROUTEINV-P0a-RECEIPT-S0 collector commit receipt.
//!
//! A receipt is issued only by the parent collector module after its
//! preflighted, infallible commit. It is an ephemeral completion witness, not
//! a second draft store or a publication/retry capability.

use super::super::module_invocation_identity::ModuleInvocationBrandV1;
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1};

#[cfg(test)]
use super::ModuleDraftCollectorV1;
#[cfg(test)]
use crate::mir::FunctionSignature;

/// Exact effect of one successful collector admission on the prior pair.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CollectedDraftReplacementDispositionV1 {
    Inserted,
    ReplacedWholePair {
        previous_key: FunctionDraftKeyV1,
        previous_symbol: Box<str>,
    },
}

/// Non-Clone witness returned after exactly one successful draft collection.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct CollectedDraftAdmissionReceiptV1 {
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
    replacement: CollectedDraftReplacementDispositionV1,
    collector_brand: Option<ModuleInvocationBrandV1>,
    _seal: CollectedDraftAdmissionReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct CollectedDraftAdmissionReceiptSealV1;

impl CollectedDraftAdmissionReceiptV1 {
    /// Restricted to the parent collector module; sibling lowering modules
    /// cannot synthesize a successful collection receipt.
    pub(super) fn new(
        key: FunctionDraftKeyV1,
        symbol: Box<str>,
        arity: usize,
        policy: DraftPublicationPolicyV1,
        replacement: CollectedDraftReplacementDispositionV1,
        collector_brand: Option<ModuleInvocationBrandV1>,
    ) -> Self {
        Self {
            key,
            symbol,
            arity,
            policy,
            replacement,
            collector_brand,
            _seal: CollectedDraftAdmissionReceiptSealV1,
        }
    }

    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) const fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir::builder) const fn policy(&self) -> DraftPublicationPolicyV1 {
        self.policy
    }

    pub(in crate::mir::builder) const fn replacement(
        &self,
    ) -> &CollectedDraftReplacementDispositionV1 {
        &self.replacement
    }

    pub(in crate::mir::builder) const fn collector_brand(&self) -> Option<ModuleInvocationBrandV1> {
        self.collector_brand
    }
}

/// Test-only exact view used to prove that rejected receipt paths preserve
/// both collector indexes. It never exists in a production build.
#[cfg(test)]
#[derive(Debug, PartialEq)]
pub(in crate::mir::builder) struct ModuleDraftCollectorReceiptProofSnapshotV1 {
    draft_rows: Box<[(FunctionDraftKeyV1, FunctionSignature)]>,
    symbol_rows: Box<[(Box<str>, FunctionDraftKeyV1)]>,
}

#[cfg(test)]
impl ModuleDraftCollectorReceiptProofSnapshotV1 {
    pub(in crate::mir::builder) fn is_bijective(&self) -> bool {
        self.draft_rows.len() == self.symbol_rows.len()
            && self.draft_rows.iter().all(|(key, signature)| {
                self.symbol_rows.iter().any(|(symbol, indexed_key)| {
                    symbol.as_ref() == signature.name && indexed_key == key
                })
            })
            && self.symbol_rows.iter().all(|(symbol, key)| {
                self.draft_rows.iter().any(|(draft_key, signature)| {
                    draft_key == key && signature.name == symbol.as_ref()
                })
            })
    }
}

#[cfg(test)]
impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn receipt_proof_snapshot(
        &self,
    ) -> ModuleDraftCollectorReceiptProofSnapshotV1 {
        let draft_rows = self
            .drafts
            .iter()
            .map(|(key, entry)| (key.clone(), entry.draft.signature.clone()))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let symbol_rows = self
            .key_by_symbol
            .iter()
            .map(|(symbol, key)| (symbol.clone().into_boxed_str(), key.clone()))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        ModuleDraftCollectorReceiptProofSnapshotV1 {
            draft_rows,
            symbol_rows,
        }
    }
}
