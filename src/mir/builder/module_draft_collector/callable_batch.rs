//! CUT0-I0-COLLECT0-BATCH0: atomic canonical callable collection.
//!
//! A verified callable owner supplies the complete unpublished draft set. This
//! box performs every collector collision, key, symbol, arity, and duplicate
//! check before it mutates either collector index. The returned commit owner
//! can therefore collect the whole batch without a fallible step.

use std::collections::BTreeSet;

use super::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1, PreparedCollectorReplacementV1,
};
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::MirFunction;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CallableCollectorBatchPrepareErrorV1 {
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
    DuplicateBatchKey(FunctionDraftKeyV1),
    DuplicateBatchSymbol(String),
}

impl std::fmt::Display for CallableCollectorBatchPrepareErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][callable_batch/collector] {self:?}"
        )
    }
}

impl std::error::Error for CallableCollectorBatchPrepareErrorV1 {}

/// A complete callable draft row supplied by the verified unpublished owner.
/// Policy is fixed by this type's collector terminal; callers cannot select a
/// legacy replacement policy for a canonical callable batch.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableCollectorDraftEntryV1 {
    key: FunctionDraftKeyV1,
    symbol: String,
    arity: usize,
    draft: MirFunction,
}

impl CallableCollectorDraftEntryV1 {
    pub(in crate::mir::builder) fn new(
        key: FunctionDraftKeyV1,
        symbol: String,
        arity: usize,
        draft: MirFunction,
    ) -> Self {
        Self {
            key,
            symbol,
            arity,
            draft,
        }
    }
}

#[derive(Debug)]
struct PreparedCallableCollectorEntryV1 {
    key: FunctionDraftKeyV1,
    replacement: PreparedCollectorReplacementV1,
    draft: MirFunction,
}

/// The collector remains owned after a rejected preflight so the caller can
/// prove that no prefix or index was mutated before it drops the invocation.
#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedCallableCollectorBatchV1 {
    collector: ModuleDraftCollectorV1,
    error: CallableCollectorBatchPrepareErrorV1,
    _seal: RejectedCallableCollectorBatchSealV1,
}

#[derive(Debug)]
struct RejectedCallableCollectorBatchSealV1;

impl RejectedCallableCollectorBatchV1 {
    pub(in crate::mir::builder) fn collector(&self) -> &ModuleDraftCollectorV1 {
        &self.collector
    }

    pub(in crate::mir::builder) fn error(&self) -> &CallableCollectorBatchPrepareErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (ModuleDraftCollectorV1, CallableCollectorBatchPrepareErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableCollectorBatchV1 {
    collector: ModuleDraftCollectorV1,
    entries: Box<[PreparedCallableCollectorEntryV1]>,
    _seal: PreparedCallableCollectorBatchSealV1,
}

#[derive(Debug)]
struct PreparedCallableCollectorBatchSealV1;

#[derive(Debug)]
pub(in crate::mir) struct CallableCollectorBatchReceiptV1 {
    admissions: Box<[CollectedDraftAdmissionReceiptV1]>,
    _seal: CallableCollectorBatchReceiptSealV1,
}

/// Collector and whole-batch receipt move as one non-Clone product.
#[derive(Debug)]
pub(in crate::mir) struct CollectedCallableCollectorBatchV1 {
    collector: super::super::module_invocation_owner_chain::BrandedCollectorV1<
        ModuleDraftCollectorV1,
    >,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
}

impl CollectedCallableCollectorBatchV1 {
    pub(in crate::mir) fn receipt_brand(&self) -> ModuleInvocationBrandV1 {
        self.receipt.brand()
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        super::super::module_invocation_owner_chain::BrandedCollectorV1<ModuleDraftCollectorV1>,
        InvocationBranded<CallableCollectorBatchReceiptV1>,
    ) {
        (self.collector, self.receipt)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableCollectorBatchBrandErrorV1 {
    CollectorUnbranded,
}

#[derive(Debug)]
struct CallableCollectorBatchReceiptSealV1;

impl CallableCollectorBatchReceiptV1 {
    pub(in crate::mir::builder) fn admissions(&self) -> &[CollectedDraftAdmissionReceiptV1] {
        &self.admissions
    }

    pub(in crate::mir::builder) fn len(&self) -> usize {
        self.admissions.len()
    }
}

impl ModuleDraftCollectorV1 {
    /// Preflight the entire callable set without mutating either collector map.
    pub(in crate::mir::builder) fn prepare_callable_batch(
        self,
        entries: Vec<CallableCollectorDraftEntryV1>,
    ) -> Result<PreparedCallableCollectorBatchV1, RejectedCallableCollectorBatchV1> {
        let mut keys = BTreeSet::new();
        let mut symbols = BTreeSet::new();
        let mut prepared = Vec::with_capacity(entries.len());
        for (ordinal, entry) in entries.into_iter().enumerate() {
            let CallableCollectorDraftEntryV1 {
                key,
                symbol,
                arity,
                draft,
            } = entry;
            if !keys.insert(key.clone()) {
                return Err(reject(
                    self,
                    CallableCollectorBatchPrepareErrorV1::DuplicateBatchKey(key),
                ));
            }
            if !symbols.insert(symbol.clone()) {
                return Err(reject(
                    self,
                    CallableCollectorBatchPrepareErrorV1::DuplicateBatchSymbol(symbol),
                ));
            }
            let actual_symbol = draft.signature.name.clone();
            if actual_symbol != symbol {
                return Err(reject(
                    self,
                    CallableCollectorBatchPrepareErrorV1::DraftSymbolMismatch {
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
                    CallableCollectorBatchPrepareErrorV1::DraftArityMismatch {
                        ordinal,
                        symbol,
                        expected: arity,
                        actual: actual_arity,
                    },
                ));
            }
            let replacement = match plan_admission_v1(
                &self,
                &key,
                &symbol,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ) {
                Ok(replacement) => replacement,
                Err(source) => {
                    return Err(reject(
                        self,
                        CallableCollectorBatchPrepareErrorV1::Admission {
                            ordinal,
                            symbol,
                            source,
                        },
                    ));
                }
            };
            prepared.push(PreparedCallableCollectorEntryV1 {
                key,
                replacement,
                draft,
            });
        }
        Ok(PreparedCallableCollectorBatchV1 {
            collector: self,
            entries: prepared.into_boxed_slice(),
            _seal: PreparedCallableCollectorBatchSealV1,
        })
    }
}

impl PreparedCallableCollectorBatchV1 {
    /// All fallible work is complete; every row is collected in one terminal.
    pub(in crate::mir::builder) fn collect_all(
        mut self,
    ) -> (ModuleDraftCollectorV1, CallableCollectorBatchReceiptV1) {
        let mut admissions = Vec::with_capacity(self.entries.len());
        for entry in self.entries.into_vec() {
            admissions.push(self.collector.collect_sealed(
                entry.key,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                entry.replacement,
                entry.draft,
            ));
        }
        (
            self.collector,
            CallableCollectorBatchReceiptV1 {
                admissions: admissions.into_boxed_slice(),
                _seal: CallableCollectorBatchReceiptSealV1,
            },
        )
    }

    /// The physical collector issues the whole-batch receipt with its own
    /// brand; no post-hoc receipt relabeling is a production terminal.
    pub(in crate::mir::builder) fn collect_all_branded(
        mut self,
    ) -> Result<CollectedCallableCollectorBatchV1, CallableCollectorBatchBrandErrorV1> {
        let brand = self
            .collector
            .receipt_brand
            .ok_or(CallableCollectorBatchBrandErrorV1::CollectorUnbranded)?;
        let mut admissions = Vec::with_capacity(self.entries.len());
        for entry in self.entries.into_vec() {
            admissions.push(self.collector.collect_sealed(
                entry.key,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                entry.replacement,
                entry.draft,
            ));
        }
        Ok(CollectedCallableCollectorBatchV1 {
            collector: InvocationBranded::from_source(brand, self.collector),
            receipt: InvocationBranded::from_source(
                brand,
                CallableCollectorBatchReceiptV1 {
                    admissions: admissions.into_boxed_slice(),
                    _seal: CallableCollectorBatchReceiptSealV1,
                },
            ),
        })
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: CallableCollectorBatchPrepareErrorV1,
) -> RejectedCallableCollectorBatchV1 {
    RejectedCallableCollectorBatchV1 {
        collector,
        error,
        _seal: RejectedCallableCollectorBatchSealV1,
    }
}

fn plan_admission_v1(
    collector: &ModuleDraftCollectorV1,
    key: &FunctionDraftKeyV1,
    symbol: &str,
    policy: DraftPublicationPolicyV1,
) -> Result<PreparedCollectorReplacementV1, ModuleDraftAdmissionErrorV1> {
    if policy != DraftPublicationPolicyV1::CanonicalRejectDuplicate {
        unreachable!("callable collector batch policy is canonical-only");
    }
    if collector.drafts.contains_key(key) {
        return Err(ModuleDraftAdmissionErrorV1::DuplicateKey(key.clone()));
    }
    if collector.key_by_symbol.contains_key(symbol) {
        return Err(ModuleDraftAdmissionErrorV1::DuplicateSymbol(symbol.to_owned()));
    }
    Ok(PreparedCollectorReplacementV1::Canonical)
}
