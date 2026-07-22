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


ROOT = pathlib.Path(__file__).resolve().parents[3]
CANDIDATE = ROOT / "src/mir/builder/module_lowering_invocation_candidate.rs"
CANDIDATE_P0 = ROOT / "src/mir/builder/module_lowering_invocation_candidate_p0.rs"
MAIN_EXPANSION = ROOT / "src/mir/builder/main_expansion.rs"
ROOT_BODY = ROOT / "src/mir/builder/root_body_completion.rs"
ROOT_BODY_P0 = ROOT / "src/mir/builder/root_body_completion_p0.rs"
MAIN_PENDING = ROOT / "src/mir/builder/main_pending_draft.rs"
MAIN_PENDING_P0 = ROOT / "src/mir/builder/main_pending_draft_p0.rs"
ROOT_BATCH = ROOT / "src/mir/builder/root_draft_batch.rs"
ROOT_BATCH_P0 = ROOT / "src/mir/builder/root_draft_batch_p0.rs"
MAIN_ROOT_WIRING = ROOT / "src/mir/builder/main_root_wiring.rs"
ACCESS_VOCAB = ROOT / "src/mir/builder/module_lowering_access_port.rs"
ACCESS_IMPL = ROOT / "src/mir/builder/module_lowering_invocation.rs"
ACCESS_LIVE = ROOT / "src/mir/builder/module_lowering_invocation_access.rs"
ACCESS_TESTS = ROOT / "src/mir/builder/module_lowering_invocation_access_tests.rs"
WIRING_P0 = ROOT / "src/mir/builder/module_wiring_parity_p0.rs"
ROUTE_INVENTORY = ROOT / "src/mir/builder/route_owned_invocation_inventory.rs"
ROUTE_MATRIX_P0E = ROOT / "src/mir/builder/module_wiring_route_matrix_p0e.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
COLLECTOR_RECEIPT = ROOT / "src/mir/builder/module_draft_collector/receipt.rs"
COLLECTOR_RECEIPT_TESTS = ROOT / "src/mir/builder/module_draft_collector_receipt_tests.rs"
COLLECTOR_RECEIPT_P0 = ROOT / "src/mir/builder/module_draft_collector_receipt_p0.rs"
RAW_LEDGER = ROOT / "src/mir/builder/raw_expansion_receipt_ledger.rs"
RAW_LEDGER_P0 = ROOT / "src/mir/builder/raw_expansion_receipt_ledger_p0.rs"
RAW_LEDGER_TESTS = ROOT / "src/mir/builder/raw_expansion_receipt_ledger_tests.rs"
SHELL_FACTS = ROOT / "src/mir/builder/module_declaration_facts.rs"
SHELL_FACTS_P0 = ROOT / "src/mir/builder/module_declaration_facts_p0.rs"
DRAINED_CANDIDATE = ROOT / "src/mir/builder/drained_module_candidate.rs"
DRAINED_CANDIDATE_P0 = ROOT / "src/mir/builder/drained_module_candidate_p0.rs"
MODULE_FINAL = ROOT / "src/mir/builder/module_finalization_split.rs"
MODULE_FINAL_P0 = ROOT / "src/mir/builder/module_finalization_split_p0.rs"
MODULE_FINAL_CANDIDATE_P0 = ROOT / "src/mir/builder/module_finalization_candidate_p0.rs"
BORROW_ROOT_P0 = ROOT / "src/mir/builder/module_lowering_borrow_root_p0.rs"
ROOT_BATCH_COMMIT = ROOT / "src/mir/builder/module_draft_collector/root_batch.rs"
ROOT_BATCH_COMMIT_P0 = ROOT / "src/mir/builder/root_draft_batch_commit_p0.rs"
DECL_FACT_COMMIT = ROOT / "src/mir/builder/module_lowering_shell/declaration_fact_commit.rs"
DECL_FACT_COMMIT_P0 = ROOT / "src/mir/builder/module_declaration_fact_shell_commit_p0.rs"
BORROW_ROOT_P0D = ROOT / "src/mir/builder/module_lowering_borrow_root_p0d.rs"
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
    candidate = CANDIDATE.read_text()
    candidate_p0 = CANDIDATE_P0.read_text()
    main_expansion = MAIN_EXPANSION.read_text()
    root_body = ROOT_BODY.read_text()
    root_body_p0 = ROOT_BODY_P0.read_text()
    main_pending = MAIN_PENDING.read_text()
    main_pending_p0 = MAIN_PENDING_P0.read_text()
    root_batch = ROOT_BATCH.read_text()
    root_batch_p0 = ROOT_BATCH_P0.read_text()
    main_root_wiring = MAIN_ROOT_WIRING.read_text()
    access_vocab = ACCESS_VOCAB.read_text()
    access_impl = ACCESS_IMPL.read_text()
    access_live = ACCESS_LIVE.read_text()
    wiring_p0 = WIRING_P0.read_text()
    route_inventory = ROUTE_INVENTORY.read_text()
    collector = COLLECTOR.read_text()
    collector_receipt = COLLECTOR_RECEIPT.read_text()
    collector_receipt_tests = COLLECTOR_RECEIPT_TESTS.read_text()
    collector_receipt_p0 = COLLECTOR_RECEIPT_P0.read_text()
    shell_facts = SHELL_FACTS.read_text()
    shell_facts_p0 = SHELL_FACTS_P0.read_text()
    drained_candidate = DRAINED_CANDIDATE.read_text()
    drained_candidate_p0 = DRAINED_CANDIDATE_P0.read_text()
    module_final = MODULE_FINAL.read_text()
    module_final_p0 = MODULE_FINAL_P0.read_text()
    module_final_candidate_p0 = MODULE_FINAL_CANDIDATE_P0.read_text()
    builder_mod = BUILDER_MOD.read_text()
    card = CARD.read_text()
    state = STATE.read_text()

    if len(candidate.splitlines()) >= 800:
        raise AssertionError("Candidate0 source must remain below 800 lines")
    if len(candidate_p0.splitlines()) >= 800:
        raise AssertionError("Candidate0 P0 source must remain below 800 lines")
    if len(main_expansion.splitlines()) >= 800:
        raise AssertionError("MAINROLE0-S0 source must remain below 800 lines")
    if len(root_body.splitlines()) >= 800:
        raise AssertionError("BODYDRAIN0-S0 source must remain below 800 lines")
    if len(root_body_p0.splitlines()) >= 800:
        raise AssertionError("BODYDRAIN0-P0 fixture source must remain below 800 lines")
    if len(main_pending.splitlines()) >= 800:
        raise AssertionError("MAINPENDING0-S0 source must remain below 800 lines")
    if len(main_pending_p0.splitlines()) >= 800:
        raise AssertionError("MAINPENDING0-P0 fixture source must remain below 800 lines")
    if len(root_batch.splitlines()) >= 800:
        raise AssertionError("ROOTBATCH0-S0 source must remain below 800 lines")
    if len(root_batch_p0.splitlines()) >= 800:
        raise AssertionError("ROOTBATCH0-P0 fixture source must remain below 800 lines")
    if len(main_root_wiring.splitlines()) >= 800:
        raise AssertionError("WIRING-S0 source must remain below 800 lines")
    if len(wiring_p0.splitlines()) >= 800:
        raise AssertionError("WIRING-P0 source must remain below 800 lines")
    if len(route_inventory.splitlines()) >= 800:
        raise AssertionError("WIRING-I0-ROUTEINV-S0 source must remain below 800 lines")
    if len(collector.splitlines()) >= 800:
        raise AssertionError("MODULEDRAFT0 collector source must remain below 800 lines")
    if len(collector_receipt.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt source must remain below 800 lines")
    if len(collector_receipt_tests.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt fixtures must remain below 800 lines")
    if len(collector_receipt_p0.splitlines()) >= 800:
        raise AssertionError("ROUTEINV-P0a receipt P0 proof must remain below 800 lines")
    if len(shell_facts.splitlines()) >= 800:
        raise AssertionError("SHELLFACT0-S0 source must remain below 800 lines")
    if len(shell_facts_p0.splitlines()) >= 800:
        raise AssertionError("SHELLFACT0-P0 fixture source must remain below 800 lines")
    if len(drained_candidate.splitlines()) >= 800:
        raise AssertionError("DRAIN0-S0 source must remain below 800 lines")
    if len(drained_candidate_p0.splitlines()) >= 800:
        raise AssertionError("DRAIN0-P0 fixture source must remain below 800 lines")
    if len(module_final.splitlines()) >= 800:
        raise AssertionError("MODULEFINAL0-SPLIT0 source must remain below 800 lines")
    if len(module_final_p0.splitlines()) >= 800:
        raise AssertionError("MODULEFINAL0-SPLIT0-P0 fixture source must remain below 800 lines")
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

    for fragment in (
        "CompletedRootBodyV1",
        "RootBodyCompletionTrackerV1",
        "RootBodyActivityTokenV1",
        "RootBodyResultV1",
        "OpenChildScopes",
        "OpenHeaderLoans",
        "OpenPendingTerminals",
        "nested_activity_closes_before_value_witness",
        "foreign_and_mismatched_tokens_fail_closed",
    ):
        require(root_body, fragment, "BODYDRAIN0-S0 source product/fixtures")
    for fragment in (
        "nested_children_close_inner_before_outer",
        "header_and_pending_tokens_close_before_root_completion",
        "each_open_activity_has_a_distinct_fail_fast_disposition",
        "failed_completion_consumes_the_tracker_without_a_witness",
    ):
        require(root_body_p0, fragment, "BODYDRAIN0-P0 closure/failure fixtures")

    for fragment in (
        "PendingMainDraftV1",
        "MainCompletionRequestV1",
        "MainHeaderLoanV1",
        "MainHeaderSourceV1",
        "finish_consumes_short_header_loan_without_storing_it",
        "foreign_symbol_or_arity_is_rejected_before_pending_product",
    ):
        require(main_pending, fragment, "MAINPENDING0-S0 source product/fixtures")
    for fragment in (
        "header_source_matrix_preserves_the_selected_route_without_fallback",
        "root_value_and_no_value_dispositions_are_preserved",
    ):
        require(main_pending_p0, fragment, "MAINPENDING0-P0 parity fixtures")

    for fragment in (
        "PreparedRootDraftBatchV1",
        "PendingConditionFnDraftV1",
        "RootDraftAdmissionPlanV1",
        "required_condition_fn_prepares_one_atomic_root_batch",
        "optional_missing_and_forbidden_present_are_explicit",
        "malformed_condition_fn_is_rejected_before_batch_product",
    ):
        require(root_batch, fragment, "ROOTBATCH0-S0 source product/fixtures")
    for fragment in (
        "condition_policy_matrix_has_one_primary_result",
        "condition_identity_failures_are_typed_before_batch_creation",
    ):
        require(root_batch_p0, fragment, "ROOTBATCH0-P0 policy/failure fixtures")

    for fragment in (
        "MainRootWiringPlanV1",
        "MainRootWiringStepV1",
        "MainRootFunctionIdentityV1",
        "StaticChildren",
        "CallableMainCompatibility",
        "InlineRootBody",
        "root_is_distinct_and_children_precede_inline_body",
        "compatibility_child_is_optional_but_root_body_is_not",
    ):
        require(main_root_wiring, fragment, "WIRING-S0 Main/root order vocabulary/fixtures")

    for fragment in (
        "ModuleLoweringAccessSurfaceV1",
        "ModuleLoweringHeaderOperationV1",
        "ModuleLoweringShellOperationV1",
        "ModuleLoweringTerminalOperationV1",
        "ModuleLoweringAccessPortV1",
        "access_port_contract_has_exact_three_surfaces",
        "shell_contract_names_current_metadata_holes",
        "terminal_contract_is_commit_only",
    ):
        require(access_vocab, fragment, "ACCESS0-S0 capability vocabulary/fixtures")

    require(access_impl, "with_access_port", "WIRING-S0 live access bundle/fixture")
    for fragment in (
        "ModuleLoweringInvocationAccessPortV1",
        "with_finalizer_headers",
    ):
        require(access_live, fragment, "WIRING-S0 live access bundle/fixture")
    require(
        ACCESS_TESTS.read_text(),
        "access_port_keeps_shell_metadata_and_headers_as_short_separate_loans",
        "WIRING-S0 live access fixture",
    )

    for fragment in (
        "HeaderPortWiringParityV1",
        "WiringParityRowV1",
        "WiringSurfaceV1",
        "WiringSourceAnchorV1",
        "WiringSourceSiteV1",
        "WiringOwnerV1",
        "WiringObservationV1",
        "ConditionPolicyObservationV1",
        "parity_derives_all_route_rows_without_redeclaring_route_identity",
        "main_and_condition_routes_keep_root_batch_and_drain_boundaries",
        "canonical_and_raw_children_require_capture_without_a_fallback_surface",
    ):
        require(wiring_p0, fragment, "WIRING-P0 route parity product/fixtures")

    for fragment in (
        "RouteOwnedInvocationInventoryV2",
        "RouteOwnedInventoryPolicyV2",
        "StaticRouteReachabilityV2",
        "InvocationInventoryAuthorityV2",
        "InvocationRootPolicyV2",
        "RouteConditionPolicyV2",
        "ExactInvocationSourceSymbolsV2",
        "InvocationRouteMatrixV1::rows()",
        "RawExpansionReceipts",
        "CanonicalResolvedOwner",
        "CanonicalCallableCatalog",
        "route_matrix_projects_to_four_policy_lanes_without_merging_families",
        "raw_and_canonical_policies_keep_distinct_root_and_condition_laws",
        "exact_ingress_and_lowering_root_symbols_are_sealed_per_family",
        "unknown_or_unreachable_source_topology_cannot_issue_a_policy",
    ):
        require(route_inventory, fragment, "WIRING-I0-ROUTEINV-S0 product/fixtures")
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
            RAW_LEDGER,
            RAW_LEDGER_P0,
            RAW_LEDGER_TESTS,
            ROOT_BATCH_COMMIT,
            ROOT_BATCH_COMMIT_P0,
            DECL_FACT_COMMIT,
            DECL_FACT_COMMIT_P0,
            BORROW_ROOT_P0D,
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
    for fragment in (
        "MirCompiler::compile_legacy_request",
        "MirBuilder::build_module",
        "MirCompiler::compile_resolved_first_family",
        "MirBuilder::build_resolved_function_module",
        "MirBuilder::build_resolved_trivial_function_module",
        "MirCompiler::compile_resolved_callable_module",
        "MirBuilder::build_acyclic_callable_module_candidate",
        "MirCompiler::compile_resolved_recursive_callable_module",
        "MirBuilder::build_recursive_callable_module_candidate",
    ):
        require(route_inventory, fragment, "WIRING-I0 exact ingress/root symbol")
    for relative, fragment in (
        ("src/mir/compiler/mod.rs", "fn compile_legacy_request"),
        ("src/mir/builder/builder_build.rs", "fn build_module"),
        ("src/mir/compiler/mod.rs", "fn compile_resolved_first_family"),
        ("src/mir/builder/resolved_lowering/mod.rs", "fn build_resolved_function_module"),
        (
            "src/mir/builder/resolved_lowering/mod.rs",
            "fn build_resolved_trivial_function_module",
        ),
        ("src/mir/compiler/mod.rs", "fn compile_resolved_callable_module"),
        (
            "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            "fn build_acyclic_callable_module_candidate",
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
    inventory_struct = route_inventory.split(
        "pub(in crate::mir::builder) struct RouteOwnedInventoryPolicyV2", 1
    )[1].split("struct RouteOwnedInventoryPolicySealV2", 1)[0]
    for fragment in (
        "MirBuilder",
        "ModuleDraftCollectorV1",
        "MirModule",
        "MirFunction",
        "ValueId",
        "TypeContext",
        "retry",
    ):
        forbid(inventory_struct, fragment, f"route inventory stores {fragment}")

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

    for fragment in (
        "SealedModuleDeclarationFactsV1",
        "user_box_field_decls",
        "record_decls",
        "enum_decls",
        "declaration_snapshot_preserves_all_four_source_fact_lanes",
        "btree_snapshot_order_is_independent_of_insertion_order",
    ):
        require(shell_facts, fragment, "SHELLFACT0-S0 source product/fixtures")
    for fragment in (
        "all_declaration_lanes_move_together_at_the_shell_boundary",
        "empty_and_nonempty_lane_shapes_remain_explicit",
    ):
        require(shell_facts_p0, fragment, "SHELLFACT0-P0 lane/failure fixtures")
    for fragment in (
        "CompletedInvocationInventoryV1",
        "DrainedModuleCandidateV1",
        "DrainedModuleCandidateErrorV1",
        "from_drained_module",
        "exact_inventory_and_root_policy_co_seal_candidate",
        "inventory_and_condition_failures_happen_before_candidate_issue",
        "candidate_does_not_expose_a_bare_module_consumer",
    ):
        require(drained_candidate, fragment, "DRAIN0-S0 source product/fixtures")
    for fragment in (
        "exact_drain_inventory_and_candidate_policy_co_seal",
        "drain_inventory_order_is_deterministic_before_candidate_issue",
        "drain_rejects_missing_main_before_candidate_issue",
        "drain_condition_policy_matrix_is_explicit",
        "candidate_rejects_inventory_mismatch_without_exposing_module",
        "candidate_rejects_missing_main_even_when_inventory_matches",
        "duplicate_inventory_is_rejected_before_drain_candidate_creation",
    ):
        require(drained_candidate_p0, fragment, "DRAIN0-P0 failure matrix fixtures")
    for fragment in (
        "DrainedModuleFinalizationInputV1",
        "into_parts",
        "post-drain finalization input",
    ):
        require(module_final, fragment, "MODULEFINAL0-SPLIT0 source product")
    for fragment in (
        "post_drain_input_co_seals_candidate_and_declaration_facts",
        "post_drain_input_consumes_both_owners_once",
        "post_drain_input_preserves_all_declaration_lanes_without_refresh",
        "post_drain_input_keeps_root_value_witness_separate_from_module_facts",
    ):
        require(module_final_p0, fragment, "MODULEFINAL0-SPLIT0 boundary fixtures")
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
    facts_struct = shell_facts.split(
        "pub(in crate::mir::builder) struct SealedModuleDeclarationFactsV1", 1
    )[1].split("#[derive(Debug, Clone, Copy", 1)[0]
    forbid(facts_struct, "MirBuilder", "declaration facts store Builder")
    forbid(facts_struct, "ModuleDraftCollector", "declaration facts store collector")
    forbid(facts_struct, "ASTNode", "declaration facts store AST body")
    batch_struct = root_batch.split(
        "pub(in crate::mir::builder) struct PreparedRootDraftBatchV1", 1
    )[1].split("#[derive(Debug)]\nstruct PreparedRootDraftBatchSealV1", 1)[0]
    forbid(batch_struct, "ModuleDraftCollector", "root batch stores collector")
    forbid(batch_struct, "MirBuilder", "root batch stores Builder")
    pending_struct = main_pending.split(
        "pub(in crate::mir::builder) struct PendingMainDraftV1", 1
    )[1].split("#[derive(Debug)]\nstruct PendingMainDraftSealV1", 1)[0]
    forbid(pending_struct, "MainHeaderLoanV1", "pending draft stores header loan")
    forbid(pending_struct, "MirBuilder", "pending draft stores Builder")
    forbid(pending_struct, "ModuleDraftCollector", "pending draft stores collector")

    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (
            MAIN_EXPANSION,
            ROOT_BODY,
            ROOT_BODY_P0,
            MAIN_PENDING,
            MAIN_PENDING_P0,
            ROOT_BATCH,
            ROOT_BATCH_P0,
            MAIN_ROOT_WIRING,
            ACCESS_VOCAB,
            ACCESS_IMPL,
            ACCESS_LIVE,
            WIRING_P0,
            ROUTE_INVENTORY,
            ROUTE_MATRIX_P0E,
            SHELL_FACTS,
            SHELL_FACTS_P0,
            DRAINED_CANDIDATE,
            DRAINED_CANDIDATE_P0,
            MODULE_FINAL,
            MODULE_FINAL_P0,
            MODULE_FINAL_CANDIDATE_P0,
            BORROW_ROOT_P0,
            ROOT_BATCH_COMMIT,
            ROOT_BATCH_COMMIT_P0,
            DECL_FACT_COMMIT,
            DECL_FACT_COMMIT_P0,
            BORROW_ROOT_P0D,
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

    for fragment in (
        "ModuleLoweringInvocationCandidateV1",
        "InvocationCandidateFailureStageV1",
        "InvocationCandidateAbortProofV1",
        "with_active_lowering",
        "boundary_unchanged",
        "InvocationCandidatePublicationV1::Unchanged",
        "InvocationCandidateRetryV1::Forbidden",
        "pub(in crate::mir::builder) fn abort",
        "pub(in crate::mir::builder) fn discard",
        "candidate_owns_shell_and_collector_until_abort",
        "builder_borrow_is_scoped_to_active_lowering_only",
    ):
        require(candidate, fragment, "Candidate0-S0 vocabulary/fixture")
    for fragment in (
        "InvocationCandidateRouteProofBuilderV1",
        "InvocationCandidateRouteProofV1",
        "UnexpectedRoute",
        "candidate_abort_proof_co_seals_all_nine_route_rows",
        "duplicate_route_is_rejected_before_seal",
        "InvocationRouteMatrixV1::rows()",
        "InvocationCandidatePublicationV1::Unchanged",
        "InvocationCandidateRetryV1::Forbidden",
    ):
        require(candidate_p0, fragment, "Candidate0-P0 route co-seal/fixture")

    # The candidate may mention MirBuilder only as a short-lived method
    # parameter.  It must not store a Builder or expose a module map.
    struct = candidate.split("pub(in crate::mir::builder) struct ModuleLoweringInvocationCandidateV1", 1)[1]
    struct = struct.split("impl ModuleLoweringInvocationCandidateV1", 1)[0]
    forbid(struct, "MirBuilder", "candidate-stored Builder")
    forbid(candidate, "self.current_module", "candidate ambient module authority")
    forbid(candidate, "builder.current_module", "candidate ambient module authority")
    forbid(candidate, "ModuleLoweringPortV1", "candidate-owned collector port")
    forbid(candidate, "fn retry(", "candidate retry implementation")

    require(builder_mod, "mod module_lowering_invocation_candidate;", "Candidate0 module registration")
    require(builder_mod, "mod root_body_completion;", "BODYDRAIN0-S0 module registration")
    require(builder_mod, "mod root_body_completion_p0;", "BODYDRAIN0-P0 fixture registration")
    require(builder_mod, "mod main_pending_draft;", "MAINPENDING0-S0 module registration")
    require(builder_mod, "mod main_pending_draft_p0;", "MAINPENDING0-P0 fixture registration")
    require(builder_mod, "mod root_draft_batch;", "ROOTBATCH0-S0 module registration")
    require(builder_mod, "mod root_draft_batch_p0;", "ROOTBATCH0-P0 fixture registration")
    require(builder_mod, "mod main_root_wiring;", "WIRING-S0 Main/root order registration")
    require(builder_mod, "mod module_wiring_parity_p0;", "WIRING-P0 parity registration")
    require(
        builder_mod,
        "mod route_owned_invocation_inventory;",
        "WIRING-I0-ROUTEINV-S0 registration",
    )
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
    require(
        builder_mod,
        "mod module_lowering_invocation_access;",
        "WIRING-S0 live access registration",
    )
    require(builder_mod, "mod module_declaration_facts;", "SHELLFACT0-S0 module registration")
    require(builder_mod, "mod module_declaration_facts_p0;", "SHELLFACT0-P0 fixture registration")
    require(builder_mod, "mod drained_module_candidate;", "DRAIN0-S0 module registration")
    require(builder_mod, "mod drained_module_candidate_p0;", "DRAIN0-P0 fixture registration")
    require(builder_mod, "mod module_finalization_split;", "MODULEFINAL0-SPLIT0 module registration")
    require(builder_mod, "mod module_finalization_split_p0;", "MODULEFINAL0-SPLIT0 fixture registration")
    require(builder_mod, "mod module_finalization_candidate_p0;", "MODULEFINAL0-CANDIDATE0-P0 registration")
    other_builder_files = []
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (CANDIDATE, CANDIDATE_P0, BUILDER_MOD):
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
            if path in (CANDIDATE_P0, BUILDER_MOD) or path.name.endswith("_tests.rs"):
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
                ACCESS_VOCAB,
                ACCESS_IMPL,
                ACCESS_LIVE,
                MAIN_ROOT_WIRING,
                WIRING_P0,
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

    print(
        "[headerport-candidate0-guard] ok disconnected=1 "
        f"source_lines={len(candidate.splitlines())} "
        f"route_inventory_lines={len(route_inventory.splitlines())} "
        "production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
