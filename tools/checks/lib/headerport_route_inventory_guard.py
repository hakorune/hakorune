#!/usr/bin/env python3
"""Reusable HEADERPORT0 route-inventory extension guard."""

from __future__ import annotations

import pathlib


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def verify_route_inventory_extension(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    source_path = root / "src/mir/builder/raw_expansion_receipt_ledger.rs"
    tests_path = root / "src/mir/builder/raw_expansion_receipt_ledger_tests.rs"
    p0_path = root / "src/mir/builder/raw_expansion_receipt_ledger_p0.rs"
    source = source_path.read_text()
    tests = tests_path.read_text()
    p0 = p0_path.read_text()
    if any(len(text.splitlines()) >= 800 for text in (source, tests, p0)):
        raise AssertionError("ROUTEINV-P0b raw ledger source/proofs must remain below 800 lines")

    for fragment in (
        "RawExpansionReceiptLedgerV1",
        "RawExpansionReservationV1",
        "RawExpansionDraftRequestV1",
        "RawExpansionCompletedEventV1",
        "SealedRawExpansionReceiptLedgerV1",
        "RawConditionDispositionV1::RequiredCompatibility",
        "CollectedDraftAdmissionReceiptV1",
        "ReplacedWholePair",
        "ForeignReservation",
        "LedgerPoisoned",
        "OpenReservations",
        "MissingRootMain",
        "MissingConditionFn",
        "RawCallableMainCompatibilityDispositionV1",
        "AbortedRawExpansionReceiptLedgerV1",
        "RawExpansionCutoverStopV1",
        "MissingCallableMainCompatibility",
        "UnexpectedCallableMainCompatibility",
    ):
        require(source, fragment, "ROUTEINV-P0b raw ledger vocabulary")
    for fragment in (
        "exact_reservations_consume_receipts_and_seal_required_raw_inventory",
        "foreign_reservation_and_identity_mismatch_fail_without_retry",
        "open_or_incomplete_required_inventory_cannot_seal",
    ):
        require(tests, fragment, "ROUTEINV-P0b raw ledger fixtures")
    for fragment in (
        "every_raw_role_is_receipt_backed_and_nested_completion_precedes_outer",
        "callable_main_selected_and_not_selected_dispositions_are_exact",
        "child_abort_preserves_completed_prefix_and_consumes_seal_authority",
        "root_abort_after_completed_children_returns_only_non_sealable_evidence",
        "duplicate_legacy_symbol_replaces_final_pair_and_keeps_event_history",
        "missing_required_condition_rejects_after_root_receipt",
        "RawExpansionDraftRoleV1::TopLevelFunction",
        "RawExpansionDraftRoleV1::StaticMethod",
        "RawExpansionDraftRoleV1::InstanceMethod",
        "RawExpansionDraftRoleV1::Constructor",
        "RawExpansionDraftRoleV1::CallableMainCompatibility",
        "RawExpansionDraftRoleV1::NestedStaticMethod",
        "RawExpansionDraftRoleV1::NestedInstanceMethod",
        "RawExpansionDraftRoleV1::NestedConstructor",
    ):
        require(p0, fragment, "ROUTEINV-P0b matrix fixture")

    ledger_struct = source.split(
        "pub(in crate::mir::builder) struct RawExpansionReceiptLedgerV1", 1
    )[1].split("struct RawExpansionReceiptLedgerSealV1", 1)[0]
    sealed_struct = source.split(
        "pub(in crate::mir::builder) struct SealedRawExpansionReceiptLedgerV1", 1
    )[1].split("struct SealedRawExpansionReceiptLedgerSealV1", 1)[0]
    for product, label in ((ledger_struct, "open ledger"), (sealed_struct, "sealed ledger")):
        for fragment in (
            "MirBuilder",
            "MirModule",
            "MirFunction",
            "ModuleDraftCollector",
            "ASTNode",
            "ValueId",
            "header",
            "retry",
            "fallback",
        ):
            forbid(product, fragment, f"{label} stores {fragment}")
    aborted_impl = source.split("impl AbortedRawExpansionReceiptLedgerV1", 1)[1]
    forbid(aborted_impl, "fn seal(", "aborted ledger regains seal authority")

    require(builder_mod, "mod raw_expansion_receipt_ledger;", "raw ledger registration")
    require(
        builder_mod,
        "mod raw_expansion_receipt_ledger_tests;",
        "raw ledger fixture registration",
    )
    require(
        builder_mod,
        "mod raw_expansion_receipt_ledger_p0;",
        "raw ledger P0 registration",
    )
    consumers = []
    for path in (root / "src/mir/builder").rglob("*.rs"):
        if path in (source_path, tests_path, p0_path, root / "src/mir/builder.rs"):
            continue
        if "RawExpansionReceiptLedgerV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("ROUTEINV-P0b production consumers: " + ", ".join(consumers))

    main_expansion = (root / "src/mir/builder/main_expansion.rs").read_text()
    legacy_main = (root / "src/mir/builder/decls.rs").read_text()
    require(
        main_expansion,
        "MainExpansionErrorV1::DuplicateMainBox",
        "duplicate Main compatibility stop source",
    )
    require(
        legacy_main,
        "let _ = self.lower_static_method_as_function(",
        "swallowed callable Main failure compatibility stop source",
    )

    require(card, "WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0 closeout", "raw ledger closeout")
    require(
        state,
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0",
        "raw ledger next pointer",
    )
