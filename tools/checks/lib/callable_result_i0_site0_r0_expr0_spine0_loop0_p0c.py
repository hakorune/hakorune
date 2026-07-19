#!/usr/bin/env python3
"""Private structural guard for disconnected LOOP0-P0c evidence."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


TEST = "src/mir/builder/control_flow/plan/features/generic_loop_p0c_tests.rs"
WHOLE = "src/mir/builder/control_flow/plan/features/generic_loop_whole_parity_tests.rs"
FEATURES = "src/mir/builder/control_flow/plan/features/mod.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-P0c {label}: expected={expected} actual={actual}")


def check_loop0_p0c(root: Path) -> str:
    test = _read(root, TEST)
    whole = _read(root, WHOLE)
    features = _read(root, FEATURES)

    _require(test, "fn p0c_seals_verifier_schedule_and_short_circuit_cfg_without_execution(", 1, "P0c test")
    _require(test, "PlanVerifier::verify(", 2, "raw/located verifier")
    _require(test, "activation rows", 1, "15-row carrier")
    _require(test, "len(),\n            15", 1, "15-row assertion")
    _require(test, "len(), 9", 1, "nine-row Loop domain")
    _require(test, "BasicBlockId(1), BasicBlockId(5), BasicBlockId(4)", 1, "short-circuit header edge")
    _require(test, "BasicBlockId(5), BasicBlockId(2), BasicBlockId(6)", 1, "short-circuit and edge")
    _require(test, "BasicBlockId(6), BasicBlockId(2), BasicBlockId(4)", 1, "short-circuit or edge")
    _require(test, "SourcePathSegmentV1::LoopCondition", 3, "condition-site topology checks")
    _require(test, "observed.len(), 3", 1, "three distinct condition sites")
    _require(features, "mod generic_loop_p0c_tests;", 1, "P0c test registration")
    _require(whole, "pub(super) fn run_raw(", 1, "fresh raw harness")
    _require(whole, "pub(super) fn run_located(", 1, "fresh located harness")

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
