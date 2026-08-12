//! MODULEDRAFT0-S0: one disconnected owner for unpublished function drafts.
//!
//! This vocabulary has no Builder, module, fact-session, or publication
//! consumer yet. It establishes the one collector that later receives a
//! completed draft together with its sealed fact session.

use std::collections::BTreeMap;

use super::module_invocation_identity::ModuleInvocationBrandV1;
use super::module_invocation_owner_chain::InvocationBranded;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};
use crate::mir::{FunctionSignature, MirFunction};

mod callable_batch;
mod collected_product;
mod drain;
mod final_row;
mod normal_collector_drain_lifecycle;
mod raw_drain;
mod receipt;
mod root_batch;
mod static_result_publication_owner;

pub(in crate::mir::builder) use drain::{
    CanonicalCollectorDrainErrorV1, CanonicalCollectorReceiptViewV1,
    PreparedCanonicalCollectorDrainV1, RejectedCanonicalCollectorDrainV1,
};
pub(in crate::mir::builder) use raw_drain::{
    raw_collector_from_branded, PreparedRawCollectorDrainV1, RawCollectorDrainErrorV1,
    RejectedRawCollectorDrainV1,
};

pub(in crate::mir::builder) use callable_batch::{
    CallableCollectorBatchBrandErrorV1, CallableCollectorBatchPrepareErrorV1,
    CallableCollectorBatchReceiptV1, CallableCollectorDraftEntryV1,
    CollectedCallableCollectorBatchV1, PreparedCallableCollectorBatchV1,
    RejectedCallableCollectorBatchV1,
};

pub(in crate::mir) use callable_batch::CallableCollectorBatchReceiptV1 as CommitCallableCollectorBatchReceiptV1;
pub(in crate::mir::builder) use collected_product::{
    CollectedDraftAdmissionProductErrorV1, CollectedDraftAdmissionProductV1,
    RejectedCollectedDraftAdmissionV1,
};
pub(in crate::mir) use receipt::CollectedDraftAdmissionReceiptV1 as CommitCollectedDraftAdmissionReceiptV1;
pub(in crate::mir) use receipt::{
    CollectedDraftAdmissionReceiptV1, CollectedDraftReplacementDispositionV1,
};
pub(in crate::mir::builder) use root_batch::{
    BrandedRootCollectorBatchReceiptV1, PreparedRootCollectorBatchV1, RejectedRootCollectorBatchV1,
    RootCollectorBatchBrandErrorV1, RootCollectorBatchPrepareErrorV1, RootCollectorBatchReceiptV1,
};

/// Semantic identity for one draft admission, distinct from fact generation.
#[allow(dead_code)] // S0 exposes every future physical identity before I0 connects callers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum FunctionDraftKeyV1 {
    Main,
    LegacySymbol(String),
    CanonicalResolvedOwner(FunctionOwnerIdV1),
    CanonicalCallable(CanonicalCallableKeyV1),
    /// Cataloged Box-method identity; unlike `CanonicalCallable`, this keeps
    /// the same-module namespace and owner/name/arity together.
    CatalogedBoxMethod(CanonicalSameModuleCallableKeyV1),
    SyntheticConditionFn,
}

/// Preserve each current route family's duplicate behavior at collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DraftPublicationPolicyV1 {
    LegacyReplaceWholePair,
    CanonicalRejectDuplicate,
}

/// Typed preflight failure before a completed draft enters the collector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleDraftAdmissionErrorV1 {
    DuplicateKey(FunctionDraftKeyV1),
    DuplicateSymbol(String),
    IndexDrift {
        symbol: String,
        key: FunctionDraftKeyV1,
    },
    SymbolMismatch {
        expected: String,
        actual: String,
    },
    ArityMismatch {
        symbol: String,
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CollectorReceiptBrandErrorV1 {
    CollectorUnbranded,
}

impl std::fmt::Display for CollectorReceiptBrandErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][module_draft/receipt_brand] {self:?}"
        )
    }
}

impl std::error::Error for CollectorReceiptBrandErrorV1 {}

impl std::fmt::Display for ModuleDraftAdmissionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][module_draft/admission] {self:?}"
        )
    }
}

impl std::error::Error for ModuleDraftAdmissionErrorV1 {}

/// A fully checked admission which can consume exactly one matching draft.
///
/// Collector collisions are checked during preparation, while the physical
/// draft's symbol and arity are checked by `seal`. Commit therefore cannot
/// fail after a child terminal has elected collect-before-restore.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedFunctionDraftAdmissionV1<'collector> {
    collector: &'collector mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    expected_symbol: String,
    expected_arity: usize,
    policy: DraftPublicationPolicyV1,
    replacement: PreparedCollectorReplacementV1,
}

/// All legacy index removals are decided before the child session is restored.
/// Commit only applies these owned removals and performs no fallible lookup or
/// post-mutation assertion.
#[derive(Debug)]
enum PreparedCollectorReplacementV1 {
    Legacy {
        symbol_key: Option<FunctionDraftKeyV1>,
        key_symbol: Option<String>,
    },
    Canonical,
}

impl<'collector> PreparedFunctionDraftAdmissionV1<'collector> {
    pub(in crate::mir::builder) fn seal(
        self,
        draft: MirFunction,
    ) -> Result<UnpublishedFunctionDraftV1<'collector>, ModuleDraftAdmissionErrorV1> {
        let actual_symbol = draft.signature.name.clone();
        if actual_symbol != self.expected_symbol {
            return Err(ModuleDraftAdmissionErrorV1::SymbolMismatch {
                expected: self.expected_symbol,
                actual: actual_symbol,
            });
        }
        let actual_arity = draft.signature.params.len();
        if actual_arity != self.expected_arity {
            return Err(ModuleDraftAdmissionErrorV1::ArityMismatch {
                symbol: draft.signature.name.clone(),
                expected: self.expected_arity,
                actual: actual_arity,
            });
        }
        Ok(UnpublishedFunctionDraftV1 {
            collector: self.collector,
            key: self.key,
            policy: self.policy,
            replacement: self.replacement,
            draft,
            _seal: UnpublishedFunctionDraftSealV1,
        })
    }

    /// Infallible terminal for callers that already compared the exact draft
    /// symbol and arity against this admission. The debug assertions guard
    /// the local preflight contract; collection remains a move-only commit.
    pub(in crate::mir::builder) fn seal_after_exact_signature_preflight(
        self,
        draft: MirFunction,
    ) -> UnpublishedFunctionDraftV1<'collector> {
        debug_assert_eq!(draft.signature.name, self.expected_symbol);
        debug_assert_eq!(draft.signature.params.len(), self.expected_arity);
        UnpublishedFunctionDraftV1 {
            collector: self.collector,
            key: self.key,
            policy: self.policy,
            replacement: self.replacement,
            draft,
            _seal: UnpublishedFunctionDraftSealV1,
        }
    }
}

/// One non-Clone completed draft which has not entered a `MirModule`.
#[derive(Debug)]
pub(in crate::mir::builder) struct UnpublishedFunctionDraftV1<'collector> {
    collector: &'collector mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    policy: DraftPublicationPolicyV1,
    replacement: PreparedCollectorReplacementV1,
    draft: MirFunction,
    _seal: UnpublishedFunctionDraftSealV1,
}

#[derive(Debug)]
struct UnpublishedFunctionDraftSealV1;

use final_row::{CollectedDraftFinalAdmissionV1, CollectedFunctionDraftV1};

impl UnpublishedFunctionDraftV1<'_> {
    /// Commit cannot fail: its collector was exclusively borrowed when
    /// admission preflight completed, so no collision can have appeared.
    fn collect_inner(self) -> CollectedDraftAdmissionReceiptV1 {
        let Self {
            collector,
            key,
            policy,
            replacement,
            draft,
            _seal: _,
        } = self;
        collector.collect_sealed(key, policy, replacement, draft)
    }

    pub(in crate::mir::builder) fn collect(self) -> CollectedDraftAdmissionReceiptV1 {
        self.collect_inner()
    }

    pub(in crate::mir::builder) fn collect_branded(
        self,
    ) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, CollectorReceiptBrandErrorV1>
    {
        let brand = self
            .collector
            .receipt_brand
            .ok_or(CollectorReceiptBrandErrorV1::CollectorUnbranded)?;
        Ok(InvocationBranded::from_source(brand, self.collect_inner()))
    }
}

/// The only signature/header view admitted during the temporary collection era.
///
/// It projects the signature owned by the same draft; it is not a copied
/// header cache and it cannot expose function body or metadata authority.
pub(in crate::mir::builder) trait CompletedDraftSignatureViewV1 {
    fn signature(&self, symbol: &str) -> Option<&FunctionSignature>;

    /// Exact header presence without exposing a second draft store.
    fn contains_symbol(&self, symbol: &str) -> bool;

    /// Deterministic header inventory cardinality.
    fn symbol_count(&self) -> usize;

    /// Visit exact owned draft symbols in deterministic collector order.
    ///
    /// This is deliberately a visitor instead of a cloned header list. The
    /// collector's `BTreeMap` remains the sole inventory owner.
    fn visit_symbols(&self, visitor: &mut dyn FnMut(&str));
}

/// Single owner for all module-invocation unpublished function drafts.
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct ModuleDraftCollectorV1 {
    drafts: BTreeMap<FunctionDraftKeyV1, CollectedFunctionDraftV1>,
    key_by_symbol: BTreeMap<String, FunctionDraftKeyV1>,
    receipt_brand: Option<ModuleInvocationBrandV1>,
    static_result_publication_owner: Option<
        crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1,
    >,
    _seal: ModuleDraftCollectorSealV1,
}
#[derive(Debug, Default)]
struct ModuleDraftCollectorSealV1;

impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn with_brand(brand: ModuleInvocationBrandV1) -> Self {
        Self {
            receipt_brand: Some(brand),
            ..Self::default()
        }
    }

    pub(in crate::mir) fn receipt_brand(&self) -> Option<ModuleInvocationBrandV1> {
        self.receipt_brand
    }

    /// Prepare every fallible collector admission check before child teardown.
    pub(in crate::mir::builder) fn prepare_admission(
        &mut self,
        key: FunctionDraftKeyV1,
        expected_symbol: String,
        expected_arity: usize,
        policy: DraftPublicationPolicyV1,
    ) -> Result<PreparedFunctionDraftAdmissionV1<'_>, ModuleDraftAdmissionErrorV1> {
        let replacement = root_batch::plan_admission_v1(self, &key, &expected_symbol, policy)?;
        Ok(PreparedFunctionDraftAdmissionV1 {
            collector: self,
            key,
            expected_symbol,
            expected_arity,
            policy,
            replacement,
        })
    }

    #[cfg(test)]
    fn inject_symbol_index_drift_for_test(&mut self, symbol: &str, key: FunctionDraftKeyV1) {
        self.key_by_symbol.insert(symbol.to_owned(), key);
    }

    /// Infallibly insert or replace a draft that completed prepared admission.
    fn collect_sealed(
        &mut self,
        key: FunctionDraftKeyV1,
        policy: DraftPublicationPolicyV1,
        replacement: PreparedCollectorReplacementV1,
        draft: MirFunction,
    ) -> CollectedDraftAdmissionReceiptV1 {
        let symbol = draft.signature.name.clone();
        let arity = draft.signature.params.len();
        let replacement_disposition = match (&policy, &replacement) {
            (
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
                PreparedCollectorReplacementV1::Legacy {
                    symbol_key: Some(previous_key),
                    key_symbol: Some(previous_symbol),
                },
            ) => CollectedDraftReplacementDispositionV1::ReplacedWholePair {
                previous_key: previous_key.clone(),
                previous_symbol: previous_symbol.clone().into_boxed_str(),
            },
            (
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
                PreparedCollectorReplacementV1::Legacy {
                    symbol_key: None,
                    key_symbol: None,
                },
            )
            | (
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                PreparedCollectorReplacementV1::Canonical,
            ) => CollectedDraftReplacementDispositionV1::Inserted,
            _ => unreachable!("prepared collector policy/replacement mismatch"),
        };
        let receipt = CollectedDraftAdmissionReceiptV1::new(
            key.clone(),
            symbol.clone().into_boxed_str(),
            arity,
            policy,
            replacement_disposition.clone(),
            self.receipt_brand,
        );
        match (policy, replacement) {
            (
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
                PreparedCollectorReplacementV1::Legacy {
                    symbol_key,
                    key_symbol,
                },
            ) => {
                if let Some(existing_key) = symbol_key {
                    self.key_by_symbol.remove(&symbol);
                    self.drafts.remove(&existing_key);
                }
                if let Some(existing_symbol) = key_symbol {
                    self.drafts.remove(&key);
                    self.key_by_symbol.remove(&existing_symbol);
                }
            }
            (
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                PreparedCollectorReplacementV1::Canonical,
            ) => {}
            _ => unreachable!("prepared collector policy/replacement mismatch"),
        }
        let final_admission = CollectedDraftFinalAdmissionV1::new(
            key.clone(),
            symbol.clone().into_boxed_str(),
            arity,
            policy,
            replacement_disposition.clone(),
        );
        self.key_by_symbol.insert(symbol, key.clone());
        self.drafts.insert(
            key,
            CollectedFunctionDraftV1 {
                draft,
                admission: final_admission,
            },
        );
        receipt
    }

    /// Move the collector-owned drafts out only for the invocation drain.
    ///
    /// No header or identity side table is returned: the physical drafts and
    /// their collector indexes are consumed together by the one drain owner.
    pub(in crate::mir::builder) fn into_draft_functions(self) -> Vec<MirFunction> {
        self.drafts.into_values().map(|entry| entry.draft).collect()
    }

    pub(in crate::mir::builder) fn key_for_symbol(
        &self,
        symbol: &str,
    ) -> Option<&FunctionDraftKeyV1> {
        self.key_by_symbol.get(symbol)
    }
}

impl CompletedDraftSignatureViewV1 for ModuleDraftCollectorV1 {
    fn signature(&self, symbol: &str) -> Option<&FunctionSignature> {
        let key = self.key_by_symbol.get(symbol)?;
        self.drafts.get(key).map(|draft| &draft.draft.signature)
    }

    fn contains_symbol(&self, symbol: &str) -> bool {
        self.key_by_symbol.contains_key(symbol)
    }

    fn symbol_count(&self) -> usize {
        self.key_by_symbol.len()
    }

    fn visit_symbols(&self, visitor: &mut dyn FnMut(&str)) {
        for symbol in self.key_by_symbol.keys() {
            visitor(symbol);
        }
    }
}

#[cfg(test)]
mod tests;
