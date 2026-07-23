#!/usr/bin/env python3
"""OWNER0-PACKAGE0 task/authority guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CONSULT = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner-consultation-2026-07-23.md"
)
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-package0-execution-task-2026-07-23.md"
)
SOURCE_MANIFEST = (
    ROOT / "src/mir/compiler/raw_source_binding.rs",
    ROOT / "src/mir/compiler/raw_root_plan0.rs",
    ROOT / "src/mir/compiler/mod.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    consult = CONSULT.read_text()
    task = TASK.read_text()

    require(
        state,
        'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PACKAGE0"',
        "active design stop",
    )
    require(
        state,
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PACKAGE0"',
        "active execution row",
    )
    require(
        state,
        'latest_card = "cut0-i0-raw-source0-lower-root-owner0-package0-execution-task-2026-07-23"',
        "latest task card",
    )
    require(consult, "Candidate RAW-OWNER-prime-r1", "decision lock")

    for fragment in (
        "SourceBoundRawRootPackageV1",
        "RejectedRawRootPlanningV1",
        "borrow the complete bound package",
        "planning failure retains exact original bound package",
        "RawRootPlan token/origin authority = 0",
        "callable selection bool fields = 0",
        "session/shell/collector/ledger/tracker construction = 0",
        "production package/owner consumer = 0",
        "all modified/new source/check files < 800 lines",
        "docs-only closeout is",
    ):
        require(task, fragment, f"PACKAGE0 contract {fragment}")

    joined = "\n".join(path.read_text() for path in SOURCE_MANIFEST)
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during OWNER0-PACKAGE0")
    if "begin_raw_root(" in joined:
        raise AssertionError("physical Raw owner consumer exists during PACKAGE0")

    implementation_count = joined.count("SourceBoundRawRootPackageV1")
    if implementation_count > 1:
        raise AssertionError(
            "root package vocabulary must have at most one source definition "
            f"during taskization: {implementation_count}"
        )

    for path in (CONSULT, TASK, *SOURCE_MANIFEST):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-owner0-package0-guard] ok "
        f"task=1 implementation_refs={implementation_count} "
        "physical_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
