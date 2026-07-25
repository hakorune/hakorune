#!/usr/bin/env python3
"""POST0-S0 guard for the direct Raw finalized-carrier handoff."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-s0-execution-task-2026-07-24.md"
)
SOURCES = (
    ROOT / "src/mir/compiler/raw_root_postprocess.rs",
    ROOT / "src/mir/compiler/module_postprocess_stages.rs",
    ROOT / "src/mir/builder/raw_root_physical/postprocess_terminal.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    require(task, "POST-CARRIER-prime-r1", "decision lock")
    require(task, "RawFinalizedInvocationV1::prepare_postprocess(self)", "sole entry")

    contents = {path: path.read_text() for path in (TASK, *SOURCES)}
    for path, text in contents.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"POST0 file must remain below 800 lines: {path}")

    raw = contents[SOURCES[0]]
    stages = contents[SOURCES[1]]
    physical = contents[SOURCES[2]]
    for fragment in (
        "RawPostprocessReadyInvocationV1",
        "run_raw_ready",
        "RawPostprocessedInvocationV1",
        "RejectedRawPostprocessInvocationV1",
        "RawPostprocessProgressV1",
        "RawPostprocessEvidenceV1",
    ):
        require(raw, fragment, f"Raw POST0 owner {fragment}")
    for fragment in (
        "run_postprocess_stages",
        "refresh_rune_plans",
        "canonicalize_callsites",
    ):
        require(stages, fragment, f"shared stage kernel {fragment}")
    for fragment in (
        "RawPostprocessModuleLoanV1",
        "begin_postprocess",
        "prepare_parity",
        "ModuleNameMismatch",
        "SignatureNameMismatch",
        "RawPostprocessedPhysicalV1",
    ):
        require(physical, fragment, f"opaque physical carrier {fragment}")
    require(raw, "verification: Option<ModuleVerificationEvidenceV1>", "failure verification evidence")

    forbidden = (
        "DerefMut",
        "AsMut<MirModule>",
        "into_module",
        "module_mut",
        "current_module",
        "OwnedRawSourceV1",
        "project_raw_drain_manifest",
        "catch_unwind",
        "rollback",
        "retry(",
        "fallback",
    )
    for fragment in forbidden:
        if fragment in raw or fragment in physical:
            raise AssertionError(f"POST0 source leaks forbidden authority: {fragment}")

    require(task, "Raw final-verifier\nrejection remains zero", "Raw verification policy")
    require(task, "external commit/public ingress/executor/CUT0 = 0", "future stop line")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-guard] ok "
        "sole_entry=1 shared_kernel=1 opaque_loan=1 route_evidence=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
