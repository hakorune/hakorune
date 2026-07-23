#!/usr/bin/env python3
"""RAW-SOURCE0-LOWER0-ROOT0-PLAN0 design/plan guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
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
    task = TASK.read_text()
    require(state, 'current_design_stop = "RAW-SOURCE0-LOWER0-ROOT0-PLAN0"', "design stop")
    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-PLAN0"', "execution row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23"', "latest card")
    for fragment in (
        "RawRootKindV1",
        "Physical root identity",
        'main        = symbol "main", arity 0',
        'condition_fn = symbol "condition_fn", arity 1',
        "complete ordered source work schedule",
        "declaration/index plan",
        "callable catalog",
        "static-table specs/plans",
        "closure source sites",
        "runtime-input snapshot",
        "current_module reads = 0",
        "process-global method-slot mutation = 0",
        "production root consumer/executor = 0",
        "retry/fallback/catch_unwind",
        "ROOT0-PLAN0",
    ):
        require(task, fragment, f"plan contract {fragment}")
    joined = "\n".join(path.read_text() for path in PROD)
    if "execute_preflighted_module_invocation" in joined:
        raise AssertionError("outer executor is wired during ROOT0-PLAN0")
    if "begin_raw_draft(" in joined:
        callers = [
            path.relative_to(ROOT)
            for path in PROD
            if "begin_raw_draft(" in path.read_text() and path.name != "mod.rs"
        ]
        if callers:
            raise AssertionError(f"unexpected Raw draft consumer: {callers}")
    for path in (TASK, *PROD):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[cut0-i0-root0-raw-source0-lower-root-plan0-guard] ok "
        "plan=1 raw_consumer=0 executor=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
