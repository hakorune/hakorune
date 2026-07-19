#!/usr/bin/env python3
"""Private guard for LOOP0-P0b-T0 whole GenericLoop parity."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


TEST = "src/mir/builder/control_flow/plan/features/generic_loop_whole_parity_tests.rs"
SUPPORT = "src/mir/builder/control_flow/plan/parity_snapshot_test_support.rs"
FEATURES = "src/mir/builder/control_flow/plan/features/mod.rs"
PLAN_MOD = "src/mir/builder/control_flow/plan/mod.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-T0-P0 {label}: expected={expected} actual={actual}"
        )


def check_loop0_p0b_t0_p0(root: Path) -> str:
    test = _read(root, TEST)
    support = _read(root, SUPPORT)
    features = _read(root, FEATURES)
    plan_mod = _read(root, PLAN_MOD)

    _require(
        test,
        "fn actual_default_and_strict_raw_and_located_whole_loops_match_after_source_only_normalization(",
        1,
        "whole-loop parity fixture",
    )
    _require(test, "with_default_and_strict_modes(", 1, "independent mode harness")
    _require(test, "fn run_raw(", 1, "raw fresh builder harness")
    _require(test, "fn run_located(", 1, "located fresh builder harness")
    _require(test, "normalized_semantic_plans(", 2, "typed plan parity comparisons")
    _require(test, "VerifiedCallableResultLoopClaimScheduleV1::verify(", 1, "source schedule owner")
    _require(test, "assert_mode_golden(mode, &raw.plan);", 1, "independent mode golden")
    _require(test, "vec![3, 4, 5, 6, 8, 7, 0, 1, 2]", 1, "plan traversal witness")
    _require(test, "raw.call_sources.len(), 9", 1, "raw call count")
    _require(test, "located.schedule.len(), 9", 1, "located schedule count")
    _require(test, "CoreCallSourceV1::Unlocated", 2, "raw unlocated provenance")
    _require(test, "CoreCallSourceV1::LocatedMethodCall", 1, "located provenance")

    _require(support, "enum NormalizedPlanV1", 1, "typed plan vocabulary")
    _require(support, "enum NormalizedEffectV1", 1, "typed effect vocabulary")
    _require(support, "fn normalized_semantic_plans(", 1, "shared normalizer owner")
    _require(support, "CorePlan::Loop", 1, "loop normalizer support")
    for variant in ("MethodCall", "GlobalCall", "ValueCall", "ExternCall"):
        if support.count(f"{variant} {{") < 1:
            raise RuntimeError(f"LOOP0-P0b-T0-P0 call variant missing: {variant}")
    _require(support, "call-source provenance", 1, "sole erased dimension")

    _require(features, "mod generic_loop_whole_parity_tests;", 1, "test registration")
    _require(plan_mod, "mod parity_snapshot_test_support;", 1, "shared support registration")

    forbidden = (
        "PlanLowerer",
        "claim_batch",
        "ledger",
        "fallback",
        "retry",
        "MirInterpreter",
        "RuntimeDataBox",
        "BackendKind",
        "ValueId remap",
        "AST equality",
        "serde_json",
        "format!(\"{normalized",
        ".sort_by(",
    )
    for token in forbidden:
        if token in test or token in support:
            raise RuntimeError(f"LOOP0-P0b-T0-P0 forbidden parity authority: {token}")

    capped = (__file_relative(), TEST, SUPPORT, FEATURES, PLAN_MOD)
    oversized = [
        relative
        for relative in capped
        if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        raise RuntimeError(
            f"LOOP0-P0b-T0-P0 source/check files reached 800 lines: {oversized}"
        )

    return "r0_p0b_t0_p0=1 modes=2 raw_runs=2 located_runs=2 sites=9 source_only=1 ledger=0"


def __file_relative() -> str:
    return (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_p0.py"
    )
