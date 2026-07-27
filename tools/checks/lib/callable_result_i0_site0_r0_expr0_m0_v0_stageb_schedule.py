#!/usr/bin/env python3
"""Guard the F6-2 one-driver selected Stage-B body schedule."""

from __future__ import annotations

from pathlib import Path


TAG = "[callable-result-i0-site0-r0-expr0-m0-v0/stageb-schedule]"


def fail(message: str) -> None:
    raise SystemExit(f"{TAG} {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def code(text: str) -> str:
    return "\n".join(line.split("//", 1)[0] for line in text.splitlines())


def require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def check_stageb_schedule(root: Path) -> None:
    base = "src/mir/builder/calls/preloop_stageb_instance_function_session"
    module_path = f"{base}/mod.rs"
    schedule_path = f"{base}/body_schedule.rs"
    rejection_path = f"{base}/rejection.rs"
    tests_path = f"{base}/body_schedule_tests.rs"
    calls_mod_path = "src/mir/builder/calls/mod.rs"

    module = read(root, module_path)
    schedule = read(root, schedule_path)
    rejection = read(root, rejection_path)
    tests = read(root, tests_path)
    calls_mod = read(root, calls_mod_path)
    schedule_code = code(schedule)
    rejection_code = code(rejection)

    require_count(
        calls_mod,
        "mod preloop_stageb_instance_function_session;",
        1,
        "F6-2 module registration",
    )
    require_count(
        schedule,
        "struct CompletedPreloopStageBBodyScheduleV1",
        1,
        "F6-2 completion owner",
    )
    require_count(
        rejection,
        "struct RejectedPreloopStageBBodyScheduleV1",
        1,
        "F6-2 rejection owner",
    )
    require_count(
        schedule,
        "enum PreloopStageBBodyScheduleStateV1",
        1,
        "F6-2 monotonic state owner",
    )
    require_count(
        schedule,
        "impl LegacyBlockDescentPortV1 for PreloopStageBBodySchedulePortV1",
        1,
        "F6-2 sole bounded block port",
    )
    require_count(
        schedule_code,
        "drive_legacy_block_v1(builder, &mut port)",
        1,
        "F6-2 sole body driver call",
    )
    require_count(
        schedule_code,
        "drive_legacy_statement_v1(",
        1,
        "F6-2 shared ordinary statement descent",
    )
    require_count(
        schedule_code,
        "self.ordinary.reborrow()",
        1,
        "F6-2 selected invocation-port reborrow",
    )

    for needle, label in (
        ("body_statement_count()", "body cardinality preflight"),
        ("prefix_statement_count()", "prefix boundary preflight"),
        ("suffix_statement_start()", "suffix boundary preflight"),
        (
            "source.selected().parent().view().declaration().body()",
            "catalog-backed body authority",
        ),
        (
            "&self.body[index..self.selected_index]",
            "prefix suffix-router fence",
        ),
        ("if index == self.selected_index", "selected suffix-router fence"),
        (
            "PreloopStageBBodyScheduleStateV1::Published(_)",
            "published-only suffix admission",
        ),
    ):
        if needle not in schedule_code:
            fail(f"missing F6-2 {label}: {needle}")

    for terminal in (
        "with_prepared_located_argument(",
        "complete_preloop_located_outer_request_v1(",
        "complete_preloop_outer_carrier_call_v1(",
        "complete_preloop_carrier_assignment_v1(",
        "publish_preloop_outer_carrier_integer_v1(",
        ".into_stageb_carrier_v1(",
    ):
        require_count(schedule_code, terminal, 1, f"F6-2 selected terminal {terminal}")

    for retained in (
        "OwnedRejectedPreloopLocatedOuterCompletionV1",
        "OwnedRejectedPreloopOuterCarrierCallV1",
        "OwnedRejectedPreloopCarrierAssignmentV1",
        "OwnedRejectedPreloopOuterCarrierIntegerPublicationV1",
        "Published(CompletedPreloopStageBCarrierV1)",
    ):
        if retained not in rejection_code:
            fail(f"missing F6-2 retained failure owner: {retained}")

    for forbidden in (
        "while ",
        "build_block_with_port_v1",
        "RawLegacyChildLoweringPortV1",
        "SourcePath",
        "finalize_function",
        "capture_legacy_function_payload",
        "add_function(",
        "try_add_function(",
        "value_types.insert",
        "thread_local!",
        "static mut",
        "into_owner",
        "retry",
        "fallback",
        "resume",
        "rearm",
    ):
        if forbidden in schedule_code or forbidden in rejection_code:
            fail(f"F6-2 forbidden authority: {forbidden}")

    production_consumers = 0
    source_root = root / "src/mir"
    for path in source_root.rglob("*.rs"):
        if path == root / schedule_path or path.name.endswith("_tests.rs"):
            continue
        production_consumers += path.read_text(encoding="utf-8").count(
            "drive_preloop_stageb_body_schedule_v1("
        )
    if production_consumers != 0:
        fail(
            "F6-2 production consumers must remain zero: "
            f"actual={production_consumers}"
        )

    for evidence in (
        "actual_parser_body_schedule_publishes_before_real_suffix_frontier",
        "prefix_failure_retains_pending_source_and_blocks_selected_and_suffix",
        "selected_barrier_clips_prefix_suffix_input_and_requires_one_shot_completion",
        "selected_route_drift_retains_rejection_and_fresh_fixture_reuses",
    ):
        if evidence not in tests:
            fail(f"missing F6-2 evidence: {evidence}")

    touched = (
        module_path,
        schedule_path,
        rejection_path,
        tests_path,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0_stageb_schedule.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"F6-2 source/check files reached 800 lines: {oversized}")


if __name__ == "__main__":
    check_stageb_schedule(Path(".").resolve())
    print(f"{TAG} ok")
