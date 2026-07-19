#!/usr/bin/env python3
"""Private PARITY0 guard for the actual strict raw/located Parts proof."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _read


TESTS = (
    "src/mir/builder/control_flow/plan/parts/associated_source/"
    "located_parity_tests.rs"
)
SOURCE = "src/mir/builder/control_flow/plan/parts/associated_source.rs"
RAW_OWNER = "src/mir/builder/control_flow/plan/parts/entry.rs"
LOCATED_OWNER = (
    "src/mir/builder/control_flow/plan/parts/associated_source/"
    "located_lowering.rs"
)
BLOCK_OWNER = (
    "src/mir/builder/control_flow/plan/parts/associated_source/block_driver.rs"
)
JOIN_OWNER = "src/mir/builder/control_flow/plan/parts/dispatch/if_join.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-T0 PARITY0 {label}: expected={expected} actual={actual}"
        )


def check_loop0_p0b_t0_parity0(root: Path) -> str:
    tests = _read(root, TESTS)
    source = _read(root, SOURCE)
    raw_owner = _read(root, RAW_OWNER)
    located_owner = _read(root, LOCATED_OWNER)

    _require(source, "mod located_parity_tests;", 1, "test module")
    _require(
        tests,
        "struct NormalizedActualStrictPartsSnapshotV1",
        1,
        "normalized snapshot owner",
    )
    _require(tests, "fn normalized_semantic_plans(", 1, "plan normalizer owner")
    _require(tests, "enum NormalizedPlanV1", 1, "typed plan snapshot")
    _require(tests, "enum NormalizedEffectV1", 1, "typed effect snapshot")
    _require(tests, "enum NormalizedExitV1", 1, "typed exit snapshot")
    _require(
        tests,
        "fn actual_strict_raw_and_located_parts_match_after_source_only_normalization(",
        1,
        "normalized parity fixture",
    )
    _require(
        tests,
        "fn actual_strict_located_parts_preserve_exact_four_body_prefix_sites(",
        1,
        "exact provenance fixture",
    )
    _require(
        tests,
        "fn foreign_located_port_rejects_before_builder_effects_and_valid_reuse_succeeds(",
        1,
        "foreign pairing rejection fixture",
    )
    _require(tests, "try_extract_generic_loop_v1(condition, body)", 1, "raw extraction")
    _require(tests, "entry::lower_exit_allowed_block(", 1, "raw Parts run")
    _require(
        tests,
        "lower_preflighted_located_parts_root_v1(",
        2,
        "located Parts parity and reuse runs",
    )
    _require(tests, "rows[8..12]", 1, "exact body-prefix row slice")
    _require(tests, "assert_eq!(raw.snapshot, located.snapshot)", 1, "exact snapshot parity")
    _require(tests, "assert_actual_golden(&raw)", 1, "raw independent golden")
    _require(tests, "assert_actual_golden(&located)", 1, "located independent golden")
    _require(
        tests,
        "representation.bind_lowering_port(&foreign_port).is_err()",
        1,
        "foreign port rejection",
    )
    _require(
        tests,
        "assert_eq!(builder.type_ctx.value_types, value_types_before)",
        1,
        "foreign rejection type-map parity",
    )

    for token in (
        "regex",
        "semantic_plans: String",
        'format!("{normalized:#?}")',
        ".replace(",
        ".sort(",
        ".sort_by(",
        "ValueId remap",
        "try_build_exit_allowed_block_recipe",
        "AST equality",
        "claim_batch",
        "fallback",
        "retry",
    ):
        if token in tests:
            raise RuntimeError(f"LOOP0-P0b-T0 PARITY0 forbidden test token: {token}")

    _require(raw_owner, "fn lower_exit_allowed_block(", 1, "existing raw owner")
    _require(
        located_owner,
        "fn lower_preflighted_located_parts_root_v1<",
        1,
        "existing located owner",
    )

    capped = (
        TESTS,
        __file_relative(),
        LOCATED_OWNER,
        BLOCK_OWNER,
        JOIN_OWNER,
    )
    oversized = [
        relative
        for relative in capped
        if len(_read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        raise RuntimeError(
            f"LOOP0-P0b-T0 PARITY0 source/check files reached 800 lines: {oversized}"
        )

    return "r0_p0_parity0=1 snapshots=1 raw_runs=1 located_runs=2 sites=4 production_callers=0 ledger=0"


def __file_relative() -> str:
    return (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0_parity0.py"
    )
