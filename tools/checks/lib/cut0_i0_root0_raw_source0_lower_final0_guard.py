#!/usr/bin/env python3
"""FINAL0-S0 direct DRAIN handoff and production-scope guard."""

from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-final0-s0-execution-task-2026-07-24.md"
)
SCOPE = ROOT / "tools/checks/lib/cut0_i0_root0_raw_source0_lower_final0_guard_scope.py"
SOURCES = (
    ROOT / "src/mir/raw_finalization_contract.rs",
    ROOT / "src/mir/builder/raw_root_physical/finalization_terminal.rs",
    ROOT / "src/mir/compiler/raw_root_finalization.rs",
    ROOT / "src/mir/compiler/raw_root_finalization_p0.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    require(task, "FINAL-DRAIN-prime-r1", "decision lock")
    require(task, "RawDrainedInvocationV1::prepare_finalization(self)", "direct entry")

    for path in (TASK, *SOURCES):
        if not path.exists():
            raise AssertionError(f"missing FINAL0 source/task: {path}")
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"FINAL0 file must remain below 800 lines: {path}")

    contract = SOURCES[0].read_text()
    physical = SOURCES[1].read_text()
    finalizer = SOURCES[2].read_text()
    fixture = SOURCES[3].read_text()
    for fragment in (
        "RawFinalizationRouteEvidenceV1",
        "RawFinalizationRouteKindV1",
    ):
        require(contract, fragment, f"neutral contract {fragment}")
    for fragment in (
        "prepare_raw_finalization",
        "PreparedRawDrainedPhysicalFinalizationV1",
        "RawFinalizedPhysicalV1",
        "prepare_module_session()",
    ):
        require(physical, fragment, f"physical terminal {fragment}")
    for fragment in (
        "impl RawDrainedInvocationV1",
        "prepare_finalization(",
        "RawFinalizedInvocationV1",
        "RejectedRawDrainFinalizationInvocationV1",
        "RawFinalizationRouteEvidenceV1::Script",
        "RawFinalizationRouteEvidenceV1::App",
    ):
        require(finalizer, fragment, f"compiler finalizer {fragment}")
    for fixture_name in (
        "empty_script_finalizes_directly_from_drain",
        "app_not_selected_finalizes_without_callable_row",
        "app_selected_finalizes_with_callable_evidence",
        "builder_readiness_rejection_retains_the_new_final0_owner",
    ):
        require(fixture, fixture_name, f"fixture {fixture_name}")

    for forbidden in (
        "MirModule",
        "RawPhysicalCompleteInvocationV1",
        "RawModuleFinalizerV1",
        "into_draft_functions",
        "finalize_module",
        "current_module",
        "project_raw_drain_manifest",
        "retry(",
        "fallback",
    ):
        if forbidden in finalizer or forbidden in physical:
            raise AssertionError(f"new FINAL0 source leaks old authority: {forbidden}")

    result = subprocess.run([sys.executable, str(SCOPE)], cwd=ROOT, text=True)
    if result.returncode:
        raise SystemExit(result.returncode)
    print(
        "[cut0-i0-root0-raw-source0-lower-final0-guard] ok "
        "direct_owner=1 witness_manifest=1 opaque_terminal=1 old_callers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
