#!/usr/bin/env python3
"""CUT0-I0-ROOT0-CANON0 C-prime disconnected completion guard."""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
CANON = SRC / "canonical_root_completion.rs"
BATCH = SRC / "module_draft_collector" / "callable_batch.rs"
BUILDER = SRC.parent / "builder.rs"
TASK = ROOT / "docs/development/current/main/investigations/cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
DECISION = ROOT / "docs/development/current/main/investigations/cut0-i0-root0-canon0-design-question-2026-07-22.md"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    canon = CANON.read_text()
    batch = BATCH.read_text()
    task = TASK.read_text()
    decision = DECISION.read_text()
    builder = BUILDER.read_text()
    for path in (CANON, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"CANON0 file must remain below 800 lines: {path}")
    require(task, "ROOT0-CANON0", "active task")
    require(decision, "C-prime", "C-prime decision")
    require(builder, "mod canonical_root_completion", "module registration")
    for fragment, label in (
        ("PreparedCanonicalSingleSourceV1", "single source package"),
        ("CanonicalSingleSourceContinuationV1", "single continuation"),
        ("PreparedCallableBatchSourceV1", "callable source package"),
        ("CallableBatchSourceContinuationV1", "callable continuation"),
        ("CanonicalSingleCompleteInvocationV1", "single completion"),
        ("CallableBatchCompleteInvocationV1", "batch completion"),
        ("CanonicalSingleDrainPlanV1", "single drain plan"),
        ("CallableBatchDrainPlanV1", "batch drain plan"),
        ("RecursiveCapabilityInstallReceiptV1", "recursive install receipt"),
    ):
        require(canon, fragment, label)
    for forbidden, label in (
        ("ConditionFnPolicyV1::Optional", "optional condition policy"),
        ("from_test_identity_unavailable", "test identity in production"),
        ("InvocationBranded::from_test", "post-hoc test branding"),
    ):
        if forbidden in canon:
            raise AssertionError(f"forbidden {label}: {forbidden}")
    require(batch, "collect_all_branded", "collector-issued batch receipt")
    print("[cut0-i0-root0-canon0-guard] ok source_binding=1 route_completion=1 drain_plan=1 production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
