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


def verify_single_header_s0(root: pathlib.Path, card: str, state: str) -> None:
    source_path = root / "src/mir/compiler/capability/resolved_owner_header.rs"
    capability_path = root / "src/mir/compiler/capability.rs"
    tests_path = root / "src/mir/compiler/capability_tests.rs"
    symbol_path = root / "src/mir/resolved_semantics/callable_symbol.rs"
    source = source_path.read_text()
    capability = capability_path.read_text()
    tests = tests_path.read_text()
    symbol = symbol_path.read_text()
    if any(
        len(text.splitlines()) >= 800
        for text in (source, capability, tests, symbol)
    ):
        raise AssertionError("ROUTEINV-P0c single-header source/proofs must remain below 800 lines")

    for fragment in (
        "VerifiedResolvedOwnerHeaderV1",
        "CanonicalFirstFamilyPlanBrandV1",
        "ResolvedOwnerHeaderFamilyV1",
        "ResolvedOwnerHeaderSealErrorV1",
        "pub(super) fn seal(",
        "CanonicalCallableSymbolV1",
        "OwnerMismatch",
        "ForeignPlan",
    ):
        require(source + capability, fragment, "ROUTEINV-P0c single-header vocabulary")
    require(
        source,
        "#[derive(Debug)]\npub(crate) struct VerifiedResolvedOwnerHeaderV1",
        "non-Clone resolved-owner header product",
    )
    product = source.split("pub(crate) struct VerifiedResolvedOwnerHeaderV1", 1)[1].split(
        "struct ResolvedOwnerHeaderSealV1", 1
    )[0]
    for fragment in (
        "MirBuilder",
        "MirModule",
        "MirFunction",
        "ModuleDraftCollector",
        "ASTNode",
        "ValueId",
        "TypeContext",
        "retry",
        "fallback",
    ):
        forbid(product, fragment, f"resolved-owner header stores {fragment}")
    forbid(source, "impl Clone for VerifiedResolvedOwnerHeaderV1", "header Clone implementation")
    forbid(source, "pub(crate) fn seal(", "crate-visible header constructor")
    forbid(source, "fn from_parts(", "caller-owned header constructor")
    forbid(source, "fn new(", "caller-owned header constructor")
    forbid(source, 'format!("{}/{}"', "duplicated physical symbol projection")
    require(
        symbol,
        "pub(crate) fn from_name_arity",
        "neutral physical symbol projection",
    )

    for fragment in (
        "resolved_owner_header_seals_zero_arity_binding_ssa_before_plan_consumption",
        "resolved_owner_header_seals_a_plus_family_without_exact_i64_profile",
        "resolved_owner_header_rejects_foreign_plan_pairing",
        "ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa",
        "ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus",
        "header.arity(), 0",
        "ResolvedOwnerHeaderSealErrorV1::ForeignPlan",
    ):
        require(tests, fragment, "ROUTEINV-P0c single-header fixture")

    consumers = []
    excluded = {source_path, capability_path, tests_path}
    for path in (root / "src/mir").rglob("*.rs"):
        if path in excluded:
            continue
        if "VerifiedResolvedOwnerHeaderV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("ROUTEINV-P0c production consumers: " + ", ".join(consumers))
    call_count = 0
    for path in (root / "src/mir").rglob("*.rs"):
        if path == tests_path:
            continue
        call_count += path.read_text().count("seal_resolved_owner_header_v1(")
    if call_count != 1:
        raise AssertionError(
            "ROUTEINV-P0c seal issuer must have zero production callers: "
            f"occurrences={call_count}"
        )

    require(card, "WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0 closeout", "single-header closeout")
    require(
        state,
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0",
        "single-header next pointer",
    )


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
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-",
        "raw ledger next pointer",
    )
    verify_single_header_s0(root, card, state)
