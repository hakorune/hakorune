//! BORROW-P0-ROOT-P0b: collector-wide Main/condition batch transaction.
//!
//! The prepared owner consumes one collector and one already-validated root
//! batch. Every key, symbol, arity, and replacement is checked before the
//! first collector mutation; commit then applies only owned plans.

use super::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1, PreparedCollectorReplacementV1,
};
use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::builder::raw_expansion_receipt_ledger::RawRootMainCommitDispositionV1;
use crate::mir::builder::root_body_completion::CompletedRootBodyV1;
use crate::mir::builder::root_draft_batch::PreparedRootDraftBatchV1;
use crate::mir::MirFunction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootCollectorBatchPrepareErrorV1 {
    MainIdentityMismatch,
    MissingConditionFn,
    InvalidAdmissionCount {
        actual: usize,
    },
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
    MissingRootBody,
    ForeignBrand {
        expected: u64,
        actual: u64,
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
    root_body: CompletedRootBodyV1,
    entries: Box<[PreparedRootCollectorEntryV1]>,
    _seal: PreparedRootCollectorBatchSealV1,
}

#[derive(Debug)]
struct PreparedRootCollectorBatchSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RootCollectorBatchReceiptV1 {
    admissions: Box<[CollectedDraftAdmissionReceiptV1]>,
    root_body: CompletedRootBodyV1,
    _seal: RootCollectorBatchReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootCollectorBatchBrandErrorV1 {
    CollectorUnbranded,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct BrandedRootCollectorBatchReceiptV1 {
    admissions: Box<[InvocationBranded<CollectedDraftAdmissionReceiptV1>]>,
    root_body: CompletedRootBodyV1,
    brand: ModuleInvocationBrandV1,
    _seal: BrandedRootCollectorBatchReceiptSealV1,
}

#[derive(Debug)]
struct BrandedRootCollectorBatchReceiptSealV1;

impl BrandedRootCollectorBatchReceiptV1 {
    pub(in crate::mir::builder) fn admissions(
        &self,
    ) -> &[InvocationBranded<CollectedDraftAdmissionReceiptV1>] {
        &self.admissions
    }

    pub(in crate::mir::builder) fn root_body(&self) -> &CompletedRootBodyV1 {
        &self.root_body
    }

    pub(in crate::mir::builder) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        Box<[InvocationBranded<CollectedDraftAdmissionReceiptV1>]>,
        CompletedRootBodyV1,
        ModuleInvocationBrandV1,
    ) {
        (self.admissions, self.root_body, self.brand)
    }
}

#[derive(Debug)]
struct RootCollectorBatchReceiptSealV1;

impl RootCollectorBatchReceiptV1 {
    pub(in crate::mir::builder) fn admissions(&self) -> &[CollectedDraftAdmissionReceiptV1] {
        &self.admissions
    }

    pub(in crate::mir::builder) fn root_body(&self) -> &CompletedRootBodyV1 {
        &self.root_body
    }
}

impl ModuleDraftCollectorV1 {
    /// Borrow-only Main replacement fact used to co-seal collector and
    /// ledger preparation before either owner is mutated.
    pub(in crate::mir::builder) fn raw_root_main_disposition(
        &self,
    ) -> Result<RawRootMainCommitDispositionV1, ModuleDraftAdmissionErrorV1> {
        match plan_admission_v1(
            self,
            &FunctionDraftKeyV1::Main,
            "main",
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )? {
            PreparedCollectorReplacementV1::Legacy {
                symbol_key: None,
                key_symbol: None,
            } => Ok(RawRootMainCommitDispositionV1::Insert),
            PreparedCollectorReplacementV1::Legacy {
                symbol_key: Some(previous_key),
                key_symbol: Some(previous_symbol),
            } => Ok(RawRootMainCommitDispositionV1::ReplaceExact {
                previous_key,
                previous_symbol: previous_symbol.into_boxed_str(),
            }),
            PreparedCollectorReplacementV1::Legacy { .. }
            | PreparedCollectorReplacementV1::Canonical => {
                unreachable!("legacy Main admission returned a canonical plan")
            }
        }
    }

    /// Consume a batch after `validate_root_batch` has sealed all fallible
    /// admission facts. The only remaining failures are invariant breaks.
    pub(in crate::mir::builder) fn prepare_root_batch_preflighted(
        self,
        batch: PreparedRootDraftBatchV1,
    ) -> PreparedRootCollectorBatchV1 {
        self.prepare_root_batch(batch)
            .unwrap_or_else(|_| unreachable!("root collector preflight drifted before commit"))
    }

    /// Borrow-only root admission validation used by the retention preflight.
    ///
    /// The prepared batch and both collector indexes remain untouched.  The
    /// consuming `prepare_root_batch` terminal repeats the same checks when
    /// the later commit row is opened, but this method is the proof boundary
    /// that lets a rejected root retain the original batch owner.
    pub(in crate::mir::builder) fn validate_root_batch(
        &self,
        batch: &PreparedRootDraftBatchV1,
        brand: ModuleInvocationBrandV1,
    ) -> Result<(), RootCollectorBatchPrepareErrorV1> {
        if self.receipt_brand != Some(brand) {
            return Err(RootCollectorBatchPrepareErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: self
                    .receipt_brand
                    .map_or(0, ModuleInvocationBrandV1::ordinal),
            });
        }
        let root_body = batch
            .root_body()
            .ok_or(RootCollectorBatchPrepareErrorV1::MissingRootBody)?;
        if root_body.brand() != brand {
            return Err(RootCollectorBatchPrepareErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: root_body.brand().ordinal(),
            });
        }
        if batch.main().identity().symbol() != "main" || batch.main().identity().arity() != 0 {
            return Err(RootCollectorBatchPrepareErrorV1::MainIdentityMismatch);
        }
        let condition = batch
            .condition_fn()
            .ok_or(RootCollectorBatchPrepareErrorV1::MissingConditionFn)?;
        if condition.draft().signature.name != "condition_fn" {
            return Err(RootCollectorBatchPrepareErrorV1::DraftSymbolMismatch {
                ordinal: 1,
                expected: "condition_fn".into(),
                actual: condition.draft().signature.name.clone(),
            });
        }
        if condition.draft().signature.params.len() != 1 {
            return Err(RootCollectorBatchPrepareErrorV1::DraftArityMismatch {
                ordinal: 1,
                symbol: "condition_fn".into(),
                expected: 1,
                actual: condition.draft().signature.params.len(),
            });
        }
        if batch.admissions().len() != 2 {
            return Err(RootCollectorBatchPrepareErrorV1::InvalidAdmissionCount {
                actual: batch.admissions().len(),
            });
        }
        for (ordinal, admission) in batch.admissions().iter().enumerate() {
            let expected = if ordinal == 0 {
                (
                    &FunctionDraftKeyV1::Main,
                    "main",
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
            } else {
                (
                    &FunctionDraftKeyV1::SyntheticConditionFn,
                    "condition_fn",
                    1,
                    DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                )
            };
            if admission.key() != expected.0
                || admission.symbol() != expected.1
                || admission.arity() != expected.2
                || admission.policy() != expected.3
            {
                return Err(RootCollectorBatchPrepareErrorV1::Admission {
                    ordinal,
                    symbol: admission.symbol().to_owned(),
                    source: ModuleDraftAdmissionErrorV1::IndexDrift {
                        symbol: admission.symbol().to_owned(),
                        key: admission.key().clone(),
                    },
                });
            }
            let (draft_symbol, draft_arity) = if ordinal == 0 {
                (
                    &batch.main().draft().signature.name,
                    batch.main().draft().signature.params.len(),
                )
            } else {
                (
                    &condition.draft().signature.name,
                    condition.draft().signature.params.len(),
                )
            };
            if draft_symbol != expected.1 {
                return Err(RootCollectorBatchPrepareErrorV1::DraftSymbolMismatch {
                    ordinal,
                    expected: expected.1.into(),
                    actual: draft_symbol.clone(),
                });
            }
            if draft_arity != expected.2 {
                return Err(RootCollectorBatchPrepareErrorV1::DraftArityMismatch {
                    ordinal,
                    symbol: expected.1.into(),
                    expected: expected.2,
                    actual: draft_arity,
                });
            }
            if let Err(source) = plan_admission_v1(self, expected.0, expected.1, expected.3) {
                return Err(RootCollectorBatchPrepareErrorV1::Admission {
                    ordinal,
                    symbol: expected.1.into(),
                    source,
                });
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn prepare_root_batch(
        self,
        mut batch: PreparedRootDraftBatchV1,
    ) -> Result<PreparedRootCollectorBatchV1, RejectedRootCollectorBatchV1> {
        let root_body = match batch.take_root_body() {
            Some(root_body) => root_body,
            None => {
                return Err(reject(
                    self,
                    RootCollectorBatchPrepareErrorV1::MissingRootBody,
                ))
            }
        };
        if let Some(brand) = self.receipt_brand {
            if brand != root_body.brand() {
                return Err(reject(
                    self,
                    RootCollectorBatchPrepareErrorV1::ForeignBrand {
                        expected: brand.ordinal(),
                        actual: root_body.brand().ordinal(),
                    },
                ));
            }
        }
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
            root_body,
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
                root_body: self.root_body,
                _seal: RootCollectorBatchReceiptSealV1,
            },
        )
    }

    pub(in crate::mir::builder) fn commit_branded(
        mut self,
    ) -> Result<
        (ModuleDraftCollectorV1, BrandedRootCollectorBatchReceiptV1),
        RootCollectorBatchBrandErrorV1,
    > {
        let brand = self
            .collector
            .receipt_brand
            .ok_or(RootCollectorBatchBrandErrorV1::CollectorUnbranded)?;
        let mut admissions = Vec::with_capacity(self.entries.len());
        for entry in self.entries.into_vec() {
            let receipt = self.collector.collect_sealed(
                entry.key,
                entry.policy,
                entry.replacement,
                entry.draft,
            );
            admissions.push(InvocationBranded::from_source(brand, receipt));
        }
        Ok((
            self.collector,
            BrandedRootCollectorBatchReceiptV1 {
                admissions: admissions.into_boxed_slice(),
                root_body: self.root_body,
                brand,
                _seal: BrandedRootCollectorBatchReceiptSealV1,
            },
        ))
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
