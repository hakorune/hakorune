#!/usr/bin/env python3
"""EXE-AOT0 guard for the normalized ny_main capability boundary."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_ny_main.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    source = SOURCE.read_text()
    if not any(
        row in state
        for row in (
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-EXE-AOT0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PARITY-G0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-FAULT-STATUS0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-RUNTIME-ACTIVATION-DESIGN-STOP"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-ENTRY-CARRY0"',
        )
    ):
        raise AssertionError("missing active or retained EXE-AOT row")
    for fragment in (
        "EXE-AOT0",
        "explicit capability adapter boundary",
        "It receives only",
        "`ProcessExitCodeV1`/normalized status",
        "ensure_ny_main",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "struct NyMainStatusV1",
        "NyMainCapabilityAdapterV1",
        "fn accept(code: ProcessExitCodeV1)",
        "normalized_i64(self)",
        "ProcessExitCodeV1",
    ):
        require(source, fragment, f"ny_main adapter {fragment}")
    if "SourceEntryResultV1" in source or "ensure_ny_main" in source:
        raise AssertionError("ny_main adapter must not accept source values or legacy helper")
    if "42" in source or "mock" in source or "process::exit" in source:
        raise AssertionError("ny_main adapter must not own mock/fallback/process exit")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-exe-aot0-guard] ok "
        "status_only=1 normalized=1 no_legacy_helper=1 no_mock=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
