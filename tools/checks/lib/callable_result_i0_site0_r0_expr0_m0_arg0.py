#!/usr/bin/env python3
"""Guard SITE0-R0-EXPR0-M0-ARG0's behavior-neutral argument port."""

from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-expr0-m0-arg0] {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    port = read(root, "src/mir/builder/calls/call_argument_descent.rs")
    tests = read(root, "src/mir/builder/calls/call_argument_descent_tests.rs")
    build = read(root, "src/mir/builder/calls/build.rs")
    child = read(root, "src/mir/builder/recursive_child_lowering.rs")
    readme = read(root, "src/mir/builder/calls/README.md")

    require_count(port, "trait CallArgumentDescentPortV1", 1, "argument port owner")
    require_count(port, "type ArgumentsInput: ?Sized;", 1, "associated input owner")
    require_count(
        port,
        "impl CallArgumentDescentPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "raw argument port impl",
    )
    require_count(port, "drive_legacy_expression_v1(builder, port, expression_input)", 1, "E0 child consumer")
    require_count(port, "enforce_moved_same_call_args_contract(port, input)?;", 1, "whole-list moved preflight")
    require_count(port, "fail_if_record_value_call_arg_by_name(name, value)?", 1, "per-argument record guard")
    require_count(port, "[call/arg_build:undefined_value]", 1, "undefined-value observation")
    require_count(build, "drive_raw_call_arguments_v1(self, args)", 1, "selected raw facade")
    require_count(build, "fn enforce_moved_same_call_args_contract", 0, "retired build-local preflight")
    require_count(child, "impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1", 1, "single raw child impl")

    builder_sources = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if not path.name.endswith("_tests.rs")
    )
    require_count(builder_sources, "build_call_args(", 10, "raw ARG0 facade surface")

    for forbidden in (
        "MemberCallRoutePlan",
        "ReservedMethodCall",
        "CallTarget",
        "EffectMask",
        "MirInstruction::Call",
        "VerifiedCallableResult",
        "CallerLedger",
        "LegacyExprInputV1",
        "ActivationDisposition",
        "Box<dyn",
        "thread_local!",
    ):
        if forbidden in port:
            fail(f"argument descent owns forbidden authority: {forbidden}")

    for evidence in (
        "associated_inputs_descend_once_in_source_order",
        "empty_arguments_publish_no_child_calls",
        "argument_failure_stops_later_descent_without_retry",
        "failed_port_does_not_poison_fresh_argument_descent",
        "selected_raw_facade_preserves_nested_argument_mir",
    ):
        if evidence not in tests:
            fail(f"missing argument parity fixture: {evidence}")

    move_tests = read(root, "src/tests/mir_move_contract.rs")
    for evidence in (
        "moved_same_call_args_is_fail_fast_in_strict_planner_required",
        "moved_same_method_call_args_is_fail_fast_in_strict_planner_required",
        "moved_same_call_args_keeps_release_mode_behavior",
    ):
        if evidence not in move_tests:
            fail(f"missing moved-state regression: {evidence}")

    for phrase in (
        "behavior-neutral argument boundary",
        "moved-state preflight happens before effects",
        "never stored in `MirBuilder`",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/build.rs",
        "src/mir/builder/calls/call_argument_descent.rs",
        "src/mir/builder/calls/call_argument_descent_tests.rs",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/recursive_child_lowering.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_arg0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-m0-arg0] ok: "
        "argument_owner=1 raw_child_impl=1 selected_facade=1 route_authority=0"
    )


if __name__ == "__main__":
    main()
