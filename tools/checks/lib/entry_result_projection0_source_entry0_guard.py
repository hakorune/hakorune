#!/usr/bin/env python3
"""SOURCE-ENTRY0 guard for the selected-route result transport."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_thunk.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    source = SOURCE.read_text()
    require(
        state,
        'current_execution_row = "ENTRY-RESULT-PROJECTION0-SOURCE-ENTRY0"',
        "active row",
    )
    for fragment in (
        "SOURCE-ENTRY0",
        "SourceEntryThunkV1",
        "must transport this typed selection to a source result",
        "must not infer route",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "struct SourceEntryThunkV1",
        "struct CompletedSourceEntryV1",
        "begin_thunk(self)",
        "fn complete(",
        "SourceEntryResultV1",
        "SelectedSourceEntryRouteV1",
    ):
        require(source, fragment, f"thunk implementation {fragment}")
    if source.count("begin_thunk(self)") != 1:
        raise AssertionError("thunk must have one selected-owner handoff")
    if "NYASH_ENTRY" in source or "module.functions" in source:
        raise AssertionError("thunk must not consult backend entry helpers")
    if "MirModule" in source or "process::exit" in source:
        raise AssertionError("thunk must not expose module or process status")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-source-entry0-guard] ok "
        "one_handoff=1 typed_result=1 no_backend_scan=1 no_exit=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
