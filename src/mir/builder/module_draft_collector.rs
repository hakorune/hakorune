//! MODULEDRAFT0-S0: one disconnected owner for unpublished function drafts.
//!
//! This vocabulary has no Builder, module, fact-session, or publication
//! consumer yet. It establishes the one collector that later receives a
//! completed draft together with its sealed fact session.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};
use crate::mir::{FunctionSignature, MirFunction};
use super::module_invocation_identity::ModuleInvocationBrandV1;
use super::module_invocation_owner_chain::InvocationBranded;

mod receipt;
mod callable_batch;
mod collected_product;
mod drain;
mod root_batch;

pub(in crate::mir::builder) use drain::{
    CanonicalCollectorReceiptViewV1, CanonicalCollectorDrainErrorV1,
    PreparedCanonicalCollectorDrainV1, RejectedCanonicalCollectorDrainV1,
};

pub(in crate::mir::builder) use callable_batch::{
    CallableCollectorBatchPrepareErrorV1, CallableCollectorDraftEntryV1,
    CallableCollectorBatchReceiptV1, CallableCollectorBatchBrandErrorV1,
    CollectedCallableCollectorBatchV1,
    PreparedCallableCollectorBatchV1,
    RejectedCallableCollectorBatchV1,
};

pub(in crate::mir::builder) use receipt::{
    CollectedDraftAdmissionReceiptV1, CollectedDraftReplacementDispositionV1,
};
pub(in crate::mir) use callable_batch::CallableCollectorBatchReceiptV1 as CommitCallableCollectorBatchReceiptV1;
pub(in crate::mir) use receipt::CollectedDraftAdmissionReceiptV1 as CommitCollectedDraftAdmissionReceiptV1;
pub(in crate::mir::builder) use collected_product::{
    CollectedDraftAdmissionProductErrorV1, CollectedDraftAdmissionProductV1,
    RejectedCollectedDraftAdmissionV1,
};
pub(in crate::mir::builder) use root_batch::{
    BrandedRootCollectorBatchReceiptV1, PreparedRootCollectorBatchV1,
    RejectedRootCollectorBatchV1, RootCollectorBatchBrandErrorV1,
    RootCollectorBatchPrepareErrorV1, RootCollectorBatchReceiptV1,
};

/// Semantic identity for one draft admission, distinct from fact generation.
#[allow(dead_code)] // S0 exposes every future physical identity before I0 connects callers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum FunctionDraftKeyV1 {
    Main,
    LegacySymbol(String),
    CanonicalResolvedOwner(FunctionOwnerIdV1),
    CanonicalCallable(CanonicalCallableKeyV1),
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
        write!(formatter, "[freeze:contract][module_draft/receipt_brand] {self:?}")
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

/// Collector-owned form after infallible admission commit.
///
/// This remains private so callers cannot separate a collected draft from its
/// collector identity or invent a second draft store.
#[derive(Debug)]
struct CollectedFunctionDraftV1 {
    draft: MirFunction,
}

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
            replacement_disposition,
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
        self.key_by_symbol.insert(symbol, key.clone());
        self.drafts.insert(key, CollectedFunctionDraftV1 { draft });
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
mod tests {
    use super::{
        CompletedDraftSignatureViewV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
        ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1,
    };
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn header_view_borrows_the_same_collector_owned_draft() {
        let mut collector = ModuleDraftCollectorV1::default();
        let prepared = collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("Parser.skip/1".into()),
                "Parser.skip/1".into(),
                1,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap();
        prepared.seal(draft("Parser.skip/1", 1)).unwrap().collect();

        let signature = collector.signature("Parser.skip/1").unwrap();
        assert_eq!(signature.params, vec![MirType::Integer]);
        assert_eq!(signature.return_type, MirType::Integer);
        assert!(collector.contains_symbol("Parser.skip/1"));
        assert!(!collector.contains_symbol("Parser.missing/0"));
        assert_eq!(collector.symbol_count(), 1);
    }

    #[test]
    fn header_view_visits_same_owned_symbols_in_deterministic_order() {
        let mut collector = ModuleDraftCollectorV1::default();
        for symbol in ["Zeta.run/0", "Alpha.run/2"] {
            let arity = symbol.rsplit_once('/').unwrap().1.parse::<usize>().unwrap();
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol(symbol.into()),
                    symbol.into(),
                    arity,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            prepared.seal(draft(symbol, arity)).unwrap().collect();
        }

        let mut visited = Vec::new();
        collector.visit_symbols(&mut |symbol| visited.push(symbol.to_owned()));
        assert_eq!(visited, ["Alpha.run/2", "Zeta.run/0"]);
        assert_eq!(collector.symbol_count(), 2);
        assert_eq!(collector.signature("Alpha.run/2").unwrap().params.len(), 2);
    }

    #[test]
    fn legacy_replacement_discards_the_whole_old_draft_pair() {
        let mut collector = ModuleDraftCollectorV1::default();
        for return_type in [MirType::Integer, MirType::String] {
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol("Legacy.f/0".into()),
                    "Legacy.f/0".into(),
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            let mut next = draft("Legacy.f/0", 0);
            next.signature.return_type = return_type;
            prepared.seal(next).unwrap().collect();
        }

        assert_eq!(
            collector.signature("Legacy.f/0").unwrap().return_type,
            MirType::String
        );
    }

    #[test]
    fn canonical_duplicate_rejects_before_draft_seal_or_collection() {
        let mut collector = ModuleDraftCollectorV1::default();
        let key = FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into());
        let prepared = collector
            .prepare_admission(
                key.clone(),
                "Canonical.f/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap();
        prepared.seal(draft("Canonical.f/0", 0)).unwrap().collect();

        let error = collector
            .prepare_admission(
                key,
                "Canonical.f/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateKey(_)
        ));
        assert_eq!(
            collector.signature("Canonical.f/0").unwrap().params.len(),
            0
        );
    }

    #[test]
    fn resolved_owner_key_is_distinct_from_legacy_symbol_identity() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuer.issue().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();

        collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
                "canonical_a_plus/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("canonical_a_plus/0", 0))
            .unwrap()
            .collect();

        let error = collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
                "canonical_a_plus/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateKey(
                FunctionDraftKeyV1::CanonicalResolvedOwner(actual)
            ) if actual == owner
        ));
    }

    #[test]
    fn canonical_symbol_collision_rejects_a_distinct_resolved_owner_key() {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let first_owner = issuer.issue().unwrap();
        let second_owner = issuer.issue().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();

        collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(first_owner),
                "same_symbol/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("same_symbol/0", 0))
            .unwrap()
            .collect();

        let error = collector
            .prepare_admission(
                FunctionDraftKeyV1::CanonicalResolvedOwner(second_owner),
                "same_symbol/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::DuplicateSymbol(symbol) if symbol == "same_symbol/0"
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("same_symbol/0"));
    }

    #[test]
    fn signature_or_arity_drift_rejects_without_collector_mutation() {
        let mut collector = ModuleDraftCollectorV1::default();
        let prepared = collector
            .prepare_admission(
                FunctionDraftKeyV1::Main,
                "main".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap();
        let error = prepared.seal(draft("main", 1)).unwrap_err();
        assert!(matches!(
            error,
            ModuleDraftAdmissionErrorV1::ArityMismatch { .. }
        ));
        assert!(collector.signature("main").is_none());
    }

    #[test]
    fn p0_main_and_synthetic_condition_drafts_share_one_header_view() {
        let mut collector = ModuleDraftCollectorV1::default();
        for (key, symbol, arity, policy) in [
            (
                FunctionDraftKeyV1::Main,
                "main",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
            (
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        ] {
            collector
                .prepare_admission(key, symbol.into(), arity, policy)
                .unwrap()
                .seal(draft(symbol, arity))
                .unwrap()
                .collect();
        }

        let mut symbols = Vec::new();
        collector.visit_symbols(&mut |symbol| symbols.push(symbol.to_owned()));
        assert_eq!(symbols, ["condition_fn", "main"]);
        assert_eq!(collector.symbol_count(), 2);
    }

    #[test]
    fn p0_route_policy_matrix_covers_every_root_and_child_family() {
        use super::super::module_invocation_route_matrix::{
            InvocationIdentityV1, InvocationRouteMatrixV1,
        };
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let mut collector = ModuleDraftCollectorV1::default();
        for row in InvocationRouteMatrixV1::rows() {
            let (symbol, key) = match row.identity() {
                InvocationIdentityV1::Main => ("main".to_owned(), FunctionDraftKeyV1::Main),
                InvocationIdentityV1::SyntheticConditionFn => (
                    "condition_fn".to_owned(),
                    FunctionDraftKeyV1::SyntheticConditionFn,
                ),
                InvocationIdentityV1::LegacySymbol => {
                    let symbol = format!("p0/{}/0", row.name());
                    (symbol.clone(), FunctionDraftKeyV1::LegacySymbol(symbol))
                }
                InvocationIdentityV1::CanonicalResolvedOwner => {
                    let symbol = format!("p0/{}/0", row.name());
                    (
                        symbol,
                        FunctionDraftKeyV1::CanonicalResolvedOwner(issuer.issue().unwrap()),
                    )
                }
                InvocationIdentityV1::CanonicalCallable => continue,
            };
            collector
                .prepare_admission(key, symbol.clone(), 0, row.publication())
                .unwrap()
                .seal(draft(&symbol, 0))
                .unwrap()
                .collect();
        }

        assert_eq!(collector.symbol_count(), 7);
        assert!(collector.contains_symbol("main"));
        assert!(collector.contains_symbol("condition_fn"));
        assert!(collector.contains_symbol("p0/canonical_a_plus_child/0"));
        assert!(!collector.contains_symbol("p0/binding_ssa_acyclic_module/0"));
    }

    #[test]
    fn p0_admission_failures_stop_before_collecting_a_new_draft() {
        let mut collector = ModuleDraftCollectorV1::default();
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("canonical/0".into()),
                "canonical/0".into(),
                0,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft("canonical/0", 0))
            .unwrap()
            .collect();

        let duplicate = collector.prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("canonical/0".into()),
            "canonical/0".into(),
            0,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        );
        assert!(matches!(
            duplicate,
            Err(ModuleDraftAdmissionErrorV1::DuplicateKey(_))
        ));

        let mismatch = collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("arity/0".into()),
                "arity/0".into(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("arity/0", 1));
        assert!(matches!(
            mismatch,
            Err(ModuleDraftAdmissionErrorV1::ArityMismatch { .. })
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("canonical/0"));
        assert!(!collector.contains_symbol("arity/0"));
    }

    #[test]
    fn p0_unwind_before_collect_leaves_the_collector_unchanged() {
        let mut collector = ModuleDraftCollectorV1::default();
        let unwind = catch_unwind(AssertUnwindSafe(|| {
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol("unwind/0".into()),
                    "unwind/0".into(),
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            let _unpublished = prepared.seal(draft("unwind/0", 0)).unwrap();
            panic!("P0 unwind before collect");
        }));

        assert!(unwind.is_err());
        assert_eq!(collector.symbol_count(), 0);
        assert!(!collector.contains_symbol("unwind/0"));
    }

    #[test]
    fn legacy_index_drift_is_rejected_before_collect_mutation() {
        let mut collector = ModuleDraftCollectorV1::default();
        collector.inject_symbol_index_drift_for_test(
            "drift/0",
            FunctionDraftKeyV1::LegacySymbol("other/0".into()),
        );

        let error = collector.prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("drift/0".into()),
            "drift/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        );
        assert!(matches!(
            error,
            Err(ModuleDraftAdmissionErrorV1::IndexDrift { .. })
        ));
        assert_eq!(collector.symbol_count(), 1);
        assert!(collector.contains_symbol("drift/0"));
    }
}
