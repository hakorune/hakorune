#!/usr/bin/env python3
"""S3 ENTRY-CARRY0 guard for the move-only Raw entry identity."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md"
)
BIND = ROOT / "src/mir/compiler/raw_source_binding.rs"
SELECT = ROOT / "src/mir/compiler/source_entry_selection.rs"
SLOT = ROOT / "src/mir/builder/root_batch_slot.rs"
POST = ROOT / "src/mir/compiler/raw_root_postprocess.rs"
PUB = ROOT / "src/mir/compiler/raw_root_publication.rs"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    bind = BIND.read_text()
    select = SELECT.read_text()
    slot = SLOT.read_text()
    post = POST.read_text()
    pub = PUB.read_text()

    if (
        'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-ENTRY-CARRY0"' not in state
        and 'current_execution_row = "ENTRY-RESULT-PROJECTION0-S3-EXECUTION0"' not in state
    ):
        raise AssertionError("S3 carry must be active or the next execution row")
    for fragment in (
        "S3-ENTRY-CARRY0",
        "SelectedSourceEntryContinuationV1",
        "Thread the continuation through",
        "route reconstruction after",
    ):
        require(task, fragment, f"task contract {fragment}")
    for fragment in (
        "SelectedSourceEntryContinuationV1",
        "selected_entry: SelectedSourceEntryContinuationV1",
        "from_projection(",
    ):
        require(bind, fragment, f"binding carry {fragment}")
    for fragment in (
        "struct SelectedSourceEntryContinuationV1",
        "raw_main_entry_target()",
        "target: crate::mir::builder::RawMainEntryTargetV1",
    ):
        require(select, fragment, f"sealed target {fragment}")
    for fragment in ("struct RawMainEntryTargetV1", "key: FunctionDraftKeyV1", "symbol: Box<str>", "arity: usize"):
        require(slot, fragment, f"Main slot target {fragment}")
    require(post, "fn selected_entry(&self) -> &SelectedSourceEntryContinuationV1", "postprocess carry")
    require(post, "main_entry_target_matches", "root witness target pairing")
    require(pub, "fn selected_entry(&self) -> &SelectedSourceEntryContinuationV1", "publication carry")

    for forbidden in ("NYASH_ENTRY", "execute_module", "module.functions"):
        if forbidden in select or forbidden in pub:
            raise AssertionError(f"entry carry must not use backend discovery: {forbidden}")
    for path in (STATE, TASK, BIND, SELECT, SLOT, POST, PUB, Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")
    print(
        "[entry-result-projection0-s3-entry-carry-guard] ok "
        "sealed_target=1 move_only=1 no_backend_discovery=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
