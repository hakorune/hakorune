#!/usr/bin/env python3
"""COMMIT0-S0 guard for RawDirect external-commit preparation."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-commit0-s0-execution-task-2026-07-24.md"
)
SOURCES = (
    ROOT / "src/mir/compiler/raw_root_external_commit.rs",
    ROOT / "src/mir/compiler/raw_root_postprocess.rs",
    ROOT / "src/mir/builder/raw_root_physical/postprocess_terminal.rs",
    ROOT / "src/mir/builder/module_invocation_session.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = (ROOT / "docs/development/current/main/CURRENT_STATE.toml").read_text()
    task = TASK.read_text()
    if not any(
        row in state
        for row in (
            'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-COMMIT0-S0"',
            'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION-CONSULT0"',
            'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION0-S0"',
        )
    ):
        raise AssertionError(
            "missing COMMIT0, publication consultation, or PUBLICATION0 row"
        )
    for fragment in (
        "RAW-COMMIT-prime-r1",
        "RawPostprocessedInvocationV1::prepare_external_commit(self)",
        "PreparedRawExternalCommitV1",
        "PreparedModuleExternalCommitV1::commit new Raw caller        = 0",
    ):
        require(task, fragment, f"task contract {fragment}")

    texts = {path: path.read_text() for path in (TASK, *SOURCES)}
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"COMMIT0 file must remain below 800 lines: {path}")

    compiler = texts[SOURCES[0]]
    post = texts[SOURCES[1]]
    physical = texts[SOURCES[2]]
    session = texts[SOURCES[3]]

    for fragment in (
        "RawPostprocessedInvocationV1",
        "prepare_external_commit",
        "PreparedRawExternalCommitV1",
        "RejectedRawExternalCommitInvocationV1",
        "RawPostprocessEvidenceV1",
        "RawExternalCommitPhysicalHandoffV1",
        "into_external_commit_preflighted",
    ):
        require(compiler, fragment, f"compiler RawDirect owner {fragment}")

    for fragment in (
        "RawPostprocessStageEvidenceV1",
        "RawPostprocessEvidenceV1",
        "schedule",
        "verification",
        "progress",
    ):
        require(post, fragment, f"evidence staging {fragment}")

    for fragment in (
        "RawExternalCommitPhysicalErrorV1",
        "RawExternalCommitPhysicalHandoffV1",
        "validate_external_commit",
        "into_external_commit_preflighted",
        "PreparedBuilderExternalCommitV1",
    ):
        require(physical, fragment, f"opaque physical handoff {fragment}")

    require(session, "pub(in crate::mir) fn into_external_commit", "readiness transition")

    forbidden = (
        "PostprocessedModuleInvocationV1",
        "Raw { ledger, root }",
        "project_raw_drain_manifest",
        "MirCompileResult",
        "commit_prepared_module",
        "prepare_module_session()",
        "DerefMut",
        "AsMut<MirModule>",
        "into_module",
        "retry(",
        "fallback(",
        "catch_unwind",
    )
    for fragment in forbidden:
        if fragment in compiler or fragment in physical:
            raise AssertionError(f"COMMIT0 leaks forbidden authority: {fragment}")

    if "PreparedBuilderExternalCommitV1::commit" in compiler:
        raise AssertionError("COMMIT0 must not call live Builder commit")

    require(task, "PreparedModuleExternalCommitV1::commit        = 0", "publication stop")
    require(task, "MirCompileResult publication                 = 0", "result stop")
    require(task, "atomic CUT0 activation                       = 0", "CUT0 stop")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-commit0-guard] ok "
        "typed_entry=1 complete_evidence=1 opaque_handoff=1 publication=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
