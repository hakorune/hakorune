//! WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0 focused fixtures.

use super::module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftCollectorV1,
};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawConditionDispositionV1,
    RawExpansionDraftRequestV1, RawExpansionDraftRoleV1, RawExpansionReceiptLedgerErrorV1,
    RawExpansionReceiptLedgerV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn draft(symbol: &str, arity: usize) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: symbol.to_owned(),
            params: vec![MirType::Integer; arity],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn receipt(
    collector: &mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: DraftPublicationPolicyV1,
) -> CollectedDraftAdmissionReceiptV1 {
    collector
        .prepare_admission(key, symbol.to_owned(), arity, policy)
        .unwrap()
        .seal(draft(symbol, arity))
        .unwrap()
        .collect()
}

#[test]
fn exact_reservations_consume_receipts_and_seal_required_raw_inventory() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let root = ledger
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    let child = ledger
        .reserve(
            RawExpansionDraftRequestV1::legacy_discovered(
                RawExpansionDraftRoleV1::NestedStaticMethod,
                "Nested.run/0",
                0,
            )
            .unwrap(),
        )
        .unwrap();
    let condition = ledger
        .reserve(RawExpansionDraftRequestV1::required_condition_fn())
        .unwrap();

    ledger
        .complete(
            child,
            receipt(
                &mut collector,
                FunctionDraftKeyV1::LegacySymbol("Nested.run/0".into()),
                "Nested.run/0",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
        )
        .unwrap();
    ledger
        .complete(
            root,
            receipt(
                &mut collector,
                FunctionDraftKeyV1::Main,
                "main",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
        )
        .unwrap();
    ledger
        .complete(
            condition,
            receipt(
                &mut collector,
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        )
        .unwrap();

    let sealed = ledger.seal().unwrap();
    assert_eq!(sealed.final_count(), 3);
    assert_eq!(
        sealed.condition(),
        RawConditionDispositionV1::RequiredCompatibility
    );
    assert_eq!(
        sealed
            .events()
            .iter()
            .map(|event| event.role())
            .collect::<Vec<_>>(),
        [
            RawExpansionDraftRoleV1::NestedStaticMethod,
            RawExpansionDraftRoleV1::RootMain,
            RawExpansionDraftRoleV1::SyntheticConditionFn,
        ]
    );
    assert_eq!(sealed.events()[0].ordinal(), 1);
    assert_eq!(sealed.events()[0].symbol(), "Nested.run/0");
    assert!(sealed.contains_symbol("main"));
    assert!(sealed.contains_symbol("condition_fn"));
}

#[test]
fn foreign_reservation_and_identity_mismatch_fail_without_retry() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut first =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let mut second =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let foreign = first
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    let error = second
        .complete(
            foreign,
            receipt(
                &mut collector,
                FunctionDraftKeyV1::Main,
                "main",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
        )
        .unwrap_err();
    assert_eq!(error, RawExpansionReceiptLedgerErrorV1::ForeignReservation);

    let expected = second
        .reserve(
            RawExpansionDraftRequestV1::legacy_discovered(
                RawExpansionDraftRoleV1::StaticMethod,
                "Expected/0",
                0,
            )
            .unwrap(),
        )
        .unwrap();
    let mismatch = second
        .complete(
            expected,
            receipt(
                &mut collector,
                FunctionDraftKeyV1::LegacySymbol("Actual/0".into()),
                "Actual/0",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
        )
        .unwrap_err();
    assert_eq!(
        mismatch,
        RawExpansionReceiptLedgerErrorV1::ReceiptKeyMismatch
    );
    assert_eq!(
        second
            .reserve(
                RawExpansionDraftRequestV1::legacy_discovered(
                    RawExpansionDraftRoleV1::StaticMethod,
                    "Retry/0",
                    0,
                )
                .unwrap(),
            )
            .unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::LedgerPoisoned
    );
    assert_eq!(
        second.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::LedgerPoisoned
    );
}

#[test]
fn open_or_incomplete_required_inventory_cannot_seal() {
    let mut open =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let _reservation = open
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    assert_eq!(
        open.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::OpenReservations { count: 1 }
    );

    let missing =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    assert_eq!(
        missing.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::MissingRootMain
    );
}
