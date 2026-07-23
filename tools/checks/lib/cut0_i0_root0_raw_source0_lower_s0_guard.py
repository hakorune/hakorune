#!/usr/bin/env python3
"""RAW-SOURCE0-LOWER0-S0 disconnected child-draft task guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-execution-task-2026-07-23.md"
)
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-consultation-2026-07-23.md"
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
    task = TASK.read_text()
    card = CARD.read_text()
    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-S0"', "execution row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-execution-task-2026-07-23"', "latest card")
    for fragment in (
        "LOWER0-D0 closeout",
        "Q1 — owner",
        "Q2 — draft-only seam",
        "Q3 — discovery and admission",
        "Q4 — source-derived root policy",
        "Q5 — failure and handoff",
        "RAW-SOURCE0-LOWER0-S0",
        "production consumers at zero",
    ):
        require(card, fragment, f"decision {fragment}")
    for fragment in (
        "RawDraftInvocationV1",
        "RawChildWorkRequestV1",
        "reserve before descent",
        "branded collector admission",
        "root Main/condition completion = 0",
        "retry/fallback/catch_unwind = 0",
        "below 800 lines",
    ):
        require(task, fragment, f"task boundary {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during LOWER0-S0")
    if "bind_raw_source(" in joined:
        callers = [
            path.relative_to(ROOT)
            for path in PROD
            if "bind_raw_source(" in path.read_text() and path.name != "mod.rs"
        ]
        if callers:
            raise AssertionError(f"unexpected Raw binding caller: {callers}")
    for path in (TASK, CARD, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[cut0-i0-root0-raw-source0-lower-s0-guard] ok "
        "decision=1 task=1 raw_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
