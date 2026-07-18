#!/usr/bin/env python3
"""Structural guard for LOOP0-P0b-O0-S0 canonical extraction."""

from __future__ import annotations

import re
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import (
    _is_test_source,
    _matching_rust_brace,
    _production,
    _read,
    _skip_rust_literal_or_comment,
)


README_PATH = "src/mir/builder/control_flow/plan/generic_loop/README.md"
FACTS_PATH = "src/mir/builder/control_flow/plan/generic_loop/facts_types.rs"
EXTRACT_PATH = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"
MOD_PATH = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/mod.rs"
TEST_PATH = (
    "src/mir/builder/control_flow/plan/generic_loop/facts/extract/"
    "successful_extraction_tests.rs"
)
HELPER_PATH = (
    "tools/checks/lib/"
    "callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0.py"
)


def _lexical_code(source: str) -> str:
    """Strip Rust literals/comments while retaining token layout."""

    output: list[str] = []
    cursor = 0
    while cursor < len(source):
        skipped = _skip_rust_literal_or_comment(source, cursor)
        if skipped is None:
            output.append(source[cursor])
            cursor += 1
            continue
        output.append(" " * (skipped - cursor))
        cursor = skipped
    return "".join(output)


def _code(text: str) -> str:
    """Strip cfg(test) items and Rust literals/comments."""

    return _lexical_code(_production(text))


def _item_body(text: str, kind: str, name: str) -> str:
    matches = list(re.finditer(rf"\b{kind}\s+{re.escape(name)}\b", text))
    if len(matches) != 1:
        raise RuntimeError(
            f"LOOP0-P0b-O0-S0 {kind} owner drift: name={name} count={len(matches)}"
        )
    opening = text.find("{", matches[0].end())
    if opening < 0:
        raise RuntimeError(f"LOOP0-P0b-O0-S0 missing {kind} body: {name}")
    return text[opening + 1 : _matching_rust_brace(text, opening) - 1]


def _function_body(text: str, name: str) -> str:
    matches = list(re.finditer(rf"\bfn\s+{re.escape(name)}\s*\(", text))
    if len(matches) != 1:
        raise RuntimeError(
            f"LOOP0-P0b-O0-S0 function owner drift: name={name} count={len(matches)}"
        )
    opening = text.find("{", matches[0].end())
    if opening < 0:
        raise RuntimeError(f"LOOP0-P0b-O0-S0 missing function body: {name}")
    return text[opening + 1 : _matching_rust_brace(text, opening) - 1]


def _field_names(body: str) -> list[str]:
    return re.findall(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?([a-z_][a-z0-9_]*)\s*:", body)


def check_loop0_p0b_o0_s0(root: Path) -> str:
    facts = _code(_read(root, FACTS_PATH))
    extract = _code(_read(root, EXTRACT_PATH))
    tests = _read(root, TEST_PATH)
    tests_code = _code(tests)
    module = _read(root, MOD_PATH)
    module_code = _lexical_code(module)
    readme = _read(root, README_PATH)

    disposition = _item_body(facts, "enum", "GenericLoopV1StepDispositionV1")
    if disposition.count("NumericProgression") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 NumericProgression disposition drift")
    if disposition.count("BodyManagedState") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 BodyManagedState disposition drift")
    if disposition.count("placement: StepPlacement") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 retained StepPlacement field drift")
    if disposition.count("canonical_body_len: usize") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 canonical body length field drift")

    product = _item_body(facts, "struct", "GenericLoopV1ExtractionV1")
    if _field_names(product) != ["facts", "step"]:
        raise RuntimeError("LOOP0-P0b-O0-S0 extraction product field drift")
    if "pub " in product or "pub(" in product:
        raise RuntimeError("LOOP0-P0b-O0-S0 extraction fields must remain private")

    resolution = _item_body(extract, "struct", "StepResolution")
    if _field_names(resolution) != ["loop_increment", "disposition"]:
        raise RuntimeError("LOOP0-P0b-O0-S0 StepResolution field drift")
    if "use_body_managed_step" in resolution:
        raise RuntimeError("LOOP0-P0b-O0-S0 boolean result truth survived")

    resolver = _function_body(extract, "resolve_step_for_candidate")
    if resolver.count("classify_step_placement(") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 step classification owner drift")
    if resolver.count("canonical_body_len: flat_body.len()") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 canonical body length publication drift")

    primary = _function_body(extract, "try_extract_generic_loop_v1")
    if primary.count("GenericLoopV1ExtractionV1::new(") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 extraction construction owner drift")
    facade = _function_body(extract, "try_extract_generic_loop_v1_facts")
    if facade.count("try_extract_generic_loop_v1(") != 1 or facade.count("into_facts") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 facts facade is not a thin projection")
    for forbidden in ("classify_step_placement", "matches_loop_increment", "RecipeBody"):
        if forbidden in facade:
            raise RuntimeError(f"LOOP0-P0b-O0-S0 facts facade owns forbidden policy: {forbidden}")
    hint = _function_body(extract, "has_generic_loop_v1_recipe_hint")
    if hint.count("try_extract_generic_loop_v1(") != 1 or hint.count(".is_some()") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 hint is not a thin canonical query")

    primary_calls: list[str] = []
    constructor_rows: dict[str, int] = {}
    for path in sorted((root / "src").rglob("*.rs")):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        code = _code(path.read_text(encoding="utf-8"))
        count = code.count("try_extract_generic_loop_v1(")
        if relative == EXTRACT_PATH:
            count -= 3  # primary definition plus facts and hint calls
        if count:
            primary_calls.extend([relative] * count)
        constructions = code.count("GenericLoopV1ExtractionV1::new(")
        if constructions:
            constructor_rows[relative] = constructions
    if primary_calls:
        raise RuntimeError(
            f"LOOP0-P0b-O0-S0 canonical extraction gained production callers: {primary_calls}"
        )
    if constructor_rows != {EXTRACT_PATH: 1}:
        raise RuntimeError(
            f"LOOP0-P0b-O0-S0 extraction constructor drift: {constructor_rows}"
        )

    facts_calls: dict[str, int] = {}
    for path in sorted((root / "src").rglob("*.rs")):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        code = _code(path.read_text(encoding="utf-8"))
        count = code.count("try_extract_generic_loop_v1_facts(")
        if relative == EXTRACT_PATH:
            count -= 1
        if count:
            facts_calls[relative] = count
    expected_facts_calls = {
        "src/mir/builder/control_flow/plan/features/nested_loop_depth1_route.rs": 1,
        "src/mir/builder/control_flow/plan/facts/loop_builder.rs": 1,
        "src/mir/builder/control_flow/plan/loop_cond/break_continue_helpers.rs": 1,
    }
    if facts_calls != expected_facts_calls:
        raise RuntimeError(
            f"LOOP0-P0b-O0-S0 raw facts caller drift: {facts_calls}"
        )

    required_tests = (
        "successful_extraction_retains_last_and_in_body_dispositions",
        "successful_extraction_retains_conditional_dispositions",
        "successful_extraction_retains_flattened_canonical_body_length",
        "successful_extraction_retains_body_managed_disposition",
        "successful_extraction_retains_post_validation_body_managed_fallback",
        "facts_facade_projects_the_canonical_extraction",
    )
    if module_code.count("mod successful_extraction_tests;") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 focused test module registration drift")
    if module_code.count("try_extract_generic_loop_v1,") != 1:
        raise RuntimeError("LOOP0-P0b-O0-S0 passive primary re-export drift")
    if "#[ignore]" in tests_code:
        raise RuntimeError("LOOP0-P0b-O0-S0 focused tests must not be ignored")
    for name in required_tests:
        if tests_code.count(f"fn {name}(") != 1:
            raise RuntimeError(f"LOOP0-P0b-O0-S0 focused test owner drift: {name}")

    for phrase in (
        "successful GenericLoopV1 step disposition",
        "successful extraction product",
        "No second step-placement classification after canonical extraction",
    ):
        if phrase not in readme:
            raise RuntimeError(f"LOOP0-P0b-O0-S0 README boundary drift: {phrase}")

    forbidden_authorities = ("MirBuilder", "ledger", "skeleton", "composer")
    for forbidden in forbidden_authorities:
        if forbidden in facts or forbidden in extract:
            raise RuntimeError(
                f"LOOP0-P0b-O0-S0 extraction owns forbidden authority: {forbidden}"
            )

    touched = (
        README_PATH,
        FACTS_PATH,
        EXTRACT_PATH,
        MOD_PATH,
        TEST_PATH,
        HELPER_PATH,
        "tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py",
        "tools/checks/lib/generic_loop_carrier_type_inventory.py",
        "tools/checks/fixtures/generic_loop_carrier_type_m0_inventory_v1.json",
    )
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-O0-S0 source/check files reached 800 lines: {oversized}")

    return "loop0_p0b_o0_s0=green"
