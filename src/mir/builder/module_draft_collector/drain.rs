//! PHYSICAL0-COLLECT0: keyed, mutation-free canonical collector drain.
//!
//! This child keeps the collector's semantic key correspondence alive while
//! a later physical terminal prepares shell mutation.  It never reads source
//! headers or compiler manifests and never mutates the collector during
//! preparation.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::MirFunction;

use super::callable_batch::CallableCollectorBatchReceiptV1;
use super::receipt::{CollectedDraftAdmissionReceiptV1, CollectedDraftReplacementDispositionV1};
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1};
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;

/// The receipt is borrowed only for prepare-time correspondence checks.  The
/// owning physical wrapper retains the receipt until the later drain row.
pub(in crate::mir::builder) enum CanonicalCollectorReceiptViewV1<'a> {
    Single(&'a InvocationBranded<CollectedDraftAdmissionReceiptV1>),
    Callable(&'a InvocationBranded<CallableCollectorBatchReceiptV1>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalCollectorDrainErrorV1 {
    BrandMismatch,
    ReceiptCountMismatch { expected: usize, actual: usize },
    DuplicateReceiptKey(FunctionDraftKeyV1),
    DuplicateReceiptSymbol(String),
    MissingKey(FunctionDraftKeyV1),
    SurplusKey(FunctionDraftKeyV1),
    SymbolIndexDrift { symbol: String },
    SymbolMismatch { key: FunctionDraftKeyV1, expected: String, actual: String },
    ArityMismatch { key: FunctionDraftKeyV1, expected: usize, actual: usize },
    ReceiptMismatch { key: FunctionDraftKeyV1 },
    LegacyAdmission { key: FunctionDraftKeyV1 },
    ReplacedAdmission { key: FunctionDraftKeyV1 },
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedCanonicalCollectorDrainV1 {
    collector: ModuleDraftCollectorV1,
    error: CanonicalCollectorDrainErrorV1,
}

impl RejectedCanonicalCollectorDrainV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (ModuleDraftCollectorV1, CanonicalCollectorDrainErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCanonicalCollectorDrainV1 {
    collector: ModuleDraftCollectorV1,
    ordered_keys: Box<[FunctionDraftKeyV1]>,
    _seal: PreparedCanonicalCollectorDrainSealV1,
}

#[derive(Debug)]
struct PreparedCanonicalCollectorDrainSealV1;

impl PreparedCanonicalCollectorDrainV1 {
    /// All key/index/receipt checks have completed, so this move cannot fail.
    pub(in crate::mir::builder) fn drain(self) -> Vec<MirFunction> {
        let Self {
            mut collector,
            ordered_keys,
            _seal: _,
        } = self;
        ordered_keys
            .into_vec()
            .into_iter()
            .map(|key| {
                collector
                    .drafts
                    .remove(&key)
                    .expect("prepared canonical key must own one draft")
                    .draft
            })
            .collect()
    }
}

impl ModuleDraftCollectorV1 {
    /// Consume the collector only after a complete, mutation-free keyed proof.
    pub(in crate::mir::builder) fn prepare_canonical_drain(
        self,
        manifest: &CanonicalPhysicalDrainManifestV1,
        receipt: CanonicalCollectorReceiptViewV1<'_>,
        brand: ModuleInvocationBrandV1,
    ) -> Result<PreparedCanonicalCollectorDrainV1, RejectedCanonicalCollectorDrainV1> {
        if self.receipt_brand != Some(brand) {
            return Err(reject(self, CanonicalCollectorDrainErrorV1::BrandMismatch));
        }
        if receipt.brand() != brand {
            return Err(reject(self, CanonicalCollectorDrainErrorV1::BrandMismatch));
        }

        let expected = expected_rows(manifest);
        let receipt_rows = receipt_rows(receipt);
        if let Err(error) = validate_rows(&self, &expected, &receipt_rows) {
            return Err(reject(self, error));
        }

        Ok(PreparedCanonicalCollectorDrainV1 {
            collector: self,
            ordered_keys: expected.into_iter().map(|(key, _, _)| key).collect(),
            _seal: PreparedCanonicalCollectorDrainSealV1,
        })
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: CanonicalCollectorDrainErrorV1,
) -> RejectedCanonicalCollectorDrainV1 {
    RejectedCanonicalCollectorDrainV1 { collector, error }
}

fn expected_rows(
    manifest: &CanonicalPhysicalDrainManifestV1,
) -> Vec<(FunctionDraftKeyV1, String, usize)> {
    if let Some(row) = manifest.single_row() {
        return vec![(
            FunctionDraftKeyV1::CanonicalResolvedOwner(row.owner()),
            row.symbol().to_owned(),
            row.arity(),
        )];
    }
    manifest
        .callable_rows()
        .unwrap_or_default()
            .iter()
            .map(|row| {
                (
                    FunctionDraftKeyV1::CanonicalCallable(row.key().clone()),
                    row.symbol().to_owned(),
                    row.arity(),
                )
            })
            .collect()
}

fn receipt_rows(
    receipt: CanonicalCollectorReceiptViewV1<'_>,
) -> Vec<(FunctionDraftKeyV1, String, usize, DraftPublicationPolicyV1, bool)> {
    match receipt {
        CanonicalCollectorReceiptViewV1::Single(receipt) => {
            let receipt = receipt.payload();
            vec![receipt_row(receipt)]
        }
        CanonicalCollectorReceiptViewV1::Callable(receipt) => receipt
            .payload()
            .admissions()
            .iter()
            .map(receipt_row)
            .collect(),
    }
}

fn receipt_row(
    receipt: &CollectedDraftAdmissionReceiptV1,
) -> (FunctionDraftKeyV1, String, usize, DraftPublicationPolicyV1, bool) {
    (
        receipt.key().clone(),
        receipt.symbol().to_owned(),
        receipt.arity(),
        receipt.policy(),
        matches!(receipt.replacement(), CollectedDraftReplacementDispositionV1::Inserted),
    )
}

fn validate_rows(
    collector: &ModuleDraftCollectorV1,
    expected: &[(FunctionDraftKeyV1, String, usize)],
    actual: &[(FunctionDraftKeyV1, String, usize, DraftPublicationPolicyV1, bool)],
) -> Result<(), CanonicalCollectorDrainErrorV1> {
    if expected.len() != actual.len() {
        return Err(CanonicalCollectorDrainErrorV1::ReceiptCountMismatch {
            expected: expected.len(),
            actual: actual.len(),
        });
    }

    let mut expected_by_key = BTreeMap::new();
    let mut expected_symbols = BTreeSet::new();
    for (key, symbol, arity) in expected {
        if expected_by_key.insert(key.clone(), (symbol.as_str(), *arity)).is_some() {
            return Err(CanonicalCollectorDrainErrorV1::SurplusKey(key.clone()));
        }
        expected_symbols.insert(symbol.as_str());
    }
    let mut actual_by_key = BTreeMap::new();
    let mut actual_symbols = BTreeSet::new();
    for (key, symbol, arity, policy, inserted) in actual {
        if actual_by_key
            .insert(key.clone(), (symbol.as_str(), *arity, *policy, *inserted))
            .is_some()
        {
            return Err(CanonicalCollectorDrainErrorV1::DuplicateReceiptKey(key.clone()));
        }
        if !actual_symbols.insert(symbol.as_str()) {
            return Err(CanonicalCollectorDrainErrorV1::DuplicateReceiptSymbol(symbol.clone()));
        }
        if *policy != DraftPublicationPolicyV1::CanonicalRejectDuplicate {
            return Err(CanonicalCollectorDrainErrorV1::LegacyAdmission { key: key.clone() });
        }
        if !inserted {
            return Err(CanonicalCollectorDrainErrorV1::ReplacedAdmission { key: key.clone() });
        }
    }

    for (key, symbol, arity) in expected {
        let Some(entry) = collector.drafts.get(key) else {
            return Err(CanonicalCollectorDrainErrorV1::MissingKey(key.clone()));
        };
        let actual_symbol = entry.draft.signature.name.as_str();
        if actual_symbol != symbol {
            return Err(CanonicalCollectorDrainErrorV1::SymbolMismatch {
                key: key.clone(),
                expected: symbol.clone(),
                actual: actual_symbol.to_owned(),
            });
        }
        let actual_arity = entry.draft.signature.params.len();
        if actual_arity != *arity {
            return Err(CanonicalCollectorDrainErrorV1::ArityMismatch {
                key: key.clone(),
                expected: *arity,
                actual: actual_arity,
            });
        }
        if collector.key_by_symbol.get(symbol) != Some(key) {
            return Err(CanonicalCollectorDrainErrorV1::SymbolIndexDrift {
                symbol: symbol.clone(),
            });
        }
        if actual_by_key.get(key).map(|(s, a, _, _)| (*s, *a))
            != Some((symbol.as_str(), *arity))
        {
            return Err(CanonicalCollectorDrainErrorV1::ReceiptMismatch { key: key.clone() });
        }
    }

    if collector.drafts.len() != expected.len()
        || collector.key_by_symbol.len() != expected_symbols.len()
        || actual_symbols != expected_symbols
    {
        let surplus = collector
            .drafts
            .keys()
            .find(|key| !expected_by_key.contains_key(*key))
            .cloned();
        return Err(CanonicalCollectorDrainErrorV1::SurplusKey(
            surplus.unwrap_or_else(|| expected[0].0.clone()),
        ));
    }
    Ok(())
}

impl CanonicalCollectorReceiptViewV1<'_> {
    fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Single(receipt) => receipt.brand(),
            Self::Callable(receipt) => receipt.brand(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::canonical_physical_drain::{
        CanonicalInsertedDispositionV1, CanonicalPhysicalSingleRowV1,
    };
    use crate::mir::module_invocation_identity::{
        ModuleInvocationFamilyV1, ModuleInvocationBrandV1,
    };
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

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

    #[test]
    fn keyed_prepare_preserves_manifest_identity_and_consumes_one_draft() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(77);
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = owners.issue().unwrap();
        let collector = InvocationBranded::from_source(
            brand,
            ModuleDraftCollectorV1::with_brand(brand),
        )
        .collect_canonical_single(
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            "owner/0".to_owned(),
            0,
            draft("owner/0"),
        )
        .unwrap();
        let (collector, receipt) = collector.into_parts();
        let manifest = CanonicalPhysicalDrainManifestV1::single(
            brand,
            ModuleInvocationFamilyV1::BindingSsaTrivial,
            CanonicalPhysicalSingleRowV1::new(
                owner,
                "owner/0".into(),
                0,
                CanonicalInsertedDispositionV1::from_canonical_source(),
            ),
        );

        let prepared = collector
            .into_payload()
            .prepare_canonical_drain(
                &manifest,
                CanonicalCollectorReceiptViewV1::Single(&receipt),
                brand,
            )
            .unwrap();
        let drafts = prepared.drain();
        assert_eq!(drafts.len(), 1);
        assert_eq!(drafts[0].signature.name, "owner/0");
    }

    #[test]
    fn keyed_prepare_rejects_index_drift_and_returns_the_collector() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(78);
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = owners.issue().unwrap();
        let product = InvocationBranded::from_source(
            brand,
            ModuleDraftCollectorV1::with_brand(brand),
        )
        .collect_canonical_single(
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            "owner/0".to_owned(),
            0,
            draft("owner/0"),
        )
        .unwrap();
        let (collector, receipt) = product.into_parts();
        let mut collector = collector.into_payload();
        collector.inject_symbol_index_drift_for_test("owner/0", FunctionDraftKeyV1::Main);
        let manifest = CanonicalPhysicalDrainManifestV1::single(
            brand,
            ModuleInvocationFamilyV1::BindingSsaTrivial,
            CanonicalPhysicalSingleRowV1::new(
                owner,
                "owner/0".into(),
                0,
                CanonicalInsertedDispositionV1::from_canonical_source(),
            ),
        );

        let rejected = collector
            .prepare_canonical_drain(
                &manifest,
                CanonicalCollectorReceiptViewV1::Single(&receipt),
                brand,
            )
            .unwrap_err();
        let (collector, error) = rejected.into_parts();
        assert!(matches!(
            error,
            CanonicalCollectorDrainErrorV1::SymbolIndexDrift { .. }
        ));
        assert_eq!(collector.symbol_count(), 1);
    }

    #[test]
    fn keyed_prepare_rejects_foreign_receipt_before_consuming_collector() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(79);
        let foreign = ModuleInvocationBrandV1::test_with_ordinal(80);
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = owners.issue().unwrap();
        let product = InvocationBranded::from_source(
            brand,
            ModuleDraftCollectorV1::with_brand(brand),
        )
        .collect_canonical_single(
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            "owner/0".to_owned(),
            0,
            draft("owner/0"),
        )
        .unwrap();
        let (collector, receipt) = product.into_parts();
        let foreign_receipt = InvocationBranded::from_test(foreign, receipt.into_payload());
        let manifest = CanonicalPhysicalDrainManifestV1::single(
            brand,
            ModuleInvocationFamilyV1::BindingSsaTrivial,
            CanonicalPhysicalSingleRowV1::new(
                owner,
                "owner/0".into(),
                0,
                CanonicalInsertedDispositionV1::from_canonical_source(),
            ),
        );

        let rejected = collector
            .into_payload()
            .prepare_canonical_drain(
                &manifest,
                CanonicalCollectorReceiptViewV1::Single(&foreign_receipt),
                brand,
            )
            .unwrap_err();
        let (collector, error) = rejected.into_parts();
        assert_eq!(error, CanonicalCollectorDrainErrorV1::BrandMismatch);
        assert!(collector.signature("owner/0").is_some());
    }
}
