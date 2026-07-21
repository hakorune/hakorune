#!/usr/bin/env python3
"""Reusable HEADERPORT0 whole-invocation borrow proof."""

from __future__ import annotations

import pathlib


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


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
    proof = proof_path.read_text()
    matrix = matrix_path.read_text()
    owners = "\n".join(
        path.read_text() for path in (batch_path, facts_path, drain_path, final_path)
    )

    for path in (proof_path, matrix_path, batch_path, facts_path, drain_path, final_path):
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

    production_calls = []
    excluded = {
        proof_path,
        root / "src/mir/builder.rs",
        root / "src/mir/builder/root_draft_batch_p0.rs",
        root / "src/mir/builder/module_declaration_facts_p0.rs",
        root / "src/mir/builder/drained_module_candidate_p0.rs",
        root / "src/mir/builder/module_finalization_split_p0.rs",
    }
    watched = (
        "PreparedRootDraftBatchV1::prepare(",
        "SealedModuleDeclarationFactsV1::new(",
        "ModuleLoweringInvocationDrainOwnerV1::new(",
        "InvocationDrainExpectationV1::new(",
        "CompletedInvocationInventoryV1::new(",
        "DrainedModuleCandidateV1::from_drained_module(",
        "DrainedModuleFinalizationInputV1::new(",
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

    require(card, "WIRING-I0-BORROW-P0-ROOT-P0a closeout", "root P0a closeout")
    require(
        state,
        "BORROW-P0-ROOT-P0a is closed; BORROW-P0-ROOT-P0b is next",
        "root P0a pointer",
    )
