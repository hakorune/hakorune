#!/usr/bin/env python3
"""Private guard for the disconnected DirectRecipeOnly I0 slice."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_l0 import _require
from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read, _production


PREFLIGHT = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/direct_preflight.rs"
)
PREFLIGHT_TESTS = (
    "src/mir/builder/control_flow/plan/generic_loop/located_representation/direct_preflight_tests.rs"
)
DIRECT = (
    "src/mir/builder/control_flow/plan/features/generic_loop_body/direct_associated.rs"
)
COMPOSER = "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs"
COMPOSER_TESTS = (
    "src/mir/builder/control_flow/plan/features/generic_loop_located_composer_tests.rs"
)


def check_loop0_p0b_t0_direct0_i0(root: Path) -> str:
    preflight = _read(root, PREFLIGHT)
    preflight_tests = _read(root, PREFLIGHT_TESTS)
    direct = _read(root, DIRECT)
    composer = _production(_read(root, COMPOSER))
    composer_tests = _read(root, COMPOSER_TESTS)

    _require(
        preflight,
        "struct VerifiedLocatedGenericLoopDirectPreflightV1",
        1,
        "direct preflight owner",
    )
    _require(
        preflight,
        "struct PreparedLocatedGenericLoopDirectExecutionV1",
        1,
        "direct execution token",
    )
    _require(preflight, "VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly", 1, "direct mode gate")
    _require(preflight, "ExprChildRoleV1::IfCondition", 1, "if condition carrier")
    _require(preflight, "ExprChildRoleV1::AssignmentTarget", 1, "assignment target carrier")
    _require(preflight, "ExprChildRoleV1::AssignmentValue", 1, "assignment value carrier")
    _require(preflight, "ExprChildRoleV1::ReturnValue", 1, "return value carrier")
    _require(preflight, "BodyChildRoleV1::IfThen", 1, "if then carrier")
    _require(preflight, "BTreeSet", 5, "canonical carrier target set")

    _require(direct, "fn lower_direct_statement_inputs<'input, P, Inputs>", 1, "statement sequence owner")
    _require(direct, "lower_direct_body_input_with_policy", 4, "body facade delegation")
    _require(composer, "PreparedLocatedGenericLoopExecutionV1", 3, "mode execution owner")
    _require(composer, "VerifiedLocatedGenericLoopDirectPreflightV1::verify", 1, "direct preflight consumer")
    _require(composer, "PreparedLocatedGenericLoopBodyExecutionV1::Direct", 1, "direct body dispatch")
    _require(composer, "PreparedLocatedGenericLoopBodyExecutionV1::ExitAllowed", 1, "strict body dispatch")
    _require(composer_tests, "both located loop modes compose", 1, "both-mode composer fixture")
    _require(composer_tests, "vec![3, 4, 5, 6, 8, 7, 0, 1, 2]", 1, "direct traversal parity")
    _require(preflight_tests, "targets.as_ref(), [\"value\"]", 1, "default carrier target")
    _require(preflight_tests, "strict mode is not DirectRecipeOnly", 1, "strict direct rejection")

    for forbidden in (
        "std::env",
        "facts.body",
        "body_no_exit",
        "try_build_no_exit_block_recipe",
        "AST equality",
        "PlanLowerer",
        "claim_batch",
        "retry",
        "fallback",
    ):
        if forbidden in preflight or forbidden in composer:
            raise RuntimeError(f"LOOP0-P0b-T0 DIRECT0-I0 owns forbidden authority: {forbidden}")

    capped = (PREFLIGHT, PREFLIGHT_TESTS, DIRECT, COMPOSER, COMPOSER_TESTS, __file_relative())
    oversized = [path for path in capped if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-T0 DIRECT0-I0 source/check files reached 800 lines: {oversized}")

    return "direct0_i0=green preflight=1 token=1 statement_owner=1 modes=2 targets=value builder=0 ledger=0"


def __file_relative() -> str:
    return "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_direct0_i0.py"
