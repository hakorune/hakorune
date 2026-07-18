#!/usr/bin/env python3
"""Structural guard for LOOP0-P0b-O0-R0 located body representation."""

from __future__ import annotations

import re
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _production, _read


BASE = "src/mir/builder/control_flow/plan/generic_loop/located_representation"
MOD = f"{BASE}/mod.rs"
PRODUCT = f"{BASE}/product.rs"
RECIPE = f"{BASE}/recipe_seal.rs"
ERROR = f"{BASE}/error.rs"
README = f"{BASE}/README.md"
TESTS = f"{BASE}/tests.rs"
PORT = "src/mir/builder/control_flow/plan/expression_port.rs"
LEGACY = "src/mir/callable_result_representation/located_legacy.rs"
EXTRACT = "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs"


def _count(text: str, token: str, expected: int, label: str) -> None:
    actual = text.count(token)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-O0-R0 {label} drift: expected={expected} actual={actual}"
        )


def check_loop0_p0b_o0_r0(root: Path) -> str:
    module = _production(_read(root, MOD))
    product = _production(_read(root, PRODUCT))
    recipe = _production(_read(root, RECIPE))
    error = _production(_read(root, ERROR))
    port = _production(_read(root, PORT))
    legacy = _production(_read(root, LEGACY))
    extract = _production(_read(root, EXTRACT))
    tests = _read(root, TESTS)
    readme = _read(root, README)

    _count(module, "fn verify_located_loop(", 1, "same-call constructor owner")
    _count(module, "port.require_exact_stmt(&loop_root)?", 1, "root carrier preflight")
    _count(module, "ExprChildRoleV1::LoopCondition", 1, "PATH0 condition demand")
    _count(module, "BodyChildRoleV1::LoopBody", 1, "PATH0 body demand")
    _count(module, "try_extract_generic_loop_v1(", 1, "canonical extraction call")
    _count(module, "StepPlacement::Last", 1, "Last admission")
    _count(module, "BodyLoweringPolicy::RecipeOnly", 1, "direct mode selection")
    _count(module, "BodyLoweringPolicy::ExitAllowed", 1, "recipe mode selection")
    for forbidden in (
        "classify_step_placement",
        "matches_loop_increment",
        "std::env",
        "body_no_exit",
        "MirBuilder",
        "ledger",
        "skeleton",
        "composer",
        "fallback",
        "retry",
    ):
        if forbidden in module:
            raise RuntimeError(f"LOOP0-P0b-O0-R0 constructor authority leak: {forbidden}")

    for owner in (
        "VerifiedLocatedGenericLoopBodyRepresentationV1",
        "VerifiedLocatedGenericLoopBodyModeV1",
        "VerifiedLocatedRecipeBlockV1",
        "VerifiedLocatedRecipeItemV1",
        "VerifiedStmtWrappedJoinIfV1",
    ):
        definitions = re.findall(rf"\b(?:struct|enum)\s+{re.escape(owner)}\b", product)
        if len(definitions) != 1:
            raise RuntimeError(
                f"LOOP0-P0b-O0-R0 product {owner} drift: count={len(definitions)}"
            )
    if "Clone" in product:
        raise RuntimeError("LOOP0-P0b-O0-R0 verified product became Clone")
    _count(product, "DirectRecipeOnly", 1, "direct representation variant")
    _count(product, "ExitAllowedRecipe", 1, "recipe representation variant")
    _count(product, "\n    StmtWrappedJoinIf {", 1, "wrapped Join If variant")

    _count(recipe, "fn seal_recipe_block", 1, "recipe seal owner")
    _count(recipe, "arena.get(block.body_id)", 1, "recipe body-id co-seal")
    _count(recipe, "enum RecipeSealDomainV1", 1, "recipe seal domain owner")
    _count(recipe, "fn require_contract(", 1, "recipe contract decision owner")
    _count(recipe, "try_build_no_exit_block_recipe(", 1, "NoExit bridge builder")
    _count(recipe, "contract: IfContractKind::Join", 1, "singleton Join contract")
    _count(recipe, "require_ordinal(0, if_stmt.index())", 1, "singleton zero ordinal witness")
    _count(recipe, "RecipeItem::LoopV0", 1, "nested Loop rejection")
    singleton_arena_reads = re.findall(
        r"singleton_recipe\s*\.arena\s*\.get\(singleton_recipe\.block\.body_id\)",
        recipe,
    )
    if len(singleton_arena_reads) != 1:
        raise RuntimeError(
            "LOOP0-P0b-O0-R0 singleton arena cardinality drift: "
            f"count={len(singleton_arena_reads)}"
        )
    for forbidden in (
        "BTreeMap",
        "HashMap",
        "std::ptr::eq",
        ".span()",
        "ValueId",
        "target_spelling",
    ):
        if forbidden in module + recipe + product:
            raise RuntimeError(f"LOOP0-P0b-O0-R0 second pairing authority: {forbidden}")

    _count(port, "fn require_exact_stmt(", 1, "located stmt facade")
    _count(port, "fn require_exact_body(", 1, "located body facade")
    _count(legacy, "fn require_located_stmt_carrier(", 1, "stmt carrier validator")
    _count(legacy, "fn require_located_body_carrier(", 1, "body carrier validator")
    if "body_exit_allowed.clone()" in extract:
        raise RuntimeError("LOOP0-P0b-O0-R0 ExitAllowed recipe ownership duplicated")

    required_tests = (
        "direct_recipe_only_seals_exact_prefix_and_cleanup",
        "strict_exit_allowed_seals_explicit_and_wrapped_if_items",
        "foreign_and_unlocated_roots_reject_before_extraction",
        "non_loop_root_rejects_without_route_fallback",
        "recipe_body_id_and_cardinality_are_co_sealed",
        "recipe_contract_domains_do_not_overlap",
    )
    for name in required_tests:
        _count(tests, f"fn {name}(", 1, f"focused test {name}")
    if "#[ignore]" in tests:
        raise RuntimeError("LOOP0-P0b-O0-R0 focused tests must not be ignored")

    for phrase in (
        "exact PATH0 `Loop` carrier",
        "DirectRecipeOnly",
        "ExitAllowedRecipe",
        "non-`Clone`",
    ):
        if phrase not in readme:
            raise RuntimeError(f"LOOP0-P0b-O0-R0 README boundary drift: {phrase}")

    production_calls = []
    for path in (root / "src").rglob("*.rs"):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs") or "tests" in path.parts:
            continue
        relative = path.relative_to(root).as_posix()
        count = _production(path.read_text(encoding="utf-8")).count(
            "verify_located_loop("
        )
        if relative == MOD:
            count -= 1
        production_calls.extend([relative] * count)
    if production_calls:
        raise RuntimeError(
            "LOOP0-P0b-O0-R0 production located execution caller drift: "
            f"{production_calls}"
        )

    touched = (MOD, PRODUCT, RECIPE, ERROR, README, TESTS, PORT, LEGACY, EXTRACT, __file__)
    oversized = []
    for path in touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(f"LOOP0-P0b-O0-R0 source/check files reached 800 lines: {oversized}")

    return (
        "loop0_p0b_o0_r0=green modes=2 canonical_extraction_calls=1 "
        "production_located_execution_callers=0 builder=0 ledger=0"
    )
