#!/usr/bin/env python3
"""HEADERPORT0 Candidate0-S0 disconnected ownership guard.

The candidate owns one shell/collector state, lends the Builder only to an
active lowering closure, and exposes only typed abort/discard outcomes.  This
guard prevents the vocabulary from becoming a production capture/commit path
before Candidate0-P0 is complete.
"""

from __future__ import annotations

import pathlib


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
SHELL_FACTS = ROOT / "src/mir/builder/module_declaration_facts.rs"
SHELL_FACTS_P0 = ROOT / "src/mir/builder/module_declaration_facts_p0.rs"
DRAINED_CANDIDATE = ROOT / "src/mir/builder/drained_module_candidate.rs"
DRAINED_CANDIDATE_P0 = ROOT / "src/mir/builder/drained_module_candidate_p0.rs"
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
    shell_facts = SHELL_FACTS.read_text()
    shell_facts_p0 = SHELL_FACTS_P0.read_text()
    drained_candidate = DRAINED_CANDIDATE.read_text()
    drained_candidate_p0 = DRAINED_CANDIDATE_P0.read_text()
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
    if len(shell_facts.splitlines()) >= 800:
        raise AssertionError("SHELLFACT0-S0 source must remain below 800 lines")
    if len(shell_facts_p0.splitlines()) >= 800:
        raise AssertionError("SHELLFACT0-P0 fixture source must remain below 800 lines")
    if len(drained_candidate.splitlines()) >= 800:
        raise AssertionError("DRAIN0-S0 source must remain below 800 lines")
    if len(drained_candidate_p0.splitlines()) >= 800:
        raise AssertionError("DRAIN0-P0 fixture source must remain below 800 lines")

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
            SHELL_FACTS,
            SHELL_FACTS_P0,
            DRAINED_CANDIDATE,
            DRAINED_CANDIDATE_P0,
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
    require(builder_mod, "mod module_declaration_facts;", "SHELLFACT0-S0 module registration")
    require(builder_mod, "mod module_declaration_facts_p0;", "SHELLFACT0-P0 fixture registration")
    require(builder_mod, "mod drained_module_candidate;", "DRAIN0-S0 module registration")
    require(builder_mod, "mod drained_module_candidate_p0;", "DRAIN0-P0 fixture registration")
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
        "HEADERPORT0-I0-MODULEFINAL0-SPLIT0\n  next code-facing row",
        "one disconnected invocation-owned shell/collector candidate",
        "typed abort/no-publication/no-retry proof",
        "production capture/commit remains forbidden",
        "`CUT0` remains forbidden",
    ):
        require(card, fragment, "Candidate0 task boundary")
    require(
        state,
        "HEADERPORT0-I0-MODULEFINAL0-SPLIT0 is next",
        "current Candidate0/MainROLE0 pointer",
    )

    print(
        "[headerport-candidate0-guard] ok disconnected=1 "
        f"source_lines={len(candidate.splitlines())} production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
