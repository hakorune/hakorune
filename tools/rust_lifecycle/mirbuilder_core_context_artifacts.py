#!/usr/bin/env python3
"""Spec and validation for the CoreContext scalar-counter artifact."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_core_context_facts import SOURCE as CORE_CONTEXT_SOURCE, extract_facts as extract_core_context_facts
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json
from verified_family_artifact_contract import ArtifactIdentity, VerifiedFamilyArtifactContractV1
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
    for method, operation, source_field, return_shape in [
        ("CoreContext::next_value", "GeneratorNext", "value_gen", "ValueId"),
        ("CoreContext::peek_next_value", "GeneratorPeekNext", "value_gen", "ValueId"),
        ("CoreContext::next_block", "GeneratorNext", "block_gen", "BasicBlockId"),
        ("CoreContext::peek_next_block", "GeneratorPeekNext", "block_gen", "BasicBlockId"),
    ]:
        body_fact = body_facts.get(method)
        if body_fact is None or body_fact.get("operation") != operation:
            raise SystemExit(f"unexpected generator body operation for {method}")
        if body_fact.get("selected_field") != source_field or body_fact.get("return_shape") != return_shape:
            raise SystemExit(f"unexpected generator body field/return for {method}")

    generator_facts = {row["id"]: row for row in facts.get("generator_state_facts", [])}
    for generator_id, transport, mutation in [
        ("ValueIdGenerator::next", "ValueIdAsI64", "PostIncrement"),
        ("ValueIdGenerator::peek_next", "ValueIdAsI64", "ReadOnly"),
        ("BasicBlockIdGenerator::next", "BasicBlockIdAsI64", "PostIncrement"),
        ("BasicBlockIdGenerator::peek_next", "BasicBlockIdAsI64", "ReadOnly"),
    ]:
        generator_fact = generator_facts.get(generator_id)
        if generator_fact is None:
            raise SystemExit(f"missing generator state fact: {generator_id}")
        if generator_fact.get("state_field") != "next_id" or generator_fact.get("range") != "u32":
            raise SystemExit(f"unexpected generator state shape: {generator_id}")
        if generator_fact.get("transport") != transport or generator_fact.get("mutation") != mutation:
            raise SystemExit(f"unexpected generator state transport/mutation: {generator_id}")

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

    for method, plan_kind, field, source_field, transport in [
        ("CoreContext::next_value", "GeneratorNextScalar", "value_next_id", "value_gen", "ValueIdAsI64"),
        ("CoreContext::peek_next_value", "GeneratorPeekScalar", "value_next_id", "value_gen", "ValueIdAsI64"),
        ("CoreContext::next_block", "GeneratorNextScalar", "block_next_id", "block_gen", "BasicBlockIdAsI64"),
        ("CoreContext::peek_next_block", "GeneratorPeekScalar", "block_next_id", "block_gen", "BasicBlockIdAsI64"),
    ]:
        plan_entry = plans.get(method)
        if plan_entry is None or plan_entry.get("plan_kind") != plan_kind:
            raise SystemExit(f"unexpected generator plan entry: {method}")
        if plan_entry.get("field") != field or plan_entry.get("source_field") != source_field:
            raise SystemExit(f"unexpected generator plan field: {method}")
        if plan_entry.get("transport") != transport:
            raise SystemExit(f"unexpected nominal transport: {method}")
    for op_name in ["next_binding", "next_temp_slot", "next_debug_join"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing oracle op: {op_name}")
    for op_name in ["next_value", "peek_next_value", "next_block", "peek_next_block"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing generator oracle op: {op_name}")
    if oracle.get("promotion_scope", {}).get("mirbuilder_wide_claim") is not False:
        raise SystemExit("CoreContext oracle must not claim MirBuilder-wide parity")


def _core_context_api_methods(facts: dict[str, Any], plan: dict[str, Any]) -> list[ApiMethodSpec]:
    return [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("core_context.scalar_counter_context", facts, plan)
    ]


def _core_context_contract(
    facts: dict[str, Any],
    plan: dict[str, Any],
) -> VerifiedFamilyArtifactContractV1:
    plan_entries = [row for row in plan.get("plans", []) if str(row.get("id", "")).startswith("CoreContext::")]
    method_universe = tuple(row["id"] for row in plan_entries)
    selected_method_ids = method_universe
    expected_fields = tuple(dict.fromkeys(row["field"] for row in plan_entries if "field" in row))
    nominal_transports = {
        row["transport"]: {
            "physical_lane": row["physical_lane"],
            "nominal_type": row["nominal_type"],
            "raw_i64_equivalence": row["raw_i64_equivalence"],
        }
        for row in facts.get("nominal_id_transport_plan", [])
    }
    if nominal_transports.get("ValueIdAsI64", {}).get("raw_i64_equivalence") is not False:
        raise ValueError("ValueIdAsI64 must not be raw-i64 equivalent")
    if nominal_transports.get("BasicBlockIdAsI64", {}).get("raw_i64_equivalence") is not False:
        raise ValueError("BasicBlockIdAsI64 must not be raw-i64 equivalent")
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::core_context",
        method_universe=method_universe,
        selected_method_ids=selected_method_ids,
        denials=(),
        semantic_transports={
            "value_id_transport": "ValueIdAsI64",
            "basic_block_id_transport": "BasicBlockIdAsI64",
            "generator_object_transport": 0,
            "invalid_id_claim": 0,
            "reserved_value_id_skipping_claim": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::core_context",
            api_name="CoreContextApi",
            pilot_scope="CoreContext_scalar_counters_and_id_generators",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/core_context.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json",
        ),
        selected_body_count_label="scalar_counter_methods_plus_newtype_id_generators",
        expected_fields=expected_fields,
    )


def core_context_contract() -> VerifiedFamilyArtifactContractV1:
    facts = extract_core_context_facts(CORE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "core-context-plan-v0.json")
    return _core_context_contract(facts, plan)


def core_context_spec() -> FamilyArtifactSpec:
    facts = extract_core_context_facts(CORE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "core-context-plan-v0.json")
    contract = _core_context_contract(facts, plan)
    api_methods = _core_context_api_methods(facts, plan)
    contract.require_selected_recipe_methods([f"CoreContext::{method.signature.split('(', 1)[0]}" for method in api_methods])
    methods = [
        BehaviorMethodSpec(id="CoreContext::next_binding", rust_operation="BindingId::new + saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_binding(ctx)"),
        BehaviorMethodSpec(id="CoreContext::next_temp_slot", rust_operation="saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_temp_slot(ctx)"),
        BehaviorMethodSpec(id="CoreContext::next_debug_join", rust_operation="saturating_add", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_debug_join(ctx)"),
        BehaviorMethodSpec(id="CoreContext::next_value", rust_operation="ValueIdGenerator::next normal-range", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_value(ctx)"),
        BehaviorMethodSpec(id="CoreContext::peek_next_value", rust_operation="ValueIdGenerator::peek_next", hako_operation="FieldGet", emits="CoreContextApi.peek_next_value(ctx)"),
        BehaviorMethodSpec(id="CoreContext::next_block", rust_operation="BasicBlockIdGenerator::next normal-range", hako_operation="TakeThenSaturatingIncrementU32", emits="CoreContextApi.next_block(ctx)"),
        BehaviorMethodSpec(id="CoreContext::peek_next_block", rust_operation="BasicBlockIdGenerator::peek_next", hako_operation="FieldGet", emits="CoreContextApi.peek_next_block(ctx)"),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_core_context_artifact.py",
        generator_version="core-context-scalar-counters-and-id-generators-derived-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json",
        family_comment="hakorune_mir_builder::core_context",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(
            name="CoreContext",
            fields=[
                FieldSpec(name="value_next_id", field_type="i64", initializer="0"),
                FieldSpec(name="block_next_id", field_type="i64", initializer="0"),
                FieldSpec(name="next_binding_id", field_type="i64", initializer="0"),
                FieldSpec(name="temp_slot_counter", field_type="i64", initializer="0"),
                FieldSpec(name="debug_join_counter", field_type="i64", initializer="0"),
            ],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="CoreContext"),
            op("StaticCall", target="value0", callee="CoreContextApi.next_value", args=["ctx"]),
            op("AssertEq", left="value0", right=0, fail_message="core_context_next_value_0=fail", fail_code=1),
            op("StaticCall", target="value1", callee="CoreContextApi.next_value", args=["ctx"]),
            op("AssertEq", left="value1", right=1, fail_message="core_context_next_value_1=fail", fail_code=2),
            op("StaticCall", target="peek_value2", callee="CoreContextApi.peek_next_value", args=["ctx"]),
            op("AssertEq", left="peek_value2", right=2, fail_message="core_context_peek_next_value_2=fail", fail_code=3),
            op("StaticCall", target="value2", callee="CoreContextApi.next_value", args=["ctx"]),
            op("AssertEq", left="value2", right=2, fail_message="core_context_next_value_2=fail", fail_code=4),
            op("StaticCall", target="block0", callee="CoreContextApi.next_block", args=["ctx"]),
            op("AssertEq", left="block0", right=0, fail_message="core_context_next_block_0=fail", fail_code=5),
            op("StaticCall", target="peek_block1", callee="CoreContextApi.peek_next_block", args=["ctx"]),
            op("AssertEq", left="peek_block1", right=1, fail_message="core_context_peek_next_block_1=fail", fail_code=6),
            op("StaticCall", target="block1", callee="CoreContextApi.next_block", args=["ctx"]),
            op("AssertEq", left="block1", right=1, fail_message="core_context_next_block_1=fail", fail_code=7),
            op("StaticCall", target="binding0", callee="CoreContextApi.next_binding", args=["ctx"]),
            op("AssertEq", left="binding0", right=0, fail_message="core_context_next_binding_0=fail", fail_code=8),
            op("StaticCall", target="binding1", callee="CoreContextApi.next_binding", args=["ctx"]),
            op("AssertEq", left="binding1", right=1, fail_message="core_context_next_binding_1=fail", fail_code=9),
            op("StaticCall", target="temp0", callee="CoreContextApi.next_temp_slot", args=["ctx"]),
            op("AssertEq", left="temp0", right=0, fail_message="core_context_next_temp_slot_0=fail", fail_code=10),
            op("StaticCall", target="debug0", callee="CoreContextApi.next_debug_join", args=["ctx"]),
            op("AssertEq", left="debug0", right=0, fail_message="core_context_next_debug_join_0=fail", fail_code=11),
            op("StaticCall", target="temp1", callee="CoreContextApi.next_temp_slot", args=["ctx"]),
            op("AssertEq", left="temp1", right=1, fail_message="core_context_next_temp_slot_1=fail", fail_code=12),
            op("StaticCall", target="debug1", callee="CoreContextApi.next_debug_join", args=["ctx"]),
            op("AssertEq", left="debug1", right=1, fail_message="core_context_next_debug_join_1=fail", fail_code=13),
            op("Print", text="core_context_scalar_counters_and_id_generators_derived_artifact=ok"),
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
        pilot_scope="CoreContext_scalar_counters_and_id_generators",
        recipe_subject="hakorune_mir_builder::core_context::CoreContext.scalar_counters_and_id_generators",
        selected_body_count=contract.selected_body_count_label,
        api_name="CoreContextApi",
        api_methods=api_methods,
        methods=methods,
        excluded_methods=list(contract.denied_method_ids),
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "core_context_full_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks=contract.verifier_checks({"rust_facts_input": "verified", "hako_lifecycle_plan": "verified", "hako_behavior_recipe": "verified", "generator_object_transport": 0, "invalid_id_claim": 0, "reserved_value_id_skipping_claim": 0, "unmapped_thir_nodes": 0, "unmapped_mir_side_effects": 0, "unresolved_call_targets": 0, "unclassified_drop_obligations": 0, "mainline_selected": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0}),
        verified_operations=["ScalarCounterInit", "TakeThenSaturatingIncrementU32", "FieldGet"],
        transport_notes=contract.transport_notes({"counter_transport": "i64", "binding_id_transport": "i64_raw"}),
        extra_manifest_fields=contract.manifest_extra_fields(),
    )
