#!/usr/bin/env python3
"""Private HOOK0 structural guard for the located Parts preflight/adapter."""

from __future__ import annotations

import re
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import (
    _is_test_source,
    _production,
    _read,
)


PREFLIGHT = (
    "src/mir/builder/control_flow/plan/parts/associated_source/located_preflight.rs"
)
LOWERING = (
    "src/mir/builder/control_flow/plan/parts/associated_source/located_lowering.rs"
)
TESTS = (
    "src/mir/builder/control_flow/plan/parts/associated_source/located_hook_tests.rs"
)
SOURCE = "src/mir/builder/control_flow/plan/parts/associated_source.rs"
LOWERING_VIEW = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/lowering_view.rs"
)
ALLOWED_LOCATED_PROVIDER_CONSUMERS = frozenset((PREFLIGHT, LOWERING))


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-T0 HOOK0 {label}: expected={expected} actual={actual}"
        )


def check_loop0_p0b_t0_hook0(root: Path) -> str:
    preflight = _production(_read(root, PREFLIGHT))
    lowering = _production(_read(root, LOWERING))
    tests = _read(root, TESTS)
    source = _read(root, SOURCE)
    lowering_view = _production(_read(root, LOWERING_VIEW))

    _require(
        preflight,
        "struct VerifiedLocatedGenericLoopPartsPreflightV1<",
        1,
        "preflight product",
    )
    _require(
        preflight,
        "fn verify(",
        1,
        "preflight constructor",
    )
    _require(
        preflight,
        "fn lower_with_parts_adapter(",
        1,
        "single-use adapter consumer",
    )
    _require(preflight, "LocatedPartsAssociatedSourceV1::new(", 1, "preflight provider")
    _require(preflight, "verify_local(&provider, root, 0)", 1, "root local zero")
    _require(preflight, "verify_local(&provider, root, 1)", 1, "root local one")
    _require(preflight, "verify_exit_if(&provider, root, 2)", 1, "root exit If")
    _require(preflight, "verify_local(&provider, root, 3)", 1, "root local three")
    _require(
        preflight,
        "verify_wrapped_join(&provider, root, 4, &mut carrier_targets)",
        1,
        "root Join with retained targets",
    )
    _require(
        preflight,
        "ExprChildRoleV1::AssignmentTarget",
        1,
        "exact Join assignment target carrier",
    )
    _require(
        preflight,
        "ASTNode::Variable { name, .. }",
        1,
        "variable-only Join assignment target",
    )
    for token in (
        "MirBuilder",
        "CondBlockView",
        "RecipeBodies",
        "try_build_no_exit_block_recipe",
        "AST equality",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in preflight:
            raise RuntimeError(f"LOOP0-P0b-T0 HOOK0 preflight owns forbidden token: {token}")
    if re.search(
        r"#\[derive\([^\]]*(?:Clone|Copy)[^\]]*\)\]\s*"
        r"pub[^\n]*struct\s+VerifiedLocatedGenericLoopPartsPreflightV1",
        preflight,
        flags=re.DOTALL,
    ):
        raise RuntimeError("LOOP0-P0b-T0 HOOK0 preflight product became Clone/Copy")

    _require(
        lowering,
        "fn lower_preflighted_located_parts_root_v1<",
        1,
        "disconnected lowering entry",
    )
    _require(
        lowering,
        "preflight.into_execution().lower_with_parts_adapter(",
        1,
        "preflight consumption",
    )
    _require(lowering, "LocatedPartsAssociatedSourceV1::new(", 1, "lowering provider")
    for owner in (
        "lower_verified_parts_associated_block::<",
        "lower_local_statement_input(",
        "lower_assignment_inputs(",
        "lower_return_statement_input(",
        "lower_cond_expr_to_if_plans_input(",
        "lower_exit_if_state_core(",
        "lower_if_join_state_core(",
    ):
        if owner not in lowering:
            raise RuntimeError(f"LOOP0-P0b-T0 HOOK0 missing shared owner: {owner}")
    for token in (
        "CondBlockView",
        "RecipeBodies",
        "try_build_no_exit_block_recipe",
        "RecipeItem::",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in lowering:
            raise RuntimeError(f"LOOP0-P0b-T0 HOOK0 adapter owns forbidden token: {token}")

    _require(source, "pub(super) mod located_preflight;", 1, "preflight module")
    _require(source, "pub(super) mod located_lowering;", 1, "lowering module")
    _require(source, "mod located_hook_tests;", 1, "test module")
    _require(lowering_view, "fn source_syntax(&self)", 1, "bounded source accessor")
    for name in (
        "actual_strict_root_seals_before_any_builder_exists",
        "actual_strict_root_reaches_the_disconnected_located_adapter",
    ):
        _require(tests, f"fn {name}(", 1, f"fixture {name}")

    production_root = root / "src/mir/builder/control_flow/plan"
    callers: list[str] = []
    providers: list[str] = []
    for path in production_root.rglob("*.rs"):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        text = _production(path.read_text(encoding="utf-8"))
        call_count = text.count("lower_preflighted_located_parts_root_v1(")
        if relative == LOWERING:
            call_count = 0
        if call_count:
            callers.append(f"{relative}:{call_count}")
        provider_count = text.count("LocatedPartsAssociatedSourceV1::new(")
        if provider_count and relative != SOURCE:
            providers.append(f"{relative}:{provider_count}")
    if callers:
        raise RuntimeError(f"LOOP0-P0b-T0 HOOK0 production located callers: {callers}")
    expected_providers = [f"{LOWERING}:1", f"{PREFLIGHT}:1"]
    if sorted(providers) != sorted(expected_providers):
        raise RuntimeError(
            "LOOP0-P0b-T0 HOOK0 located provider consumers drift: "
            f"expected={expected_providers} actual={providers}"
        )

    touched = (PREFLIGHT, LOWERING, TESTS, SOURCE, LOWERING_VIEW, __file_relative())
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-T0 HOOK0 files reached 800 lines: {oversized}")
    return "r0_p0_hook0=1 preflight=1 located_adapter=1 production_callers=0 ledger=0"


def __file_relative() -> str:
    return "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_hook0.py"
