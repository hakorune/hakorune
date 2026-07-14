#!/usr/bin/env python3
"""Validate the disconnected D-prime SSA-S1 Binding SSA box."""

from __future__ import annotations

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"SSA-S1 Binding SSA: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_builder.py ROOT")
    root = Path(sys.argv[1]).resolve()
    box = root / "src/mir/builder/ssa/binding"
    paths = {
        "readme": box / "README.md",
        "adapter": box / "adapter.rs",
        "error": box / "error.rs",
        "builder": box / "mod.rs",
        "tests": box / "tests.rs",
        "cfg_session": root
        / "src/mir/builder/resolved_lowering/canonical_cfg/session.rs",
        "cfg_tests": root
        / "src/mir/builder/resolved_lowering/canonical_cfg/tests.rs",
    }
    for path in paths.values():
        if not path.is_file():
            fail(f"missing required file {path}")
    text = {name: path.read_text() for name, path in paths.items()}

    for anchor in (
        "struct BindingSsaBuilderV1",
        "fn define(",
        "fn read<",
        "fn seal<",
        "fn finish(",
        "VerifiedPredecessorsV1",
        "define_provisional_phi(block)",
        "verify_phi_input(predecessor, value)",
        "patch_phi_inputs(token, &inputs)",
        "rollback_phi(phi.token)",
    ):
        require(text["builder"], anchor, "sealed-block builder")

    for anchor in (
        "ForeignBinding",
        "MissingDefinition",
        "BlockSealedTwice",
        "WitnessBlockMismatch",
        "DuringPhiCleanup",
        "Poisoned",
        "UnsealedAtFinish",
        "IncompleteAtFinish",
    ):
        require(text["error"], anchor, "typed failures")

    forbidden = (
        "crate::ast",
        "ASTNode",
        "Span",
        "SourceStmtSite",
        "SourceExprSite",
        "ScopeId",
        "RegionId",
        "resolved_region_flow",
        "variable_map",
        "may_rebind",
        "carrier",
    )
    production_text = text["adapter"] + text["error"] + text["builder"]
    for token in forbidden:
        if token in production_text:
            fail(f"forbidden dependency/policy token reached the box: {token}")

    test_count = text["tests"].count("#[test]")
    if test_count != 12:
        fail(f"focused fixture count must remain 12, got {test_count}")
    for anchor in (
        "entry_definition_and_same_block_overwrite",
        "single_predecessor_forwards_without_phi",
        "diamond_keeps_same_input_and_one_sided_phis",
        "two_sided_and_nested_diamonds_use_exact_predecessors",
        "open_loop_header_completes_zero_and_backedge_inputs",
        "open_loop_retains_self_phi_when_backedge_has_no_redefinition",
        "multiple_backedges_are_ordered_and_exact",
        "missing_definition_and_foreign_owner_are_typed",
        "finish_rejects_open_and_incomplete_blocks",
        "patch_failure_rolls_back_phi_and_poisons_builder",
        "rollback_failures_are_retained_after_input_verification_failure",
    ):
        require(text["tests"], anchor, "focused fixtures")

    for anchor in (
        "duplicate_branch_edge_is_rejected_before_mutation",
        "edge_after_seal_is_rejected_before_source_mutation",
    ):
        require(text["cfg_tests"], anchor, "inherited C1 edge fixtures")

    code_root = root / "src"
    callers = []
    for path in code_root.rglob("*.rs"):
        if path == paths["builder"] or path == paths["tests"]:
            continue
        if "BindingSsaBuilderV1" in path.read_text():
            callers.append(str(path.relative_to(root)))
    if callers:
        fail(f"production/disconnected external callers must remain zero: {callers}")

    taskboard = (
        root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md"
    ).read_text()
    for anchor in (
        "### SSA-S1 — disconnected Binding SSA — closed",
        "entry/single/diamond/nested/Loop/multi-backedge/error fixtures = 12/12 green",
        "production Binding SSA callers = 0",
        "### SSA-S2 — identity/value separation — closed",
    ):
        require(taskboard, anchor, "taskboard")

    for path in (*paths.values(), Path(__file__)):
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    print("canonical_ssa_s1_owner=one-function")
    print("canonical_ssa_s1_predecessor_authority=verified-cfg-witness")
    print("canonical_ssa_s1_phi_policy=sealed-block-on-demand")
    print("canonical_ssa_s1_same-input-phi=retained")
    print("canonical_ssa_s1_self-phi=retained")
    print("canonical_ssa_s1_phi-failure=rollback-all-and-poison")
    print(f"canonical_ssa_s1_focused_fixtures={test_count}")
    print("canonical_ssa_s1_production_callers=0")
    print("canonical_ssa_s1_accepted_grammar_delta=0")


if __name__ == "__main__":
    main()
