#!/usr/bin/env python3
"""PARITY-G0 guard for the disconnected entry-result authority chain."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE_DIR = ROOT / "src/mir/compiler"
SOURCES = (
    SOURCE_DIR / "source_entry_result.rs",
    SOURCE_DIR / "source_entry_selection.rs",
    SOURCE_DIR / "source_entry_thunk.rs",
    SOURCE_DIR / "source_entry_physical.rs",
    SOURCE_DIR / "source_entry_reference.rs",
    SOURCE_DIR / "source_entry_ny_main.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    source = "\n".join(path.read_text() for path in SOURCES)
    if not any(
        row in state
        for row in (
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PARITY-G0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S1-DESIGN-STOP"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-DESIGN-STOP"',
        )
    ):
        raise AssertionError("missing active or retained parity row")
    for fragment in (
        "PARITY-G0",
        "one projection owner",
        "one selection producer",
        "zero\nnew backend status converters",
        "zero changes to JSON/normal entry callers",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "pub(in crate::mir) fn project(",
        "fn select_source_entry(",
        "begin_thunk(self)",
        "into_physical(self)",
        "ProcessExitProjectionV1",
        "SelectedSourceEntryV1",
        "SourceEntryThunkV1",
        "PhysicalSourceEntryCarrierV1",
        "NyMainCapabilityAdapterV1",
    ):
        require(source, fragment, f"authority chain {fragment}")
    if source.count("pub(in crate::mir) fn project(") != 1:
        raise AssertionError("projection owner must have one implementation")
    if source.count("fn select_source_entry(") != 1:
        raise AssertionError("selection producer must be unique")
    for forbidden in (
        "compile_raw_with_source",
        "compile_with_source",
        "run_vm_compiled_module",
        "build_module(",
        "NYASH_ENTRY",
        "MirModule",
        "process::exit",
        "std::process::exit",
    ):
        if forbidden in source:
            raise AssertionError(f"new S0 source must not add caller/authority: {forbidden}")
    for path in (STATE, TASK, *SOURCES):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-parity-g0-guard] ok "
        "one_projection=1 one_selection=1 no_new_callers=1 no_fallback=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
