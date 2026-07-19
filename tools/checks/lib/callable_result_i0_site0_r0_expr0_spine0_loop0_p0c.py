#!/usr/bin/env python3
"""Private structural guard for disconnected LOOP0-P0c evidence."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


TEST = "src/mir/builder/control_flow/plan/features/generic_loop_p0c_tests.rs"
WHOLE = "src/mir/builder/control_flow/plan/features/generic_loop_whole_parity_tests.rs"
FEATURES = "src/mir/builder/control_flow/plan/features/mod.rs"
LOCATED = "src/mir/builder/control_flow/plan/located_loop_tests.rs"
REPRESENTATION = "src/mir/builder/control_flow/plan/generic_loop/located_representation/tests.rs"
REMAPPER = "src/mir/builder/control_flow/plan/normalizer/cond_lowering_freshen/remapper.rs"
PORT_TESTS = "src/mir/builder/control_flow/plan/expression_port_tests.rs"
RECIPE_SEAL = "src/mir/builder/control_flow/plan/generic_loop/located_representation/recipe_seal.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-P0c {label}: expected={expected} actual={actual}")


def check_loop0_p0c(root: Path) -> str:
    test = _read(root, TEST)
    whole = _read(root, WHOLE)
    features = _read(root, FEATURES)
    located = _read(root, LOCATED)
    representation = _read(root, REPRESENTATION)
    remapper = _read(root, REMAPPER)
    port_tests = _read(root, PORT_TESTS)
    recipe_seal = _read(root, RECIPE_SEAL)

    _require(test, "fn p0c_seals_verifier_schedule_and_short_circuit_cfg_without_execution(", 1, "P0c test")
    _require(test, "PlanVerifier::verify(", 2, "raw/located verifier")
    _require(test, "activation rows", 1, "15-row carrier")
    _require(test, "len(),\n            15", 1, "15-row assertion")
    _require(test, "len(), 9", 1, "nine-row Loop domain")
    _require(test, "let header = loop_plan.header_bb", 1, "short-circuit header edge")
    _require(test, "let first_rhs = header_branch.1", 1, "short-circuit and edge")
    _require(test, "let second_rhs = if first_branch.1 == body", 1, "short-circuit or edge")
    _require(test, "SourcePathSegmentV1::LoopCondition", 3, "condition-site topology checks")
    _require(test, "observed.len(), 3", 1, "three distinct condition sites")
    _require(test, "let header = loop_plan.header_bb", 1, "typed header owner")
    _require(test, "let body = loop_plan.body_bb", 1, "typed body owner")
    _require(test, "let after = loop_plan.after_bb", 1, "typed after owner")
    _require(test, "let wires = loop_plan", 1, "loop wire topology")
    _require(features, "mod generic_loop_p0c_tests;", 1, "P0c test registration")
    _require(whole, "pub(super) fn run_raw(", 1, "fresh raw harness")
    _require(whole, "pub(super) fn run_located(", 1, "fresh located harness")
    _require(remapper, "fn call_source_survives_value_id_remap_for_every_call_variant(", 1, "remap source preservation")
    _require(located, "fn malformed_core_loop_rejects_before_location_sealing(", 1, "pre-seal malformed plan")
    _require(located, "fn unrelated_unlocated_effect_does_not_fabricate_a_source_occurrence(", 1, "unlocated rejection")
    _require(representation, "fn foreign_and_unlocated_roots_reject_before_extraction(", 1, "foreign/unlocated preflight")
    _require(representation, "fn non_loop_root_rejects_without_route_fallback(", 1, "non-loop preflight")
    _require(recipe_seal, "fn reject_unsupported_nested_statements_rejects_scopebox_program_and_nested_loop(", 1, "nested statement preflight")
    _require(port_tests, "fn raw_port_keeps_every_call_family_unlocated(", 1, "raw carrier rejection boundary")
    _require(port_tests, "fn foreign_located_expression_is_rejected_by_the_port(", 1, "wrong-port rejection boundary")

    forbidden = (
        "PlanLowerer",
        "ledger",
        "claim_batch",
        "MirInterpreter",
        "RuntimeDataBox",
        "BackendKind",
        "fallback",
        "retry",
        "AST equality",
        "target spelling",
        "ValueId identity",
        "serde_json",
        "eprintln!",
    )
    for token in forbidden:
        if token in test:
            raise RuntimeError(f"LOOP0-P0c forbidden authority: {token}")

    touched = (__file_relative(), TEST)
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0c source/check files reached 800 lines: {oversized}")

    return "r0_p0c=1 verifier=2 carrier=15 loop_sites=9 short_circuit_blocks=3 production=0"


def __file_relative() -> str:
    return "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0_p0c.py"
