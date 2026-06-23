#!/usr/bin/env python3
"""Spec and validation for MetadataContext.value_caller direct conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_metadata_context_value_caller_facts import SOURCE as METADATA_CONTEXT_SOURCE, extract_facts
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json, run_validated_family_generator, stable_json, write_if_changed
from mirbuilder_storage_access_facts import ELIDE_TO_LEAF_PROJECTION, classify_storage_access, storage_access_from_borrow_use
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {operation["op"] for vector in oracle["vectors"] for operation in vector["operations"]}


def validate_metadata_value_caller(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::metadata_context::MetadataContext.value_caller"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected MetadataContext value-caller fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("MetadataContext value-caller subject mismatch")
    field = {row["id"]: row for row in facts["field_facts"]}.get("MetadataContext.value_origin_callers")
    if field is None or field.get("value_transport") != "ImmutableStringAtom":
        raise SystemExit("MetadataContext value_origin_callers transport mismatch")
    if field.get("map_identity_escapes") is not False:
        raise SystemExit("MetadataContext value_origin_callers map identity must not escape")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["MetadataContext.value_origin_callers"].get("shape_rule") != "map.immutable_leaf_projection":
        raise SystemExit("MetadataContext value_caller must use map.immutable_leaf_projection")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    if body_facts.get("MetadataContext::value_caller", {}).get("value_projection") != "ImmutableStringAtom":
        raise SystemExit("MetadataContext value_caller projection mismatch")
    if body_facts.get("MetadataContext::value_caller", {}).get("returned_aggregate_alias") is not False:
        raise SystemExit("MetadataContext value_caller must not return aggregate alias")
    borrow_use = {row["id"]: row for row in facts.get("borrow_use_facts", [])}
    use_fact = borrow_use.get("MetadataContext::value_origin_callers.get_cloned")
    if use_fact is None:
        raise SystemExit("MetadataContext value_origin_callers get/cloned borrow-use fact missing")
    storage_fact = storage_access_from_borrow_use(use_fact)
    if classify_storage_access(storage_fact) != ELIDE_TO_LEAF_PROJECTION:
        raise SystemExit("MetadataContext value_origin_callers get/cloned must lower by ElideToLeafProjection")
    for op_name in ["prefill_value_caller", "value_caller_some", "value_caller_none"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing MetadataContext value-caller oracle op: {op_name}")


def metadata_value_caller_spec() -> FamilyArtifactSpec:
    facts = extract_facts(METADATA_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "metadata-context-value-caller-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("map.immutable_leaf_projection", facts, plan)
    ]
    excluded = ["MetadataContext::record_value_caller", "MetadataContext::value_origin_callers", "MetadataContext::current_region_stack"]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_metadata_value_caller_artifact.py",
        generator_version="metadata-context-value-caller-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_value_caller.artifact.json",
        family_comment="hakorune_mir_builder::metadata_context::MetadataContext.value_caller",
        using_module="",
        extra_using_modules=[],
        box=BoxSpec(
            name="MetadataContext",
            fields=[FieldSpec(name="value_origin_callers", field_type="MapBox", initializer_operation={"kind": "NewMap"})],
        ),
        static_boxes=[
            StaticBoxSpec(name="MetadataContextApi", methods=api_methods),
            StaticBoxSpec(
                name="MetadataContextHarnessApi",
                methods=[
                    ApiMethodSpec(
                        signature="prefill_value_caller(ctx, value_id, caller): i64",
                        operations=[op("MapSet", field="value_origin_callers", key="value_id", value="caller", storage="MapBox").to_json()],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="ctx", box="MetadataContext"),
            op("StaticCall", callee="MetadataContextHarnessApi.prefill_value_caller", args=["ctx", "7", {"literal": "main.hako:1:2"}]),
            op("StaticCall", target="caller", callee="MetadataContextApi.value_caller", args=["ctx", "7"]),
            op("AssertEq", left="caller", right={"expr": "Option::Some(\"main.hako:1:2\")"}, fail_message="metadata_context_value_caller_some=fail", fail_code=1),
            op("StaticCall", target="missing", callee="MetadataContextApi.value_caller", args=["ctx", "99"]),
            op("AssertEq", left="missing", right={"expr": "Option::None()"}, fail_message="metadata_context_value_caller_none=fail", fail_code=2),
            op("Print", text="metadata_context_value_caller_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::metadata_context",
        state="DerivedShadow",
        source_rust_file=METADATA_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "metadata_context_value_caller.hako",
        facts_path=FIXTURES / "metadata-context-value-caller-facts-v0.json",
        plan_path=FIXTURES / "metadata-context-value-caller-plan-v0.json",
        oracle_path=FIXTURES / "metadata-context-value-caller-oracle-v0.json",
        recipe_path=FIXTURES / "metadata-context-value-caller-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "metadata-context-value-caller-derived-artifact-verifier-result-v0.json",
        pilot_scope="MetadataContext_value_caller_only",
        recipe_subject="hakorune_mir_builder::metadata_context::MetadataContext.value_caller",
        selected_body_count="value_caller_method_only",
        methods=[BehaviorMethodSpec(id="MetadataContext::value_caller", rust_operation="HashMap::get map as_str", hako_operation="MapGetOption", emits="MetadataContextApi.value_caller(ctx, value_id)")],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_metadata_context_claim": 0, "generic_metadata_context_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "map.immutable_leaf_projection", "storage_access_normalized": 1, "borrow_lowering_decision": "ElideToLeafProjection", "selected_body_count": "value_caller_method_only", "returned_aggregate_alias": 0, "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0, "needs_semantic_capsule": 0},
        verified_operations=["MapGetOption"],
        transport_notes={"key_transport": "ValueIdAsI64", "value_transport": "ImmutableStringAtom", "return_transport": "OptionStringBox", "source_access_order": "Unobserved"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_metadata_value_caller_artifact_generator(*, check: bool) -> None:
    spec = metadata_value_caller_spec()
    facts = extract_facts(METADATA_CONTEXT_SOURCE)
    facts_text = stable_json(facts)
    if not check:
        write_if_changed(spec.facts_path, facts_text)
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [(spec.facts_path, facts_text), (spec.recipe_path, recipe_text), (spec.verifier_path, verifier_text), (spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)]
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_metadata_context_value_caller_artifact=unchanged",
        load_facts=lambda: extract_facts(METADATA_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_metadata_value_caller,
        outputs_factory=lambda: outputs,
    )
