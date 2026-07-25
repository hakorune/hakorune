#!/usr/bin/env python3
"""PHYSICAL-THUNK0 guard for the backend-neutral entry carrier."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s0-execution-task-2026-07-25.md"
)
SOURCE = ROOT / "src/mir/compiler/source_entry_physical.rs"


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
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-PHYSICAL-THUNK0"',
            'current_execution_row = "ENTRY-RESULT-PROJECTION0-VM-REFERENCE0"',
        )
    ):
        raise AssertionError("missing active or retained physical-thunk row")
    for fragment in (
        "PHYSICAL-THUNK0",
        "Builder/backend-neutral",
        "bare mutable `MirModule`",
        "process status before projection",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "enum PhysicalEntryRoleV1",
        "struct PhysicalSourceEntryCarrierV1",
        "into_physical(self)",
        "CompletedSourceEntryV1",
        "SourceEntryResultV1",
    ):
        require(source, fragment, f"physical carrier implementation {fragment}")
    if source.count("into_physical(self)") != 1:
        raise AssertionError("physical carrier must have one consuming handoff")
    if "MirModule" in source or "ProcessExitCodeV1" in source:
        raise AssertionError("physical carrier must not expose module or status")
    if "NYASH_ENTRY" in source or "module.functions" in source:
        raise AssertionError("physical carrier must not select a backend entry")
    for path in (STATE, TASK, SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-physical-thunk0-guard] ok "
        "one_handoff=1 backend_neutral=1 no_module=1 no_status=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
