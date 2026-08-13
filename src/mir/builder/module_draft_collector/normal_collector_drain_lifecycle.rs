//! Normal candidate collector drain and final module-publication lifecycle.
//!
//! This owner preserves the selected normal LegacySymbol and CatalogedBoxMethod
//! admission semantics, while binding the collector to the already-issued
//! candidate-session brand. It neither reads source nor opens another
//! module/publication route.

use std::collections::BTreeSet;

use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::function::FunctionPublicationErrorV1;
use crate::mir::MirModule;

use super::receipt::CollectedDraftReplacementDispositionV1;
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalCollectorDrainLifecycleErrorV1 {
    BrandMismatch,
    FinalAdmissionDrift { key: FunctionDraftKeyV1 },
    NonLegacyKey { key: FunctionDraftKeyV1 },
    NonLegacyPolicy { key: FunctionDraftKeyV1 },
    SymbolIndexDrift { symbol: String },
    Publication(FunctionPublicationErrorV1),
}

impl std::fmt::Display for NormalCollectorDrainLifecycleErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Publication(error) => error.fmt(formatter),
            _ => write!(
                formatter,
                "[freeze:contract][normal-collector-drain] {self:?}"
            ),
        }
    }
}

impl std::error::Error for NormalCollectorDrainLifecycleErrorV1 {}

/// Failure retains every unpublished draft; the outer normal lifecycle owns
/// the candidate session and source disposal.
#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedNormalCollectorDrainLifecycleV1 {
    collector: ModuleDraftCollectorV1,
    error: NormalCollectorDrainLifecycleErrorV1,
}

impl RejectedNormalCollectorDrainLifecycleV1 {
    pub(in crate::mir::builder) fn error(&self) -> &NormalCollectorDrainLifecycleErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {}

    #[cfg(test)]
    fn into_parts(self) -> (ModuleDraftCollectorV1, NormalCollectorDrainLifecycleErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
struct SealedNormalCollectorDrainReceiptV1 {
    brand: ModuleInvocationBrandV1,
    ordered_keys: Box<[FunctionDraftKeyV1]>,
}

/// The target module is borrowed only after all correspondence and collision
/// checks complete. `commit` therefore has no fallible operation.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedNormalCollectorDrainLifecycleV1<'module> {
    collector: ModuleDraftCollectorV1,
    receipt: SealedNormalCollectorDrainReceiptV1,
    target: &'module mut MirModule,
    _seal: PreparedNormalCollectorDrainLifecycleSealV1,
}

#[derive(Debug)]
struct PreparedNormalCollectorDrainLifecycleSealV1;

impl PreparedNormalCollectorDrainLifecycleV1<'_> {
    pub(in crate::mir::builder) fn commit(self) {
        let Self {
            mut collector,
            receipt,
            target,
            _seal: _,
        } = self;
        debug_assert_eq!(collector.receipt_brand(), Some(receipt.brand));
        for key in receipt.ordered_keys.into_vec() {
            let entry = collector
                .drafts
                .remove(&key)
                .expect("prepared normal collector key must own one draft");
            target.add_function(entry.draft);
        }
    }
}

impl ModuleDraftCollectorV1 {
    /// Consume exactly one normal candidate's final LegacySymbol/CatalogedBoxMethod
    /// rows after source-free correspondence, session-brand, and module-collision
    /// preflight.
    pub(in crate::mir::builder) fn prepare_normal_collector_drain<'module>(
        self,
        target: &'module mut MirModule,
        brand: ModuleInvocationBrandV1,
    ) -> Result<
        PreparedNormalCollectorDrainLifecycleV1<'module>,
        RejectedNormalCollectorDrainLifecycleV1,
    > {
        let receipt = match SealedNormalCollectorDrainReceiptV1::seal(&self, target, brand) {
            Ok(receipt) => receipt,
            Err(error) => return Err(reject(self, error)),
        };
        Ok(PreparedNormalCollectorDrainLifecycleV1 {
            collector: self,
            receipt,
            target,
            _seal: PreparedNormalCollectorDrainLifecycleSealV1,
        })
    }
}

impl SealedNormalCollectorDrainReceiptV1 {
    fn seal(
        collector: &ModuleDraftCollectorV1,
        target: &MirModule,
        brand: ModuleInvocationBrandV1,
    ) -> Result<Self, NormalCollectorDrainLifecycleErrorV1> {
        if collector.receipt_brand() != Some(brand) {
            return Err(NormalCollectorDrainLifecycleErrorV1::BrandMismatch);
        }
        if collector.drafts.len() != collector.key_by_symbol.len() {
            let symbol = collector
                .key_by_symbol
                .keys()
                .next()
                .cloned()
                .unwrap_or_default();
            return Err(NormalCollectorDrainLifecycleErrorV1::SymbolIndexDrift { symbol });
        }

        let mut symbols = BTreeSet::new();
        let mut ordered_keys = Vec::with_capacity(collector.drafts.len());
        for (key, entry) in &collector.drafts {
            let admission = &entry.admission;
            if admission.key != *key
                || admission.symbol.as_ref() != entry.draft.signature.name
                || admission.arity != entry.draft.signature.params.len()
            {
                return Err(NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                    key: key.clone(),
                });
            }
            match key {
                FunctionDraftKeyV1::LegacySymbol(symbol)
                    if symbol == admission.symbol.as_ref()
                        && admission.policy == DraftPublicationPolicyV1::LegacyReplaceWholePair
                        && matches!(
                            &admission.replacement,
                            CollectedDraftReplacementDispositionV1::Inserted
                                | CollectedDraftReplacementDispositionV1::ReplacedWholePair { .. }
                        ) => {}
                FunctionDraftKeyV1::CatalogedBoxMethod(_)
                    if admission.policy == DraftPublicationPolicyV1::CanonicalRejectDuplicate
                        && matches!(
                            &admission.replacement,
                            CollectedDraftReplacementDispositionV1::Inserted
                        ) => {}
                _ => {
                    return Err(match key {
                        FunctionDraftKeyV1::LegacySymbol(_) => {
                            NormalCollectorDrainLifecycleErrorV1::NonLegacyPolicy {
                                key: key.clone(),
                            }
                        }
                        FunctionDraftKeyV1::CatalogedBoxMethod(_) => {
                            NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                                key: key.clone(),
                            }
                        }
                        _ => {
                            NormalCollectorDrainLifecycleErrorV1::NonLegacyKey { key: key.clone() }
                        }
                    });
                }
            }
            if collector.key_by_symbol.get(admission.symbol.as_ref()) != Some(key)
                || !symbols.insert(admission.symbol.as_ref())
            {
                return Err(NormalCollectorDrainLifecycleErrorV1::SymbolIndexDrift {
                    symbol: admission.symbol.to_string(),
                });
            }
            ordered_keys.push(key.clone());
        }

        target
            .preflight_add_function_symbols(symbols.into_iter())
            .map_err(NormalCollectorDrainLifecycleErrorV1::Publication)?;
        Ok(Self {
            brand,
            ordered_keys: ordered_keys.into_boxed_slice(),
        })
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: NormalCollectorDrainLifecycleErrorV1,
) -> RejectedNormalCollectorDrainLifecycleV1 {
    RejectedNormalCollectorDrainLifecycleV1 { collector, error }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn brand() -> ModuleInvocationBrandV1 {
        ModuleInvocationBrandV1::test_with_ordinal(701)
    }

    fn draft(symbol: &str) -> MirFunction {
        draft_with_arity(symbol, 0)
    }

    fn draft_with_arity(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn collect(collector: &mut ModuleDraftCollectorV1, symbol: &str) {
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol(symbol.to_owned()),
                symbol.to_owned(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft(symbol))
            .unwrap()
            .collect();
    }

    fn collect_cataloged(collector: &mut ModuleDraftCollectorV1) {
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        );
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedBoxMethod(key),
                "ParserScanLoopBox.skip_while/4".to_owned(),
                4,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
            .unwrap()
            .collect();
    }

    #[test]
    fn mixed_normal_drain_accepts_legacy_and_cataloged_rows() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "legacy/0");
        collect_cataloged(&mut collector);
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert_eq!(
            target.function_names(),
            vec!["ParserScanLoopBox.skip_while/4", "legacy/0"]
        );
    }

    #[test]
    fn normal_drain_rejects_cataloged_legacy_policy_without_publication() {
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        );
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedBoxMethod(key),
                "ParserScanLoopBox.skip_while/4".to_owned(),
                4,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
            .unwrap()
            .collect();
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                key: FunctionDraftKeyV1::CatalogedBoxMethod(_)
            }
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }

    #[test]
    fn prepared_normal_drain_preserves_final_legacy_key_order_until_commit() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "Zeta.run/0");
        collect(&mut collector, "Alpha.run/0");
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert_eq!(target.function_names(), vec!["Alpha.run/0", "Zeta.run/0"]);
    }

    #[test]
    fn empty_normal_collector_prepares_and_commits_without_publication() {
        let collector = ModuleDraftCollectorV1::with_brand(brand());
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejection_retains_collector_and_leaves_target_unchanged() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "same/0");
        let mut target = MirModule::new("normal".to_owned());
        target.add_function(draft("same/0"));

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::Publication(_)
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert_eq!(target.function_names(), vec!["same/0"]);
    }

    #[test]
    fn normal_drain_rejects_foreign_or_missing_session_brand_without_consuming_rows() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "same/0");
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(
                &mut target,
                ModuleInvocationBrandV1::test_with_ordinal(702),
            )
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::BrandMismatch
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejects_nonlegacy_final_rows_without_consuming_them() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::Main,
                "main".to_owned(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("main"))
            .unwrap()
            .collect();
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::NonLegacyKey { .. }
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }
}
