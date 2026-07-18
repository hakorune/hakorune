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
    member = read(root, "src/mir/builder/calls/member_route.rs")
    member_tests = read(root, "src/mir/builder/calls/member_route_descent_tests.rs")
    handlers = read(root, "src/mir/builder/method_call_handlers.rs")
    helpers = read(root, "src/mir/builder/record_helper_args.rs")
    helper_tests = read(root, "src/mir/builder/record_helper_args_tests.rs")
    exprs = read(root, "src/mir/builder/exprs.rs")
    property_reads = read(root, "src/mir/builder/property_reads.rs")
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
    require_count(port, "trait MethodCallArgumentDescentV1", 1, "route argument capability")
    require_count(port, "struct AssociatedMethodCallArgumentsV1", 1, "associated route adapter")
    require_count(port, "struct LegacyMethodCallArgumentsV1", 1, "materialized-receiver adapter")
    require_count(port, "fn into_parts", 0, "retired raw AST split")

    if re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(in crate::mir::builder\) enum MethodCallChildDemandV1", port):
        fail("stage vocabulary must remain non-Clone")

    production = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if path.name != "method_call_descent.rs" and not path.name.endswith("_tests.rs")
    )
    require_count(production, "lower_method_call_receiver_v1(", 2, "M0 receiver consumers")
    require_count(production, "lower_method_call_arguments_v1(", 1, "R0 REPL ARG0 consumer")
    require_count(production, "lower_method_call_argument_v1(", 2, "R0 indexed E0 consumers")
    require_count(production, "RawLegacyMethodCallInputV1::new(", 1, "R0 raw selector")
    require_count(production, ".method_call_syntax(", 6, "M0 syntax-view consumers")
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
    require_count(build, "RawLegacyMethodCallInputV1::new(", 1, "production raw MethodCall selector")
    require_count(build, "build_member_method_call_v1(port, input)", 1, "ordinary member driver")
    require_count(build, "is_typeop_method(", 1, "source TypeOp decision owner")
    require_count(exprs, "is_typeop_method(", 0, "retired expression TypeOp decision")
    require_count(member, "is_typeop_method(", 0, "retired member TypeOp reprobe")
    require_count(member, "build_expression(object.clone())", 0, "raw receiver bypass")
    require_count(member, "AssociatedMethodCallArgumentsV1::new(", 5, "route argument adapters")
    require_count(handlers, "build_call_args(arguments)", 0, "handler ARG0 bypass")
    require_count(handlers, "descent.lower_all(", 3, "static me standard ARG0 demand")
    require_count(member, "descent.lower_all(self)?", 1, "env ARG0 demand")
    require_count(helpers, "let value = self.build_expression(arg.clone())?", 0, "helper arg E0 bypass")
    require_count(helpers, "descent.lower_index(self,", 2, "helper indexed E0 consumers")
    require_count(
        property_reads,
        "handle_standard_method_call(object_value, getter_name, &[])",
        1,
        "materialized property-read consumer",
    )
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

    for evidence in (
        "typeop_descends_receiver_once_and_keeps_type_string_syntax_only",
        "static_route_skips_receiver_and_descends_arguments_left_to_right",
        "standard_route_descends_receiver_before_arguments",
        "standard_receiver_failure_descends_no_arguments_and_builder_is_reusable",
        "malformed_typeop_uses_standard_receiver_then_argument_demand",
        "env_route_keeps_receiver_syntax_only_and_descends_arguments",
        "bound_me_route_keeps_source_receiver_syntax_only",
        "materialized_property_receiver_is_forwarded_without_source_redescent",
    ):
        if evidence not in member_tests:
            fail(f"missing M0 fixture: {evidence}")

    for evidence in (
        "inlineable_setter_accepts_simple_assignment_and_return",
        "setter_allowlist_rejects_before_catalog_query",
        "structured_catalog_lookup_preserves_static_and_instance_namespaces",
    ):
        if evidence not in helper_tests:
            fail(f"missing split record-helper fixture: {evidence}")

    for phrase in (
        "associated-input MethodCall child boundary",
        "never stored in `MirBuilder`",
        "S0 adds this disconnected port",
        "S0 production consumers = 0",
        "Exact route demand remains owned by the later",
        "the full ARG0 boundary",
        "FastMem keeps its syntax preflight before indexed E0",
        "ROUTE0-M0 is closed through S0/H0/I0/P0/G0",
        "Record-helper scalarization is intentionally not a full-ARG0 consumer",
        "already owns a materialized",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/method_call_descent.rs",
        "src/mir/builder/calls/method_call_descent_tests.rs",
        "src/mir/builder/calls/mod.rs",
        "src/mir/builder/calls/build.rs",
        "src/mir/builder/calls/member_route.rs",
        "src/mir/builder/calls/member_route_descent_tests.rs",
        "src/mir/builder/calls/reserved_method_route.rs",
        "src/mir/builder/calls/reserved_method_route_tests.rs",
        "src/mir/builder/calls/debug_method_routing.rs",
        "src/mir/builder/fastmem/calls.rs",
        "src/mir/builder/method_call_handlers.rs",
        "src/mir/builder/record_helper_args.rs",
        "src/mir/builder/record_helper_args_tests.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-m0-route0] ok: "
        "port_owner=1 reserved_selector=1 receiver_consumers=2"
    )


if __name__ == "__main__":
    main()
