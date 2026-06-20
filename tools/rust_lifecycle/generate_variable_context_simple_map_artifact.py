#!/usr/bin/env python3
"""Generate the focused VariableContext simple-map Hako artifact.

This 1519 pilot is intentionally narrower than full VariableContext. It emits
only lookup/contains/len/is_empty/insert/remove over the checked simple-map
fixtures. Returned map borrows, snapshot/restore, and carrier-sensitive
behavior remain excluded.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from textwrap import dedent
from typing import Any

from extract_variable_context_simple_map_facts import (
    SOURCE as VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE,
    extract_facts as extract_live_facts,
)
from shared_family_generator import (
    build_derived_artifact_verifier_result,
    build_common_rust_derived_inputs,
    build_hako_behavior_recipe,
    build_rust_derived_hako_manifest,
    read_json,
    sha256_file,
    sha256_text,
    run_family_generator,
    rust_manifest_file_entry,
    stable_json,
)
from shared_mirbuilder_emitter import emit_verified_family_hako


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

FACTS = FIXTURES / "variable-context-simple-map-facts-v0.json"
PLAN = FIXTURES / "variable-context-simple-map-plan-v0.json"
ORACLE = FIXTURES / "variable-context-simple-map-oracle-vectors-v0.json"
RECIPE = FIXTURES / "variable-context-simple-map-behavior-recipe-v0.json"
VERIFIER = FIXTURES / "variable-context-simple-map-derived-artifact-verifier-result-v0.json"
HAKO = OUT_DIR / "variable_context_simple_map.hako"
MANIFEST = OUT_DIR / "variable_context_simple_map.artifact.json"

SUBJECT = "hakorune_mir_builder::variable_context::VariableContext.simple_map"
FAMILY_ID = "hakorune_mir_builder::variable_context"
SCOPE = "VariableContext_simple_map_only"
METHODS = [
    "VariableContext::lookup",
    "VariableContext::contains",
    "VariableContext::len",
    "VariableContext::is_empty",
    "VariableContext::insert",
    "VariableContext::remove",
]
EXCLUDED = [
    "VariableContext::variable_map",
    "VariableContext::variable_map_mut",
    "VariableContext::snapshot",
    "VariableContext::restore",
]


def validate_inputs(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    if facts.get("kind") != "RustLifecycleFacts":
        raise SystemExit("unexpected facts kind")
    if plan.get("kind") != "HakoLifecyclePlan":
        raise SystemExit("unexpected plan kind")
    if oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected oracle kind")
    if facts.get("subject") != SUBJECT or plan.get("subject") != SUBJECT or oracle.get("subject") != SUBJECT:
        raise SystemExit("subject mismatch")

    fields = {row["id"]: row for row in facts["field_facts"]}
    variable_map = fields.get("VariableContext.variable_map")
    if variable_map is None:
        raise SystemExit("missing VariableContext.variable_map facts")
    if variable_map.get("rust_type") != "BTreeMap<String, ValueId>":
        raise SystemExit("unexpected variable_map rust_type")
    if variable_map.get("deterministic_order_required") is not True:
        raise SystemExit("missing deterministic order fact")
    if variable_map.get("drop_fact") != "TrivialMemory":
        raise SystemExit("variable_map drop must be TrivialMemory")

    plans = {row["id"]: row for row in plan["plans"]}
    if plans["VariableContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("VariableContext must be LocalBox")
    if plans["VariableContext.variable_map"]["plan_kind"] != "OrderedMapBox":
        raise SystemExit("variable_map must project to OrderedMapBox")

    method_ids = {row["id"] for row in facts["method_facts"]}
    missing = sorted(set(METHODS) - method_ids)
    if missing:
        raise SystemExit(f"missing method facts: {missing}")

    excluded = {row["id"] for row in facts["excluded_methods"]}
    missing_excluded = sorted(set(EXCLUDED) - excluded)
    if missing_excluded:
        raise SystemExit(f"missing excluded methods: {missing_excluded}")

    oracle_ops = {op["op"] for vector in oracle["vectors"] for op in vector["operations"]}
    for op in ["new", "is_empty", "len", "contains", "lookup", "insert", "remove"]:
        if op not in oracle_ops:
            raise SystemExit(f"missing oracle op: {op}")


def build_recipe() -> dict[str, Any]:
    return build_hako_behavior_recipe(
        family_id=FAMILY_ID,
        pilot_scope=SCOPE,
        subject=SUBJECT,
        source_plan="variable-context-simple-map-plan-v0.json",
        source_oracle="variable-context-simple-map-oracle-vectors-v0.json",
        selected_body_count="simple_map_methods_only",
        methods=[
            {
                "id": "VariableContext::lookup",
                "rust_operation": "BTreeMap::get(...).copied",
                "hako_operation": "OrderedMapBox.get",
                "emits": "VariableContextApi.lookup(ctx, name)",
            },
            {
                "id": "VariableContext::contains",
                "rust_operation": "BTreeMap::contains_key",
                "hako_operation": "OrderedMapBox.has as i64_bool_v0",
                "emits": "VariableContextApi.contains(ctx, name)",
            },
            {
                "id": "VariableContext::len",
                "rust_operation": "BTreeMap::len",
                "hako_operation": "OrderedMapBox.length",
                "emits": "VariableContextApi.len(ctx)",
            },
            {
                "id": "VariableContext::is_empty",
                "rust_operation": "BTreeMap::is_empty",
                "hako_operation": "OrderedMapBox.length == 0 as i64_bool_v0",
                "emits": "VariableContextApi.is_empty(ctx)",
            },
            {
                "id": "VariableContext::insert",
                "rust_operation": "BTreeMap::insert",
                "hako_operation": "OrderedMapBox.set",
                "emits": "VariableContextApi.insert(ctx, name, value_id)",
            },
            {
                "id": "VariableContext::remove",
                "rust_operation": "BTreeMap::remove",
                "hako_operation": "OrderedMapBox.remove",
                "emits": "VariableContextApi.remove(ctx, name)",
            },
        ],
        excluded_methods=EXCLUDED,
    )


def build_verifier(recipe: dict[str, Any]) -> dict[str, Any]:
    return build_derived_artifact_verifier_result(
        family_id=FAMILY_ID,
        pilot_scope=SCOPE,
        subject=SUBJECT,
        source_facts="variable-context-simple-map-facts-v0.json",
        source_plan="variable-context-simple-map-plan-v0.json",
        source_oracle="variable-context-simple-map-oracle-vectors-v0.json",
        source_recipe="variable-context-simple-map-behavior-recipe-v0.json",
        checks={
            "rust_facts_input": "verified",
            "hako_lifecycle_plan": "verified",
            "hako_behavior_recipe": "verified",
            "selected_body_count": recipe["selected_body_count"],
            "full_variable_context_claim": 0,
            "excluded_methods": EXCLUDED,
            "unmapped_thir_nodes": 0,
            "unmapped_mir_side_effects": 0,
            "unresolved_call_targets": 0,
            "unclassified_drop_obligations": 0,
            "mainline_selected": 0,
            "rust_bootstrap_retained": 1,
            "backend_behavior_changed": 0,
        },
        verified_operations=[method["hako_operation"] for method in recipe["methods"]],
        transport_notes={
            "bool_return_transport": "i64_bool_v0",
            "value_id_transport": "i64",
        },
    )


def build_hako() -> str:
    verified_ir = {
        "generated_by": "tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py",
        "artifact_manifest": "lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json",
        "family_comment": "hakorune_mir_builder::variable_context",
        "pilot_scope": SCOPE,
        "using_module": "apps.lib.collections.ordered_map",
        "box": {
            "name": "VariableContext",
            "field_name": "variable_map",
            "field_type": "OrderedMapBox",
            "initializer": "OrderedMap.create()",
        },
        "api": {
            "name": "VariableContextApi",
            "methods": [
                {
                    "signature": "is_empty(ctx): i64",
                    "body_lines": dedent(
                        """
                        if ctx.variable_map.length() == 0 {
                            return 1
                        }
                        return 0
                        """
                    ).strip("\n").splitlines(),
                },
                {
                    "signature": "len(ctx): i64",
                    "body_lines": ["return ctx.variable_map.length()"],
                },
                {
                    "signature": "contains(ctx, name): i64",
                    "body_lines": dedent(
                        """
                        if ctx.variable_map.has(name) == true {
                            return 1
                        }
                        return 0
                        """
                    ).strip("\n").splitlines(),
                },
                {
                    "signature": "lookup(ctx, name)",
                    "body_lines": ["return ctx.variable_map.get(name)"],
                },
                {
                    "signature": "insert(ctx, name, value_id): i64",
                    "body_lines": [
                        "ctx.variable_map.set(name, value_id)",
                        "return 0",
                    ],
                },
                {
                    "signature": "remove(ctx, name)",
                    "body_lines": ["return ctx.variable_map.remove(name)"],
                },
            ],
        },
        "main": {
            "lines": dedent(
                """
                local ctx = new VariableContext()
                if VariableContextApi.is_empty(ctx) != 1 {
                    print("variable_context_new_empty=fail")
                    return 1
                }
                if VariableContextApi.len(ctx) != 0 {
                    print("variable_context_new_len=fail")
                    return 2
                }

                VariableContextApi.insert(ctx, "x", 42)
                if VariableContextApi.lookup(ctx, "x") != 42 {
                    print("variable_context_lookup_x=fail")
                    return 3
                }
                if VariableContextApi.len(ctx) != 1 {
                    print("variable_context_len_after_insert=fail")
                    return 4
                }
                if VariableContextApi.is_empty(ctx) != 0 {
                    print("variable_context_empty_after_insert=fail")
                    return 5
                }
                if VariableContextApi.remove(ctx, "x") != 42 {
                    print("variable_context_remove_x=fail")
                    return 6
                }
                if VariableContextApi.lookup(ctx, "x") != null {
                    print("variable_context_lookup_removed=fail")
                    return 7
                }

                local contains_ctx = new VariableContext()
                if VariableContextApi.contains(contains_ctx, "x") != 0 {
                    print("variable_context_contains_empty=fail")
                    return 8
                }
                VariableContextApi.insert(contains_ctx, "x", 1)
                if VariableContextApi.contains(contains_ctx, "x") != 1 {
                    print("variable_context_contains_x=fail")
                    return 9
                }

                local ssa_ctx = new VariableContext()
                VariableContextApi.insert(ssa_ctx, "x", 1)
                VariableContextApi.insert(ssa_ctx, "x", 2)
                VariableContextApi.insert(ssa_ctx, "x", 4)
                if VariableContextApi.lookup(ssa_ctx, "x") != 4 {
                    print("variable_context_ssa_update=fail")
                    return 10
                }

                print("variable_context_simple_map_derived_artifact=ok")
                return 0
                """
            ).strip("\n").splitlines(),
        },
    }
    return emit_verified_family_hako(verified_ir)


def build_manifest(hako_text: str, recipe_text: str, verifier_text: str) -> dict[str, Any]:
    return build_rust_derived_hako_manifest(
        family_id=FAMILY_ID,
        pilot_scope=SCOPE,
        state="DerivedShadow",
        source_rust_files=[
            rust_manifest_file_entry(
                path=ROOT / "crates/hakorune_mir_builder/src/variable_context.rs",
                root=ROOT,
            )
        ],
        generator_tool="tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py",
        generator_version="variable-context-simple-map-derived-artifact-v0",
        hako_path=str(HAKO.relative_to(ROOT)),
        hako_sha256=sha256_text(hako_text),
        claims={
            "generated_hako_manual_edit": 0,
            "mainline_selected": 0,
            "full_variable_context_claim": 0,
            "rust_bootstrap_retained": 1,
            "backend_behavior_changed": 0,
            "source_selfhost_claim": 0,
        },
        inputs=build_common_rust_derived_inputs(
            root=ROOT,
            facts=FACTS,
            plan=PLAN,
            oracle=ORACLE,
            recipe=(RECIPE, recipe_text),
            verifier=(VERIFIER, verifier_text),
        ),
        extra_fields={"excluded_methods": EXCLUDED},
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    facts = extract_live_facts(VARIABLE_CONTEXT_SIMPLE_MAP_SOURCE)
    plan = read_json(PLAN)
    oracle = read_json(ORACLE)
    validate_inputs(facts, plan, oracle)

    recipe = build_recipe()
    verifier = build_verifier(recipe)
    hako_text = build_hako()
    recipe_text = stable_json(recipe)
    verifier_text = stable_json(verifier)
    manifest_text = stable_json(build_manifest(hako_text, recipe_text, verifier_text))

    run_family_generator(
        check=args.check,
        root=ROOT,
        unchanged_label="generated_variable_context_simple_map_artifact=unchanged",
        outputs_factory=lambda: [
            (RECIPE, recipe_text),
            (VERIFIER, verifier_text),
            (HAKO, hako_text),
            (MANIFEST, manifest_text),
        ],
    )


if __name__ == "__main__":
    main()
