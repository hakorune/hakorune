#!/usr/bin/env python3
"""Reusable HEADERPORT0 whole-invocation borrow proof."""

from __future__ import annotations

import pathlib


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_exact_count(text: str, fragment: str, count: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != count:
        raise AssertionError(
            f"{label} count mismatch: expected={count} actual={actual} fragment={fragment!r}"
        )


def verify_borrow_root_p0(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    proof_path = root / "src/mir/builder/module_lowering_borrow_root_p0.rs"
    matrix_path = root / "src/mir/builder/module_finalization_candidate_p0.rs"
    batch_path = root / "src/mir/builder/root_draft_batch.rs"
    facts_path = root / "src/mir/builder/module_declaration_facts.rs"
    drain_path = root / "src/mir/builder/module_invocation_drain.rs"
    final_path = root / "src/mir/builder/module_finalization_split.rs"
    batch_commit_path = root / "src/mir/builder/module_draft_collector/root_batch.rs"
    batch_commit_proof_path = root / "src/mir/builder/root_draft_batch_commit_p0.rs"
    fact_commit_path = root / "src/mir/builder/module_lowering_shell/declaration_fact_commit.rs"
    fact_commit_proof_path = (
        root / "src/mir/builder/module_declaration_fact_shell_commit_p0.rs"
    )
    route_commit_proof_path = root / "src/mir/builder/module_lowering_borrow_root_p0d.rs"
    proof = proof_path.read_text()
    matrix = matrix_path.read_text()
    owners = "\n".join(
        path.read_text() for path in (batch_path, facts_path, drain_path, final_path)
    )

    for path in (
        proof_path,
        matrix_path,
        batch_path,
        facts_path,
        drain_path,
        final_path,
        batch_commit_path,
        batch_commit_proof_path,
        fact_commit_path,
        fact_commit_proof_path,
        route_commit_proof_path,
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"BORROW-P0-ROOT source/proof reached 800 lines: {path}")

    for fragment in (
        "exact_eleven_root_phases_split_raw_prefix_from_common_tail",
        "steps[..5]",
        "InvocationBorrowRouteScopeV1::RawOnly",
        "steps[5..]",
        "InvocationBorrowRouteScopeV1::AllRoutes",
        "root_batch_shell_drain_and_external_commits_are_infallible_after_preflight",
        "every_root_failure_owner_discards_candidate_without_retry_or_publication",
    ):
        require(proof, fragment, "BORROW-P0-ROOT schedule proof")
    for fragment in (
        "RootBatchPreflight",
        "DeclarationFactsSeal",
        "DrainPreflight",
        "PostDrainFinalize",
        "const FAILURE_ROWS: [ModuleFinalizationFailureRowV1; 8]",
        "root_batch_shell_drain_and_finalizer_failures_have_no_fallback_route",
    ):
        require(matrix, fragment, "BORROW-P0-ROOT failure matrix")
    for fragment in (
        "PreparedRootDraftBatchV1",
        "SealedModuleDeclarationFactsV1",
        "PreparedInvocationDrainV1",
        "DrainedModuleFinalizationInputV1",
    ):
        require(owners, fragment, "BORROW-P0-ROOT existing owner")
    require(
        builder_mod,
        "#[cfg(test)]\nmod module_lowering_borrow_root_p0;",
        "BORROW-P0-ROOT proof registration",
    )
    batch_commit = batch_commit_path.read_text()
    batch_commit_proof = batch_commit_proof_path.read_text()
    for fragment in (
        "PreparedRootCollectorBatchV1",
        "RejectedRootCollectorBatchV1",
        "RootCollectorBatchPrepareErrorV1",
        "plan_admission_v1",
        "pub(in crate::mir::builder) fn prepare_root_batch",
        "pub(in crate::mir::builder) fn commit",
    ):
        require(batch_commit, fragment, "BORROW-P0-ROOT-P0b transaction owner")
    for fragment in (
        "second_root_admission_failure_preserves_exact_collector_prefix",
        "prepared_root_batch_commits_main_and_condition_once_after_full_preflight",
        "legacy_main_replacement_is_prepared_with_the_whole_root_batch",
    ):
        require(batch_commit_proof, fragment, "BORROW-P0-ROOT-P0b proof")
    require(
        builder_mod,
        "#[cfg(test)]\nmod root_draft_batch_commit_p0;",
        "BORROW-P0-ROOT-P0b proof registration",
    )
    fact_commit = fact_commit_path.read_text()
    fact_commit_proof = fact_commit_proof_path.read_text()
    for fragment in (
        "PreparedModuleDeclarationFactShellCommitV1",
        "RejectedModuleDeclarationFactShellCommitV1",
        "ModuleDeclarationFactShellPrepareErrorV1",
        "prepare_declaration_fact_commit",
        "pub(in crate::mir::builder) fn commit",
    ):
        require(fact_commit, fragment, "BORROW-P0-ROOT-P0c transaction owner")
    for fragment in (
        "prepared_shell_commit_moves_all_four_declaration_lanes_once",
        "failed_preparation_returns_the_exact_unmodified_shell_and_sealed_facts",
    ):
        require(fact_commit_proof, fragment, "BORROW-P0-ROOT-P0c proof")
    require(
        builder_mod,
        "#[cfg(test)]\nmod module_declaration_fact_shell_commit_p0;",
        "BORROW-P0-ROOT-P0c proof registration",
    )
    route_commit_proof = route_commit_proof_path.read_text()
    for fragment in (
        "all_nine_routes_co_seal_with_the_exact_raw_prefix_or_common_tail",
        "every_route_observes_external_commit_zero_on_failure_and_one_on_success",
        "assert_eq!(projected_route_step_count, 4 * 11 + 5 * 6)",
        "assert_eq!(observations.len(), 9 * 3)",
    ):
        require(route_commit_proof, fragment, "BORROW-P0-ROOT-P0d proof")
    require(
        builder_mod,
        "#[cfg(test)]\nmod module_lowering_borrow_root_p0d;",
        "BORROW-P0-ROOT-P0d proof registration",
    )

    production_calls = []
    excluded = {
        proof_path,
        root / "src/mir/builder.rs",
        root / "src/mir/builder/root_draft_batch_p0.rs",
        root / "src/mir/builder/module_declaration_facts_p0.rs",
        root / "src/mir/builder/drained_module_candidate_p0.rs",
        root / "src/mir/builder/module_finalization_split_p0.rs",
        batch_commit_path,
        batch_commit_proof_path,
        fact_commit_path,
        fact_commit_proof_path,
        route_commit_proof_path,
    }
    watched = (
        "PreparedRootDraftBatchV1::prepare(",
        "SealedModuleDeclarationFactsV1::new(",
        "ModuleLoweringInvocationDrainOwnerV1::new(",
        "InvocationDrainExpectationV1::new(",
        "CompletedInvocationInventoryV1::new(",
        "DrainedModuleCandidateV1::from_drained_module(",
        "DrainedModuleFinalizationInputV1::new(",
        "prepare_root_batch(",
        "prepare_declaration_fact_commit(",
    )
    for path in (root / "src/mir").rglob("*.rs"):
        if path in excluded:
            continue
        source = path.read_text().split("#[cfg(test)]", 1)[0]
        if any(fragment in source for fragment in watched):
            production_calls.append(str(path.relative_to(root)))
    if production_calls:
        raise AssertionError(
            "BORROW-P0-ROOT production orchestration consumers: "
            + ", ".join(production_calls)
        )

    for source, fragment, label in (
        (
            batch_commit,
            "pub(in crate::mir::builder) fn prepare_root_batch",
            "root batch preparation owner",
        ),
        (
            fact_commit,
            "pub(in crate::mir::builder) fn prepare_declaration_fact_commit",
            "shell declaration-fact preparation owner",
        ),
        (
            drain_path.read_text(),
            "pub(in crate::mir::builder) struct PreparedInvocationDrainV1",
            "invocation drain owner",
        ),
        (
            final_path.read_text(),
            "pub(in crate::mir::builder) struct DrainedModuleFinalizationInputV1",
            "post-drain finalization owner",
        ),
    ):
        require_exact_count(source, fragment, 1, label)

    require(card, "WIRING-I0-BORROW-P0-ROOT-P0a closeout", "root P0a closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0b closeout", "root P0b closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0c closeout", "root P0c closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0d closeout", "root P0d closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-G0 closeout", "root G0 closeout")
    require(card, "WIRING-I0-BORROW-G0 closeout", "whole BORROW G0 closeout")
    if not any(
        pointer in state
        for pointer in (
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed and WIRING-I0-HDR0-P0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted and HDR0-P0-AUTHORITY-ERASURE0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0 is closed, and HDR0-P0-METHODTAIL-COMPAT0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0 and HDR0-P0-METHODTAIL-COMPAT0 are closed, and HDR0-P0-CALLER-CENSUS0 is next",
        )
    ):
        raise AssertionError("missing BORROW-G0 pointer or its documented HDR0 follow-on")
