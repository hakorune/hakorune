#!/usr/bin/env python3
"""Keep the retired completion boundary explicit."""

from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[3]
SRC = ROOT / "src/mir/builder"
BUILDER = SRC.parent / "builder.rs"
TASK = ROOT / "docs/development/current/main/investigations/cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
DECISION = ROOT / "docs/development/current/main/investigations/cut0-i0-root0-canon0-design-question-2026-07-22.md"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    task = TASK.read_text()
    decision = DECISION.read_text()
    builder = BUILDER.read_text()
    if len(pathlib.Path(__file__).read_text().splitlines()) >= 800:
        raise AssertionError("CANON0 guard must remain below 800 lines")
    require(task, "ROOT0-CANON0", "active task")
    require(decision, "C-prime", "C-prime decision")
    if "mod canonical_root_completion;" in builder or (SRC / "canonical_root_completion.rs").exists():
        raise AssertionError("retired disconnected completion returned")
    print("[cut0-i0-root0-canon0-guard] ok retired_completion=1")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
