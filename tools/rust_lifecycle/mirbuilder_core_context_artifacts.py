#!/usr/bin/env python3
"""Spec and validation for the CoreContext scalar-counter artifact."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_core_context_facts import SOURCE as CORE_CONTEXT_SOURCE, extract_facts as extract_core_context_facts
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {operation["op"] for vector in oracle["vectors"] for operation in vector["operations"]}


def validate_core_context(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::core_context::CoreContext"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected CoreContext fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("CoreContext subject mismatch")

    type_fact = {row["id"]: row for row in facts["type_facts"]}.get("CoreContext")
    if type_fact is None or type_fact.get("drop_fact") != "TrivialMemory":
        raise SystemExit("CoreContext drop fact mismatch")

    body_facts = {row["id"]: row for row in facts["body_facts"]}
    for method, operation, field in [
        ("CoreContext::next_binding", "BindingIdNewAndIncrement", "next_binding_id"),
        ("CoreContext::next_temp_slot", "CounterNext", "temp_slot_counter"),
        ("CoreContext::next_debug_join", "CounterNext", "debug_join_counter"),
    ]:
        body_fact = body_facts.get(method)
        if body_fact is None or body_fact.get("operation") != operation:
            raise SystemExit(f"unexpected body operation for {method}")
        if body_fact.get("selected_field") != field:
            raise SystemExit(f"unexpected body field for {method}")

    plans = {row["id"]: row for row in plan["plans"]}
    if plans["CoreContext"]["plan_kind"] != "LocalBox":
        raise SystemExit("CoreContext must be LocalBox")
    for method, plan_kind in [
        ("CoreContext::next_binding", "BindingIdNewAndIncrement"),
        ("CoreContext::next_temp_slot", "CounterNext"),
        ("CoreContext::next_debug_join", "CounterNext"),
    ]:
        plan_entry = plans.get(method)
        if plan_entry is None or plan_entry.get("plan_kind") != plan_kind:
            raise SystemExit(f"unexpected plan entry: {method}")
        if plan_entry.get("transport") != "i64":
            raise SystemExit(f"unexpected transport: {method}")

    denied = {row["id"]: row for row in plan.get("denied", [])}
    for method in ["CoreContext::next_value", "CoreContext::next_block", "CoreContext::peek_next_value", "CoreContext::peek_next_block"]:
        if denied.get(method, {}).get("deny_reason") != "GeneratorObjectTransportDeferred":
            raise SystemExit(f"missing generator-object deny: {method}")
    for op_name in ["next_binding", "next_temp_slot", "next_debug_join"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing oracle op: {op_name}")
    if oracle.get("promotion_scope", {}).get("mirbuilder_wide_claim") is not False:
        raise SystemExit("CoreContext oracle must not claim MirBuilder-wide parity")


def core_context_spec() -> FamilyArtifactSpec:
    excluded = ["CoreContext::next_value", "CoreContext::next_block", "CoreContext::peek_next_value", "CoreContext::peek_next_block"]
    facts = extract_core_context_facts(CORE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "core-context-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("core_context.scalar_counter_context", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_core_context_artifact.py",
        generator_version="core-context-scalar-counters-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json",
        family_comment="hakorune_mir_builder::core_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="CoreContext",
            fields=[
                FieldSpec(name="next_binding_id", field_type="i64", initializer="0"),
                FieldSpec(name="temp_slot_counter", field_type="i64", initializer="0"),
                FieldSpec(name="debug_join_counter", field_type="i64", initializer="0"),
            ],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="CoreContext"),
            op("StaticCall", target="binding0", callee="CoreContextApi.next_binding", args=["ctx"]),
            op("AssertEq", left="binding0", right=0, fail_message="core_context_next_binding_0=fail", fail_code=1),
            op("StaticCall", target="binding1", callee="CoreContextApi.next_binding", args=["ctx"]),
            op("AssertEq", left="binding1", right=1, fail_message="core_context_next_binding_1=fail", fail_code=2),
            op("StaticCall", target="temp0", callee="CoreContextApi.next_temp_slot", args=["ctx"]),
            op("AssertEq", left="temp0", right=0, fail_message="core_context_next_temp_slot_0=fail", fail_code=3),
            op("StaticCall", target="debug0", callee="CoreContextApi.next_debug_join", args=["ctx"]),
            op("AssertEq", left="debug0", right=0, fail_message="core_context_next_debug_join_0=fail", fail_code=4),
            op("StaticCall", target="temp1", callee="CoreContextApi.next_temp_slot", args=["ctx"]),
            op("AssertEq", left="temp1", right=1, fail_message="core_context_next_temp_slot_1=fail", fail_code=5),
            op("StaticCall", target="debug1", callee="CoreContextApi.next_debug_join", args=["ctx"]),
            op("AssertEq", left="debug1", right=1, fail_message="core_context_next_debug_join_1=fail", fail_code=6),
            op("Print", text="core_context_scalar_counters_derived_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::core_context",
        state="DerivedShadow",
        source_rust_file=CORE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "core_context.hako",
        facts_path=FIXTURES / "core-context-facts-v0.json",
        plan_path=FIXTURES / "core-context-plan-v0.json",
        oracle_path=FIXTURES / "core-context-oracle-v0.json",
        recipe_path=FIXTURES / "core-context-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "core-context-derived-artifact-verifier-result-v0.json",
        pilot_scope="CoreContext_scalar_counters_only",
        recipe_subject="hakorune_mir_builder::core_context::CoreContext.scalar_counters",
        selected_body_count="scalar_counter_methods_only",
        api_name="CoreContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="CoreContext::next_binding", rust_operation="BindingId::new + saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_binding(ctx)"),
            BehaviorMethodSpec(id="CoreContext::next_temp_slot", rust_operation="saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_temp_slot(ctx)"),
            BehaviorMethodSpec(id="CoreContext::next_debug_join", rust_operation="saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_debug_join(ctx)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "core_context_full_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "selected_body_count": "scalar_counter_methods_only", "denied_generator_object_methods": excluded, "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verified_operations=["ScalarCounterInit", "TakeThenSaturatingIncrementU32"],
        transport_notes={"counter_transport": "i64", "binding_id_transport": "i64_raw"},
        extra_manifest_fields={"excluded_methods": excluded},
    )
