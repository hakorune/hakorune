//! WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0 proof matrix.

use super::module_draft_collector::{
    CompletedDraftSignatureViewV1, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    ModuleDraftCollectorV1,
};
use super::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionAbortReasonV1,
    RawExpansionCutoverStopV1, RawExpansionDraftRequestV1, RawExpansionDraftRoleV1,
    RawExpansionReceiptLedgerErrorV1, RawExpansionReceiptLedgerV1, RawExpansionReplacementEventV1,
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

fn collect(
    collector: &mut ModuleDraftCollectorV1,
    key: FunctionDraftKeyV1,
    symbol: &str,
    arity: usize,
    policy: DraftPublicationPolicyV1,
) -> super::module_draft_collector::CollectedDraftAdmissionReceiptV1 {
    collector
        .prepare_admission(key, symbol.to_owned(), arity, policy)
        .unwrap()
        .seal(draft(symbol, arity))
        .unwrap()
        .collect()
}

fn collect_legacy(
    collector: &mut ModuleDraftCollectorV1,
    symbol: &str,
    arity: usize,
) -> super::module_draft_collector::CollectedDraftAdmissionReceiptV1 {
    collect(
        collector,
        FunctionDraftKeyV1::LegacySymbol(symbol.to_owned()),
        symbol,
        arity,
        DraftPublicationPolicyV1::LegacyReplaceWholePair,
    )
}

fn reserve_legacy(
    ledger: &mut RawExpansionReceiptLedgerV1,
    role: RawExpansionDraftRoleV1,
    symbol: &str,
    arity: usize,
) -> super::raw_expansion_receipt_ledger::RawExpansionReservationV1 {
    ledger
        .reserve(RawExpansionDraftRequestV1::legacy_discovered(role, symbol, arity).unwrap())
        .unwrap()
}

fn complete_required_root_and_condition(
    ledger: &mut RawExpansionReceiptLedgerV1,
    collector: &mut ModuleDraftCollectorV1,
) {
    let root = ledger
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    let condition = ledger
        .reserve(RawExpansionDraftRequestV1::required_condition_fn())
        .unwrap();
    ledger
        .complete(
            root,
            collect(
                collector,
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
            collect(
                collector,
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        )
        .unwrap();
}

#[test]
fn every_raw_role_is_receipt_backed_and_nested_completion_precedes_outer() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::Selected);
    let root = ledger
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    let condition = ledger
        .reserve(RawExpansionDraftRequestV1::required_condition_fn())
        .unwrap();
    let top = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::TopLevelFunction,
        "top/0",
        0,
    );
    let outer_static = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::StaticMethod,
        "Outer.static/1",
        1,
    );
    let nested_static = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::NestedStaticMethod,
        "Nested.static/0",
        0,
    );
    let outer_instance = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::InstanceMethod,
        "Outer.instance/2",
        2,
    );
    let nested_instance = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::NestedInstanceMethod,
        "Nested.instance/1",
        1,
    );
    let outer_constructor = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::Constructor,
        "Outer.birth/1",
        1,
    );
    let nested_constructor = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::NestedConstructor,
        "Nested.birth/0",
        0,
    );
    let callable_main = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::CallableMainCompatibility,
        "Main.main/1",
        1,
    );

    for (reservation, symbol, arity) in [
        (nested_static, "Nested.static/0", 0),
        (outer_static, "Outer.static/1", 1),
        (nested_instance, "Nested.instance/1", 1),
        (outer_instance, "Outer.instance/2", 2),
        (nested_constructor, "Nested.birth/0", 0),
        (outer_constructor, "Outer.birth/1", 1),
        (top, "top/0", 0),
        (callable_main, "Main.main/1", 1),
    ] {
        ledger
            .complete(reservation, collect_legacy(&mut collector, symbol, arity))
            .unwrap();
    }
    ledger
        .complete(
            root,
            collect(
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
            collect(
                &mut collector,
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        )
        .unwrap();

    let sealed = ledger.seal().unwrap();
    assert_eq!(sealed.final_count(), 10);
    assert_eq!(
        sealed.callable_main(),
        RawCallableMainCompatibilityDispositionV1::Selected
    );
    assert_eq!(
        sealed
            .events()
            .iter()
            .map(|event| event.role())
            .collect::<Vec<_>>(),
        [
            RawExpansionDraftRoleV1::NestedStaticMethod,
            RawExpansionDraftRoleV1::StaticMethod,
            RawExpansionDraftRoleV1::NestedInstanceMethod,
            RawExpansionDraftRoleV1::InstanceMethod,
            RawExpansionDraftRoleV1::NestedConstructor,
            RawExpansionDraftRoleV1::Constructor,
            RawExpansionDraftRoleV1::TopLevelFunction,
            RawExpansionDraftRoleV1::CallableMainCompatibility,
            RawExpansionDraftRoleV1::RootMain,
            RawExpansionDraftRoleV1::SyntheticConditionFn,
        ]
    );
    assert_eq!(
        sealed.cutover_stops(),
        &[
            RawExpansionCutoverStopV1::DuplicateMainSourcePolicySelectionRequired,
            RawExpansionCutoverStopV1::CallableMainFailurePropagationPolicySelectionRequired,
        ]
    );
}

#[test]
fn callable_main_selected_and_not_selected_dispositions_are_exact() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut not_selected =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    complete_required_root_and_condition(&mut not_selected, &mut collector);
    assert_eq!(
        not_selected.seal().unwrap().callable_main(),
        RawCallableMainCompatibilityDispositionV1::NotSelected
    );

    let mut collector = ModuleDraftCollectorV1::default();
    let mut selected =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::Selected);
    complete_required_root_and_condition(&mut selected, &mut collector);
    assert_eq!(
        selected.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::MissingCallableMainCompatibility
    );

    let mut collector = ModuleDraftCollectorV1::default();
    let mut unexpected =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let compat = reserve_legacy(
        &mut unexpected,
        RawExpansionDraftRoleV1::CallableMainCompatibility,
        "Main.main/0",
        0,
    );
    unexpected
        .complete(compat, collect_legacy(&mut collector, "Main.main/0", 0))
        .unwrap();
    complete_required_root_and_condition(&mut unexpected, &mut collector);
    assert_eq!(
        unexpected.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::UnexpectedCallableMainCompatibility
    );
}

#[test]
fn child_abort_preserves_completed_prefix_and_consumes_seal_authority() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let prefix = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::TopLevelFunction,
        "prefix/0",
        0,
    );
    ledger
        .complete(prefix, collect_legacy(&mut collector, "prefix/0", 0))
        .unwrap();
    let _outer = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::StaticMethod,
        "Outer.run/0",
        0,
    );
    let nested = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::NestedStaticMethod,
        "Nested.fail/0",
        0,
    );

    let aborted = ledger
        .abort(nested, RawExpansionAbortReasonV1::Primary)
        .unwrap();
    assert_eq!(aborted.final_count(), 1);
    assert_eq!(aborted.events()[0].symbol(), "prefix/0");
    assert_eq!(aborted.failed_ordinal(), 2);
    assert_eq!(
        aborted.failed_role(),
        RawExpansionDraftRoleV1::NestedStaticMethod
    );
    assert_eq!(aborted.reason(), RawExpansionAbortReasonV1::Primary);
    assert_eq!(aborted.outstanding_reservations(), 1);
    assert_eq!(collector.symbol_count(), 1);
    assert!(collector.contains_symbol("prefix/0"));
    assert!(!collector.contains_symbol("Outer.run/0"));
    assert!(!collector.contains_symbol("Nested.fail/0"));
}

#[test]
fn root_abort_after_completed_children_returns_only_non_sealable_evidence() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let child = reserve_legacy(
        &mut ledger,
        RawExpansionDraftRoleV1::StaticMethod,
        "Main.helper/0",
        0,
    );
    ledger
        .complete(child, collect_legacy(&mut collector, "Main.helper/0", 0))
        .unwrap();
    let condition = ledger
        .reserve(RawExpansionDraftRequestV1::required_condition_fn())
        .unwrap();
    ledger
        .complete(
            condition,
            collect(
                &mut collector,
                FunctionDraftKeyV1::SyntheticConditionFn,
                "condition_fn",
                1,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            ),
        )
        .unwrap();
    let root = ledger
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();

    let aborted = ledger
        .abort(root, RawExpansionAbortReasonV1::Cleanup)
        .unwrap();
    assert_eq!(aborted.final_count(), 2);
    assert_eq!(aborted.failed_role(), RawExpansionDraftRoleV1::RootMain);
    assert_eq!(aborted.reason(), RawExpansionAbortReasonV1::Cleanup);
    assert_eq!(aborted.outstanding_reservations(), 0);
    assert_eq!(collector.symbol_count(), 2);
    assert!(collector.contains_symbol("Main.helper/0"));
    assert!(collector.contains_symbol("condition_fn"));
    assert!(!collector.contains_symbol("main"));
}

#[test]
fn duplicate_legacy_symbol_replaces_final_pair_and_keeps_event_history() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    for role in [
        RawExpansionDraftRoleV1::StaticMethod,
        RawExpansionDraftRoleV1::NestedStaticMethod,
    ] {
        let reservation = reserve_legacy(&mut ledger, role, "Replace.run/0", 0);
        ledger
            .complete(
                reservation,
                collect_legacy(&mut collector, "Replace.run/0", 0),
            )
            .unwrap();
    }
    complete_required_root_and_condition(&mut ledger, &mut collector);

    let sealed = ledger.seal().unwrap();
    assert_eq!(sealed.events().len(), 4);
    assert_eq!(sealed.final_count(), 3);
    let final_event = sealed.final_event_for_symbol("Replace.run/0").unwrap();
    assert_eq!(
        final_event.role(),
        RawExpansionDraftRoleV1::NestedStaticMethod
    );
    assert_eq!(
        final_event.replacement(),
        &RawExpansionReplacementEventV1::ReplacedWholePair {
            previous_key: FunctionDraftKeyV1::LegacySymbol("Replace.run/0".into()),
            previous_symbol: "Replace.run/0".into(),
        }
    );
}

#[test]
fn missing_required_condition_rejects_after_root_receipt() {
    let mut collector = ModuleDraftCollectorV1::default();
    let mut ledger =
        RawExpansionReceiptLedgerV1::new(RawCallableMainCompatibilityDispositionV1::NotSelected);
    let root = ledger
        .reserve(RawExpansionDraftRequestV1::root_main())
        .unwrap();
    ledger
        .complete(
            root,
            collect(
                &mut collector,
                FunctionDraftKeyV1::Main,
                "main",
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            ),
        )
        .unwrap();
    assert_eq!(
        ledger.seal().unwrap_err(),
        RawExpansionReceiptLedgerErrorV1::MissingConditionFn
    );
}
