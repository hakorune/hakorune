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
EXPRESSION_PORT = "src/mir/builder/control_flow/plan/expression_port.rs"
LOCATED_VIEW = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/lowering_view.rs"
)
LOCATED_VIEW_TESTS = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/lowering_view_tests.rs"
)
LOCATED_MOD = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/mod.rs"
)
IF_RAW = "src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan.rs"
IF_CORE = "src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan_port.rs"
IF_CORE_TESTS = (
    "src/mir/builder/control_flow/plan/normalizer/cond_lowering_if_plan_port_tests.rs"
)
PARTS_ASSOCIATED_SOURCE = (
    "src/mir/builder/control_flow/plan/parts/associated_source.rs"
)
PARTS_ASSOCIATED_SOURCE_TESTS = (
    "src/mir/builder/control_flow/plan/parts/associated_source_tests.rs"
)
PARTS_ASSOCIATED_DISPATCH = (
    "src/mir/builder/control_flow/plan/parts/associated_source/dispatch.rs"
)
PARTS_ASSOCIATED_DISPATCH_TESTS = (
    "src/mir/builder/control_flow/plan/parts/associated_source/dispatch_tests.rs"
)
PARTS_ASSOCIATED_RAW = (
    "src/mir/builder/control_flow/plan/parts/associated_source/raw_lowering.rs"
)
PARTS_ASSOCIATED_RAW_TESTS = (
    "src/mir/builder/control_flow/plan/parts/associated_source/raw_parity_tests.rs"
)
PARTS_ASSOCIATED_BLOCK_DRIVER = (
    "src/mir/builder/control_flow/plan/parts/associated_source/block_driver.rs"
)
PARTS_BLOCK = "src/mir/builder/control_flow/plan/parts/dispatch/block.rs"
PARTS_IF_EXIT_ONLY = (
    "src/mir/builder/control_flow/plan/parts/dispatch/if_exit_only.rs"
)
PARTS_MOD = "src/mir/builder/control_flow/plan/parts/mod.rs"


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


def _rust_code(text: str) -> str:
    """Remove standalone Rust comments before structural symbol scans."""
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return re.sub(r"(?m)^\s*//.*$", "", text)


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


def _guard_r0_v0_c0(root: Path) -> None:
    expression_port = _production(_read(root, EXPRESSION_PORT))
    located_view = _production(_read(root, LOCATED_VIEW))
    located_tests = _read(root, LOCATED_VIEW_TESTS)
    located_mod = _read(root, LOCATED_MOD)
    if_raw = _production(_read(root, IF_RAW))
    if_core = _production(_read(root, IF_CORE))
    if_tests = _read(root, IF_CORE_TESTS)
    normalizer_mod = _read(root, NORMALIZER_MOD)

    for carrier in ("LegacyExprInputV1", "LegacyStmtInputV1", "LegacyBodyInputV1"):
        _require(
            expression_port,
            f"BorrowedLocated(&'syntax {carrier}<'plan>)",
            1,
            f"R0-V0 borrowed {carrier}",
        )
    for constructor in ("borrowed_expr", "borrowed_stmt", "borrowed_body"):
        _require(expression_port, f"fn {constructor}<", 1, f"R0-V0 {constructor}")

    _require(located_view, "fn bind_lowering_port<", 1, "R0-V0 bind owner")
    bind = _function(located_view, "bind_lowering_port")
    _require(bind, "port.require_exact_stmt(&self.loop_root)?", 1, "R0-V0 exact root")
    _require(bind, "Ok(VerifiedLocatedGenericLoopLoweringViewV1", 1, "R0-V0 view publication")
    if bind.index("require_exact_stmt") >= bind.index("Ok(VerifiedLocatedGenericLoopLoweringViewV1"):
        raise RuntimeError("LOOP0-P0b-T0 R0-V0 publishes before exact root verification")
    for owner, count in (
        ("condition", 2),
        ("cleanup", 1),
        ("mode", 1),
        ("singleton_recipe", 1),
        ("singleton_root", 1),
    ):
        _require(located_view, f"fn {owner}(", count, f"R0-V0 {owner} view")
    for token in (
        "try_build_no_exit_block_recipe",
        "classify_step_placement",
        "matches_loop_increment",
        "std::env",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in located_view:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-V0 owns forbidden authority: {token}")
    if "#[derive(Clone" in located_view or "impl Clone" in located_view:
        raise RuntimeError("LOOP0-P0b-T0 R0-V0 bound proof must remain non-Clone")
    for name in (
        "bound_view_borrows_default_prefix_condition_and_cleanup",
        "bound_view_retains_strict_items_and_wrapped_join_product",
        "foreign_port_rejects_before_a_bound_view_is_published",
    ):
        _fixture(located_tests, name)
    _require(located_mod, "mod lowering_view;", 1, "R0-V0 module")
    _require(located_mod, "mod lowering_view_tests;", 1, "R0-V0 tests module")

    _require(if_core, "fn lower_cond_expr_to_if_plans_input<", 1, "R0-C0 associated owner")
    for role in (
        "ExprChildRoleV1::UnaryOperand",
        "ExprChildRoleV1::BinaryLeft",
        "ExprChildRoleV1::BinaryRight",
    ):
        if role not in if_core:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-C0 misses PATH0 role: {role}")
    if "lower_cond_value_input(" not in if_core:
        raise RuntimeError("LOOP0-P0b-T0 R0-C0 misses associated leaf owner")
    if_core_owner = _function(if_core, "lower_cond_expr_to_if_plans_input")
    if "CondBlockView" in if_core_owner:
        raise RuntimeError("LOOP0-P0b-T0 R0-C0 core must not own raw CondBlockView")
    for owner in ("lower_cond_to_if_plans", "lower_cond_to_if_plans_with_plan_prelude"):
        body = _function(if_raw, owner)
        _require(body, "RawLoopPlanExpressionPortV1::new()", 1, f"R0-C0 raw port {owner}")
        _require(body, "lower_cond_expr_to_if_plans_input(", 1, f"R0-C0 delegation {owner}")
    for name in (
        "raw_if_condition_facade_matches_explicit_raw_port_core",
        "raw_join_bearing_and_or_facade_matches_explicit_port_core",
        "borrowed_located_loop_condition_preserves_exact_call_sites",
    ):
        _fixture(if_tests, name)
    _require(normalizer_mod, "mod cond_lowering_if_plan_port_tests;", 1, "R0-C0 tests module")


def _guard_r0_d0_s0(root: Path) -> None:
    source = _production(_read(root, PARTS_ASSOCIATED_SOURCE))
    tests = _read(root, PARTS_ASSOCIATED_SOURCE_TESTS)
    parts_mod = _read(root, PARTS_MOD)
    located_view = _production(_read(root, LOCATED_VIEW))

    _require(source, "trait PartsAssociatedSourceV1", 1, "R0-D0 source provider owner")
    _require(source, "enum PartsAssociatedRecipeItemV1", 1, "R0-D0 item vocabulary")
    _require(source, "PartsAssociatedSourceV1: sealed::Sealed", 1, "R0-D0 sealed provider")
    _require(source, "impl sealed::Sealed for", 2, "R0-D0 sealed implementations")
    _require(source, "PartsAssociatedSourceV1 for", 2, "R0-D0 provider implementations")
    _require(
        source,
        "struct VerifiedPartsAssociatedItemV1",
        1,
        "R0-D0 port/item product",
    )
    _require(source, "\n    port: PortHandle,", 1, "R0-D0 exact port retention")
    _require(source, "ForeignRawBlock", 2, "R0-D0 raw pairing rejection")
    _require(source, "ForeignLocatedBlock", 2, "R0-D0 located pairing rejection")
    _require(source, "type BodyInput = &'source [ASTNode]", 1, "R0-D0 raw body port shape")
    _require(
        located_view,
        "fn expression_port(",
        1,
        "R0-D0 bound located port projection",
    )

    for token in (
        "MirBuilder",
        "CorePlan",
        "LoweredRecipe",
        "try_build_no_exit_block_recipe",
        "classify_step_placement",
        "matches_loop_increment",
        "std::env",
        "facts.body",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in source:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-D0-S0 owns forbidden authority: {token}")

    for name in (
        "raw_provider_projects_stmt_exit_and_explicit_if_without_lowering",
        "located_provider_projects_actual_strict_root_and_retained_join_bridge",
        "raw_provider_rejects_a_block_issued_by_a_foreign_arena",
        "located_provider_rejects_a_block_bound_to_a_foreign_port",
    ):
        _fixture(tests, name)
    _require(parts_mod, "mod associated_source;", 1, "R0-D0 source module")
    _require(parts_mod, "mod associated_source_tests;", 1, "R0-D0 source tests module")

    located_consumers = []
    for path in (root / "src/mir/builder/control_flow/plan").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if relative in (PARTS_ASSOCIATED_SOURCE, PARTS_ASSOCIATED_DISPATCH) or _is_test_source(path):
            continue
        text = _production(path.read_text(encoding="utf-8"))
        count = text.count("LocatedPartsAssociatedSourceV1::new")
        if count:
            located_consumers.append(f"{relative}:{count}")
    if located_consumers:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-S0 premature located consumers: "
            f"{located_consumers}"
        )


def _guard_r0_d0_dispatch0_s0(root: Path) -> None:
    dispatch = _production(_read(root, PARTS_ASSOCIATED_DISPATCH))
    tests = _read(root, PARTS_ASSOCIATED_DISPATCH_TESTS)
    source = _read(root, PARTS_ASSOCIATED_SOURCE)

    _require(
        dispatch,
        "fn lower_verified_parts_associated_item<",
        1,
        "R0-D0 sole item dispatcher",
    )
    owner = _function(dispatch, "lower_verified_parts_associated_item")
    signature = owner[: owner.index("{")]
    if "verified: VerifiedPartsAssociatedItemV1<" not in signature or "verified: &" in signature:
        raise RuntimeError("LOOP0-P0b-T0 R0-D0 dispatcher must consume the verified pair by value")
    _require(owner, "match (mode, item)", 1, "R0-D0 sole acceptance match")
    for variant in (
        "OpaqueStmt",
        "OpaqueExit",
        "ExplicitIfV2",
        "StmtWrappedJoinIf",
        "RawLoopV0",
    ):
        if f"PartsAssociatedRecipeItemV1::{variant}" not in owner:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-D0 dispatcher misses variant: {variant}")
    for mode in ("ExitOnly", "ExitAllowed", "StmtOnly", "NoExit"):
        if f"PartsAssociatedBlockModeV1::{mode}" not in owner:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-D0 dispatcher misses mode: {mode}")
    for if_mode in ("ExitIf", "ExitAll", "ThenOnlyExit", "ElseOnlyExit"):
        if f"IfMode::{if_mode}" not in owner:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-D0 dispatcher misses If mode: {if_mode}")
    for token in (
        "MirBuilder",
        "CorePlan",
        "LoweredRecipe",
        "RecipeItem::",
        "try_build_no_exit_block_recipe",
        "CondBlockView::from_expr",
        "ASTNode::If",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in dispatch:
            raise RuntimeError(f"LOOP0-P0b-T0 R0-D0 dispatcher owns forbidden authority: {token}")
    for name in (
        "sole_dispatcher_accepts_the_existing_block_mode_matrix",
        "sole_dispatcher_rejects_cross_mode_items_without_invoking_hooks",
        "invalid_if_contract_modes_reject_without_invoking_hooks",
    ):
        _fixture(tests, name)
    _require(source, "pub(super) mod dispatch;", 1, "R0-D0 dispatcher module")
    _require(source, "mod dispatch_tests;", 1, "R0-D0 dispatcher tests module")

    # RAW0 below owns the exact production consumer counts. This checkpoint
    # continues to guard the dispatcher vocabulary and admission law.


def _guard_r0_d0_raw0(root: Path) -> None:
    raw = _rust_code(_production(_read(root, PARTS_ASSOCIATED_RAW)))
    raw_tests = _read(root, PARTS_ASSOCIATED_RAW_TESTS)
    source_all = _read(root, PARTS_ASSOCIATED_SOURCE)
    source = _production(source_all)
    old_block = _rust_code(_production(_read(root, PARTS_BLOCK)))
    old_if_exit = _rust_code(_production(_read(root, PARTS_IF_EXIT_ONLY)))

    hook_impls = len(
        re.findall(
            r"impl(?:\s*<[^>]*>)?\s+PartsAssociatedLoweringHooksV1\s*<\s*"
            r"RawPartsAssociatedSourceV1",
            raw,
        )
    )
    if hook_impls != 1:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 raw hook impl: "
            f"expected=1 actual={hook_impls}"
        )
    _require(
        raw,
        "struct RawPartsAssociatedLoweringHooksV1",
        1,
        "R0-D0-RAW0 raw hook type owner",
    )
    production_root = root / "src/mir/builder/control_flow/plan"
    located_consumers = []
    dispatcher_calls = []
    raw_provider_calls = []
    raw_hook_impls = []
    for path in production_root.rglob("*.rs"):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        text = _rust_code(_production(path.read_text(encoding="utf-8")))
        located = text.count("LocatedPartsAssociatedSourceV1::new(")
        if located:
            located_consumers.append(f"{relative}:{located}")
        calls = len(
            re.findall(
                r"\blower_verified_parts_associated_item\s*(?:::<|\()",
                text,
            )
        )
        if relative == PARTS_ASSOCIATED_DISPATCH:
            calls = 0
        if calls:
            dispatcher_calls.append(f"{relative}:{calls}")
        providers = text.count("RawPartsAssociatedSourceV1::new(")
        if providers:
            raw_provider_calls.append(f"{relative}:{providers}")
        impls = len(
            re.findall(
                r"impl(?:\s*<[^>]*>)?\s+PartsAssociatedLoweringHooksV1\s*<\s*"
                r"RawPartsAssociatedSourceV1",
                text,
            )
        )
        if impls:
            raw_hook_impls.append(f"{relative}:{impls}")
    if located_consumers:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 premature located consumers: "
            f"{located_consumers}"
        )
    if dispatcher_calls != [f"{PARTS_ASSOCIATED_BLOCK_DRIVER}:1"]:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 dispatcher consumers drift: "
            f"{dispatcher_calls}"
        )
    if raw_provider_calls != [f"{PARTS_BLOCK}:1"]:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 raw provider consumers drift: "
            f"{raw_provider_calls}"
        )
    if raw_hook_impls != [f"{PARTS_ASSOCIATED_RAW}:1"]:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 raw hook owners drift: "
            f"{raw_hook_impls}"
        )

    for relative, text in ((PARTS_BLOCK, old_block), (PARTS_IF_EXIT_ONLY, old_if_exit)):
        if "RecipeItem::" in text:
            raise RuntimeError(
                f"LOOP0-P0b-T0 R0-D0-RAW0 legacy RecipeItem lowering remains: {relative}"
            )
    legacy_owner_defs = []
    for path in production_root.rglob("*.rs"):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        text = _rust_code(_production(path.read_text(encoding="utf-8")))
        count = len(re.findall(r"\bfn\s+lower_exit_only_item(?:\s*<|\s*\()", text))
        if count:
            legacy_owner_defs.append(f"{relative}:{count}")
    if legacy_owner_defs:
        raise RuntimeError(
            "LOOP0-P0b-T0 R0-D0-RAW0 legacy lowering owners remain: "
            f"{legacy_owner_defs}"
        )

    forbidden = (
        "LocatedPartsAssociatedSourceV1",
        "VerifiedLocatedGenericLoop",
        "try_build_no_exit_block_recipe",
        "classify_step_placement",
        "matches_loop_increment",
        "facts.body",
        "body_no_exit",
        "std::env",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
        "AST equality",
    )
    for relative, text in ((PARTS_ASSOCIATED_RAW, raw), (PARTS_BLOCK, old_block)):
        for token in forbidden:
            if token in text:
                raise RuntimeError(
                    "LOOP0-P0b-T0 R0-D0-RAW0 owns forbidden authority: "
                    f"{relative}:{token}"
                )
    if "pub(super) mod raw_lowering;" not in source_all:
        raise RuntimeError("LOOP0-P0b-T0 R0-D0-RAW0 raw module is not registered")
    if "mod raw_parity_tests;" not in source_all:
        raise RuntimeError("LOOP0-P0b-T0 R0-D0-RAW0 raw tests are not registered")
    for name in (
        "raw_exit_only_facade_matches_associated_block_driver",
        "raw_exit_allowed_facade_matches_associated_block_driver",
        "raw_no_exit_join_facade_matches_associated_block_driver",
        "raw_stmt_only_facade_matches_associated_block_driver_and_golden_state",
    ):
        _fixture(raw_tests, name)


def _guard_no_premature_located_consumer(root: Path) -> None:
    callers = []
    for path in (root / "src/mir/builder/control_flow/plan").rglob("*.rs"):
        if _is_test_source(path):
            continue
        text = _production(path.read_text(encoding="utf-8"))
        verify_calls = text.count("verify_located_loop(")
        if verify_calls and path.name == "mod.rs" and "located_representation" in path.parts:
            verify_calls -= 1
        bind_calls = text.count("bind_lowering_port(")
        if bind_calls and path.name == "lowering_view.rs":
            bind_calls -= 1
        if verify_calls or bind_calls:
            callers.append(
                f"{path.relative_to(root)}:verify={verify_calls}:bind={bind_calls}"
            )
    if callers:
        raise RuntimeError(f"LOOP0-P0b-T0 premature located body consumers: {callers}")


def check_loop0_p0b_t0(root: Path) -> str:
    _guard_c0(root)
    _guard_b0(root)
    _guard_r0_v0_c0(root)
    _guard_r0_d0_s0(root)
    _guard_r0_d0_dispatch0_s0(root)
    _guard_r0_d0_raw0(root)
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
        EXPRESSION_PORT,
        LOCATED_VIEW,
        LOCATED_VIEW_TESTS,
        LOCATED_MOD,
        IF_RAW,
        IF_CORE,
        IF_CORE_TESTS,
        PARTS_ASSOCIATED_SOURCE,
        PARTS_ASSOCIATED_SOURCE_TESTS,
        PARTS_ASSOCIATED_DISPATCH,
        PARTS_ASSOCIATED_DISPATCH_TESTS,
        PARTS_ASSOCIATED_RAW,
        PARTS_ASSOCIATED_RAW_TESTS,
        PARTS_BLOCK,
        PARTS_IF_EXIT_ONLY,
        PARTS_MOD,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0.py",
    )
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-T0 source/check files reached 800 lines: {oversized}")
    return "loop0_p0b_t0_c0=1 loop0_p0b_t0_b0=1 r0_v0=1 r0_c0=1 r0_d0_s0=1 r0_d0_dispatch0_s0=1 r0_d0_raw0=1 located_consumers=0"
