#!/usr/bin/env python3
"""Guard the disconnected SITE0-R0-EXPR0-M0-V0-S0 terminal boundary."""

from __future__ import annotations

import re
import sys
from pathlib import Path


TAG = "[callable-result-i0-site0-r0-expr0-m0-v0]"


def fail(message: str) -> None:
    raise SystemExit(f"{TAG} {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def require_definition_count(text: str, name: str, expected: int, label: str) -> None:
    actual = len(re.findall(rf"\bfn\s+{re.escape(name)}\s*\(", text))
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    terminal = read(root, "src/mir/builder/calls/method_call_terminal.rs")
    tests = read(root, "src/mir/builder/calls/method_call_terminal_tests.rs")
    readme = read(root, "src/mir/builder/calls/README.md")
    calls_mod = read(root, "src/mir/builder/calls/mod.rs")

    require_count(
        terminal,
        "trait MethodCallValueTerminalPortV1",
        1,
        "terminal port owner",
    )
    require_count(
        terminal,
        "impl MethodCallValueTerminalPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "raw terminal implementation",
    )
    require_count(
        terminal,
        "MethodCallDescentPortV1",
        2,
        "associated-input terminal inheritance",
    )

    terminal_methods = (
        "emit_typeop_value_terminal",
        "emit_static_global_value_terminal",
        "emit_me_lowered_global_value_terminal",
        "emit_env_value_terminal",
        "emit_standard_value_terminal",
    )
    for name in terminal_methods:
        require_definition_count(terminal, name, 2, f"terminal method {name}")

    raw_helpers = (
        "emit_typeop_value_terminal_raw_v1",
        "emit_global_value_terminal_raw_v1",
        "emit_env_value_terminal_raw_v1",
        "emit_standard_value_terminal_raw_v1",
    )
    for name in raw_helpers:
        require_definition_count(terminal, name, 1, f"raw helper {name}")

    require_count(
        terminal,
        "emit_global_value_terminal_raw_v1(",
        3,
        "shared static/me global helper",
    )
    require_count(calls_mod, "mod method_call_terminal;", 1, "private terminal module")
    if "pub mod method_call_terminal;" in calls_mod:
        fail("terminal module must remain private")

    production = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if path.name not in {"method_call_terminal.rs", "method_call_terminal_tests.rs"}
        and not path.name.endswith("_tests.rs")
    )
    finish_methods = (
        "finish_typeop_value_terminal",
        "finish_static_global_value_terminal",
        "finish_me_lowered_global_value_terminal",
        "finish_env_value_terminal",
        "finish_standard_value_terminal",
    )
    for name in finish_methods:
        require_count(production, f".{name}(", 1, f"I0 production consumer {name}")
    require_count(
        production,
        "emit_standard_value_terminal_raw_v1(",
        1,
        "I0 materialized property consumer",
    )

    build = read(root, "src/mir/builder/calls/build.rs")
    member = read(root, "src/mir/builder/calls/member_route.rs")
    handlers = read(root, "src/mir/builder/method_call_handlers.rs")
    property_reads = read(root, "src/mir/builder/property_reads.rs")
    static_scalar = read(root, "src/mir/builder/static_scalar_facts.rs")
    weak_ref = read(root, "src/mir/builder/utils/weak_ref.rs")
    record_helper = read(root, "src/mir/builder/record_helper_args.rs")
    record_helper_tests = read(root, "src/mir/builder/record_helper_args_tests.rs")
    reserved = read(root, "src/mir/builder/calls/reserved_method_route.rs")
    reserved_tests = read(root, "src/mir/builder/calls/reserved_method_route_tests.rs")
    debug_routes = read(root, "src/mir/builder/calls/debug_method_routing.rs")
    fastmem_calls = read(root, "src/mir/builder/fastmem/calls.rs")
    require_count(handlers, "handle_typeop_method(", 0, "retired direct TypeOp handler")
    require_count(member, "emit_resolved_env_method_call", 0, "retired direct Env helper")
    require_count(handlers, "emit_unified_call(", 0, "ordinary direct call emission")
    require_count(handlers, "MirInstruction::TypeOp", 0, "ordinary direct TypeOp emission")
    require_count(property_reads, "RawLegacyMethodCallInputV1", 0, "property fake source input")
    require_count(build, "MethodCallValueTerminalPortV1", 2, "source terminal port bounds")

    for needle, expected, label in (
        ("static_scalar_method_fact(&func_name)", 1, "static scalar selector"),
        ("emit_static_scalar_fact_const(", 1, "static scalar emitter"),
        ('method == "weak_to_strong"', 1, "weak-load selector"),
        ("emit_weak_load(", 1, "weak-load emitter"),
        ('method == "upgrade"', 1, "deprecated upgrade preflight"),
        (
            "try_inline_record_helper_call_with_descent(",
            3,
            "record helper selectors",
        ),
        (
            "try_inline_same_module_helper_setter_call_with_descent(",
            1,
            "direct setter selector",
        ),
        (
            "try_inline_same_module_helper_setter_call_from_receiver_with_descent(",
            1,
            "receiver setter selector",
        ),
    ):
        require_count(handlers, needle, expected, label)

    custom_owners = "\n".join(
        (static_scalar, weak_ref, record_helper, reserved, debug_routes, fastmem_calls)
    )
    for name in (*terminal_methods, *(f"finish_{name[5:]}" for name in terminal_methods)):
        require_count(custom_owners, name, 0, f"custom owner ordinary terminal {name}")

    for forbidden in (
        "ASTNode",
        "method_call_syntax(",
        "build_expression",
        "MemberCallRoutePlan",
        "ReceiverNormalizationPlan",
        "VerifiedCallableResult",
        "CallerLedger",
        "LegacyExprInputV1",
        "ActivationDisposition",
        "CanonicalSameModuleCallableKeyV1",
        "current_claim",
        "retry",
        "fallback",
        "thread_local!",
        "Arc<",
    ):
        if forbidden in terminal:
            fail(f"terminal boundary owns forbidden authority: {forbidden}")

    if re.search(r"#\[derive\([^]]*Clone[^]]*\)\]", terminal):
        fail("terminal boundary must not add Clone state")
    if re.search(r"\benum\s+.*Terminal", terminal):
        fail("stored terminal enum is forbidden")

    for evidence in (
        "disconnected_typeop_terminals_preserve_check_cast_value_type_and_destination",
        "disconnected_static_and_me_global_terminals_preserve_semantic_target_and_arguments",
        "disconnected_env_terminals_preserve_returning_and_no_result_laws",
        "disconnected_standard_terminal_preserves_method_identity_and_completed_destination",
    ):
        if evidence not in tests:
            fail(f"missing disconnected terminal fixture: {evidence}")

    route_tests = read(root, "src/mir/builder/calls/member_route_descent_tests.rs")
    for evidence in (
        '"terminal:typeop"',
        '"terminal:static"',
        '"terminal:me"',
        '"terminal:env"',
        '"terminal:standard"',
    ):
        if evidence not in route_tests:
            fail(f"missing I0 terminal-order evidence: {evidence}")
    for evidence in (
        "lowered_me_arguments_precede_terminal_and_keep_receiver_prefix",
        "generic_terminal_failure_follows_children_without_retry_and_builder_reuses",
    ):
        if evidence not in route_tests:
            fail(f"missing I0 terminal wiring fixture: {evidence}")

    for evidence in (
        "argument_failure_enters_no_terminal_and_builder_reuses",
        "static_scalar_fact_returns_const_without_generic_terminal",
        "weak_load_and_upgrade_preflight_bypass_generic_terminal",
        "materialized_property_receiver_is_forwarded_without_source_redescent",
    ):
        if evidence not in route_tests:
            fail(f"missing P0 route/custom evidence: {evidence}")
    if "helper_setter_completion_bypasses_generic_terminal" not in record_helper_tests:
        fail("missing P0 helper-setter custom-terminal evidence")

    for evidence in (
        "selected_mir_debug_route_preserves_debug_payload",
        "selected_mir_mark_evaluates_neither_label_nor_extra_arguments",
        "selected_mir_log_stops_at_first_failed_suffix_and_builder_is_reusable",
        "ordinary_reserved_decision_descends_no_children",
        "selected_mir_debug_zero_argument_failure_is_stable",
        "selected_repl_route_preserves_extern_call",
        "selected_repl_unsupported_method_failure_is_stable",
        "selected_fastmem_method_route_preserves_memop_lowering",
        "selected_fastmem_arity_failure_precedes_argument_effects",
        "selected_fastmem_table_id_preflight_precedes_argument_effects",
        "selected_fastmem_positive_upper_preflight_precedes_argument_effects",
    ):
        if evidence not in reserved_tests:
            fail(f"missing P0 reserved custom-terminal evidence: {evidence}")

    for phrase in (
        "disconnected V0 value-only terminal port",
        "Route selection, syntax preflight, and child descent must finish",
        "owns no route table",
        "caller ledger, retry, or fallback",
        "V0-S0 production consumers = 0",
        "V0-I0 threads exactly the five ordinary source completions",
        "creates no MethodCall source carrier",
        "Located source, caller-ledger, activation, and result authority remain absent",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = (
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/calls/method_call_terminal.rs",
        "src/mir/builder/calls/method_call_terminal_tests.rs",
        "src/mir/builder/calls/method_call_descent.rs",
        "src/mir/builder/calls/build.rs",
        "src/mir/builder/calls/member_route.rs",
        "src/mir/builder/calls/member_route_descent_tests.rs",
        "src/mir/builder/method_call_handlers.rs",
        "src/mir/builder/record_helper_args_tests.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(f"{TAG} ok: terminal_owner=1 raw_impl=1 source_consumers=5 property_raw=1")


if __name__ == "__main__":
    main()
