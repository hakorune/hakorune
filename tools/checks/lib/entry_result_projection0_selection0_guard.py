#!/usr/bin/env python3
"""ENTRY-SELECTION0 guard for the one-shot sealed route handoff."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_selection.rs"


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
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-ENTRY-SELECTION0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-SOURCE-ENTRY0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PHYSICAL-THUNK0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-VM-REFERENCE0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-EXE-AOT0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PARITY-G0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-FAULT-STATUS0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-RUNTIME-ACTIVATION-DESIGN-STOP"',
        )
    ):
        raise AssertionError("missing active or retained selection row")
    for fragment in (
        "ENTRY-SELECTION0",
        "one compiler-internal producer of `SelectedSourceEntryV1`",
        "must not re-use the backend-local selection helpers",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "enum SelectedSourceEntryRouteV1",
        "struct SelectedSourceEntryV1",
        "fn select_source_entry(",
        "RawRootEnvironmentManifestV1",
        "RawRootSourceRouteV1::Script",
        "RawRootSourceRouteV1::App",
        "into_parts(",
    ):
        require(source, fragment, f"selection implementation {fragment}")
    if source.count("fn select_source_entry(") != 1:
        raise AssertionError("selection must have one producer")
    if "select_entry_function" in source or "NYASH_ENTRY" in source:
        raise AssertionError("selection must not consult backend entry helpers")
    if "module.functions" in source or "MirModule" in source:
        raise AssertionError("selection must not scan module symbols")
    if "std::process::exit" in source or "process::exit" in source:
        raise AssertionError("selection must not terminate the process")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-selection0-guard] ok "
        "one_producer=1 sealed_manifest=1 no_backend_scan=1 no_exit=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
