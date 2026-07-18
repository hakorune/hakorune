#!/usr/bin/env python3
"""Closeout guard for LOOP0-P0b-O0 actual default/strict proof."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


BASE = "src/mir/builder/control_flow/plan/generic_loop/located_representation"
MODULE = f"{BASE}/mod.rs"
ACTUAL = f"{BASE}/actual_parser_tests.rs"
PORT = "src/mir/builder/control_flow/plan/expression_port.rs"
PORT_TESTS = "src/mir/builder/control_flow/plan/expression_port_tests.rs"
LEGACY = "src/mir/callable_result_representation/located_legacy.rs"
EXTRACT = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"
FACTS = "src/mir/builder/control_flow/plan/generic_loop/facts_types.rs"


def _count(text: str, token: str, expected: int, label: str) -> None:
    actual = text.count(token)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-O0-G0 {label} drift: expected={expected} actual={actual}"
        )


def _require(text: str, token: str, label: str) -> None:
    if token not in text:
        raise RuntimeError(f"LOOP0-P0b-O0-G0 {label} missing: {token}")


def check_loop0_p0b_o0_g0(root: Path) -> str:
    module = _read(root, MODULE)
    actual = _read(root, ACTUAL)

    _count(module, "mod actual_parser_tests;", 1, "actual proof registration")
    _count(
        actual,
        "fn actual_parser_loop_seals_default_and_strict_representations()",
        1,
        "focused actual proof owner",
    )
    if "#[ignore]" in actual:
        raise RuntimeError("LOOP0-P0b-O0-G0 actual proof must not be ignored")

    for token, expected, label in (
        ("with_default_and_strict_modes(", 1, "shared process mode lock"),
        ("actual_parser_add_fixture::plan()", 1, "shared actual plan"),
        ("actual_parser_add_fixture::caller(&plan)", 1, "shared actual caller"),
        ("actual_parser_add_fixture::selected_static_sites()[1]", 1, "cleanup site"),
        ("body_stmt(&root_body, 4)", 1, "actual Loop root"),
        ("verify_located_loop(&port, loop_root)", 1, "same-call located seal"),
        ("canonical_body_len: 6", 1, "canonical body length"),
        ("SourcePathSegmentV1::LoopCondition", 1, "root condition path"),
        ("BodyLoweringPolicy::RecipeOnly", 1, "default policy"),
        ("VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly", 1, "default mode"),
        ("prefix.len(), 5", 1, "exact direct prefix cardinality"),
        ("BodyLoweringPolicy::ExitAllowed", 1, "strict policy"),
        ("VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe", 1, "strict mode"),
        ("root.items.len(), 5", 1, "strict root cardinality"),
        ("VerifiedLocatedRecipeItemV1::ExplicitIfV2", 1, "ordinal 2 IfV2"),
        ("IfContractKind::ExitOnly", 1, "ordinal 2 ExitOnly"),
        ("VerifiedLocatedRecipeItemV1::StmtWrappedJoinIf", 1, "ordinal 4 bridge"),
        ("contract: IfContractKind::Join", 1, "singleton Join"),
        ("bridge.singleton_recipe.block.items.len(), 1", 1, "singleton root cardinality"),
        ("bridge.singleton_root.then_block.items.len(), 1", 1, "singleton then cardinality"),
    ):
        _count(actual, token, expected, label)

    for token, label in (
        ("SourcePathSegmentV1::LoopBody(2)", "ordinal 2 path"),
        ("SourcePathSegmentV1::LoopBody(4)", "ordinal 4 path"),
        ("SourcePathSegmentV1::IfThen(0)", "exact then carriers"),
        ("SourcePathSegmentV1::IfElse(0)", "exact else carrier"),
        ("ExprChildRoleV1::ReturnValue", "ordinal 2 Return value"),
        ("ExprChildRoleV1::AssignmentValue", "wrapped Join values"),
        ("singleton Join retains sealed else block", "singleton else cardinality"),
    ):
        _require(actual, token, label)

    for forbidden in (
        "MirBuilder",
        "composer",
        "skeleton",
        "ledger",
        "claim(",
        "fallback",
        "retry",
        "std::env",
        "ScopedTestConfig",
        "try_build_no_exit_block_recipe",
        "std::ptr::eq",
    ):
        if forbidden in actual:
            raise RuntimeError(f"LOOP0-P0b-O0-G0 proof authority leak: {forbidden}")

    line_cap_paths = [
        path.relative_to(root).as_posix()
        for path in (root / BASE).rglob("*.rs")
    ]
    line_cap_paths.extend(
        (
            PORT,
            PORT_TESTS,
            LEGACY,
            EXTRACT,
            FACTS,
            "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py",
            "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0.py",
            "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_r0.py",
            "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0_siteproj0.py",
            __file__,
        )
    )
    oversized = []
    for path in line_cap_paths:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-O0-G0 files reached 800 lines: {oversized}")

    return (
        "loop0_p0b_o0_g0=green actual_fixture=1 mode_locks=1 "
        "default=DirectRecipeOnly strict=ExitAllowedRecipe "
        "builder=0 composer=0 ledger=0 production_located_roots=0"
    )
