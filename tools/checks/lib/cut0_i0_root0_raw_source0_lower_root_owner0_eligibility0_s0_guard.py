#!/usr/bin/env python3
"""Task guard for RAW OWNER0 ELIGIBILITY0 S0.

This is intentionally a design/task guard until the implementation product is
landed. It prevents the task card from silently widening into a physical or
production owner.
"""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-eligibility0-s0-execution-task-2026-07-24.md"
)
CONSULT = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-eligibility-consultation-2026-07-23.md"
)
QUESTION = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-eligibility-question-2026-07-23.md"
)
SOURCE = (
    ROOT / "src/mir/compiler/raw_root_package.rs",
    ROOT / "src/mir/compiler/raw_root_plan0.rs",
    ROOT / "src/mir/compiler/raw_source_binding.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    consult = CONSULT.read_text()
    question = QUESTION.read_text()

    require(
        state,
        'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0"',
        "active S0 design stop",
    )
    require(
        state,
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-S0"',
        "active S0 execution row",
    )
    require(
        state,
        'latest_card = "cut0-i0-raw-source0-lower-root-owner0-eligibility0-s0-execution-task-2026-07-24"',
        "latest S0 task",
    )
    for fragment in (
        "Decision: ELIGIBILITY-prime-r1",
        "Q1 CaptureOnce",
        "Q2 NarrowExhaustive / ScalarControl0",
        "Q3 NarrowReject",
        "Q4 closure and static data typed-reject",
        "Q5 process-global slot typed-reject",
        "prepare_eligibility(self)",
        "RejectedRawRootEligibilityV1",
        "ScalarControl0",
        "UnsupportedClosureAccess",
        "UnsupportedStaticDataAuthority",
        "UnsupportedProcessGlobalSlot",
        "physical effects",
        "production consumers",
        "all touched source/check files < 800 lines",
    ):
        require(task, fragment, f"S0 task {fragment}")
    require(consult, "Status: **Closed", "closed consultation")
    require(consult, "ELIGIBILITY-prime-r1", "consultation decision")
    require(question, "Required answer format", "historical question")

    joined = "\n".join(path.read_text() for path in SOURCE)
    forbidden = (
        "begin_raw_root(",
        "execute_preflighted_module_invocation",
        "ModuleLoweringInvocationStateV1::capture_main",
        "ModuleLoweringInvocationStateV1::complete_root",
        "get_or_assign_type_id(",
        "reserve_method_slot(",
        "resolve_slot_by_type_name(",
        "catch_unwind",
    )
    for fragment in forbidden:
        if fragment in joined:
            raise AssertionError(f"physical/production wiring present during S0: {fragment}")

    for path in (STATE, TASK, CONSULT, QUESTION, *SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-owner0-eligibility0-s0-guard] ok "
        "task=1 physical_consumer=0 production_consumer=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
