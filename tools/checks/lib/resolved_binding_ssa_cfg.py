#!/usr/bin/env python3
"""Validate the disconnected D-prime SSA-C1 canonical CFG substrate."""

from __future__ import annotations

from pathlib import Path
import sys


EXPECTED_FILES = {
    "README.md",
    "error.rs",
    "mod.rs",
    "predecessors.rs",
    "session.rs",
    "tests.rs",
}


def fail(message: str) -> None:
    raise SystemExit(f"SSA-C1 canonical CFG: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def production_rs(root: Path):
    for path in (root / "src").rglob("*.rs"):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs"):
            continue
        yield path


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_cfg.py ROOT")
    root = Path(sys.argv[1]).resolve()
    cfg_dir = root / "src/mir/builder/resolved_lowering/canonical_cfg"
    if not cfg_dir.is_dir():
        fail(f"missing directory {cfg_dir}")

    actual_files = {path.name for path in cfg_dir.iterdir() if path.is_file()}
    if actual_files != EXPECTED_FILES:
        fail(
            "module manifest drifted: "
            f"expected={sorted(EXPECTED_FILES)} actual={sorted(actual_files)}"
        )

    module = (cfg_dir / "mod.rs").read_text()
    errors = (cfg_dir / "error.rs").read_text()
    predecessors = (cfg_dir / "predecessors.rs").read_text()
    session = (cfg_dir / "session.rs").read_text()
    tests = (cfg_dir / "tests.rs").read_text()
    readme = (cfg_dir / "README.md").read_text()

    for anchor in (
        "mod error;",
        "mod predecessors;",
        "mod session;",
        "CanonicalCfgSessionV1",
        "VerifiedPredecessorsV1",
        "#![allow(dead_code)]",
    ):
        require(module, anchor, "facade")

    for anchor in (
        "SourceAlreadyTerminated",
        "DuplicateEdge",
        "EdgeAfterSeal",
        "SealTwice",
        "CachedSuccessorsMismatch",
        "CachedPredecessorsMismatch",
        "SealedPredecessorsChanged",
    ):
        require(errors, anchor, "typed errors")

    for anchor in (
        "successors_from_terminator()",
        "terminator_successors != block.successors",
        "expected != &block.predecessors",
        "DanglingTerminatorTarget",
    ):
        require(predecessors, anchor, "terminator truth")

    for anchor in (
        "fn emit_jump(",
        "fn emit_branch(",
        "fn seal_block(",
        "fn finish(",
        "derive_and_verify_predecessors(function)?",
        "target_block.is_sealed()",
    ):
        require(session, anchor, "session")

    combined = "\n".join((module, errors, predecessors, session))
    for forbidden in (
        "ASTNode",
        "SourceSite",
        "ScopeId",
        "RegionId",
        "BindingRefV1",
        "compute_predecessors(",
        ".update_cfg(",
        "cf_common::",
        "branch::emit_",
        "emit_instruction(",
        "materialize_all_phi_inputs",
    ):
        if forbidden in combined:
            fail(f"forbidden repair/legacy dependency found: {forbidden}")

    require(readme, "MIR terminators remain", "README")
    require(readme, "production If, Loop, or Binding SSA activation", "README")
    test_count = tests.count("#[test]")
    if test_count < 15:
        fail(f"focused fixture count dropped below 15: {test_count}")

    for path in cfg_dir.iterdir():
        if path.is_file():
            lines = len(path.read_text().splitlines())
            if lines >= 800:
                fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    production_callers = 0
    for path in production_rs(root):
        if cfg_dir in path.parents:
            continue
        production_callers += path.read_text(errors="ignore").count("CanonicalCfgSessionV1")
    if production_callers != 0:
        fail(f"production activation must remain zero, got {production_callers}")

    taskboard = (
        root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md"
    ).read_text()
    for anchor in (
        "### SSA-C1 — canonical CFG/seal prerequisite",
        "terminator-derived predecessor truth",
        "Production activation remains zero",
    ):
        require(taskboard, anchor, "taskboard")

    print("canonical_ssa_c1_facade=one-fallible-session")
    print("canonical_ssa_c1_predecessor_truth=mir-terminators")
    print("canonical_ssa_c1_cached_graph=checked-not-repaired")
    print("canonical_ssa_c1_late_edge=typed-error")
    print("canonical_ssa_c1_seal_witness=immutable")
    print(f"canonical_ssa_c1_focused_fixtures={test_count}")
    print("canonical_ssa_c1_production_callers=0")
    print("canonical_ssa_c1_accepted_grammar_delta=0")


if __name__ == "__main__":
    main()
