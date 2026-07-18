#!/usr/bin/env python3
"""Structural guard for LOOP0-P0b-T0 C0/B0 associated-input prerequisites."""

from __future__ import annotations

import re
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import (
    _is_test_source,
    _matching_rust_brace,
    _production,
    _read,
)


C0_RAW = "src/mir/builder/control_flow/plan/normalizer/cond_lowering_loop_header.rs"
C0_CORE = (
    "src/mir/builder/control_flow/plan/normalizer/cond_lowering_loop_header_port.rs"
)
C0_TESTS = (
    "src/mir/builder/control_flow/plan/normalizer/cond_lowering_loop_header_port_tests.rs"
)
COMPARE = "src/mir/builder/control_flow/plan/normalizer/helpers.rs"
DIRECT = "src/mir/builder/control_flow/plan/features/generic_loop_body/direct_port.rs"
V1 = "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs"
CLEANUP = "src/mir/builder/control_flow/plan/features/generic_loop_body/cleanup.rs"
ASSOCIATED = (
    "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering_associated_input.rs"
)
RAW_STMTS = "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs"
ASSOCIATED_TESTS = (
    "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering_associated_input_tests.rs"
)
NORMALIZER_MOD = "src/mir/builder/control_flow/plan/normalizer/mod.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-P0b-T0 {label}: expected={expected} actual={actual}")


def _function(text: str, name: str) -> str:
    match = re.search(rf"\bfn\s+{re.escape(name)}(?:\s*<|\s*\()", text)
    if match is None:
        raise RuntimeError(f"LOOP0-P0b-T0 missing function: {name}")
    opening = text.find("{", match.end())
    if opening < 0:
        raise RuntimeError(f"LOOP0-P0b-T0 missing function body: {name}")
    return text[match.start() : _matching_rust_brace(text, opening)]


def _fixture(text: str, name: str) -> None:
    _require(text, f"fn {name}(", 1, f"focused fixture {name}")
    if re.search(rf"#\[(?:ignore|should_panic)[^\]]*\]\s*[^\n]*fn\s+{name}\b", text):
        raise RuntimeError(f"LOOP0-P0b-T0 focused fixture disabled: {name}")


def _guard_c0(root: Path) -> None:
    raw = _production(_read(root, C0_RAW))
    core = _production(_read(root, C0_CORE))
    compare = _production(_read(root, COMPARE))
    tests = _read(root, C0_TESTS)

    _require(raw, "fn lower_loop_header_cond(", 1, "C0 raw facade owner")
    _require(core, "fn lower_loop_header_cond_input<", 1, "C0 associated owner")
    raw_body = _function(raw, "lower_loop_header_cond")
    _require(raw_body, "RawLoopPlanExpressionPortV1::new()", 1, "C0 raw port selection")
    _require(raw_body, "lower_loop_header_cond_input(", 1, "C0 raw delegation")
    _require(core, "lower_loop_header_cond_input(", 5, "C0 recursive owner calls")
    _require(core, "PlanNormalizer::lower_compare_input(", 1, "C0 compare leaf")
    _require(core, "PlanNormalizer::lower_value_input(", 1, "C0 value leaf")

    _require(compare, "fn lower_compare_ast(", 1, "C0 compare raw facade")
    _require(compare, "fn lower_compare_input<", 1, "C0 compare associated owner")
    compare_raw = _function(compare, "lower_compare_ast")
    _require(compare_raw, "RawLoopPlanExpressionPortV1::new()", 1, "compare raw port")
    _require(compare_raw, "Self::lower_compare_input(", 1, "compare raw delegation")
    compare_core = _function(compare, "lower_compare_input")
    _require(compare_core, "ExprChildRoleV1::BinaryLeft", 1, "compare left PATH0")
    _require(compare_core, "ExprChildRoleV1::BinaryRight", 1, "compare right PATH0")
    _require(compare_core, "Self::lower_value_input(", 2, "compare child lowering")
    _require(compare_core, "set_current_span(ast.span())", 2, "compare span restore")
    _fixture(tests, "raw_loop_header_facade_matches_explicit_raw_port_core")


def _guard_b0(root: Path) -> None:
    direct = _read(root, DIRECT)
    direct_prod = _production(direct)
    v1 = _production(_read(root, V1))
    cleanup = _read(root, CLEANUP)
    cleanup_prod = _production(cleanup)
    associated = _production(_read(root, ASSOCIATED))
    raw_stmts = _production(_read(root, RAW_STMTS))
    associated_tests = _read(root, ASSOCIATED_TESTS)
    normalizer_mod = _read(root, NORMALIZER_MOD)

    _require(direct_prod, "fn lower_generic_loop_v1_direct_inputs<", 1, "B0 sequence owner")
    direct_core = _function(direct_prod, "lower_generic_loop_v1_direct_inputs")
    for token in ("for statement in statements", "lower_statement(", "body_plans_exit_on_all_paths"):
        if token not in direct_core:
            raise RuntimeError(f"LOOP0-P0b-T0 direct sequence misses: {token}")
    if direct_core.index("lower_statement(") >= direct_core.index("body_plans_exit_on_all_paths"):
        raise RuntimeError("LOOP0-P0b-T0 direct sequence terminal check precedes lowering")

    _require(v1, "fn lower_direct_raw_body(", 1, "B0 raw direct facade")
    direct_raw = _function(v1, "lower_direct_raw_body")
    _require(direct_raw, "RawLoopPlanExpressionPortV1::new()", 1, "B0 raw port")
    _require(direct_raw, "matches_loop_increment(", 1, "B0 raw-only step filter")
    _require(direct_raw, "lower_generic_loop_v1_direct_inputs(", 1, "B0 direct delegation")
    body_router = _function(v1, "lower_generic_loop_v1_body")
    _require(body_router, "lower_direct_raw_body(", 2, "B0 raw policy branches")
    _require(body_router, "apply_generic_loop_v1_fallthrough_cleanup(", 1, "B0 cleanup facade")

    _require(cleanup_prod, "fn apply_generic_loop_v1_fallthrough_cleanup(", 1, "cleanup raw facade")
    _require(cleanup_prod, "fn apply_generic_loop_v1_fallthrough_cleanup_input<", 1, "cleanup associated owner")
    cleanup_raw = _function(cleanup_prod, "apply_generic_loop_v1_fallthrough_cleanup")
    _require(cleanup_raw, "RawLoopPlanExpressionPortV1::new()", 1, "cleanup raw port")
    _require(cleanup_raw, "apply_generic_loop_v1_fallthrough_cleanup_input(", 1, "cleanup delegation")
    cleanup_core = _function(cleanup_prod, "apply_generic_loop_v1_fallthrough_cleanup_input")
    terminal_at = cleanup_core.index("body_plans_exit_on_all_paths")
    lower_at = cleanup_core.index("PlanNormalizer::lower_value_input(")
    if terminal_at >= lower_at:
        raise RuntimeError("LOOP0-P0b-T0 cleanup must stop before child lowering")

    owners = (
        "lower_assignment_inputs",
        "lower_local_initializer_inputs",
        "lower_local_statement_input",
        "lower_method_call_statement_input",
        "lower_function_call_statement_input",
        "lower_return_statement_input",
    )
    for owner in owners:
        _require(associated, f"fn {owner}", 1, f"associated primitive {owner}")
    _require(
        associated,
        "let source = port.call_source(&input)",
        2,
        "associated call-source producers",
    )
    _require(associated, "source,", 7, "associated call-source forwardings")
    _require(
        associated,
        "source: CoreCallSourceV1::Unlocated",
        1,
        "synthetic index-set source",
    )
    function_call = _function(associated, "lower_function_call_statement_input")
    _require(
        function_call,
        "ExprChildRoleV1::CallArgument(0)",
        1,
        "extern target PATH0 child",
    )
    _require(
        function_call,
        "port.expr_syntax(&target)",
        1,
        "extern target syntax projection",
    )
    for role in (
        "ExprChildRoleV1::Receiver",
        "ExprChildRoleV1::IndexTarget",
        "ExprChildRoleV1::IndexSubscript",
        "ExprChildRoleV1::LocalInitializer",
        "ExprChildRoleV1::CallArgument",
        "ExprChildRoleV1::ReturnValue",
    ):
        if role not in associated:
            raise RuntimeError(f"LOOP0-P0b-T0 associated primitives miss PATH0 role: {role}")

    facade_pairs = (
        ("lower_assignment_stmt", "lower_assignment_inputs"),
        ("lower_local_init_values", "lower_local_initializer_inputs"),
        ("lower_method_call_stmt", "lower_method_call_statement_input"),
        ("lower_function_call_stmt", "lower_function_call_statement_input"),
    )
    for facade, owner in facade_pairs:
        body = _function(raw_stmts, facade)
        _require(body, "RawLoopPlanExpressionPortV1::new()", 1, f"raw port {facade}")
        _require(body, f"loop_body_lowering_associated_input::{owner}(", 1, f"raw delegation {facade}")

    forbidden = (
        "std::env",
        "joinir_dev",
        "facts.body",
        "body_no_exit",
        "matches_loop_increment",
        "classify_step_placement",
        "try_build_no_exit_block_recipe",
        "VerifiedLocatedGenericLoopBodyRepresentationV1",
        "StmtWrappedJoinIf",
        "RecipeItem",
        "LocatedLoopPlanExpressionPortV1",
        "ledger",
        "claim_batch",
        "PlanLowerer",
        "fallback",
        "retry",
    )
    for relative, text in ((DIRECT, direct_prod), (CLEANUP, cleanup_prod), (ASSOCIATED, associated)):
        for token in forbidden:
            if token in text:
                raise RuntimeError(f"LOOP0-P0b-T0 {relative} owns forbidden authority: {token}")

    for name in (
        "generic_loop_v1_direct_port_preserves_order_and_stops_after_terminal",
        "generic_loop_v1_direct_port_failure_stops_before_later_inputs",
        "generic_loop_v1_direct_port_accepts_empty_prefix",
    ):
        _fixture(direct, name)
    for name in (
        "generic_loop_v1_cleanup_appends_fallthrough_continue",
        "generic_loop_v1_cleanup_raw_facade_matches_explicit_raw_port_core",
        "generic_loop_v1_cleanup_skips_when_body_already_exits",
        "generic_loop_v1_cleanup_respects_nested_terminality",
    ):
        _fixture(cleanup, name)
    for name in (
        "raw_function_call_statement_facade_matches_associated_input",
        "raw_assignment_and_local_facades_match_associated_inputs",
        "raw_method_statement_and_associated_return_preserve_statement_semantics",
    ):
        _fixture(associated_tests, name)
    _require(normalizer_mod, "mod loop_body_lowering_associated_input;", 1, "associated module")
    _require(normalizer_mod, "mod loop_body_lowering_associated_input_tests;", 1, "associated tests module")


def _guard_no_premature_located_consumer(root: Path) -> None:
    callers = []
    for path in (root / "src/mir/builder/control_flow/plan").rglob("*.rs"):
        if _is_test_source(path):
            continue
        text = _production(path.read_text(encoding="utf-8"))
        occurrences = text.count("verify_located_loop(")
        if occurrences and path.name == "mod.rs" and "located_representation" in path.parts:
            occurrences -= 1
        if occurrences:
            callers.append(f"{path.relative_to(root)}:{occurrences}")
    if callers:
        raise RuntimeError(f"LOOP0-P0b-T0 premature located body consumers: {callers}")


def check_loop0_p0b_t0(root: Path) -> str:
    _guard_c0(root)
    _guard_b0(root)
    _guard_no_premature_located_consumer(root)

    touched = (
        C0_RAW,
        C0_CORE,
        C0_TESTS,
        COMPARE,
        DIRECT,
        V1,
        CLEANUP,
        ASSOCIATED,
        RAW_STMTS,
        ASSOCIATED_TESTS,
        NORMALIZER_MOD,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0.py",
    )
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-T0 source/check files reached 800 lines: {oversized}")
    return "loop0_p0b_t0_c0=1 loop0_p0b_t0_b0=1 located_consumers=0"
