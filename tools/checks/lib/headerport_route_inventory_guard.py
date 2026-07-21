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
    source = source_path.read_text()
    tests = tests_path.read_text()
    if len(source.splitlines()) >= 800 or len(tests.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0b raw ledger source/tests must remain below 800 lines")

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
    ):
        require(source, fragment, "ROUTEINV-P0b raw ledger vocabulary")
    for fragment in (
        "exact_reservations_consume_receipts_and_seal_required_raw_inventory",
        "foreign_reservation_and_identity_mismatch_fail_without_retry",
        "open_or_incomplete_required_inventory_cannot_seal",
    ):
        require(tests, fragment, "ROUTEINV-P0b raw ledger fixtures")

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

    require(builder_mod, "mod raw_expansion_receipt_ledger;", "raw ledger registration")
    require(
        builder_mod,
        "mod raw_expansion_receipt_ledger_tests;",
        "raw ledger fixture registration",
    )
    consumers = []
    for path in (root / "src/mir/builder").rglob("*.rs"):
        if path in (source_path, tests_path, root / "src/mir/builder.rs"):
            continue
        if "RawExpansionReceiptLedgerV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("ROUTEINV-P0b production consumers: " + ", ".join(consumers))

    require(card, "WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0 closeout", "raw ledger closeout")
    require(
        state,
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0",
        "raw ledger next pointer",
    )
