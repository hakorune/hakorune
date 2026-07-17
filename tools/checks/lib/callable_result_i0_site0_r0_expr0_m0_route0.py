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

    if re.search(r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(in crate::mir::builder\) enum MethodCallChildDemandV1", port):
        fail("stage vocabulary must remain non-Clone")

    production = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/builder").rglob("*.rs")
        if path.name != "method_call_descent.rs" and not path.name.endswith("_tests.rs")
    )
    require_count(production, "lower_method_call_receiver_v1(", 0, "S0 receiver consumers")
    require_count(production, "lower_method_call_arguments_v1(", 0, "S0 argument consumers")
    require_count(production, "RawLegacyMethodCallInputV1::new(", 0, "S0 raw selectors")
    require_count(production, ".method_call_syntax(", 0, "S0 syntax-view consumers")
    require_count(production, ".receiver_expression_input(", 0, "S0 receiver-input consumers")
    require_count(production, ".call_arguments_input(", 0, "S0 argument-input consumers")
    require_count(
        production,
        "impl MethodCallDescentPortV1 for",
        0,
        "external method-port implementations",
    )
    require_count(calls_mod, "mod method_call_descent;", 1, "private method port module")
    if "pub mod method_call_descent;" in calls_mod:
        fail("method descent module must remain private")

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
    ):
        if evidence not in tests:
            fail(f"missing S0 fixture: {evidence}")

    for phrase in (
        "associated-input MethodCall child boundary",
        "never stored in `MirBuilder`",
        "S0 adds this disconnected port",
        "S0 production consumers = 0",
        "Exact route demand remains owned by the later",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/calls/README.md",
        "src/mir/builder/calls/method_call_descent.rs",
        "src/mir/builder/calls/method_call_descent_tests.rs",
        "src/mir/builder/calls/mod.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-m0-route0] ok: "
        "port_owner=1 raw_impl=1 production_consumers=0"
    )


if __name__ == "__main__":
    main()
