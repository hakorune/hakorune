#!/usr/bin/env python3
"""Guard the F6 session and sole F7 exact-function activation owner."""

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
    lifecycle_path = "src/mir/builder/module_lifecycle.rs"
    lifecycle_tests_path = "src/mir/builder/module_lifecycle_capture_tests.rs"
    collector_path = f"{base}/collector_terminal.rs"
    activation_path = "src/mir/builder/preloop_stageb_function_activation.rs"
    compiler_ledger_path = "src/mir/compiler/legacy_module_activation/ledger.rs"

    session = read(root, session_path)
    rejection = read(root, rejection_path)
    tests = read(root, tests_path)
    ingress = read(root, ingress_path)
    completion = read(root, completion_path)
    lifecycle = read(root, lifecycle_path)
    lifecycle_tests = read(root, lifecycle_tests_path)
    collector = read(root, collector_path)
    activation = read(root, activation_path)
    compiler_ledger = read(root, compiler_ledger_path)
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

    for needle, label in (
        ("trait InstanceMethodCapturePortV1", "F7 capture capability"),
        (
            "struct OrdinaryInstanceMethodCapturePortV1",
            "F7 ordinary adapter",
        ),
        (
            "fn lower_root_after_callable_catalog_install_with_instance_port_v1",
            "F7 shared root kernel",
        ),
        (
            "instance_methods.lower_instance_method(",
            "F7 sole instance-method terminal",
        ),
    ):
        require_count(lifecycle, needle, 1, label)
    if (
        "shared_root_kernel_lends_each_instance_method_to_one_stack_port"
        not in lifecycle_tests
    ):
        fail("missing F7 behavior-neutral capture-seam proof")

    for needle, label in (
        (
            "struct CollectedPreloopStageBInstanceFunctionV1",
            "F7 collected function owner",
        ),
        (
            "fn collect_preloop_stageb_instance_function_v1(",
            "F7 sole collector terminal",
        ),
        (
            "enum PreloopStageBFunctionActivationStateV1",
            "F7 sole transition state",
        ),
        (
            "struct PreparedPreloopStageBFunctionActivationV1",
            "F7 stack-owned ledger",
        ),
        (
            "lower_root_with_preloop_stageb_function_activation_v1(",
            "F7 selected root terminal",
        ),
    ):
        require_count(collector + activation, needle, 1, label)

    for state in ("Armed(", "InFlight", "Completed(", "Rejected {"):
        if state not in activation:
            fail(f"missing F7 retained ledger state: {state}")
    for forbidden in ("enum PreloopStageBFunctionActivationLedgerErrorV1", "SelectedCallerNotObserved"):
        if forbidden in compiler_ledger:
            fail(f"compiler ledger duplicated F7 transition authority: {forbidden}")
    for evidence in (
        "exact_key_is_claimed_once_and_duplicate_claim_is_typed",
        "selected_identity_drift_rejects_before_consuming_armed_owner",
    ):
        if evidence not in activation:
            fail(f"missing F7 exact-ledger evidence: {evidence}")

    activation_consumers = 0
    for path in (root / "src/mir").rglob("*.rs"):
        if path == root / activation_path or path.name.endswith("_tests.rs"):
            continue
        activation_consumers += code(path.read_text(encoding="utf-8")).count(
            "lower_root_with_preloop_stageb_function_activation_v1("
        )
    if activation_consumers != 1:
        fail(f"F7 selected root consumers: expected=1 actual={activation_consumers}")

    compiler_text = read(root, "src/mir/compiler/mod.rs")
    if "PreloopStageBWholeSourceProducerV1::select(" in code(compiler_text):
        fail("F7 must keep compile_request production selector callers at zero")

    touched = (
        session_path,
        rejection_path,
        tests_path,
        ingress_path,
        completion_path,
        lifecycle_path,
        lifecycle_tests_path,
        collector_path,
        activation_path,
        compiler_ledger_path,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0_stageb_session.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"F6-3 source/check files reached 800 lines: {oversized}")


if __name__ == "__main__":
    check_stageb_session(Path(".").resolve())
    print(f"{TAG} ok")
