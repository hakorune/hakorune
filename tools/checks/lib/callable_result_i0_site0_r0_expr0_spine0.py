#!/usr/bin/env python3
"""Reusable structural guard for SITE0-R0 EXPR0-SPINE0."""

from __future__ import annotations

import re
import sys
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment import check_asn0_s0
from callable_result_i0_site0_r0_expr0_spine0_stmt0_return import check_ret0_s0
from callable_result_i0_site0_r0_expr0_spine0_stmt0 import check_lcl0_s0
from callable_result_i0_site0_r0_expr0_spine0_if0 import check_if0_s0
from callable_result_i0_site0_r0_expr0_spine0_loop0 import check_loop0_s0a
from callable_result_i0_site0_r0_expr0_spine0_loop0_p0 import check_loop0_p0a
from callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0 import (
    check_loop0_p0b_o0_s0,
)
from callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_r0 import (
    check_loop0_p0b_o0_r0,
)
from callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_siteproj0 import (
    check_loop0_p0b_o0_siteproj0,
)
from callable_result_i0_site0_r0_expr0_spine0_suffix0 import (
    check_suffix0_i0,
    check_suffix0_s0,
)
from callable_result_i0_site0_r0_bodydomain0 import check_bodydomain0


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
    parity_tests_path = "src/mir/builder/ops/binary_expression_parity_tests.rs"
    raw_tests_path = "src/mir/builder/ops/binary_expression_raw_tests.rs"
    readme_path = "src/mir/builder/ops/README.md"
    ops_root_path = "src/mir/builder/ops/mod.rs"
    located_path = "src/mir/builder/located_legacy_lowering.rs"
    located_tests_path = (
        "src/mir/callable_result_representation/tests/located_legacy_lowering.rs"
    )
    short_circuit_path = "src/mir/builder/ops/short_circuit_expression_descent.rs"
    short_circuit_tests_path = (
        "src/mir/builder/ops/short_circuit_expression_descent_tests.rs"
    )
    short_circuit_raw_tests_path = (
        "src/mir/builder/ops/short_circuit_expression_raw_tests.rs"
    )
    short_circuit_parity_tests_path = (
        "src/mir/builder/ops/short_circuit_expression_parity_tests.rs"
    )
    located_short_circuit_tests_path = (
        "src/mir/callable_result_representation/tests/"
        "located_short_circuit_lowering.rs"
    )
    logical_owner_path = "src/mir/builder/ops/logical_shortcircuit.rs"
    module = read(root, module_path)
    tests = read(root, tests_path)
    parity_tests = read(root, parity_tests_path)
    raw_tests = read(root, raw_tests_path)
    readme = read(root, readme_path)
    ops_root = read(root, ops_root_path)
    located = read(root, located_path)
    located_tests = read(root, located_tests_path)
    short_circuit = read(root, short_circuit_path)
    short_circuit_tests = read(root, short_circuit_tests_path)
    short_circuit_raw_tests = read(root, short_circuit_raw_tests_path)
    short_circuit_parity_tests = read(root, short_circuit_parity_tests_path)
    located_short_circuit_tests = read(root, located_short_circuit_tests_path)
    logical_owner = read(root, logical_owner_path)

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
    require_count(
        located,
        "impl<'plan> BinaryExpressionDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan>",
        1,
        "BIN0-L0 located implementation",
    )
    require_count(
        located,
        "ExprChildRoleV1::BinaryLeft",
        2,
        "PATH0 BinaryLeft ordinary/short-circuit consumers",
    )
    require_count(
        located,
        "ExprChildRoleV1::BinaryRight",
        2,
        "PATH0 BinaryRight ordinary/short-circuit consumers",
    )
    require_count(
        located,
        "drive_ordinary_binary_expression_v1(builder, self, &input)",
        1,
        "located ordinary Binary selector",
    )
    if "RowsUnderPrefix" in located:
        fail("located Binary must not select a route from RowsUnderPrefix")
    binary_selector_at = located.index("if let ASTNode::BinaryOp { operator, .. } = input.node()")
    inactive_proof_at = located.index(".prove_expr_inactive(&input)")
    if binary_selector_at >= inactive_proof_at:
        fail("ordinary Binary must select located descent before whole-prefix proof")

    require_count(
        short_circuit,
        "trait ShortCircuitExpressionDescentPortV1",
        1,
        "short-circuit child-demand port owner",
    )
    require_count(
        short_circuit,
        "type ShortCircuitInput;",
        1,
        "associated short-circuit input",
    )
    require_count(
        short_circuit,
        "fn drive_short_circuit_expression_v1",
        1,
        "short-circuit associated-input driver",
    )
    require_count(
        short_circuit,
        "drive_legacy_expression_v1(",
        2,
        "short-circuit lhs/rhs E0 descent",
    )
    logical_reject_at = short_circuit.index(
        "if !matches!(operator, BinaryOperator::And | BinaryOperator::Or)"
    )
    left_input_at = short_circuit.index("port.short_circuit_left_input(input)?")
    left_descent_at = short_circuit.index(
        "drive_legacy_expression_v1(builder, port, left_input)?"
    )
    control_owner_at = short_circuit.index("build_logical_shortcircuit_after_lhs_v1(")
    right_input_at = short_circuit.index("port.short_circuit_right_input(input)?")
    right_descent_at = short_circuit.index(
        "drive_legacy_expression_v1(builder, port, right_input)"
    )
    if not (
        logical_reject_at
        < left_input_at
        < left_descent_at
        < control_owner_at
        < right_input_at
        < right_descent_at
    ):
        fail("SC0 order must be logical admit -> lhs -> control owner -> deferred rhs")
    require_count(
        logical_owner,
        "fn build_logical_shortcircuit_after_lhs_v1",
        1,
        "existing short-circuit control owner",
    )
    require_count(
        logical_owner,
        "let rhs_val = lower_rhs(builder)?;",
        1,
        "RHS closure invocation inside existing owner",
    )
    rhs_block_at = logical_owner.index("builder.start_new_block(eval_rhs_block)?")
    rhs_lower_at = logical_owner.index("let rhs_val = lower_rhs(builder)?;")
    if rhs_block_at >= rhs_lower_at:
        fail("RHS lowering must occur only after entering eval-RHS block")
    require_count(
        logical_owner,
        "fn build_logical_shortcircuit_pre_sc0_i0_reference_v1",
        1,
        "SC0-P0 pre-I0 raw reference",
    )
    if (
        "#[cfg(test)]\npub(in crate::mir::builder) fn "
        "build_logical_shortcircuit_pre_sc0_i0_reference_v1"
        not in logical_owner
    ):
        fail("SC0-P0 pre-I0 raw reference must remain cfg(test)")
    if "fn build_logical_shortcircuit(" in logical_owner:
        fail("retired production raw short-circuit facade must remain absent")
    require_count(
        short_circuit,
        "impl ShortCircuitExpressionDescentPortV1 for RawLegacyChildLoweringPortV1",
        1,
        "SC0-I0 raw short-circuit adapter",
    )
    require_count(
        ops_root,
        "short_circuit_expression_descent::drive_raw_short_circuit_expression_v1(",
        1,
        "SC0-I0 raw selector",
    )
    require_count(
        located,
        "ShortCircuitExpressionDescentPortV1 for LocatedLegacyLoweringSessionV1",
        1,
        "SC0-L0 located short-circuit adapter",
    )
    require_count(
        located,
        "drive_short_circuit_expression_v1(builder, self, &input)",
        1,
        "SC0-L0 located selector",
    )

    short_circuit_callers = 0
    short_circuit_ignored = {
        (root / short_circuit_path).resolve(),
        (root / short_circuit_tests_path).resolve(),
        (root / short_circuit_raw_tests_path).resolve(),
    }
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() in short_circuit_ignored:
            continue
        short_circuit_callers += path.read_text(encoding="utf-8").count(
            "drive_short_circuit_expression_v1("
        )
    if short_circuit_callers != 1:
        fail(
            "SC0-L0 must have exactly one disconnected located driver caller: "
            f"actual={short_circuit_callers}"
        )

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
    if production_callers != 1:
        fail(f"generic Binary driver located consumers: expected=1 actual={production_callers}")

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
        "mod binary_expression_parity_tests;",
        1,
        "BIN0-P0 parity fixture module",
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

    for fixture in (
        "ordinary_operator_matrix_has_exact_legacy_snapshot_parity",
        "method_call_on_each_side_has_exact_legacy_snapshot_parity",
        "nested_binary_depth_two_through_four_has_exact_legacy_snapshot_parity",
        "child_failures_and_reuse_have_exact_legacy_snapshot_parity",
    ):
        if fixture not in parity_tests:
            fail(f"missing BIN0-P0 fixture: {fixture}")

    for fixture in (
        "active_row_under_ordinary_binary_claims_through_located_child",
        "located_binary_claims_left_then_right_and_accepts_nested_ordinary_children",
        "located_binary_accepts_actual_if_condition_shape",
        "logical_binary_accepts_and_unlocated_binary_rejects_before_child_effects",
    ):
        if fixture not in located_tests:
            fail(f"missing BIN0-L0 fixture: {fixture}")

    for fixture in (
        "logical_driver_requests_left_before_rhs_and_rhs_only_in_eval_block",
        "and_and_or_share_the_existing_short_circuit_completion",
        "ordinary_operator_rejects_before_child_input_or_cfg_effects",
        "syntax_and_left_failures_stop_before_short_circuit_cfg",
        "rhs_input_and_lowering_fail_only_after_entering_eval_block",
        "failed_driver_does_not_poison_a_fresh_driver",
    ):
        if fixture not in short_circuit_tests:
            fail(f"missing SC0-S0 fixture: {fixture}")

    for fixture in (
        "raw_short_circuit_selector_preserves_and_or_completion",
        "raw_rhs_is_materialized_only_inside_the_eval_block",
        "raw_lhs_failure_stops_before_short_circuit_cfg",
        "raw_rhs_failure_occurs_after_entering_eval_block",
        "ordinary_binary_remains_on_bin0_after_short_circuit_cutover",
        "failed_raw_short_circuit_allows_a_fresh_builder",
    ):
        if fixture not in short_circuit_raw_tests:
            fail(f"missing SC0-I0 fixture: {fixture}")

    for fixture in (
        "and_or_bool_matrix_has_exact_pre_i0_snapshot_parity",
        "nested_and_or_comparison_tree_has_exact_pre_i0_snapshot_parity",
        "method_call_children_have_exact_pre_i0_snapshot_parity",
        "child_failures_and_reuse_have_exact_pre_i0_snapshot_parity",
    ):
        if fixture not in short_circuit_parity_tests:
            fail(f"missing SC0-P0 fixture: {fixture}")
    for snapshot_fact in (
        "blocks:",
        "value_types:",
        "value_kinds:",
        "value_origins:",
        "variable_map:",
        "pin_slots:",
        "current_block:",
        "next_value_id:",
        "recursion_depth:",
    ):
        if snapshot_fact not in short_circuit_parity_tests:
            fail(f"SC0-P0 snapshot misses {snapshot_fact}")

    for fixture in (
        "located_short_circuit_claims_left_before_deferred_right_inside_eval_block",
        "located_short_circuit_accepts_nested_and_or_comparison_tree",
        "located_short_circuit_accepts_actual_loop_condition_shape",
        "located_short_circuit_route_failure_poisons_only_that_session",
    ):
        if fixture not in located_short_circuit_tests:
            fail(f"missing SC0-L0 fixture: {fixture}")

    for snapshot_fact in (
        "blocks:",
        "value_types:",
        "value_kinds:",
        "value_origins:",
        "next_value_id:",
        "recursion_depth:",
    ):
        if snapshot_fact not in parity_tests:
            fail(f"BIN0-P0 snapshot misses fact: {snapshot_fact}")

    try:
        lcl_summary = check_lcl0_s0(root, located)
        asn_summary = check_asn0_s0(root)
        ret_summary = check_ret0_s0(root)
        if_summary = check_if0_s0(root)
        bodydomain_summary = check_bodydomain0(root)
        suffix_summary = check_suffix0_s0(root)
        suffix_parity_summary = check_suffix0_i0(root)
        loop0_summary = check_loop0_s0a(root)
        loop0_p0a_summary = check_loop0_p0a(root)
        loop0_p0b_o0_summary = check_loop0_p0b_o0_s0(root)
        loop0_p0b_o0_r0_summary = check_loop0_p0b_o0_r0(root)
        loop0_p0b_o0_siteproj0_summary = check_loop0_p0b_o0_siteproj0(root)
    except RuntimeError as error:
        fail(str(error))
    require_count(
        parity_tests,
        "fn lower_legacy_reference(",
        1,
        "test-only pre-I0 reference",
    )
    for path in (root / "src").rglob("*.rs"):
        if path.resolve() == (root / parity_tests_path).resolve():
            continue
        if "lower_legacy_reference(" in path.read_text(encoding="utf-8"):
            fail(f"test-only Binary reference escaped parity module: {path}")

    for phrase in (
        "child-demand boundary",
        "reject `And` and `Or` before child effects",
        "existing `build_binary_op_from_values` owner",
        "never stored in `MirBuilder`",
        "BIN0-I0 selects the ordinary raw source entry",
        "BIN0-L0 adds one disconnected located port",
        "never catches `RowsUnderPrefix`",
        "disconnected SC0-S0 child-demand",
        "one deferred RHS closure",
        "SC0-I0 adds one owned raw short-circuit input",
        "SC0-P0 retains the pre-I0 raw orchestration",
        "SC0-L0 implements the same port once",
        "production located callers remain zero",
    ):
        if phrase not in readme:
            fail(f"missing BIN0 README boundary: {phrase}")

    touched = (
        module_path,
        tests_path,
        parity_tests_path,
        raw_tests_path,
        readme_path,
        ops_root_path,
        located_path,
        located_tests_path,
        short_circuit_path,
        short_circuit_tests_path,
        short_circuit_raw_tests_path,
        short_circuit_parity_tests_path,
        located_short_circuit_tests_path,
        logical_owner_path,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_r0.py",
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_siteproj0.py",
    )
    oversized = [relative for relative in touched if len(read(root, relative).splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    if re.search(r"Arc<|Rc<|thread_local!|static mut", module):
        fail("BIN0 substrate must remain stack-scoped and immutable")

    print(
        f"{TAG} ok: driver=1 child_descents=2 raw_selector=1 "
        "raw_impl=1 parity_reference=1 located_impl=1 sc_driver=1 "
        "sc_raw_selector=1 sc_raw_impl=1 sc_located_impl=1 "
        "sc_parity_reference=1 logical_owner_preserved=1 "
        f"{lcl_summary} {asn_summary} {ret_summary} {if_summary} "
        f"{bodydomain_summary} {suffix_summary}"
        f" {suffix_parity_summary} {loop0_summary} {loop0_p0a_summary}"
        f" {loop0_p0b_o0_summary} {loop0_p0b_o0_r0_summary}"
        f" {loop0_p0b_o0_siteproj0_summary}"
    )


if __name__ == "__main__":
    main()
