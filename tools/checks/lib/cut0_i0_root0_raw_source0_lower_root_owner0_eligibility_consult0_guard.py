#!/usr/bin/env python3
"""OWNER0-ELIGIBILITY0 design-stop guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-owner0-eligibility-consultation-2026-07-23.md"
)
PROD = (
    ROOT / "src/mir/compiler/mod.rs",
    ROOT / "src/mir/compiler/raw_root_package.rs",
    ROOT / "src/mir/compiler/raw_root_plan0.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    card = CARD.read_text()
    require(
        state,
        'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-CONSULT0"',
        "active design stop",
    )
    require(
        state,
        'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-CONSULT0"',
        "active execution row",
    )
    require(
        state,
        'latest_card = "cut0-i0-raw-source0-lower-root-owner0-eligibility-consultation-2026-07-23"',
        "latest consultation card",
    )
    for fragment in (
        "Q1 — runtime inputs",
        "Q2 — source work schedule",
        "Q3 — declaration/callable coverage",
        "Q4 — closure/static-data access",
        "Q5 — process-global slots",
        "Candidate ELIGIBILITY-prime",
        "physical effects = 0",
        "No child traversal",
        "CURRENT_STATE.toml",
    ):
        require(card, fragment, f"eligibility consultation {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    for forbidden in (
        "begin_raw_root(",
        "execute_preflighted_module_invocation",
        "ModuleLoweringInvocationStateV1::capture_main",
        "reserve_method_slot(",
        "get_or_assign_type_id(",
    ):
        if forbidden in joined:
            raise AssertionError(f"physical effect wired during eligibility stop: {forbidden}")
    for path in (CARD, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-owner0-eligibility-consult0-guard] ok "
        "design_stop=1 physical_consumer=0 global_slot_mutation=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
