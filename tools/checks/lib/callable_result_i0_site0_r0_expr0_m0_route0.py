#!/usr/bin/env python3
"""Guard the behavior-neutral SITE0-R0-EXPR0-M0-ROUTE0 series."""

from __future__ import annotations

import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-expr0-m0-route0] {message}")


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
    port = read(root, "src/mir/builder/calls/method_call_descent.rs")
    tests = read(root, "src/mir/builder/calls/method_call_descent_tests.rs")
    readme = read(root, "src/mir/builder/calls/README.md")
    calls_mod = read(root, "src/mir/builder/calls/mod.rs")
    reserved = read(root, "src/mir/builder/calls/reserved_method_route.rs")
    debug = read(root, "src/mir/builder/calls/debug_method_routing.rs")
    fastmem = read(root, "src/mir/builder/fastmem/calls.rs")
    build = read(root, "src/mir/builder/calls/build.rs")
    reserved_tests = read(root, "src/mir/builder/calls/reserved_method_route_tests.rs")

    require_count(port, "struct MethodCallSyntaxViewV1", 1, "syntax view owner")
    require_count(port, "trait MethodCallDescentPortV1", 1, "method port owner")
    require_count(port, "type MethodCallInput;", 1, "associated method input")
    require_count(port, "struct RawLegacyMethodCallInputV1", 1, "raw stack carrier")
    require_count(
        port,
        "impl MethodCallDescentPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "raw method port impl",
    )
    require_count(
        port,
        "drive_legacy_expression_v1(builder, port, receiver)",
        1,
        "E0 receiver primitive",
    )
    require_count(
        port,
        "drive_call_arguments_v1(builder, port, arguments)",
        1,
        "ARG0 argument primitive",
    )
    require_count(
        port,
        "fn lower_method_call_argument_v1",
        1,
        "indexed E0 primitive owner",
    )

    if re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(in crate::mir::builder\) enum MethodCallChildDemandV1", port):
        fail("stage vocabulary must remain non-Clone")

    production = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if path.name != "method_call_descent.rs" and not path.name.endswith("_tests.rs")
    )
    require_count(production, "lower_method_call_receiver_v1(", 0, "R0 receiver consumers")
    require_count(production, "lower_method_call_arguments_v1(", 1, "R0 REPL ARG0 consumer")
    require_count(production, "lower_method_call_argument_v1(", 2, "R0 indexed E0 consumers")
    require_count(production, "RawLegacyMethodCallInputV1::new(", 1, "R0 raw selector")
    require_count(production, ".method_call_syntax(", 1, "R0 syntax-view consumer")
    require_count(production, ".receiver_expression_input(", 0, "R0 receiver-input consumers")
    require_count(
        production,
        "impl MethodCallDescentPortV1 for",
        0,
        "external method-port implementations",
    )
    require_count(calls_mod, "mod method_call_descent;", 1, "private method port module")
    if "pub mod method_call_descent;" in calls_mod:
        fail("method descent module must remain private")

    require_count(reserved, "enum PreparedReservedMethodCallV1", 1, "prepared route owner")
    require_count(reserved, "fn prepare_reserved_method_call_v1(", 1, "prepare owner")
    require_count(reserved, "fn build_reserved_method_call_v1<Port>", 1, "reserved driver owner")
    require_count(
        reserved,
        "classify_source_method_reserved_route_v1(context, object, method, arguments)",
        1,
        "classify-once consumer",
    )
    require_count(build, "build_reserved_method_call_v1(", 1, "production reserved selector")
    require_count(reserved, "lower_method_call_arguments_v1(builder, port, input)?", 1, "REPL full ARG0")
    require_count(reserved, "lower_method_call_argument_v1(builder, port, input, index)?", 1, "Debug indexed E0")
    require_count(fastmem, "lower_method_call_argument_v1(builder, port, input, index)", 1, "FastMem indexed E0")
    require_count(reserved, "lower_method_call_receiver_v1", 0, "reserved receiver descent")
    require_count(reserved, ".to_vec()", 0, "reserved AST clone terminal")
    require_count(debug, "build_call_args(", 0, "debug ARG0 bypass")
    require_count(debug, "build_expression(", 0, "debug raw descent bypass")
    require_count(fastmem, "fn lower_fastmem_args(", 0, "retired FastMem raw loop")

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
        "value_origin_newbox",
        "Box<dyn",
        "thread_local!",
        "emit_",
        "next_value_id",
        "type_ctx",
        "value_types",
        "current_module",
    ):
        if forbidden in port:
            fail(f"S0 port owns forbidden authority: {forbidden}")

    for evidence in (
        "raw_method_input_exposes_one_borrowed_syntax_view",
        "raw_receiver_and_arguments_use_existing_e0_and_arg0_ports",
        "associated_inputs_keep_receiver_and_arguments_independent",
        "raw_single_argument_descent_skips_syntax_only_neighbors",
    ):
        if evidence not in tests:
            fail(f"missing S0 fixture: {evidence}")

    for evidence in (
        "selected_mir_mark_evaluates_neither_label_nor_extra_arguments",
        "selected_mir_log_stops_at_first_failed_suffix_and_builder_is_reusable",
        "ordinary_reserved_decision_descends_no_children",
        "selected_fastmem_arity_failure_precedes_argument_effects",
        "selected_fastmem_table_id_preflight_precedes_argument_effects",
        "selected_fastmem_positive_upper_preflight_precedes_argument_effects",
    ):
        if evidence not in reserved_tests:
            fail(f"missing R0 fixture: {evidence}")

    for phrase in (
        "associated-input MethodCall child boundary",
        "never stored in `MirBuilder`",
        "S0 adds this disconnected port",
        "S0 production consumers = 0",
        "Exact route demand remains owned by the later",
        "the full ARG0 boundary",
        "FastMem keeps its syntax preflight before indexed E0",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/method_call_descent.rs",
        "src/mir/builder/calls/method_call_descent_tests.rs",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/calls/reserved_method_route.rs",
        "src/mir/builder/calls/reserved_method_route_tests.rs",
        "src/mir/builder/calls/debug_method_routing.rs",
        "src/mir/builder/fastmem/calls.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-m0-route0] ok: "
        "port_owner=1 reserved_selector=1 receiver_consumers=0"
    )


if __name__ == "__main__":
    main()
