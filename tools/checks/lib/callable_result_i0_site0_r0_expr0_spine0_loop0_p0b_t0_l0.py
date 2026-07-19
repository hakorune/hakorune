#!/usr/bin/env python3
"""Private T0-L0 guard for the disconnected located GenericLoop composer."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _production, _read


COMPOSER = "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs"
TESTS = "src/mir/builder/control_flow/plan/features/generic_loop_located_composer_tests.rs"
FEATURES = "src/mir/builder/control_flow/plan/features/mod.rs"
STEP = "src/mir/builder/control_flow/plan/features/generic_loop_step.rs"
PREFLIGHT = (
    "src/mir/builder/control_flow/plan/parts/associated_source/located_preflight.rs"
)
LOWERING = (
    "src/mir/builder/control_flow/plan/parts/associated_source/located_lowering.rs"
)
CARRIERS = (
    "src/mir/builder/control_flow/plan/features/generic_loop_body/carriers.rs"
)
ORCHESTRATION = (
    "src/mir/builder/control_flow/plan/features/generic_loop_body/carrier_orchestration.rs"
)
HELPERS = "src/mir/builder/control_flow/plan/features/generic_loop_body/helpers.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-P0b-T0 L0 {label}: expected={expected} actual={actual}")


def check_loop0_p0b_t0_l0(root: Path) -> str:
    composer = _read(root, COMPOSER)
    production = _production(composer)
    tests = _read(root, TESTS)
    features = _read(root, FEATURES)
    step = _read(root, STEP)
    preflight = _read(root, PREFLIGHT)
    lowering = _read(root, LOWERING)
    carriers = _read(root, CARRIERS)
    orchestration = _read(root, ORCHESTRATION)
    helpers = _read(root, HELPERS)

    _require(features, "mod generic_loop_located_composer;", 1, "module registration")
    _require(production, "fn compose_located_generic_loop_v1<'plan>(", 1, "composer owner")
    _require(production, "bind_lowering_port(port)", 1, "same-call O0 binding")
    _require(production, "prepare_located_generic_loop_parts_execution_v1(&lowering)", 1, "pre-effect strict preflight")
    _require(production, "orchestrate_generic_loop_v1_carriers_from_targets(", 1, "target-owned carrier orchestration")
    _require(production, "apply_generic_loop_v1_fallthrough_cleanup_input(", 1, "located cleanup")
    _require(production, "apply_generic_loop_condition_input(", 1, "located condition")
    _require(production, "VerifiedLocatedCoreLoopPlanV1::verify(", 1, "final located seal")
    _require(step, "fn apply_generic_loop_condition_input<'input, P>(", 1, "associated condition core")
    _require(preflight, "carrier_targets: Box<[String]>", 2, "single retained target inventory")
    _require(lowering, "fn into_parts(", 1, "single-use execution split")
    _require(carriers, "fn prepare_generic_loop_v1_carriers_from_targets(", 1, "neutral target carrier core")
    _require(orchestration, "fn orchestrate_generic_loop_v1_carriers_from_targets<", 1, "neutral orchestration core")
    _require(helpers, "fn collect_loop_carrier_targets(", 1, "raw target facade")

    for forbidden in (
        "facts.body",
        "body_no_exit",
        "std::env",
        "PlanLowerer",
        "ledger",
        "claim_batch",
        "fallback",
        "retry",
        "AST equality",
        "try_build_no_exit_block_recipe",
    ):
        if forbidden in production:
            raise RuntimeError(f"LOOP0-P0b-T0 L0 composer owns forbidden authority: {forbidden}")

    _require(tests, "fn actual_strict_loop_composes_and_final_seals_in_one_call()", 1, "actual strict fixture")
    _require(tests, "DirectRecipeOnly must remain parked at L0", 1, "direct-mode pre-effect reject")
    _require(tests, "vec![3, 4, 5, 6, 8, 7, 0, 1, 2]", 1, "plan traversal order")
    _require(tests, "schedule.len(), 9", 1, "exact Loop call count")

    capped = (
        COMPOSER,
        TESTS,
        STEP,
        PREFLIGHT,
        LOWERING,
        CARRIERS,
        ORCHESTRATION,
        HELPERS,
        __file_relative(),
    )
    oversized = [path for path in capped if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-T0 L0 source/check files reached 800 lines: {oversized}")

    return "t0_l0=green composer=1 strict=1 direct=0 sites=9 builder=disconnected ledger=0"


def __file_relative() -> str:
    return "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_l0.py"
