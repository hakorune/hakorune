#!/usr/bin/env python3
"""RAW-SOURCE0-LOWER0 design-stop guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
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
    card = CARD.read_text()
    if 'current_execution_row = "RAW-SOURCE0-LOWER0-S0"' in state:
        require(card, "LOWER0-D0 closeout", "decision closeout")
        require(card, "RAW-SOURCE0-LOWER0-S0", "next executable row")
        print(
            "[cut0-i0-root0-raw-source0-lower-consult0-guard] closed "
            "next=RAW-SOURCE0-LOWER0-S0"
        )
        return 0
    require(state, 'current_design_stop = "RAW-SOURCE0-LOWER0-CONSULT0"', "design stop")
    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-CONSULT0"', "execution row")
    for fragment in (
        "Q1 — root lowering owner",
        "Q2 — draft-only seam",
        "Q3 — discovery and admission order",
        "Q4 — root mode and callable Main",
        "Q5 — failure and handoff",
        "Option 1",
        "production Raw consumer = 0",
        "MirBuilder::build_module retirement = 0",
    ):
        require(card, fragment, f"consultation boundary {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during LOWER0 design stop")
    if "bind_raw_source(" in joined:
        # The disconnected compiler method is allowed; no public wrapper may
        # call it while the source-to-draft seam is under consultation.
        callers = [
            path.relative_to(ROOT)
            for path in PROD
            if "bind_raw_source(" in path.read_text()
            and path.name != "mod.rs"
        ]
        if callers:
            raise AssertionError(f"unexpected Raw binding caller: {callers}")
    for path in (CARD, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"LOWER0 consultation file must remain below 800 lines: {path}")
    print(
        "[cut0-i0-root0-raw-source0-lower-consult0-guard] ok "
        "design_stop=1 raw_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
