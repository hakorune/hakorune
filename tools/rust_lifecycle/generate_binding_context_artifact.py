#!/usr/bin/env python3
"""Generate the focused BindingContext Rust-derived Hako artifact.

This is a bounded 1512 pilot generator. It does not invoke rustc, select a
selfhost route, or infer Hako policy from Rust names. It consumes the checked
facts/plan/oracle fixtures and emits deterministic recipe, verifier, Hako, and
artifact manifest files for the BindingContext family only.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from textwrap import dedent
from typing import Any

from extract_binding_context_facts import (
    SOURCE as BINDING_CONTEXT_SOURCE,
    extract_facts as extract_live_facts,
)
from shared_family_generator import (
    build_derived_artifact_verifier_result,
    build_common_rust_derived_inputs,
    build_common_rust_derived_manifest,
    build_hako_behavior_recipe,
    read_json,
    run_family_generator,
    stable_json,
)
from shared_mirbuilder_emitter import emit_verified_family_hako


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

FACTS = FIXTURES / "binding-context-adapter-facts-v0.json"
PLAN = FIXTURES / "binding-context-plan-v0.json"
ORACLE = FIXTURES / "binding-context-oracle-vectors-v0.json"
RECIPE = FIXTURES / "binding-context-behavior-recipe-v0.json"
VERIFIER = FIXTURES / "binding-context-derived-artifact-verifier-result-v0.json"
HAKO = OUT_DIR / "binding_context.hako"
MANIFEST = OUT_DIR / "binding_context.artifact.json"

METHODS = [
    "BindingContext::new",
    "BindingContext::is_empty",
    "BindingContext::len",
    "BindingContext::contains",
    "BindingContext::lookup",
    "BindingContext::insert",
    "BindingContext::remove",
    "BindingContext::clear_for_function_entry",
]


def validate_inputs(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::binding_context::BindingContext"
    if facts.get("kind") != "RustLifecycleAdapterFacts":
        raise SystemExit("unexpected facts kind")
    if plan.get("kind") != "HakoLifecyclePlan":
        raise SystemExit("unexpected plan kind")
    if oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected oracle kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("subject mismatch")

    fields = {row["id"]: row for row in facts["fields"]}
    binding_map = fields.get("BindingContext.binding_map")
    if binding_map is None:
        raise SystemExit("missing BindingContext.binding_map facts")
    if binding_map.get("rust_type") != "BTreeMap<String, BindingId>":
        raise SystemExit("unexpected binding_map rust_type")
    if binding_map.get("deterministic_order_required") is not True:
        raise SystemExit("missing deterministic order fact")
    if binding_map.get("drop_class") != "TrivialMemory":
        raise SystemExit("binding_map drop must be TrivialMemory")

    plans = {row["id"]: row for row in plan["plans"]}
    if plans["BindingContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("BindingContext must be LocalBox")
    if plans["BindingContext.binding_map"]["plan_kind"] != "OrderedMapBox":
        raise SystemExit("binding_map must project to OrderedMapBox")
    if plans["BindingContext"]["cleanup_policy"] != "erase":
        raise SystemExit("BindingContext cleanup must erase")

    method_ids = {row["id"] for row in facts["methods"]}
    required = {method for method in METHODS if method != "BindingContext::new"}
    missing = sorted(required - method_ids)
    if missing:
        raise SystemExit(f"missing method facts: {missing}")

    oracle_ops = {
        op["op"]
        for vector in oracle["vectors"]
        for op in vector["operations"]
    }
    for op in ["new", "is_empty", "len", "contains", "lookup", "insert", "remove", "clear_for_function_entry"]:
        if op not in oracle_ops:
            raise SystemExit(f"missing oracle op: {op}")


def build_recipe() -> dict[str, Any]:
    return build_hako_behavior_recipe(
        family_id="hakorune_mir_builder::binding_context",
        subject="hakorune_mir_builder::binding_context::BindingContext",
        source_plan="binding-context-plan-v0.json",
        source_oracle="binding-context-oracle-vectors-v0.json",
        selected_body_count="all_non_test_methods",
        methods=[
            {
                "id": "BindingContext::new",
                "rust_operation": "BTreeMap::new",
                "hako_operation": "OrderedMap.create",
                "emits": "birth initializes me.binding_map",
            },
            {
                "id": "BindingContext::is_empty",
                "rust_operation": "BTreeMap::is_empty",
                "hako_operation": "OrderedMapBox.length == 0 as i64_bool_v0",
                "emits": "BindingContextApi.is_empty(ctx)",
            },
            {
                "id": "BindingContext::len",
                "rust_operation": "BTreeMap::len",
                "hako_operation": "OrderedMapBox.length",
                "emits": "BindingContextApi.len(ctx)",
            },
            {
                "id": "BindingContext::contains",
                "rust_operation": "BTreeMap::contains_key",
                "hako_operation": "OrderedMapBox.has as i64_bool_v0",
                "emits": "BindingContextApi.contains(ctx, name)",
            },
            {
                "id": "BindingContext::lookup",
                "rust_operation": "BTreeMap::get(...).copied",
                "hako_operation": "OrderedMapBox.get",
                "emits": "BindingContextApi.lookup(ctx, name)",
            },
            {
                "id": "BindingContext::insert",
                "rust_operation": "BTreeMap::insert",
                "hako_operation": "OrderedMapBox.set",
                "emits": "BindingContextApi.insert(ctx, name, binding_id)",
            },
            {
                "id": "BindingContext::remove",
                "rust_operation": "BTreeMap::remove",
                "hako_operation": "OrderedMapBox.remove",
                "emits": "BindingContextApi.remove(ctx, name)",
            },
            {
                "id": "BindingContext::clear_for_function_entry",
                "rust_operation": "BTreeMap::clear",
                "hako_operation": "OrderedMapBox.clear",
                "emits": "BindingContextApi.clear_for_function_entry(ctx)",
            },
        ],
    )


def build_verifier(recipe: dict[str, Any]) -> dict[str, Any]:
    return build_derived_artifact_verifier_result(
        family_id="hakorune_mir_builder::binding_context",
        subject="hakorune_mir_builder::binding_context::BindingContext",
        source_facts="binding-context-adapter-facts-v0.json",
        source_plan="binding-context-plan-v0.json",
        source_oracle="binding-context-oracle-vectors-v0.json",
        source_recipe="binding-context-behavior-recipe-v0.json",
        checks={
            "rust_facts_input": "verified",
            "hako_lifecycle_plan": "verified",
            "hako_behavior_recipe": "verified",
            "selected_body_count": recipe["selected_body_count"],
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
            "reason": "pure-first global helper ABI expects scalar i64 returns in this pilot",
        },
        denied_boundaries=[
            "selfhost mainline selection",
            "HakoAdopted native source decision",
            "MirBuilder-wide lifecycle parity",
            "runtime try-Hako-then-Rust fallback",
        ],
    )


def build_hako() -> str:
    verified_ir = {
        "generated_by": "tools/rust_lifecycle/generate_binding_context_artifact.py",
        "artifact_manifest": "lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json",
        "family_comment": "hakorune_mir_builder::binding_context",
        "using_module": "apps.lib.collections.ordered_map",
        "box": {
            "name": "BindingContext",
            "field_name": "binding_map",
            "field_type": "OrderedMapBox",
            "initializer": "OrderedMap.create()",
        },
        "api": {
            "name": "BindingContextApi",
            "trailing_blank_line": True,
            "methods": [
                {
                    "signature": "is_empty(ctx): i64",
                    "body_lines": dedent(
                        """
                        if ctx.binding_map.length() == 0 {
                            return 1
                        }
                        return 0
                        """
                    ).strip("\n").splitlines(),
                },
                {
                    "signature": "len(ctx): i64",
                    "body_lines": ["return ctx.binding_map.length()"],
                },
                {
                    "signature": "contains(ctx, name): i64",
                    "body_lines": dedent(
                        """
                        if ctx.binding_map.has(name) == true {
                            return 1
                        }
                        return 0
                        """
                    ).strip("\n").splitlines(),
                },
                {
                    "signature": "lookup(ctx, name)",
                    "body_lines": ["return ctx.binding_map.get(name)"],
                },
                {
                    "signature": "insert(ctx, name, binding_id): i64",
                    "body_lines": [
                        "ctx.binding_map.set(name, binding_id)",
                        "return 0",
                    ],
                },
                {
                    "signature": "remove(ctx, name)",
                    "body_lines": ["return ctx.binding_map.remove(name)"],
                },
                {
                    "signature": "clear_for_function_entry(ctx): i64",
                    "body_lines": [
                        "ctx.binding_map.clear()",
                        "return 0",
                    ],
                },
            ],
        },
        "main": {
            "lines": dedent(
                """
                local ctx = new BindingContext()
                if BindingContextApi.is_empty(ctx) != 1 {
                    print("binding_context_new_empty=fail")
                    return 1
                }
                if BindingContextApi.len(ctx) != 0 {
                    print("binding_context_new_len=fail")
                    return 2
                }

                BindingContextApi.insert(ctx, "x", 0)
                if BindingContextApi.lookup(ctx, "x") != 0 {
                    print("binding_context_lookup_x=fail")
                    return 3
                }
                if BindingContextApi.len(ctx) != 1 {
                    print("binding_context_len_after_insert=fail")
                    return 4
                }
                if BindingContextApi.is_empty(ctx) != 0 {
                    print("binding_context_empty_after_insert=fail")
                    return 5
                }
                if BindingContextApi.remove(ctx, "x") != 0 {
                    print("binding_context_remove_x=fail")
                    return 6
                }
                if BindingContextApi.lookup(ctx, "x") != null {
                    print("binding_context_lookup_removed=fail")
                    return 7
                }
                if BindingContextApi.is_empty(ctx) != 1 {
                    print("binding_context_empty_after_remove=fail")
                    return 8
                }

                local contains_ctx = new BindingContext()
                if BindingContextApi.contains(contains_ctx, "x") != 0 {
                    print("binding_context_contains_empty=fail")
                    return 9
                }
                BindingContextApi.insert(contains_ctx, "x", 0)
                if BindingContextApi.contains(contains_ctx, "x") != 1 {
                    print("binding_context_contains_x=fail")
                    return 10
                }

                local order_ctx = new BindingContext()
                BindingContextApi.insert(order_ctx, "b", 2)
                BindingContextApi.insert(order_ctx, "a", 1)
                if BindingContextApi.len(order_ctx) != 2 {
                    print("binding_context_order_len=fail")
                    return 11
                }
                if BindingContextApi.lookup(order_ctx, "a") != 1 {
                    print("binding_context_lookup_a=fail")
                    return 12
                }
                if BindingContextApi.lookup(order_ctx, "b") != 2 {
                    print("binding_context_lookup_b=fail")
                    return 13
                }
                local clear_ctx = new BindingContext()
                BindingContextApi.insert(clear_ctx, "a", 1)
                BindingContextApi.clear_for_function_entry(clear_ctx)
                if BindingContextApi.is_empty(clear_ctx) != 1 {
                    print("binding_context_clear_empty=fail")
                    return 14
                }

                print("binding_context_derived_artifact=ok")
                return 0
                """
            ).strip("\n").splitlines(),
        },
    }
    return emit_verified_family_hako(verified_ir)


def build_manifest(hako_text: str, recipe_text: str, verifier_text: str) -> dict[str, Any]:
    return build_common_rust_derived_manifest(
        root=ROOT,
        family_id="hakorune_mir_builder::binding_context",
        state="DerivedShadow",
        source_rust_file=ROOT / "crates/hakorune_mir_builder/src/binding_context.rs",
        generator_tool="tools/rust_lifecycle/generate_binding_context_artifact.py",
        generator_version="binding-context-derived-artifact-v0",
        hako_path=HAKO,
        hako_text=hako_text,
        claims={
            "generated_hako_manual_edit": 0,
            "mainline_selected": 0,
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
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    facts = extract_live_facts(BINDING_CONTEXT_SOURCE)
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
        unchanged_label="generated_binding_context_artifact=unchanged",
        outputs_factory=lambda: [
            (RECIPE, recipe_text),
            (VERIFIER, verifier_text),
            (HAKO, hako_text),
            (MANIFEST, manifest_text),
        ],
    )


if __name__ == "__main__":
    main()
