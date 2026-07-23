#!/usr/bin/env python3
"""RAW-SOURCE0-LOWER0 ROOT design-stop guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-consultation-2026-07-23.md"
)
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23.md"
)
PROD = (
    ROOT / "src/mir/compiler/mod.rs",
    ROOT / "src/runtime/mirbuilder_emit.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    card = CARD.read_text()
    if 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-PLAN0"' in state:
        require(card, "ROOT0-D0 closeout", "decision closeout")
        require(TASK.read_text(), "ROOT0-PLAN0", "next plan row")
        print(
            "[cut0-i0-root0-raw-source0-lower-root-consult0-guard] closed "
            "next=RAW-SOURCE0-LOWER0-ROOT0-PLAN0"
        )
        return 0
    require(state, 'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT-CONSULT0"', "design stop")
    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT-CONSULT0"', "execution row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-consultation-2026-07-23"', "latest card")
    for fragment in (
        "Q1 — root owner/state",
        "Q2 — Script root and atomic batch",
        "Q3 — App source inventory",
        "Q4 — callable Main disposition",
        "Q5 — root failure and handoff",
        "Recommendation: **1**",
        "Root0 is the\nnext design row",
        "public executor / public wrapper wiring = 0",
        "retry/fallback/catch_unwind = 0",
    ):
        require(card, fragment, f"root consultation {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during ROOT consultation")
    if "bind_raw_source(" in joined:
        callers = [
            path.relative_to(ROOT)
            for path in PROD
            if "bind_raw_source(" in path.read_text() and path.name != "mod.rs"
        ]
        if callers:
            raise AssertionError(f"unexpected Raw binding caller: {callers}")
    for path in (CARD, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-consult0-guard] ok "
        "design_stop=1 raw_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
