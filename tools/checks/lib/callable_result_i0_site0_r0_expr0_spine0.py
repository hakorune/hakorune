#!/usr/bin/env python3
"""Reusable structural guard for SITE0-R0 EXPR0-SPINE0."""

from __future__ import annotations

import re
import sys
from pathlib import Path


TAG = "[callable-result-i0-site0-r0-expr0-spine0]"


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


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    module_path = "src/mir/builder/ops/binary_expression_descent.rs"
    tests_path = "src/mir/builder/ops/binary_expression_descent_tests.rs"
    raw_tests_path = "src/mir/builder/ops/binary_expression_raw_tests.rs"
    readme_path = "src/mir/builder/ops/README.md"
    ops_root_path = "src/mir/builder/ops/mod.rs"
    module = read(root, module_path)
    tests = read(root, tests_path)
    raw_tests = read(root, raw_tests_path)
    readme = read(root, readme_path)
    ops_root = read(root, ops_root_path)

    require_count(
        module,
        "trait BinaryExpressionDescentPortV1",
        1,
        "ordinary Binary port owner",
    )
    require_count(module, "type BinaryInput;", 1, "associated Binary input")
    require_count(module, "fn binary_syntax", 2, "syntax query declaration plus raw impl")
    require_count(module, "fn binary_left_input", 2, "left query declaration plus raw impl")
    require_count(module, "fn binary_right_input", 2, "right query declaration plus raw impl")
    require_count(
        module,
        "fn drive_ordinary_binary_expression_v1",
        1,
        "ordinary Binary driver",
    )
    require_count(
        module,
        "drive_legacy_expression_v1(",
        2,
        "left/right E0 descent",
    )
    require_count(
        module,
        "builder.build_binary_op_from_values(",
        1,
        "existing completion consumer",
    )

    logical_at = module.index("BinaryOperator::And | BinaryOperator::Or")
    left_input_at = module.index("port.binary_left_input(input)?")
    left_descent_at = module.index("drive_legacy_expression_v1(builder, port, left_input)?")
    right_input_at = module.index("port.binary_right_input(input)?")
    right_descent_at = module.index("drive_legacy_expression_v1(builder, port, right_input)?")
    completion_at = module.index("builder.build_binary_op_from_values(")
    if not (
        logical_at
        < left_input_at
        < left_descent_at
        < right_input_at
        < right_descent_at
        < completion_at
    ):
        fail("ordinary Binary order must be logical reject -> left -> right -> completion")

    for forbidden in (
        "build_expression(",
        "build_expression_impl(",
        "logical_shortcircuit::",
        "ExprChildRoleV1",
        "LegacyExprInputV1",
        "LocatedLegacy",
        "CallableResult",
        "SourcePath",
        "ledger",
        "recursion_depth",
        "next_value_id",
        "type_ctx",
        "MirInstruction",
        "CallTarget",
        "retry",
        "fallback",
    ):
        if forbidden in module:
            fail(f"BIN0 substrate owns forbidden authority: {forbidden}")

    require_count(
        module,
        "impl BinaryExpressionDescentPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "BIN0-I0 raw implementation",
    )
    require_count(module, "struct RawLegacyBinaryInputV1", 1, "owned raw Binary input")
    require_count(
        module,
        "fn drive_raw_ordinary_binary_expression_v1",
        1,
        "thin raw Binary facade",
    )
    if "BinaryExpressionDescentPortV1 for LocatedLegacyLoweringSessionV1" in read(
        root, "src/mir/builder/located_legacy_lowering.rs"
    ):
        fail("BIN0-S0 located implementation must remain zero")

    production_callers = 0
    ignored = {
        (root / module_path).resolve(),
        (root / tests_path).resolve(),
    }
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in ignored:
            continue
        production_callers += path.read_text(encoding="utf-8").count(
            "drive_ordinary_binary_expression_v1("
        )
    if production_callers != 0:
        fail(f"generic Binary driver must remain facade-private: actual={production_callers}")

    raw_selectors = 0
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in {
            (root / module_path).resolve(),
            (root / raw_tests_path).resolve(),
        }:
            continue
        raw_selectors += path.read_text(encoding="utf-8").count(
            "drive_raw_ordinary_binary_expression_v1("
        )
    if raw_selectors != 1:
        fail(f"BIN0-I0 raw production selectors: expected=1 actual={raw_selectors}")

    logical_selector_at = ops_root.index(
        "if matches!(operator, BinaryOperator::And | BinaryOperator::Or)"
    )
    raw_selector_at = ops_root.index(
        "binary_expression_descent::drive_raw_ordinary_binary_expression_v1("
    )
    if logical_selector_at >= raw_selector_at:
        fail("And/Or selection must precede the ordinary raw adapter")
    for retired_direct_descent in (
        "let lhs_raw = self.build_expression(left)?",
        "let rhs_raw = self.build_expression(right)?",
    ):
        if retired_direct_descent in ops_root:
            fail(f"old direct Binary descent remains selected: {retired_direct_descent}")

    require_count(
        ops_root,
        "mod binary_expression_descent;",
        1,
        "private Binary substrate module",
    )
    require_count(
        ops_root,
        "mod binary_expression_descent_tests;",
        1,
        "focused Binary fixture module",
    )
    require_count(
        ops_root,
        "mod binary_expression_raw_tests;",
        1,
        "focused raw Binary fixture module",
    )

    for fixture in (
        "ordinary_arithmetic_descends_left_then_right_once_and_uses_existing_terminal",
        "ordinary_comparison_uses_same_order_and_existing_bool_terminal",
        "ordinary_operator_boundary_rejects_only_and_or_before_child_effects",
        "syntax_and_input_failures_precede_later_child_effects",
        "child_failure_stops_later_descent_and_fresh_driver_is_independent",
        "terminal_failure_occurs_after_both_children_without_retry",
    ):
        if fixture not in tests:
            fail(f"missing BIN0-S0 fixture: {fixture}")

    for fixture in (
        "raw_ordinary_binary_entry_preserves_left_right_and_existing_terminal",
        "raw_ordinary_binary_accepts_method_calls_on_both_sides",
        "nested_raw_ordinary_binary_restores_depth_and_allows_reuse",
        "raw_ordinary_binary_failure_stops_later_child_or_terminal",
        "logical_operators_remain_on_existing_short_circuit_owner",
        "raw_binary_child_depth_failure_restores_parent_depth",
    ):
        if fixture not in raw_tests:
            fail(f"missing BIN0-I0 fixture: {fixture}")

    for phrase in (
        "child-demand boundary",
        "reject `And` and `Or` before child effects",
        "existing `build_binary_op_from_values` owner",
        "never stored in `MirBuilder`",
        "BIN0-I0 selects the ordinary raw source entry",
        "production located callers remain zero",
    ):
        if phrase not in readme:
            fail(f"missing BIN0 README boundary: {phrase}")

    touched = (
        module_path,
        tests_path,
        raw_tests_path,
        readme_path,
        ops_root_path,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    if re.search(r"Arc<|Rc<|thread_local!|static mut", module):
        fail("BIN0 substrate must remain stack-scoped and immutable")

    print(
        f"{TAG} ok: driver=1 child_descents=2 raw_selector=1 "
        "raw_impl=1 located_impl=0 logical_owner_preserved=1"
    )


if __name__ == "__main__":
    main()
