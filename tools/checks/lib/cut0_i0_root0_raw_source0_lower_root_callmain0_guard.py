#!/usr/bin/env python3
"""CALLMAIN0-S0 implementation boundary guard."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-callmain0-s0-execution-task-2026-07-24.md"
)
SOURCE = (
    ROOT / "src/mir/compiler/raw_root_callable_main.rs",
    ROOT / "src/mir/compiler/raw_root_callable_main_p0.rs",
    ROOT / "src/mir/compiler/raw_root_children.rs",
    ROOT / "src/mir/compiler/raw_root_plan0.rs",
    ROOT / "src/mir/compiler/raw_source_binding.rs",
    ROOT / "src/mir/builder/raw_root_physical.rs",
    ROOT / "src/mir/builder/raw_root_physical/callable_main_terminal.rs",
    ROOT / "src/mir/builder/raw_root_child_work.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    joined = "\n".join(path.read_text() for path in SOURCE)

    require(state, 'current_execution_row = "RAW-SOURCE0-LOWER0-ROOT0-BODY0-CONSULT0"', "next row")
    require(state, 'latest_card = "cut0-i0-raw-source0-lower-root-body-question-2026-07-24"', "next card")
    for fragment in (
        "Decision: **CALLMAIN-prime-r1**",
        "RawSourceContinuationV1::callable_main()",
        "complete_callable_main(self, work)",
        "CallableMainCompatibility",
        "RawCallableMainReadyInvocationV1",
        "RejectedRawCallableMainInvocationV1",
        "RootBodyCompletionTrackerV1",
        "production consumers remain zero",
        "all modified/new source and check files < 800 lines",
    ):
        require(task, fragment, f"task contract {fragment}")

    require(joined, "finish_callable_main", "one-shot completion terminal")
    require(joined, "into_callable_main_decision", "consuming disposition split")
    require(joined, "callable_main_compatibility", "dedicated ledger request")
    require(joined, "RawCallableMainRoleV1::CallableMainCompatibility", "typed role evidence")
    require(joined, "selected_failure_retains_prefix_and_blocks_body_entry", "failure fixture")
    require(joined, "app_selected_uses_callable_main_role_and_same_brand", "success fixture")
    require(joined, "app_not_selected_does_not_reserve_or_emit_callable_receipt", "not-selected fixture")

    for forbidden in (
        "RawDraftInvocationV1",
        "ModuleLoweringInvocationStateV1",
        "MainPending",
        "MainCaptured",
        "with_shell_collector",
        "legacy_discovered(RawExpansionDraftRoleV1::CallableMainCompatibility",
        "catch_unwind",
        "execute_preflighted_module_invocation",
    ):
        if forbidden in joined:
            raise AssertionError(f"forbidden CALLMAIN0 authority: {forbidden}")

    if joined.count("RawExpansionDraftRequestV1::callable_main_compatibility") != 1:
        raise AssertionError("CALLMAIN compatibility request must have one consumer")
    for tracker_operation in (
        "begin_child(",
        "close_child(",
        "begin_header_loan(",
        "close_header_loan(",
        "begin_pending_terminal(",
        "close_pending_terminal(",
        "complete(",
    ):
        if tracker_operation in joined:
            raise AssertionError(f"CALLMAIN0 must not operate the root-body tracker: {tracker_operation}")

    for path in (STATE, TASK, *SOURCE):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"file must remain below 800 lines: {path}")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-callmain0-guard] ok "
        "same_owner=1 selected_role=1 no_body=1 production_consumer=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
