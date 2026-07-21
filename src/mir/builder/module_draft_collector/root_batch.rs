//! BORROW-P0-ROOT-P0b: collector-wide Main/condition batch transaction.
//!
//! The prepared owner consumes one collector and one already-validated root
//! batch. Every key, symbol, arity, and replacement is checked before the
//! first collector mutation; commit then applies only owned plans.

use super::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1, PreparedCollectorReplacementV1,
};
use crate::mir::builder::root_draft_batch::PreparedRootDraftBatchV1;
use crate::mir::MirFunction;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootCollectorBatchPrepareErrorV1 {
    Admission {
        ordinal: usize,
        symbol: String,
        source: ModuleDraftAdmissionErrorV1,
    },
    DraftSymbolMismatch {
        ordinal: usize,
        expected: String,
        actual: String,
    },
    DraftArityMismatch {
        ordinal: usize,
        symbol: String,
        expected: usize,
        actual: usize,
    },
}

impl std::fmt::Display for RootCollectorBatchPrepareErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][root_batch/collector] {self:?}"
        )
    }
}

impl std::error::Error for RootCollectorBatchPrepareErrorV1 {}

#[derive(Debug)]
struct PreparedRootCollectorEntryV1 {
    key: FunctionDraftKeyV1,
    policy: DraftPublicationPolicyV1,
    replacement: PreparedCollectorReplacementV1,
    draft: MirFunction,
}

/// Failure retains the unchanged collector so the invocation owner may drop
/// the whole candidate or prove the exact prefix without reconstructing it.
#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedRootCollectorBatchV1 {
    collector: ModuleDraftCollectorV1,
    error: RootCollectorBatchPrepareErrorV1,
    _seal: RejectedRootCollectorBatchSealV1,
}

#[derive(Debug)]
struct RejectedRootCollectorBatchSealV1;

impl RejectedRootCollectorBatchV1 {
    pub(in crate::mir::builder) fn collector(&self) -> &ModuleDraftCollectorV1 {
        &self.collector
    }

    pub(in crate::mir::builder) fn error(&self) -> &RootCollectorBatchPrepareErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (ModuleDraftCollectorV1, RootCollectorBatchPrepareErrorV1) {
        (self.collector, self.error)
    }
}

/// Non-Clone, single-use commit owner. All fallible work is complete.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRootCollectorBatchV1 {
    collector: ModuleDraftCollectorV1,
    entries: Box<[PreparedRootCollectorEntryV1]>,
    _seal: PreparedRootCollectorBatchSealV1,
}

#[derive(Debug)]
struct PreparedRootCollectorBatchSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RootCollectorBatchReceiptV1 {
    admissions: Box<[CollectedDraftAdmissionReceiptV1]>,
    _seal: RootCollectorBatchReceiptSealV1,
}

#[derive(Debug)]
struct RootCollectorBatchReceiptSealV1;

impl RootCollectorBatchReceiptV1 {
    pub(in crate::mir::builder) fn admissions(&self) -> &[CollectedDraftAdmissionReceiptV1] {
        &self.admissions
    }
}

impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn prepare_root_batch(
        self,
        batch: PreparedRootDraftBatchV1,
    ) -> Result<PreparedRootCollectorBatchV1, RejectedRootCollectorBatchV1> {
        let mut entries = Vec::new();
        for (ordinal, entry) in batch
            .into_collector_entries()
            .into_vec()
            .into_iter()
            .enumerate()
        {
            let (key, symbol, arity, policy, draft) = entry.into_parts();
            let actual_symbol = draft.signature.name.clone();
            if actual_symbol != symbol {
                return Err(reject(
                    self,
                    RootCollectorBatchPrepareErrorV1::DraftSymbolMismatch {
                        ordinal,
                        expected: symbol,
                        actual: actual_symbol,
                    },
                ));
            }
            let actual_arity = draft.signature.params.len();
            if actual_arity != arity {
                return Err(reject(
                    self,
                    RootCollectorBatchPrepareErrorV1::DraftArityMismatch {
                        ordinal,
                        symbol,
                        expected: arity,
                        actual: actual_arity,
                    },
                ));
            }
            let replacement = match plan_admission_v1(&self, &key, &symbol, policy) {
                Ok(replacement) => replacement,
                Err(source) => {
                    return Err(reject(
                        self,
                        RootCollectorBatchPrepareErrorV1::Admission {
                            ordinal,
                            symbol,
                            source,
                        },
                    ));
                }
            };
            entries.push(PreparedRootCollectorEntryV1 {
                key,
                policy,
                replacement,
                draft,
            });
        }
        Ok(PreparedRootCollectorBatchV1 {
            collector: self,
            entries: entries.into_boxed_slice(),
            _seal: PreparedRootCollectorBatchSealV1,
        })
    }
}

impl PreparedRootCollectorBatchV1 {
    /// Commit is infallible because every entry owns its exact replacement
    /// plan and every physical draft was checked before this product issued.
    pub(in crate::mir::builder) fn commit(
        mut self,
    ) -> (ModuleDraftCollectorV1, RootCollectorBatchReceiptV1) {
        let mut admissions = Vec::with_capacity(self.entries.len());
        for entry in self.entries.into_vec() {
            admissions.push(self.collector.collect_sealed(
                entry.key,
                entry.policy,
                entry.replacement,
                entry.draft,
            ));
        }
        (
            self.collector,
            RootCollectorBatchReceiptV1 {
                admissions: admissions.into_boxed_slice(),
                _seal: RootCollectorBatchReceiptSealV1,
            },
        )
    }
}

pub(super) fn plan_admission_v1(
    collector: &ModuleDraftCollectorV1,
    key: &FunctionDraftKeyV1,
    expected_symbol: &str,
    policy: DraftPublicationPolicyV1,
) -> Result<PreparedCollectorReplacementV1, ModuleDraftAdmissionErrorV1> {
    let symbol_key = collector.key_by_symbol.get(expected_symbol).cloned();
    let key_symbol = collector
        .drafts
        .get(key)
        .map(|entry| entry.draft.signature.name.clone());
    match policy {
        DraftPublicationPolicyV1::CanonicalRejectDuplicate => {
            if collector.drafts.contains_key(key) {
                return Err(ModuleDraftAdmissionErrorV1::DuplicateKey(key.clone()));
            }
            if collector.key_by_symbol.contains_key(expected_symbol) {
                return Err(ModuleDraftAdmissionErrorV1::DuplicateSymbol(
                    expected_symbol.to_owned(),
                ));
            }
            Ok(PreparedCollectorReplacementV1::Canonical)
        }
        DraftPublicationPolicyV1::LegacyReplaceWholePair => {
            let pairing_matches = match (&symbol_key, &key_symbol) {
                (Some(symbol_key), Some(key_symbol)) => {
                    symbol_key == key && key_symbol == expected_symbol
                }
                (None, None) => true,
                _ => false,
            };
            if !pairing_matches {
                return Err(ModuleDraftAdmissionErrorV1::IndexDrift {
                    symbol: expected_symbol.to_owned(),
                    key: key.clone(),
                });
            }
            Ok(PreparedCollectorReplacementV1::Legacy {
                symbol_key,
                key_symbol,
            })
        }
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: RootCollectorBatchPrepareErrorV1,
) -> RejectedRootCollectorBatchV1 {
    RejectedRootCollectorBatchV1 {
        collector,
        error,
        _seal: RejectedRootCollectorBatchSealV1,
    }
}
