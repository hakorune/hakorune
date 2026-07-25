#!/usr/bin/env python3
"""VM-REFERENCE0 guard for pure projection fixtures."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_reference.rs"


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
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-VM-REFERENCE0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-EXE-AOT0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PARITY-G0"',
        )
    ):
        raise AssertionError("missing active or retained VM reference row")
    for fragment in (
        "VM-REFERENCE0",
        "reference fixtures",
        "do not replace `run_vm_compiled_module`",
        "These fixtures consume the new pure",
        "projection only; they do not replace `run_vm_compiled_module`",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "ProcessExitProjectionV1",
        "UnitOriginV1::EmptyBody",
        "Integer(255)",
        "ExitCodeOutOfRange",
        "UnsupportedProcessResult",
        "SourceFault",
    ):
        require(source, fragment, f"reference fixture {fragment}")
    if "run_vm_compiled_module" in source or "MirBuilder" in source:
        raise AssertionError("reference fixtures must not execute or compile VM modules")
    if "process::exit" in source or "NYASH_ENTRY" in source:
        raise AssertionError("reference fixtures must not own process/backend entry")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-vm-reference0-guard] ok "
        "pure_projection=1 typed_matrix=1 no_vm_execution=1 no_exit=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
