//! Normal Program drain preflight over the collector's final legacy rows.
//!
//! This owner neither reads source nor lowers a draft. It validates the final
//! collector state once, reserves one exact candidate module, and leaves only
//! an infallible physical insertion for the following integration row.

use std::collections::BTreeSet;

use crate::mir::function::FunctionPublicationErrorV1;
use crate::mir::MirModule;

use super::receipt::CollectedDraftReplacementDispositionV1;
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalLegacyCollectorDrainErrorV1 {
    FinalAdmissionDrift { key: FunctionDraftKeyV1 },
    NonLegacyKey { key: FunctionDraftKeyV1 },
    NonLegacyPolicy { key: FunctionDraftKeyV1 },
    SymbolIndexDrift { symbol: String },
    Publication(FunctionPublicationErrorV1),
}

impl std::fmt::Display for NormalLegacyCollectorDrainErrorV1 {
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

impl std::error::Error for NormalLegacyCollectorDrainErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedNormalLegacyCollectorDrainV1 {
    collector: ModuleDraftCollectorV1,
    error: NormalLegacyCollectorDrainErrorV1,
}

impl RejectedNormalLegacyCollectorDrainV1 {
    pub(in crate::mir::builder) fn error(&self) -> &NormalLegacyCollectorDrainErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {}

    #[cfg(test)]
    fn into_parts(self) -> (ModuleDraftCollectorV1, NormalLegacyCollectorDrainErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
struct NormalLegacyCollectorFinalInventoryV1 {
    ordered_keys: Box<[FunctionDraftKeyV1]>,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedNormalLegacyCollectorDrainV1<'module> {
    collector: ModuleDraftCollectorV1,
    inventory: NormalLegacyCollectorFinalInventoryV1,
    target: &'module mut MirModule,
    _seal: PreparedNormalLegacyCollectorDrainSealV1,
}

#[derive(Debug)]
struct PreparedNormalLegacyCollectorDrainSealV1;

impl PreparedNormalLegacyCollectorDrainV1<'_> {
    /// Preparation has reserved the only mutable module loan and checked every
    /// final collector row, so no fallible work remains during insertion.
    pub(in crate::mir::builder) fn commit(self) {
        let Self {
            mut collector,
            inventory,
            target,
            _seal: _,
        } = self;
        for key in inventory.ordered_keys.into_vec() {
            let entry = collector
                .drafts
                .remove(&key)
                .expect("prepared normal collector key must own one draft");
            target.add_function(entry.draft);
        }
    }
}

impl ModuleDraftCollectorV1 {
    /// Consume exactly the final normal legacy rows after source-free
    /// correspondence and candidate-module collision checks.
    pub(in crate::mir::builder) fn prepare_normal_legacy_drain<'module>(
        self,
        target: &'module mut MirModule,
    ) -> Result<PreparedNormalLegacyCollectorDrainV1<'module>, RejectedNormalLegacyCollectorDrainV1>
    {
        let inventory = match NormalLegacyCollectorFinalInventoryV1::seal(&self, target) {
            Ok(inventory) => inventory,
            Err(error) => return Err(reject(self, error)),
        };
        Ok(PreparedNormalLegacyCollectorDrainV1 {
            collector: self,
            inventory,
            target,
            _seal: PreparedNormalLegacyCollectorDrainSealV1,
        })
    }
}

impl NormalLegacyCollectorFinalInventoryV1 {
    fn seal(
        collector: &ModuleDraftCollectorV1,
        target: &MirModule,
    ) -> Result<Self, NormalLegacyCollectorDrainErrorV1> {
        if collector.drafts.len() != collector.key_by_symbol.len() {
            let symbol = collector
                .key_by_symbol
                .keys()
                .next()
                .cloned()
                .unwrap_or_default();
            return Err(NormalLegacyCollectorDrainErrorV1::SymbolIndexDrift { symbol });
        }

        let mut symbols = BTreeSet::new();
        let mut ordered_keys = Vec::with_capacity(collector.drafts.len());
        for (key, entry) in &collector.drafts {
            let admission = &entry.admission;
            if admission.key != *key
                || admission.symbol.as_ref() != entry.draft.signature.name
                || admission.arity != entry.draft.signature.params.len()
            {
                return Err(NormalLegacyCollectorDrainErrorV1::FinalAdmissionDrift {
                    key: key.clone(),
                });
            }
            if !matches!(key, FunctionDraftKeyV1::LegacySymbol(symbol) if symbol == admission.symbol.as_ref())
            {
                return Err(NormalLegacyCollectorDrainErrorV1::NonLegacyKey { key: key.clone() });
            }
            if admission.policy != DraftPublicationPolicyV1::LegacyReplaceWholePair {
                return Err(NormalLegacyCollectorDrainErrorV1::NonLegacyPolicy {
                    key: key.clone(),
                });
            }
            if !matches!(
                &admission.replacement,
                CollectedDraftReplacementDispositionV1::Inserted
                    | CollectedDraftReplacementDispositionV1::ReplacedWholePair { .. }
            ) {
                return Err(NormalLegacyCollectorDrainErrorV1::FinalAdmissionDrift {
                    key: key.clone(),
                });
            }
            if collector.key_by_symbol.get(admission.symbol.as_ref()) != Some(key)
                || !symbols.insert(admission.symbol.as_ref())
            {
                return Err(NormalLegacyCollectorDrainErrorV1::SymbolIndexDrift {
                    symbol: admission.symbol.to_string(),
                });
            }
            ordered_keys.push(key.clone());
        }

        target
            .preflight_add_function_symbols(symbols.into_iter())
            .map_err(NormalLegacyCollectorDrainErrorV1::Publication)?;
        Ok(Self {
            ordered_keys: ordered_keys.into_boxed_slice(),
        })
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: NormalLegacyCollectorDrainErrorV1,
) -> RejectedNormalLegacyCollectorDrainV1 {
    RejectedNormalLegacyCollectorDrainV1 { collector, error }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn draft(symbol: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: Vec::new(),
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

    #[test]
    fn prepared_normal_drain_preserves_final_legacy_key_order_until_commit() {
        let mut collector = ModuleDraftCollectorV1::default();
        collect(&mut collector, "Zeta.run/0");
        collect(&mut collector, "Alpha.run/0");
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_legacy_drain(&mut target)
            .unwrap()
            .commit();

        assert_eq!(target.function_names(), vec!["Alpha.run/0", "Zeta.run/0"]);
    }

    #[test]
    fn empty_normal_collector_prepares_and_commits_without_publication() {
        let collector = ModuleDraftCollectorV1::default();
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_legacy_drain(&mut target)
            .unwrap()
            .commit();

        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejection_retains_collector_and_leaves_target_unchanged() {
        let mut collector = ModuleDraftCollectorV1::default();
        collect(&mut collector, "same/0");
        let mut target = MirModule::new("normal".to_owned());
        target.add_function(draft("same/0"));

        let rejected = collector
            .prepare_normal_legacy_drain(&mut target)
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalLegacyCollectorDrainErrorV1::Publication(_)
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert_eq!(target.function_names(), vec!["same/0"]);
    }

    #[test]
    fn normal_drain_rejects_nonlegacy_final_rows_without_consuming_them() {
        let mut collector = ModuleDraftCollectorV1::default();
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
            .prepare_normal_legacy_drain(&mut target)
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalLegacyCollectorDrainErrorV1::NonLegacyKey { .. }
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }
}
