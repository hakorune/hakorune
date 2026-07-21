//! WIRING-I0-ROUTEINV-P0a-RECEIPT-S0 collector commit receipt.
//!
//! A receipt is issued only by the parent collector module after its
//! preflighted, infallible commit. It is an ephemeral completion witness, not
//! a second draft store or a publication/retry capability.

use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1};

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
pub(in crate::mir::builder) struct CollectedDraftAdmissionReceiptV1 {
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
    replacement: CollectedDraftReplacementDispositionV1,
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
    ) -> Self {
        Self {
            key,
            symbol,
            arity,
            policy,
            replacement,
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
}
