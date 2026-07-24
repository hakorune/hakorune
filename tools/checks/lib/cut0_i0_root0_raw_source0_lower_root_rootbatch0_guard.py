#!/usr/bin/env python3
"""ROOTBATCH0-S0 identity, prepare, and consuming-owner guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-rootbatch0-s0-execution-task-2026-07-24.md"
)
SOURCES = tuple(
    ROOT / path
    for path in (
        "src/mir/builder/root_batch_slot.rs",
        "src/mir/builder/raw_required_condition_draft.rs",
        "src/mir/builder/raw_expansion_receipt_ledger.rs",
        "src/mir/builder/raw_expansion_receipt_ledger/root_pair.rs",
        "src/mir/builder/module_draft_collector/root_batch.rs",
        "src/mir/builder/raw_root_physical/root_batch_terminal.rs",
        "src/mir/builder/raw_root_environment_install.rs",
        "src/mir/compiler/raw_root_decl_access.rs",
    )
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    for path in SOURCES:
        if not path.exists():
            raise AssertionError(f"missing ROOTBATCH0 source: {path}")
    joined = "\n".join(path.read_text() for path in SOURCES)

    active_row = 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-ROOTBATCH0-S0"'
    closed_row = "ROOTBATCH-prime-r1 S0a/S0b/S0c/S0d/G0 are closed"
    if active_row not in state and closed_row not in state:
        raise AssertionError("ROOTBATCH0 must be active or explicitly closed")
    require(task, "Decision: **ROOTBATCH-prime-r1**", "decision lock")
    for fragment in (
        "RawRootBatchSlotV1",
        "RawRequiredConditionDraftV1::build()",
        "prepare_root_batch(self)",
        "`RawExpansionReceiptLedgerV1::reserve()` is forbidden during prepare.",
        "DRAIN0",
        "production consumer",
    ):
        require(task, fragment, f"task contract {fragment}")

    for fragment in (
        'symbol: "main"',
        'symbol: "condition_fn"',
        "RawRequiredConditionDraftV1::build",
        "prepare_required_root_pair",
        "commit_reservations",
        "prepare_raw_root_batch",
        "RawRootBatchCompleteInvocationV1",
        "RejectedRawRootBatchInvocationV1",
    ):
        require(joined, fragment, f"ROOTBATCH0 authority {fragment}")

    body = (ROOT / "src/mir/builder/raw_root_body_lowering.rs").read_text()
    if 'name: "main/0"' in body:
        raise AssertionError("BODY0 must not produce slash-zero root signatures")
    pair = (ROOT / "src/mir/builder/raw_expansion_receipt_ledger/root_pair.rs").read_text()
    if ".reserve(" in pair:
        raise AssertionError("ledger reserve is forbidden during ROOTBATCH0 prepare")
    compiler = (ROOT / "src/mir/compiler/raw_root_decl_access.rs").read_text()
    for forbidden in (
        "InvocationPhysicalStateV1::into_parts",
        "collector.into_parts",
        "execute_preflighted_module_invocation",
        "retry",
        "fallback",
    ):
        if forbidden in compiler:
            raise AssertionError(f"compiler-side ROOTBATCH0 authority is forbidden: {forbidden}")

    for path in (STATE, TASK, *SOURCES):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-rootbatch0-guard] ok "
        "identity=1 condition_factory=1 prepare_no_reserve=1 "
        "consuming_handoff=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
