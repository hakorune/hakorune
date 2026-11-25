#!/usr/bin/env python3
"""HEADERPORT0 Candidate0-S0 disconnected ownership guard.

The candidate owns one shell/collector state, lends the Builder only to an
active lowering closure, and exposes only typed abort/discard outcomes.  This
guard prevents the vocabulary from becoming a production capture/commit path
before Candidate0-P0 is complete.
"""

from __future__ import annotations

import pathlib

from headerport_route_inventory_guard import verify_route_inventory_extension
from headerport_header_reader_census import verify_header_reader_census
from headerport_authority_erasure_guard import verify_authority_erasure
from headerport_method_tail_compat_guard import verify_method_tail_compat


ROOT = pathlib.Path(__file__).resolve().parents[3]
MAIN_EXPANSION = ROOT / "src/mir/builder/main_expansion.rs"
ACCESS_IMPL = ROOT / "src/mir/builder/module_lowering_invocation.rs"
ROUTE_MATRIX_P0E = ROOT / "src/mir/builder/module_wiring_route_matrix_p0e.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
COLLECTOR_RECEIPT = ROOT / "src/mir/builder/module_draft_collector/receipt.rs"
COLLECTOR_RECEIPT_TESTS = ROOT / "src/mir/builder/module_draft_collector_receipt_tests.rs"
COLLECTOR_RECEIPT_P0 = ROOT / "src/mir/builder/module_draft_collector_receipt_p0.rs"
DRAINED_CANDIDATE = ROOT / "src/mir/builder/drained_module_candidate.rs"
DRAINED_CANDIDATE_P0 = ROOT / "src/mir/builder/drained_module_candidate_p0.rs"
MODULE_FINAL_CANDIDATE_P0 = ROOT / "src/mir/builder/module_finalization_candidate_p0.rs"
MODULE_INVOCATION_DRAIN_S0_TESTS = ROOT / "src/mir/builder/module_invocation_drain_s0_tests.rs"
BUILDER_MOD = ROOT / "src/mir/builder.rs"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def main() -> int:
    main_expansion = MAIN_EXPANSION.read_text()
    access_impl = ACCESS_IMPL.read_text()
    collector = COLLECTOR.read_text()
    collector_receipt = COLLECTOR_RECEIPT.read_text()
    collector_receipt_tests = COLLECTOR_RECEIPT_TESTS.read_text()
    collector_receipt_p0 = COLLECTOR_RECEIPT_P0.read_text()
    module_final_candidate_p0 = MODULE_FINAL_CANDIDATE_P0.read_text()
    builder_mod = BUILDER_MOD.read_text()
    if "mod module_lowering_access_port" in builder_mod:
        raise AssertionError("retired ACCESS0-S0 module must not re-enter")
    card = CARD.read_text()
    state = STATE.read_text()

    if len(main_expansion.splitlines()) >= 800:
        raise AssertionError("MAINROLE0-S0 source must remain below 800 lines")
    if len(collector.splitlines()) >= 800:
        raise AssertionError("MODULEDRAFT0 collector source must remain below 800 lines")
    if len(collector_receipt.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt source must remain below 800 lines")
    if len(collector_receipt_tests.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt fixtures must remain below 800 lines")
    if len(collector_receipt_p0.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt P0 proof must remain below 800 lines")
    if len(module_final_candidate_p0.splitlines()) >= 800:
        raise AssertionError("MODULEFINAL0-CANDIDATE0-P0 source must remain below 800 lines")

    for fragment in (
        "VerifiedMainExpansionV1",
        "VerifiedMainRootBodyV1",
        "VerifiedMainStaticChildV1",
        "callable_main_compat",
        "MainExpansionErrorV1",
        "app_shape_ignores_non_main_top_level_statements",
        "script_shape_without_static_main_stays_out_of_this_product",
        "child_and_root_static_contracts_are_checked_before_builder_effects",
        "duplicate_main_boxes_are_rejected_without_order_dependence",
    ):
        require(main_expansion, fragment, "MAINROLE0-S0/P0 source product/fixtures")




    require(access_impl, "with_access_port", "WIRING-S0 live access bundle/fixture")
    require(
        ACCESS_TESTS.read_text(),
        "access_port_keeps_shell_metadata_and_headers_as_short_separate_loans",
        "WIRING-S0 live access fixture",
    )

    for fragment in (
        "CollectedDraftAdmissionReceiptV1",
        "CollectedDraftReplacementDispositionV1",
        "Inserted",
        "ReplacedWholePair",
        "pub(super) fn new",
        "FunctionDraftKeyV1",
        "DraftPublicationPolicyV1",
    ):
        require(collector_receipt, fragment, "WIRING-I0-ROUTEINV-P0a receipt product")
    for fragment in (
        "fn collect(self) -> CollectedDraftAdmissionReceiptV1",
        "fn collect_sealed(",
        ") -> CollectedDraftAdmissionReceiptV1",
        "CollectedDraftAdmissionReceiptV1::new(",
    ):
        require(collector, fragment, "WIRING-I0-ROUTEINV-P0a sole receipt producer")
    for fragment in (
        "successful_commit_returns_exact_insert_receipt",
        "legacy_replacement_receipt_names_the_discarded_whole_pair",
        "canonical_commit_reports_insert_and_duplicate_stops_before_a_receipt",
        "symbol_or_arity_failure_has_no_collector_or_receipt_effect",
    ):
        require(collector_receipt_tests, fragment, "WIRING-I0-ROUTEINV-P0a fixtures")
    for fragment in (
        "ModuleDraftCollectorReceiptProofSnapshotV1",
        "receipt_proof_snapshot",
        "is_bijective",
    ):
        require(collector_receipt, fragment, "WIRING-I0-ROUTEINV-P0a exact snapshot")
    for fragment in (
        "canonical_duplicate_key_and_symbol_preserve_exact_prefix_and_indexes",
        "seal_failures_and_drop_before_collect_preserve_exact_prefix_and_indexes",
        "legacy_replacement_changes_only_one_whole_pair_and_keeps_bijection",
    ):
        require(collector_receipt_p0, fragment, "WIRING-I0-ROUTEINV-P0a-P0 matrix")
    require(
        collector,
        "legacy_index_drift_is_rejected_before_collect_mutation",
        "WIRING-I0-ROUTEINV-P0a legacy index drift fixture",
    )
    forbid(
        collector,
        "fn collect(self) -> Result",
        "fallible collector commit receipt terminal",
    )
    receipt_struct = collector_receipt.split(
        "pub(in crate::mir::builder) struct CollectedDraftAdmissionReceiptV1", 1
    )[1].split("struct CollectedDraftAdmissionReceiptSealV1", 1)[0]
    for fragment in (
        "MirBuilder",
        "ModuleDraftCollector",
        "MirModule",
        "MirFunction",
        "ValueId",
        "TypeContext",
        "header",
        "retry",
        "fallback",
    ):
        forbid(receipt_struct, fragment, f"receipt stores {fragment}")
    forbid(
        collector_receipt,
        "derive(Debug, Clone",
        "receipt product derives Clone",
    )
    receipt_constructor_users = []
    receipt_consumers = []
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        text = path.read_text()
        if "CollectedDraftAdmissionReceiptV1::new(" in text:
            receipt_constructor_users.append(path)
        if path in (
            COLLECTOR,
            COLLECTOR_RECEIPT,
            COLLECTOR_RECEIPT_TESTS,
            ACCESS_IMPL,
            BUILDER_MOD,
        ):
            continue
        if "CollectedDraftAdmissionReceiptV1" in text:
            receipt_consumers.append(str(path.relative_to(ROOT)))
    if receipt_constructor_users != [COLLECTOR]:
        raise AssertionError(
            "receipt constructor owner drift: "
            + ", ".join(str(path.relative_to(ROOT)) for path in receipt_constructor_users)
        )
    if receipt_consumers:
        raise AssertionError(
            "ROUTEINV-P0a production receipt consumer exists: "
            + ", ".join(receipt_consumers)
        )
    for relative, fragment in (
        ("src/mir/compiler/mod.rs", "fn compile_resolved_first_family"),
        ("src/mir/builder/resolved_lowering/mod.rs", "fn build_resolved_function_module"),
        (
            "src/mir/builder/resolved_lowering/mod.rs",
            "fn build_resolved_trivial_function_module",
        ),
        (
            "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            "fn lower_acyclic_callable_drafts",
        ),
        (
            "src/mir/compiler/mod.rs",
            "fn compile_resolved_recursive_callable_module",
        ),
        (
            "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            "fn build_recursive_callable_module_candidate",
        ),
    ):
        require(
            (ROOT / relative).read_text(),
            fragment,
            f"WIRING-I0 source symbol {relative}",
        )
    for relative, fragment in (
        ("src/mir/builder/module_lifecycle.rs", "lower_root"),
        ("src/mir/builder/decls.rs", "build_static_main_box"),
        ("src/mir/builder/recursive_child_lowering.rs", "RawInvocationChildPortV1"),
        ("src/mir/builder/calls/materializer.rs", "condition_fn"),
        ("src/mir/compiler/mod.rs", "compile_resolved_first_family"),
        (
            "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            "lower_resolved_trivial_function_draft",
        ),
        (
            "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            "build_recursive_callable_module_candidate",
        ),
    ):
        require(
            (ROOT / relative).read_text(),
            fragment,
            f"WIRING-P0 source anchor {relative}",
        )

    normal_drain_tests = (ROOT / "src/mir/builder/module_draft_collector/normal_collector_drain_lifecycle_tests.rs").read_text()
    require(normal_drain_tests, "prepared_normal_drain_preserves_final_legacy_key_order_until_commit", "live drain order fixture")
    for fragment in (
        "ModuleFinalizationFailureStageV1",
        "ModuleFinalizationCandidateDispositionV1",
        "ModuleFinalizationFailureMatrixV1",
        "child_failures_preserve_prefix_and_restore_parent",
        "root_and_drain_failures_discard_unpublished_invocation",
        "post_drain_finalization_has_no_fallback_route",
        "matrix_has_one_row_per_failure_owner",
    ):
        require(module_final_candidate_p0, fragment, "MODULEFINAL0-CANDIDATE0-P0 matrix")

    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (
            MAIN_EXPANSION,
            ACCESS_IMPL,
            ROUTE_MATRIX_P0E,
            DRAINED_CANDIDATE,
            DRAINED_CANDIDATE_P0,
            MODULE_FINAL_CANDIDATE_P0,
            BUILDER_MOD,
        ) or path.name.endswith("_tests.rs"):
            continue
        text = path.read_text()
        if "VerifiedMainExpansionV1" in text:
            raise AssertionError(
                f"MAINROLE0-S0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "CompletedRootBodyV1" in text or "RootBodyCompletionTrackerV1" in text:
            raise AssertionError(
                f"BODYDRAIN0-S0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "PendingMainDraftV1" in text or "MainCompletionRequestV1" in text:
            raise AssertionError(
                f"MAINPENDING0-S0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "DrainedModuleCandidateV1" in text or "CompletedInvocationInventoryV1" in text:
            raise AssertionError(
                f"DRAIN0-S0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "DrainedModuleFinalizationInputV1" in text:
            raise AssertionError(
                f"MODULEFINAL0-SPLIT0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "ModuleFinalizationFailureMatrixV1" in text:
            raise AssertionError(
                f"MODULEFINAL0-CANDIDATE0-P0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "MainRootWiringPlanV1" in text:
            raise AssertionError(
                f"WIRING-S0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "HeaderPortWiringParityV1" in text:
            raise AssertionError(
                f"WIRING-P0 production consumer exists: {path.relative_to(ROOT)}"
            )
        if "RouteOwnedInvocationInventoryV2" in text:
            raise AssertionError(
                f"WIRING-I0-ROUTEINV-S0 production consumer exists: {path.relative_to(ROOT)}"
            )

    real_fault_tests = (ROOT / "src/mir/builder/calls/function_session_tests.rs").read_text()
    real_fault_tests += (ROOT / "src/mir/builder/calls/function_session_tail_tests.rs").read_text()
    for fragment in (
        "every_fallible_checkpoint_restores_caller_and_publishes_nothing",
        "primary_and_cleanup_errors_are_both_preserved",
        "panic_backstop_restores_without_publishing",
    ):
        require(real_fault_tests, fragment, "live failure fixture")
    require(
        builder_mod,
        "mod module_draft_collector_receipt_tests;",
        "WIRING-I0-ROUTEINV-P0a fixture registration",
    )
    require(
        builder_mod,
        "mod module_draft_collector_receipt_p0;",
        "WIRING-I0-ROUTEINV-P0a-P0 registration",
    )
    if "mod drained_module_candidate;" in builder_mod or "mod drained_module_candidate_p0;" in builder_mod:
        raise AssertionError("retired drain candidate must not re-enter")
    require(builder_mod, "mod module_finalization_candidate_p0;", "MODULEFINAL0-CANDIDATE0-P0 registration")
    other_builder_files = []
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (
            BUILDER_MOD,
            MODULE_INVOCATION_DRAIN_S0_TESTS,
        ):
            continue
        if "ModuleLoweringInvocationCandidateV1" in path.read_text():
            other_builder_files.append(str(path.relative_to(ROOT)))
    if other_builder_files:
        raise AssertionError(
            "Candidate0 production/test consumer exists outside the disconnected owner: "
            + ", ".join(other_builder_files)
        )
    for symbol in (
        "InvocationCandidateRouteProofBuilderV1",
        "InvocationCandidateRouteProofV1",
    ):
        for path in (ROOT / "src/mir/builder").rglob("*.rs"):
            if path == BUILDER_MOD or path.name.endswith("_tests.rs"):
                continue
            if symbol in path.read_text():
                raise AssertionError(
                    f"Candidate0-P0 production consumer exists: {path.relative_to(ROOT)}"
                )

    for symbol in (
        "ModuleLoweringAccessPortV1",
        "MainRootWiringPlanV1",
        "HeaderPortWiringParityV1",
    ):
        for path in (ROOT / "src/mir/builder").rglob("*.rs"):
            if path in (
                ACCESS_IMPL,
                BUILDER_MOD,
            ):
                continue
            if path.name.endswith("_tests.rs"):
                continue
            if symbol in path.read_text():
                raise AssertionError(
                    f"WIRING-S0 production consumer exists: {path.relative_to(ROOT)}"
                )

    for fragment in (
        "HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-S0 (closed)",
        "HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-P0 (closed)",
        "M-root-prime decision lock and task order",
        "HEADERPORT0-I0-MAINROLE0-S0/P0 (closed)",
        "HEADERPORT0-I0-BODYDRAIN0-S0/P0 (closed)",
        "HEADERPORT0-I0-MAINPENDING0-S0/P0 (closed)",
        "HEADERPORT0-I0-ROOTBATCH0-S0/P0 (closed)",
        "HEADERPORT0-I0-SHELLFACT0-S0/P0 (closed)",
        "HEADERPORT0-I0-DRAIN0-S0 (closed)\n  route-owned inventory witness and non-Clone drained candidate",
        "HEADERPORT0-I0-DRAIN0-P0\n  exact drain/inventory/condition policy and failure matrix",
        "HEADERPORT0-I0-MODULEFINAL0-SPLIT0 (closed)\n  post-drain finalization input",
        "HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0 (closed)\n  ownership and declaration/fact failure matrix",
        "HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0\n  child/root/drain/finalizer failure matrix",
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0\n  closed",
        "HEADERPORT0-REENTRANT-TERM0-I0-WIRING-P0\n  closed",
        "Candidate A-prime",
        "WIRING-I0-ROUTEINV-S0",
        "WIRING-I0-ROUTEINV-S0 closeout",
        "WIRING-I0-ROUTEINV-P0a-RECEIPT-S0 closeout",
        "WIRING-I0-ROUTEINV-P0a-RECEIPT-P0 closeout",
        "WIRING-I0-BORROW-S0",
        "WIRING-I0-HDR0",
        "production capture/commit and\nCUT0 remain forbidden",
        "WIRING-S0 closeout",
        "WIRING-P0 closeout",
        "Candidate A: route-owned invocation inventory",
        "Candidate B: common collector, route-specific drain expectations",
        "Selected refinement: Candidate A-prime",
        "one disconnected invocation-owned shell/collector candidate",
        "typed abort/no-publication/no-retry proof",
        "production capture/commit remains forbidden",
        "`CUT0` remains forbidden",
    ):
        require(card, fragment, "Candidate0 task boundary")
    verify_route_inventory_extension(ROOT, builder_mod, card, state)
    verify_header_reader_census(ROOT)
    verify_authority_erasure(ROOT)
    verify_method_tail_compat(ROOT)

    print(
        "[headerport-candidate0-guard] ok disconnected=1 "
        "production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
