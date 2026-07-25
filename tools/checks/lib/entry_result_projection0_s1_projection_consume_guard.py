#!/usr/bin/env python3
"""S1 projection-consume guard for carrier-retaining process projection."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s1-projection-consume-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_projection.rs"


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
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S2-FAULT-STATUS0"',
        )
    ):
        raise AssertionError("missing active or retained S1 projection row")
    for fragment in (
        "S1-PROJECTION-CONSUME0",
        "backend-neutral prepared",
        "The carrier is retained by value",
        "exact carrier with stage, typed cause, and `discard(self)` only",
        "normal/JSON/backend callers -> 0",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "struct RejectedSourceEntryProjectionV1",
        "struct PreparedSourceEntryProjectionV1",
        "struct ProjectedSourceEntryV1",
        "prepare_process_projection(",
        "fn project(self)",
        "ProcessExitProjectionV1::project_borrowed",
        "discard(self)",
        "PhysicalSourceEntryCarrierV1",
    ):
        require(source, fragment, f"projection implementation {fragment}")
    if source.count("pub(in crate::mir) fn prepare_process_projection(") != 1:
        raise AssertionError("projection must have one carrier prepare boundary")
    if source.count("fn project(self)") != 1:
        raise AssertionError("prepared projection must have one infallible commit")
    for forbidden in (
        "MirModule",
        "compile_with_source",
        "compile_raw_with_source",
        "run_vm_compiled_module",
        "NYASH_ENTRY",
        "process::exit",
        "std::process::exit",
    ):
        if forbidden in source:
            raise AssertionError(f"projection row must not add authority: {forbidden}")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s1-projection-consume-guard] ok "
        "one_prepare=1 one_commit=1 carrier_retained=1 no_backend=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
