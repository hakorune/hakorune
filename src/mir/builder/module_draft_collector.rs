//! MODULEDRAFT0-S0: one disconnected owner for unpublished function drafts.
//!
//! This vocabulary has no Builder, module, fact-session, or publication
//! consumer yet. It establishes the one collector that later receives a
//! completed draft together with its sealed fact session.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};
use crate::mir::{FunctionSignature, MirFunction};

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
    pub(in crate::mir::builder) fn collect(self) {
        let Self {
            collector,
            key,
            policy,
            replacement,
            draft,
            _seal: _,
        } = self;
        collector.collect_sealed(key, policy, replacement, draft);
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
    _seal: ModuleDraftCollectorSealV1,
}

#[derive(Debug, Default)]
struct ModuleDraftCollectorSealV1;

impl ModuleDraftCollectorV1 {
    /// Prepare every fallible collector admission check before child teardown.
    pub(in crate::mir::builder) fn prepare_admission(
        &mut self,
        key: FunctionDraftKeyV1,
        expected_symbol: String,
        expected_arity: usize,
        policy: DraftPublicationPolicyV1,
    ) -> Result<PreparedFunctionDraftAdmissionV1<'_>, ModuleDraftAdmissionErrorV1> {
        let symbol_key = self.key_by_symbol.get(&expected_symbol).cloned();
        let key_symbol = self
            .drafts
            .get(&key)
            .map(|entry| entry.draft.signature.name.clone());
        let replacement = match policy {
            DraftPublicationPolicyV1::CanonicalRejectDuplicate => {
                if self.drafts.contains_key(&key) {
                    return Err(ModuleDraftAdmissionErrorV1::DuplicateKey(key));
                }
                if self.key_by_symbol.contains_key(&expected_symbol) {
                    return Err(ModuleDraftAdmissionErrorV1::DuplicateSymbol(
                        expected_symbol,
                    ));
                }
                PreparedCollectorReplacementV1::Canonical
            }
            DraftPublicationPolicyV1::LegacyReplaceWholePair => {
                if let (Some(symbol_key), Some(key_symbol)) = (&symbol_key, &key_symbol) {
                    if symbol_key != &key || key_symbol != &expected_symbol {
                        return Err(ModuleDraftAdmissionErrorV1::IndexDrift {
                            symbol: expected_symbol,
                            key,
                        });
                    }
                }
                if symbol_key.is_some() && key_symbol.is_none() {
                    return Err(ModuleDraftAdmissionErrorV1::IndexDrift {
                        symbol: expected_symbol,
                        key,
                    });
                }
                if symbol_key.is_none() && key_symbol.is_some() {
                    return Err(ModuleDraftAdmissionErrorV1::IndexDrift {
                        symbol: expected_symbol,
                        key,
                    });
                }
                PreparedCollectorReplacementV1::Legacy {
                    symbol_key,
                    key_symbol,
                }
            }
        };
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
    ) {
        let symbol = draft.signature.name.clone();
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
    }

    /// Move the collector-owned drafts out only for the invocation drain.
    ///
    /// No header or identity side table is returned: the physical drafts and
    /// their collector indexes are consumed together by the one drain owner.
    pub(in crate::mir::builder) fn into_draft_functions(self) -> Vec<MirFunction> {
        self.drafts.into_values().map(|entry| entry.draft).collect()
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
        let routes = [
            (
                "legacy_main_root",
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
            (
                "canonical_a_plus_root",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
            (
                "binding_ssa_trivial_root",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
            (
                "binding_ssa_acyclic_module_root",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
            (
                "binding_ssa_recursive_module_root",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
            (
                "legacy_static_free_child",
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
            (
                "legacy_instance_constructor_child",
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
            (
                "canonical_a_plus_child",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
            (
                "binding_ssa_child",
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        ];

        let mut collector = ModuleDraftCollectorV1::default();
        for (route, policy) in routes {
            let symbol = format!("p0/{route}/0");
            let key = if route == "legacy_main_root" {
                FunctionDraftKeyV1::Main
            } else {
                FunctionDraftKeyV1::LegacySymbol(symbol.clone())
            };
            collector
                .prepare_admission(key, symbol.clone(), 0, policy)
                .unwrap()
                .seal(draft(&symbol, 0))
                .unwrap()
                .collect();
        }

        assert_eq!(collector.symbol_count(), 9);
        assert!(collector.contains_symbol("p0/legacy_main_root/0"));
        assert!(collector.contains_symbol("p0/canonical_a_plus_child/0"));
        assert!(collector.contains_symbol("p0/binding_ssa_recursive_module_root/0"));
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
