#!/usr/bin/env python3
"""CONTRACT0 guard for the disconnected source-result projection box."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_result.rs"
LEGACY = tuple(
    ROOT / path
    for path in (
        "src/runner/modes/common_util/vm_execution.rs",
        "src/runner/modes/mir_interpreter.rs",
        "src/runner/product/llvm/fallback_executor.rs",
        "src/mir/join_ir_vm_bridge_dispatch/exec_routes.rs",
    )
)


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
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-S0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-ENTRY-SELECTION0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-SOURCE-ENTRY0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PHYSICAL-THUNK0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-VM-REFERENCE0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-EXE-AOT0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PARITY-G0"',
        )
    ):
        raise AssertionError("CONTRACT0 row is not active or retained")
    require(task, "CONTRACT0", "S0 order")
    for fragment in (
        "SourceEntryResultV1",
        "ProcessTerminationV1",
        "CanonicalProcessExitV1",
        "ProcessExitProjectionV1::project",
        "LegacyRunnerExitProjectionV1",
        "Integer outside range",
        "source Fault",
    ):
        require(task, fragment, f"task contract {fragment}")

    for fragment in (
        "enum UnitOriginV1",
        "struct SealedObjectResultV1",
        "struct SealedSourceFaultV1",
        "enum SourceEntryResultV1",
        "enum ProcessTerminationV1",
        "enum ProcessExitProfileV1",
        "struct ProcessExitProjectionV1",
        "fn project(",
        "LegacyProfileDisconnected",
        "ExitCodeOutOfRange",
        "UnsupportedProcessResult",
        "reserved_fault",
        "unit_and_integer_byte_values_project_without_wrapping",
        "unsupported_values_never_become_success_zero",
    ):
        require(source, fragment, f"CONTRACT0 implementation {fragment}")

    if source.count("struct ProcessExitProjectionV1") != 1:
        raise AssertionError("CONTRACT0 must have one projection owner")
    if source.count("fn project(") != 1:
        raise AssertionError("CONTRACT0 must have one projection terminal")
    if "std::process::exit" in source or "process::exit" in source:
        raise AssertionError("contract box must not terminate the process")
    if "as i32" in source or "as i64" in source:
        raise AssertionError("contract box must not use unchecked status casts")
    for path in LEGACY:
        if "ProcessExitProjectionV1" in path.read_text():
            raise AssertionError(f"legacy caller widened during CONTRACT0: {path}")

    for path in (STATE, TASK, SOURCE, *LEGACY):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[entry-result-projection0-contract0-guard] ok "
        "one_projection=1 typed_faults=1 no_exit=1 legacy_callers=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
