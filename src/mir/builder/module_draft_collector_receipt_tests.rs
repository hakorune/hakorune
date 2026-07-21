//! WIRING-I0-ROUTEINV-P0a-RECEIPT-S0 focused fixtures.

use super::module_draft_collector::{
    CollectedDraftReplacementDispositionV1, CompletedDraftSignatureViewV1,
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1,
    ModuleDraftCollectorV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn draft(symbol: &str, arity: usize, return_type: MirType) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

#[test]
fn successful_commit_returns_exact_insert_receipt() {
    let mut collector = ModuleDraftCollectorV1::default();
    let receipt = collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Parser.skip/1".into()),
            "Parser.skip/1".into(),
            1,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Parser.skip/1", 1, MirType::Integer))
        .unwrap()
        .collect();

    assert_eq!(
        receipt.key(),
        &FunctionDraftKeyV1::LegacySymbol("Parser.skip/1".into())
    );
    assert_eq!(receipt.symbol(), "Parser.skip/1");
    assert_eq!(receipt.arity(), 1);
    assert_eq!(
        receipt.policy(),
        DraftPublicationPolicyV1::LegacyReplaceWholePair
    );
    assert_eq!(
        receipt.replacement(),
        &CollectedDraftReplacementDispositionV1::Inserted
    );
    assert!(collector.contains_symbol("Parser.skip/1"));
}

#[test]
fn legacy_replacement_receipt_names_the_discarded_whole_pair() {
    let mut collector = ModuleDraftCollectorV1::default();
    let key = FunctionDraftKeyV1::LegacySymbol("Legacy.f/0".into());
    collector
        .prepare_admission(
            key.clone(),
            "Legacy.f/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Legacy.f/0", 0, MirType::Integer))
        .unwrap()
        .collect();

    let receipt = collector
        .prepare_admission(
            key.clone(),
            "Legacy.f/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Legacy.f/0", 0, MirType::String))
        .unwrap()
        .collect();

    assert_eq!(
        receipt.replacement(),
        &CollectedDraftReplacementDispositionV1::ReplacedWholePair {
            previous_key: key,
            previous_symbol: "Legacy.f/0".into(),
        }
    );
    assert_eq!(
        collector.signature("Legacy.f/0").unwrap().return_type,
        MirType::String
    );
    assert_eq!(collector.symbol_count(), 1);
}

#[test]
fn canonical_commit_reports_insert_and_duplicate_stops_before_a_receipt() {
    let mut collector = ModuleDraftCollectorV1::default();
    let key = FunctionDraftKeyV1::LegacySymbol("Canonical.f/0".into());
    let receipt = collector
        .prepare_admission(
            key.clone(),
            "Canonical.f/0".into(),
            0,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap()
        .seal(draft("Canonical.f/0", 0, MirType::Integer))
        .unwrap()
        .collect();
    assert_eq!(
        receipt.replacement(),
        &CollectedDraftReplacementDispositionV1::Inserted
    );

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
    assert_eq!(collector.symbol_count(), 1);
}

#[test]
fn symbol_or_arity_failure_has_no_collector_or_receipt_effect() {
    let mut collector = ModuleDraftCollectorV1::default();
    let result = collector
        .prepare_admission(
            FunctionDraftKeyV1::Main,
            "main".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("wrong", 1, MirType::Integer));

    assert!(matches!(
        result,
        Err(ModuleDraftAdmissionErrorV1::SymbolMismatch { .. })
    ));
    assert_eq!(collector.symbol_count(), 0);
}
