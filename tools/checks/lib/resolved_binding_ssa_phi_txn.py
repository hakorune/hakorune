#!/usr/bin/env python3
"""Validate the disconnected D-prime SSA-P1 PHI transaction cleanup."""

from __future__ import annotations

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"SSA-P1 PHI transaction: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_phi_txn.py ROOT")
    root = Path(sys.argv[1]).resolve()
    lifecycle_path = root / "src/mir/builder/emission/phi_lifecycle.rs"
    tests_path = root / "src/mir/builder/emission/phi_lifecycle_tests.rs"
    caller_path = (
        root
        / "src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs"
    )
    for path in (lifecycle_path, tests_path, caller_path):
        if not path.is_file():
            fail(f"missing required file {path}")

    lifecycle = lifecycle_path.read_text()
    tests = tests_path.read_text()
    caller = caller_path.read_text()

    for anchor in (
        "struct PhiRollbackFailureV1",
        "struct PhiTxnAbortErrorV1",
        "primary: String",
        "cleanup_failures: Box<[PhiRollbackFailureV1]>",
        "fn abort_on_err(",
        ") -> PhiTxnAbortErrorV1",
        "let mut cleanup_failures = Vec::new();",
        "match rollback",
        "Ok(false) => cleanup_failures.push",
        "Err(error) => cleanup_failures.push",
        "cleanup_failures: cleanup_failures.into_boxed_slice()",
    ):
        require(lifecycle, anchor, "all-attempt abort")

    for anchor in (
        "fn commit(",
        "builder: &mut MirBuilder",
        "Result<(), PhiTxnAbortErrorV1>",
        "Err(self.abort_on_err(builder, err))",
        "[freeze:contract][phi_lifecycle/provisional_left_unpatched]",
    ):
        require(lifecycle, anchor, "commit cleanup")

    abort_body = lifecycle.split("fn abort_on_err(", 1)[1]
    abort_body = abort_body.split("/// Define a provisional PHI", 1)[0]
    if "rollback_provisional_phi(" not in abort_body:
        fail("abort no longer attempts rollback")
    if "rollback_provisional_phi(" in abort_body and ")?;" in abort_body:
        fail("abort regained first-error early return")

    for anchor in (
        "txn.commit(builder).map_err(|error| error.to_string())?",
        "Err(txn.abort_on_err(builder, err).to_string())",
    ):
        require(caller, anchor, "legacy caller adapter")

    test_count = tests.count("#[test]")
    if test_count != 6:
        fail(f"focused fixture count must remain 6, got {test_count}")
    for anchor in (
        "rollback_continues_after_one_pending_block_was_removed",
        "abort_retains_every_cleanup_failure_with_the_primary_error",
        "commit_with_pending_phis_rolls_them_all_back",
        "missing_provisional_phi_is_retained_as_a_cleanup_failure",
    ):
        require(tests, anchor, "focused fixtures")

    for path in (lifecycle_path, tests_path, caller_path, Path(__file__)):
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    taskboard = (
        root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md"
    ).read_text()
    for anchor in (
        "### SSA-P1 — PHI transaction cleanup prerequisite",
        "every pending rollback is attempted",
        "primary and cleanup errors are both retained",
        "No accepted syntax or production Binding SSA call is added",
    ):
        require(taskboard, anchor, "taskboard")

    print("canonical_ssa_p1_rollback_policy=all-pending-attempted")
    print("canonical_ssa_p1_error_shape=primary-plus-all-cleanup")
    print("canonical_ssa_p1_missing_provisional=cleanup-failure")
    print("canonical_ssa_p1_failed_commit=rollback-all")
    print(f"canonical_ssa_p1_focused_fixtures={test_count}")
    print("canonical_ssa_p1_binding_ssa_production_callers=0")
    print("canonical_ssa_p1_accepted_grammar_delta=0")


if __name__ == "__main__":
    main()
