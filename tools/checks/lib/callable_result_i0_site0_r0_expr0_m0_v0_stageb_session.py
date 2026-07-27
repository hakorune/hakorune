#!/usr/bin/env python3
"""Guard the F6-3 unpublished Stage-B instance-function session."""

from __future__ import annotations

from pathlib import Path


TAG = "[callable-result-i0-site0-r0-expr0-m0-v0/stageb-session]"


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


def check_stageb_session(root: Path) -> None:
    base = "src/mir/builder/calls/preloop_stageb_instance_function_session"
    session_path = f"{base}/session.rs"
    rejection_path = f"{base}/session_rejection.rs"
    tests_path = f"{base}/session_tests.rs"
    ingress_path = "src/mir/preloop_stageb_carrier/function_ingress.rs"
    completion_path = "src/mir/builder/port_aware_function_draft_impl.rs"

    session = read(root, session_path)
    rejection = read(root, rejection_path)
    tests = read(root, tests_path)
    ingress = read(root, ingress_path)
    completion = read(root, completion_path)
    session_code = code(session)
    rejection_code = code(rejection)

    for needle, label in (
        ("struct PreparedPreloopStageBInstanceFunctionV1", "prepared owner"),
        (
            "struct CompletedPreloopStageBInstanceFunctionPayloadV1",
            "completed payload",
        ),
        (
            "struct PendingPreloopStageBInstanceFunctionSessionV1",
            "pending owner",
        ),
        (
            "struct CompletedPreloopStageBInstanceFunctionV1",
            "completed owner",
        ),
    ):
        require_count(session, needle, 1, f"F6-3 {label}")

    for needle, label in (
        (
            "struct PreloopStageBInstanceFunctionPrimaryRejectionV1",
            "primary rejection",
        ),
        (
            "struct RejectedPreloopStageBInstanceFunctionSessionV1",
            "session rejection",
        ),
        ("CleanupAfterSuccess", "cleanup-after-success retention"),
        ("DuringCleanup", "during-cleanup retention"),
    ):
        if needle not in rejection_code:
            fail(f"missing F6-3 {label}: {needle}")

    require_count(
        session_code,
        "capture_legacy_function_payload_pending_session_v1(",
        1,
        "F6-3 generic payload-session capture",
    )
    required_order = (
        "prepare_instance_method_draft_body_v1(",
        "run_function_body_step_tree_guard_v1(",
        "drive_preloop_stageb_body_schedule_v1(",
        "prepare_port_aware_draft_body_completion_v1(",
        "finalize_function_draft_with_headers(",
    )
    positions = [session_code.find(needle) for needle in required_order]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        fail(f"F6-3 authority order drift: {list(zip(required_order, positions))}")

    require_count(
        session_code,
        "schedule: CompletedPreloopStageBBodyScheduleV1",
        1,
        "F6-3 full schedule payload",
    )
    require_count(
        rejection_code,
        "Finalizer(CompletedPreloopStageBBodyScheduleV1)",
        1,
        "F6-3 finalizer retains schedule",
    )
    require_count(
        ingress,
        "fn instance_draft_source(",
        1,
        "F6-3 exact catalog-backed source projection",
    )
    require_count(
        completion,
        "fn prepare_port_aware_draft_body_completion_v1(",
        1,
        "F6-3 shared finalizer preparation",
    )

    for forbidden in (
        "build_instance_method_draft_with_port_v1(",
        "drive_legacy_block_v1(",
        "add_function(",
        "try_add_function(",
        "value_types.insert",
        "ParserStringUtilsBox\"",
        "into_owner",
        "retry",
        "fallback",
        "resume",
        "rearm",
    ):
        if forbidden in session_code or forbidden in rejection_code:
            fail(f"F6-3 forbidden authority: {forbidden}")

    consumers = 0
    for path in (root / "src/mir").rglob("*.rs"):
        if path == root / session_path or path.name.endswith("_tests.rs"):
            continue
        consumers += path.read_text(encoding="utf-8").count(
            "capture_preloop_stageb_instance_function_v1("
        )
    if consumers != 0:
        fail(f"F6-3 production consumers must remain zero: actual={consumers}")

    for evidence in (
        "phase_a_indexed_actual_parser_completes_one_unpublished_stageb_function",
        "suffix_failure_restores_parent_retains_carrier_then_fresh_session_succeeds",
    ):
        if evidence not in tests:
            fail(f"missing F6-3/F6-4 evidence: {evidence}")

    touched = (
        session_path,
        rejection_path,
        tests_path,
        ingress_path,
        completion_path,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0_stageb_session.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"F6-3 source/check files reached 800 lines: {oversized}")


if __name__ == "__main__":
    check_stageb_session(Path(".").resolve())
    print(f"{TAG} ok")
