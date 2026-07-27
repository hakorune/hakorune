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
    actual = len(
        re.findall(
            rf"\bfn\s+{re.escape(name)}(?:<[^>]+>)?\s*\(",
            text,
        )
    )
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def rust_code(text: str) -> str:
    """Drop line comments before authority checks.

    Boundary documentation must be able to describe rejected routes.  These
    checks guard executable authority, not diagnostic vocabulary.
    """

    return "\n".join(
        line.split("//", 1)[0]
        for line in text.splitlines()
    )


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    terminal = read(root, "src/mir/builder/calls/method_call_terminal.rs")
    tests = read(root, "src/mir/builder/calls/method_call_terminal_tests.rs")
    readme = read(root, "src/mir/builder/calls/README.md")
    calls_mod = read(root, "src/mir/builder/calls/mod.rs")
    candidate = read(
        root, "src/mir/builder/calls/preloop_located_argument_port.rs"
    )
    capability = read(root, "src/mir/builder/me_call_header_observation.rs")

    require_count(
        terminal,
        "trait MethodCallValueTerminalPortV1",
        1,
        "terminal port owner",
    )
    require_count(
        terminal,
        "impl<Port> MethodCallValueTerminalPortV1 for Port",
        1,
        "single raw-compatible terminal implementation",
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
        1,
        "raw global compatibility facade",
    )
    require_definition_count(
        terminal,
        "emit_global_value_terminal_with_lookup_v1",
        1,
        "shared static/me global helper definition",
    )
    require_count(
        terminal,
        "emit_global_value_terminal_with_lookup_v1",
        4,
        "shared static/me global helper uses",
    )
    require_count(calls_mod, "mod method_call_terminal;", 1, "private terminal module")
    if "pub mod method_call_terminal;" in calls_mod:
        fail("terminal module must remain private")

    production = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if path.name not in {"method_call_terminal.rs", "method_call_terminal_tests.rs"}
        and path.name != "located_legacy_lowering.rs"
        and not path.name.endswith("_tests.rs")
    )
    finish_methods = (
        "finish_typeop_value_terminal",
        "finish_static_global_value_terminal",
        "finish_me_lowered_global_value_terminal",
        "finish_env_value_terminal",
        "finish_standard_value_terminal",
    )
    finish_counts = {
        "finish_typeop_value_terminal": 1,
        "finish_static_global_value_terminal": 1,
        "finish_me_lowered_global_value_terminal": 1,
        "finish_env_value_terminal": 1,
        "finish_standard_value_terminal": 2,
    }
    for name in finish_methods:
        require_count(
            production,
            f".{name}(",
            finish_counts[name],
            f"I0 production consumer {name}",
        )
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
    require_count(
        build,
        "MethodCallValueTerminalPortV1",
        0,
        "retired direct source terminal bounds",
    )
    require_count(
        capability,
        "MethodCallValueTerminalPortV1",
        3,
        "source-neutral capability bundle",
    )
    require_count(
        capability,
        "trait MethodCallLoweringPortV1",
        1,
        "method-call capability owner",
    )

    for needle, expected, label in (
        ("static_scalar_method_fact(&func_name)", 1, "static scalar selector"),
        ("emit_static_scalar_fact_const(", 1, "static scalar emitter"),
        ('method == "weak_to_strong"', 1, "weak-load selector"),
        ("emit_weak_load(", 1, "weak-load emitter"),
        ('method == "upgrade"', 1, "deprecated upgrade preflight"),
        (
            "try_inline_record_helper_call_with_descent(",
            1,
            "retired prepared record helper compatibility selector",
        ),
        (
            "prepare_record_helper_inline",
            2,
            "prepared record helper selectors",
        ),
        (
            "execute_prepared_record_helper_inline",
            2,
            "prepared record helper executors",
        ),
        (
            "try_inline_same_module_helper_setter_call_with_descent(",
            0,
            "retired direct setter selector",
        ),
        (
            "try_inline_same_module_helper_setter_call_from_receiver_with_descent(",
            0,
            "retired receiver setter selector",
        ),
        (
            "prepare_same_module_helper_setter_inline(",
            1,
            "prepared direct setter selector",
        ),
        (
            "prepare_same_module_helper_setter_inline_from_receiver(",
            1,
            "prepared receiver setter selector",
        ),
        (
            "execute_prepared_same_module_helper_setter_inline",
            2,
            "prepared setter executors",
        ),
    ):
        require_count(handlers, needle, expected, label)

    require_count(
        candidate,
        "impl<'site, 'view, 'catalog, Port> MethodCallValueTerminalPortV1",
        1,
        "candidate terminal adapter",
    )
    for name in terminal_methods:
        require_definition_count(candidate, name, 1, f"candidate terminal method {name}")
    terminal_code = rust_code(terminal)
    for forbidden in (
        "MirInstruction",
        "emit_unified_call",
        "next_value_id",
        "type_ctx",
        "value_types",
    ):
        if forbidden in candidate:
            fail(f"candidate terminal adapter owns forbidden authority: {forbidden}")

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
        if forbidden in terminal_code:
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

    # REP0 keeps source policy out of the shared terminal.  The terminal may
    # issue exactly one generic value-call receipt; the pre-loop owner pairs it
    # with source evidence only after that physical Call succeeds.
    ingress = read(root, "src/mir/builder/calls/preloop_located_argument_ingress.rs")
    receipt = read(root, "src/mir/builder/calls/preloop_nested_result_receipt.rs")
    ingress_tests = read(
        root, "src/mir/builder/calls/preloop_located_argument_ingress_tests.rs"
    )
    ingress_p0_tests = read(
        root, "src/mir/builder/calls/preloop_located_argument_ingress_p0_tests.rs"
    )
    nested_type = read(root, "src/mir/builder/calls/preloop_nested_result_type.rs")
    nested_type_p0 = read(
        root, "src/mir/builder/calls/preloop_nested_result_type_p0_tests.rs"
    )
    require_definition_count(
        terminal,
        "emit_standard_value_terminal_with_receipt_v1",
        1,
        "REP0 generic receipt terminal",
    )
    require_count(
        terminal_code,
        "UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(",
        1,
        "REP0 sole generic physical receipt writer",
    )
    for forbidden in (
        "PreparedPreloop",
        "ReachedPreloop",
        "EmittedNested",
        "SourceExprSiteV1",
        "type_ctx",
        "value_types",
    ):
        if forbidden in terminal_code:
            fail(f"REP0 generic terminal owns forbidden authority: {forbidden}")

    require_count(
        receipt,
        "struct ReachedPreloopNestedPhysicalCallV1",
        1,
        "REP0 source plus physical receipt owner",
    )
    require_count(
        receipt,
        "struct EmittedNestedInstanceCallV1",
        1,
        "REP0 emitted nested receipt owner",
    )
    require_definition_count(
        receipt,
        "complete_after_outer_success",
        1,
        "REP0 outer-success receipt terminal",
    )
    for forbidden in (
        "MirInstruction",
        "emit_unified_call",
        "type_ctx",
        "value_types",
    ):
        if forbidden in rust_code("\n".join((ingress, candidate, receipt))):
            fail(f"REP0 pre-loop receipt path owns forbidden authority: {forbidden}")
    for evidence in (
        "configured_preloop_ingress_reaches_existing_inner_and_outer_call_terminals",
        "production_prefix_outer_failure_retains_source_and_physical_receipt",
        "production_prefix_physical_inner_call_failure_retains_source_without_receipt",
    ):
        if evidence not in ingress_tests and evidence not in ingress_p0_tests:
            fail(f"missing REP0 production-prefix evidence: {evidence}")

    nested_type_code = rust_code(nested_type)
    require_definition_count(
        nested_type,
        "publish_preloop_nested_integer_result_v1",
        1,
        "TYPE-I0 sole receipt consumer",
    )
    require_count(
        nested_type_code,
        "TypeFactDecisionV1::prepare(",
        1,
        "TYPE-I0 sole fact decision",
    )
    require_count(
        nested_type_code,
        "type_ctx.set_type(",
        1,
        "TYPE-I0 sole fact writer",
    )
    require_count(
        production,
        "publish_preloop_nested_integer_result_v1(",
        1,
        "TYPE-I0 production caller zero",
    )
    for forbidden in (
        "MirInstruction",
        "emit_unified_call",
        "ASTNode",
        "SourceExprSiteV1",
        "GenericLoop",
        "value_types",
        "retry",
        "fallback",
    ):
        if forbidden in nested_type_code:
            fail(f"TYPE-I0 owner owns forbidden authority: {forbidden}")
    for evidence in (
        "production_prefix_publishes_none_unknown_and_matching_integer",
        "production_prefix_conflict_preserves_fact_then_fresh_fixture_succeeds",
    ):
        if evidence not in nested_type_p0:
            fail(f"missing TYPE-I0 production-prefix evidence: {evidence}")

    for phrase in (
        "disconnected V0 value-only terminal port",
        "Route selection, syntax preflight, and child descent must finish",
        "owns no route table",
        "caller ledger, retry, or fallback",
        "V0-S0 production consumers = 0",
        "V0-I0 threads exactly the five ordinary source completions",
        "creates no MethodCall source carrier",
        "Located source, caller-ledger, activation, and result authority remain absent",
        "V0-P0/G0 fixes normalized destination allocation",
        "record/helper-setter, FastMem, MIR-debug, and REPL remain explicit custom",
        "never retry, and leave the Builder reusable",
        "is not a route, result, type, or effect authority",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = (
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/calls/method_call_terminal.rs",
        "src/mir/builder/calls/method_call_terminal_tests.rs",
        "src/mir/builder/calls/method_call_descent.rs",
        "src/mir/builder/calls/preloop_located_argument_port.rs",
        "src/mir/builder/calls/preloop_located_argument_ingress.rs",
        "src/mir/builder/calls/preloop_nested_result_receipt.rs",
        "src/mir/builder/calls/preloop_nested_result_type.rs",
        "src/mir/builder/calls/preloop_nested_result_type_tests.rs",
        "src/mir/builder/calls/preloop_nested_result_type_p0_tests.rs",
        "src/mir/builder/calls/preloop_nested_result_test_support.rs",
        "src/mir/builder/calls/preloop_located_argument_ingress_tests.rs",
        "src/mir/builder/calls/preloop_located_argument_ingress_p0_tests.rs",
        "src/mir/builder/calls/build.rs",
        "src/mir/builder/calls/member_route.rs",
        "src/mir/builder/calls/member_route_descent_tests.rs",
        "src/mir/builder/method_call_handlers.rs",
        "src/mir/builder/me_call_header_observation.rs",
        "src/mir/builder/record_helper_args_tests.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(f"{TAG} ok: terminal_owner=1 raw_impl=1 source_consumers=5 property_raw=1")


if __name__ == "__main__":
    main()
