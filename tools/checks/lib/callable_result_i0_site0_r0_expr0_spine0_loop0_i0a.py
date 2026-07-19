#!/usr/bin/env python3
"""Structural guard for behavior-neutral LOOP0-I0a route selection."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


ROUTER = "src/mir/builder/control_flow/joinir/route_entry/router.rs"
REGISTRY = "src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs"
SELECTION = "src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs"
OBSERVER = "src/mir/builder/control_flow/joinir/route_entry/registry/legacy_observer.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-I0a {label}: expected={expected} actual={actual}")


def check_loop0_i0a(root: Path) -> str:
    router = _read(root, ROUTER)
    registry = _read(root, REGISTRY)
    selection = _read(root, SELECTION)
    observer = _read(root, OBSERVER)

    _require(selection, "struct RecipeFirstRouteSelectionV1", 1, "selection product")
    _require(selection, "raw_execution:", 4, "raw execution projection")
    _require(selection, "diagnostic_effective:", 4, "diagnostic projection")
    _require(selection, "pub(crate) fn select_recipe_first_routes", 1, "pure selection owner")
    _require(
        selection,
        "fn verify_located_generic_loop_v1",
        1,
        "located GenericLoopV1-only selector",
    )
    _require(
        selection,
        "fn actual_located_loop_selects_generic_loop_v1_without_a_builder",
        1,
        "actual Builder-free selection proof",
    )
    _require(
        selection,
        "fn located_generic_selection_uses_raw_execution_not_diagnostic_projection",
        1,
        "raw versus diagnostic split proof",
    )
    _require(
        registry,
        "fn execute_selected_routes_in_order",
        1,
        "ordered raw executor",
    )
    _require(
        registry,
        "selection.raw_execution_routes()",
        2,
        "raw executor consumes only raw order",
    )
    _require(
        registry,
        "fn raw_execution_continues_after_a_selected_route_returns_none",
        1,
        "raw None continuation proof",
    )
    _require(
        registry,
        "fn raw_execution_propagates_error_without_trying_later_routes",
        1,
        "raw error propagation proof",
    )
    _require(
        router,
        "let selection = registry::select_recipe_first_routes(outcome.facts.as_ref());",
        1,
        "router selection construction",
    )
    _require(
        router,
        "selection.diagnostic_effective_names()",
        2,
        "router diagnostic projection consumers",
    )
    _require(
        router,
        "registry::try_execute_recipe_first_selection(builder, ctx, &outcome, &env, &selection)?",
        1,
        "router executes selected raw order",
    )
    _require(
        observer,
        "selection.diagnostic_effective_routes().to_vec()",
        1,
        "legacy observer consumes diagnostic projection",
    )

    production_selection = selection.split("#[cfg(test)]", 1)[0]
    for forbidden in (
        "MirBuilder",
        "PlanLowerer",
        "ledger",
        "claim_batch",
        "ASTNode",
        "SourceExprSiteV1",
        "CorePlan",
    ):
        if forbidden in production_selection:
            raise RuntimeError(f"LOOP0-I0a selection owns forbidden authority: {forbidden}")

    touched = (
        SELECTION,
        REGISTRY,
        ROUTER,
        OBSERVER,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_i0a.py",
    )
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-I0a source/check files reached 800 lines: {oversized}")

    return "i0a=1 selector=1 raw_order=1 diagnostic_split=1 located_roots=0"
