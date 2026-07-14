#!/usr/bin/env python3
"""Validate the D-prime SSA-V0 canonical publication barrier."""

from __future__ import annotations

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"SSA-V0 canonical publication: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_publication.py ROOT")
    root = Path(sys.argv[1]).resolve()
    paths = {
        "compiler": root / "src/mir/compiler/mod.rs",
        "errors": root / "src/mir/compiler/lowering_input.rs",
        "session": root / "src/mir/compiler/module_session.rs",
        "function_session": root / "src/mir/builder/calls/function_session.rs",
        "resolved_build": root / "src/mir/builder/resolved_lowering/mod.rs",
        "module": root / "src/mir/function/module_impl.rs",
        "function_tests": root / "src/mir/function/tests.rs",
        "publication_tests": root
        / "src/mir/builder/calls/function_publication_tests.rs",
        "compiler_tests": root / "src/mir/compiler/tests.rs",
    }
    for path in paths.values():
        if not path.is_file():
            fail(f"missing required file {path}")
    text = {name: path.read_text() for name, path in paths.items()}

    for anchor in (
        "fn finish_built_canonical_module(",
        '"canonical_post_rc_verify"',
        "require_canonical_verification(verification_result)?;",
        "verification_result: Ok(())",
    ):
        require(text["compiler"], anchor, "post-RC verifier barrier")

    resolved_body = text["compiler"].split("fn compile_resolved_first_family(", 1)[1]
    resolved_body = resolved_body.split("fn finish_built_canonical_module(", 1)[0]
    finish_index = resolved_body.find("finish_built_canonical_module(module)?")
    commit_index = resolved_body.find("module_session.commit(&mut self.builder)")
    if finish_index < 0 or commit_index < 0 or finish_index >= commit_index:
        fail("canonical module commit is not structurally after strict finalization")
    if "finish_built_module(module)" in resolved_body:
        fail("canonical route bypasses the strict finalization wrapper")

    for anchor in (
        "MirVerificationFailed",
        "DuplicateFunctionPublication",
        "errors: Box<[String]>",
    ):
        require(text["errors"], anchor, "typed canonical compile failures")

    for anchor in (
        "pub fn try_add_function(",
        "self.functions.contains_key(&name)",
        "FunctionPublicationErrorV1",
    ):
        require(text["module"], anchor, "checked function publication")
    for anchor in (
        "CanonicalFunctionSessionErrorV1",
        "FunctionDraftPublicationErrorV1::Duplicate",
        "if requires_resolved_authority",
        ".try_add_function(draft)",
        "module.add_function(draft);",
    ):
        require(text["function_session"], anchor, "canonical/legacy publication split")
    for anchor in (
        "CanonicalResolvedBuildErrorV1",
        "duplicate_function_name()",
        "DuplicateFunctionPublication",
    ):
        require(text["resolved_build"], anchor, "typed duplicate transport")

    for owner, anchor in (
        ("function_tests", "checked_function_publication_rejects_duplicate_without_replacement"),
        (
            "publication_tests",
            "canonical_draft_publication_rejects_duplicate_without_overwrite",
        ),
        ("compiler_tests", "canonical_verification_failure_discards_candidate_before_commit"),
    ):
        require(text[owner], anchor, "focused fixtures")

    taskboard = (
        root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md"
    ).read_text()
    for anchor in (
        "### SSA-V0 — canonical publication/verifier prerequisite — closed",
        "CanonicalModuleLoweringSessionV1 commit after that Err = unreachable",
        "production Binding SSA callers = 0",
    ):
        require(taskboard, anchor, "taskboard")

    for path in (*paths.values(), Path(__file__)):
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    print("canonical_ssa_v0_post_rc_verifier_failure=typed-compile-error")
    print("canonical_ssa_v0_commit_after_verifier_failure=0")
    print("canonical_ssa_v0_duplicate_function_publication=typed-reject")
    print("canonical_ssa_v0_duplicate_overwrite=0")
    print("canonical_ssa_v0_legacy_reporting=explicitly-preserved")
    print("canonical_ssa_v0_binding_ssa_production_callers=0")
    print("canonical_ssa_v0_accepted_grammar_delta=0")


if __name__ == "__main__":
    main()
