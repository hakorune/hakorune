#!/usr/bin/env python3
"""CUT0-I0-COLLECT0-BATCH0 disconnected callable-batch guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
BUILDER = ROOT / "src/mir/builder.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
TRANSACTION = ROOT / (
    "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
)

ALLOWED = {
    BUILDER.relative_to(ROOT),
    COLLECTOR.relative_to(ROOT),
    TRANSACTION.relative_to(ROOT),
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    builder = BUILDER.read_text()
    transaction = TRANSACTION.read_text()

    for path in (*[pathlib.Path(p) for p in ALLOWED], pathlib.Path(__file__)):
        if len((ROOT / path).read_text().splitlines()) >= 800:
            raise AssertionError(f"BATCH0 file must remain below 800 lines: {path}")

    require(
        state,
        "CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof",
        "S0 closeout",
    )
    require(
        state,
        "CUT0-I0-COLLECT0-BATCH0 is closed as a disconnected atomic callable-batch proof",
        "BATCH0 closeout",
    )
    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "session closeout")
    require(state, "CUT0-I0-ROOT0-BRAND0 is closed as a disconnected real branded physical-owner proof", "ROOT0 brand successor")
    require(task, "### CUT0-I0-COLLECT0-BATCH0 — closed", "task row")
    require(task, "late collision -> collector delta = 0", "atomic acceptance")
    require(task, "recursive capability preserved exactly once", "recursive acceptance")
    require(task, "CUT0-I0-SESSION0", "successor task")

    require(transaction, "publish_into", "callable publication remains present")
    require((ROOT / "src/mir/compiler/recursive_callable_module_plan/tests.rs").read_text(),
            "rejects_zero_call_acyclic_and_nontrivial_profiles", "recursive source rejection")

    if transaction.count("fn publish_into(") != 1:
        raise AssertionError("existing callable publish_into owner must remain singular")

    print(
        "[cut0-i0-collect0-batch0-guard] ok "
        "recursive_source_check=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
