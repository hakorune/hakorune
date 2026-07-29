#!/usr/bin/env python3
"""Guard SITE0-R0-EXPR0-E0's behavior-neutral child-lowering port."""

from __future__ import annotations

import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[callable-result-i0-site0-r0-expr0-e0] {message}")


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
    port = read(root, "src/mir/builder/recursive_child_lowering.rs")
    builder_build = read(root, "src/mir/builder/builder_build.rs")
    statements = read(root, "src/mir/builder/stmts/mod.rs")
    fastmem = read(root, "src/mir/builder/fastmem.rs")
    tests = read(root, "src/mir/builder/recursive_child_lowering_tests.rs")
    readme = read(root, "src/mir/builder/README.md")

    require_count(port, "trait RecursiveChildLoweringPortV1", 1, "port owner")
    require_count(port, "type BodyInput;", 1, "body input owner")
    require_count(port, "type StatementInput;", 1, "statement input owner")
    require_count(port, "type ExpressionInput;", 1, "expression input owner")
    require_count(port, "struct RawLegacyChildLoweringPortV1", 1, "raw port owner")
    require_count(
        port,
        "impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1",
        1,
        "invocation port impl",
    )
    require_count(
        port,
        "impl RecursiveChildLoweringPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "raw compatibility port impl",
    )

    require_count(
        builder_build,
        "drive_raw_legacy_expression_v1(self, ast)",
        1,
        "selected expression facade",
    )
    require_count(
        statements,
        "drive_raw_legacy_body_v1(self, statements)",
        1,
        "selected body facade",
    )
    require_count(
        statements,
        "drive_raw_legacy_statement_v1(self, node)",
        1,
        "selected statement facade",
    )
    require_count(
        port,
        "block_stmt::build_block_with_port_v1(builder, self, input)",
        2,
        "raw child-port body leaves",
    )
    require_count(
        port,
        "block_stmt::build_statement_with_port_v1(builder, self, input)",
        2,
        "raw child-port statement leaves",
    )
    require_count(
        port,
        "builder.build_expression_impl_with_port_v1(port, input)",
        1,
        "shared raw expression leaf",
    )
    require_count(fastmem, "block_stmt::build_block", 0, "fastmem raw bypass")
    require_count(fastmem, "builder.build_block(body)", 1, "fastmem selected facade")

    combined = port + builder_build + statements + fastmem
    for forbidden in (
        "VerifiedCallableResult",
        "CallerLedger",
        "LegacyBodyInputV1",
        "LegacyStmtInputV1",
        "LegacyExprInputV1",
        "ActivationDisposition",
        "MethodCallExpr",
        "Box<dyn",
        "thread_local!",
        "retry",
        "fallback",
    ):
        if forbidden in combined:
            fail(f"selected raw child boundary contains forbidden authority: {forbidden}")

    for evidence in (
        "associated_inputs_dispatch_each_child_kind_exactly_once",
        "child_driver_propagates_failure_without_retry",
        "selected_raw_body_and_statement_ports_preserve_order_and_last_value",
        "expression_failure_restores_recursion_depth_for_reuse",
    ):
        if evidence not in tests:
            fail(f"missing child-lowering parity fixture: {evidence}")
    require_count(tests, ".build_expression(", 0, "retired expression facade callers")
    require_count(
        tests,
        "drive_raw_legacy_expression_v1(",
        6,
        "durable raw-expression evidence callers",
    )

    for phrase in (
        "recursive child-lowering boundary",
        "associated-input",
        "never stored in `MirBuilder`",
    ):
        if phrase not in readme:
            fail(f"missing README boundary: {phrase}")

    touched = [
        "src/mir/builder/README.md",
        "src/mir/builder.rs",
        "src/mir/builder/builder_build.rs",
        "src/mir/builder/fastmem.rs",
        "src/mir/builder/recursive_child_lowering.rs",
        "src/mir/builder/recursive_child_lowering_tests.rs",
        "src/mir/builder/stmts/mod.rs",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_e0.py",
    ]
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        "[callable-result-i0-site0-r0-expr0-e0] ok: "
        "port=1 raw_impl=1 selected_facades=3 located_consumers=0"
    )


if __name__ == "__main__":
    main()
