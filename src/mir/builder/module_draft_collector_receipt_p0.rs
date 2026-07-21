//! WIRING-I0-ROUTEINV-P0a-RECEIPT-P0 failure and replacement proof.

use super::module_draft_collector::{
    CollectedDraftReplacementDispositionV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftAdmissionErrorV1, ModuleDraftCollectorV1,
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

fn collect(
    collector: &mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: DraftPublicationPolicyV1,
    return_type: MirType,
) {
    collector
        .prepare_admission(key, symbol.to_owned(), arity, policy)
        .unwrap()
        .seal(draft(symbol, arity, return_type))
        .unwrap()
        .collect();
}

fn seeded_collector() -> ModuleDraftCollectorV1 {
    let mut collector = ModuleDraftCollectorV1::default();
    collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("Legacy.keep/0".into()),
        "Legacy.keep/0",
        0,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
        MirType::Integer,
    );
    collect(
        &mut collector,
        FunctionDraftKeyV1::LegacySymbol("Canonical.keep/1".into()),
        "Canonical.keep/1",
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        MirType::Integer,
    );
    collector
}

#[test]
fn canonical_duplicate_key_and_symbol_preserve_exact_prefix_and_indexes() {
    let mut collector = seeded_collector();
    let before = collector.receipt_proof_snapshot();
    assert!(before.is_bijective());

    let duplicate_key = collector.prepare_admission(
        FunctionDraftKeyV1::LegacySymbol("Canonical.keep/1".into()),
        "Canonical.other/1".into(),
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    assert!(matches!(
        duplicate_key,
        Err(ModuleDraftAdmissionErrorV1::DuplicateKey(_))
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let duplicate_symbol = collector.prepare_admission(
        FunctionDraftKeyV1::LegacySymbol("Canonical.other/1".into()),
        "Canonical.keep/1".into(),
        1,
        DraftPublicationPolicyV1::CanonicalRejectDuplicate,
    );
    assert!(matches!(
        duplicate_symbol,
        Err(ModuleDraftAdmissionErrorV1::DuplicateSymbol(symbol))
            if symbol == "Canonical.keep/1"
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);
}

#[test]
fn seal_failures_and_drop_before_collect_preserve_exact_prefix_and_indexes() {
    let mut collector = seeded_collector();
    let before = collector.receipt_proof_snapshot();

    let symbol_error = collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Bad.symbol/0".into()),
            "Bad.symbol/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Wrong.symbol/0", 0, MirType::Integer));
    assert!(matches!(
        symbol_error,
        Err(ModuleDraftAdmissionErrorV1::SymbolMismatch { .. })
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let arity_error = collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Bad.arity/0".into()),
            "Bad.arity/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Bad.arity/0", 1, MirType::Integer));
    assert!(matches!(
        arity_error,
        Err(ModuleDraftAdmissionErrorV1::ArityMismatch { .. })
    ));
    assert_eq!(collector.receipt_proof_snapshot(), before);

    let unpublished = collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Dropped/0".into()),
            "Dropped/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Dropped/0", 0, MirType::Integer))
        .unwrap();
    drop(unpublished);
    assert_eq!(collector.receipt_proof_snapshot(), before);
    assert!(collector.receipt_proof_snapshot().is_bijective());
}

#[test]
fn legacy_replacement_changes_only_one_whole_pair_and_keeps_bijection() {
    let mut collector = seeded_collector();
    let before = collector.receipt_proof_snapshot();
    let key = FunctionDraftKeyV1::LegacySymbol("Legacy.keep/0".into());

    let receipt = collector
        .prepare_admission(
            key.clone(),
            "Legacy.keep/0".into(),
            0,
            DraftPublicationPolicyV1::LegacyReplaceWholePair,
        )
        .unwrap()
        .seal(draft("Legacy.keep/0", 0, MirType::String))
        .unwrap()
        .collect();
    let after = collector.receipt_proof_snapshot();

    assert_ne!(after, before);
    assert!(after.is_bijective());
    assert_eq!(
        receipt.replacement(),
        &CollectedDraftReplacementDispositionV1::ReplacedWholePair {
            previous_key: key,
            previous_symbol: "Legacy.keep/0".into(),
        }
    );

    let canonical_unchanged = collector
        .prepare_admission(
            FunctionDraftKeyV1::LegacySymbol("Canonical.keep/1".into()),
            "Canonical.keep/1".into(),
            1,
            DraftPublicationPolicyV1::CanonicalRejectDuplicate,
        )
        .unwrap_err();
    assert!(matches!(
        canonical_unchanged,
        ModuleDraftAdmissionErrorV1::DuplicateKey(_)
    ));
}
