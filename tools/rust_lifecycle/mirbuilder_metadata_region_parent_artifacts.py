#!/usr/bin/env python3
"""Spec and validation for MetadataContext region-parent borrow-use elimination."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_metadata_context_region_parent_facts import SOURCE as METADATA_CONTEXT_SOURCE, extract_facts
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json, run_validated_family_generator
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {operation["op"] for vector in oracle["vectors"] for operation in vector["operations"]}


def validate_metadata_region_parent(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::metadata_context::MetadataContext.region_parent"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected MetadataContext region-parent fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("MetadataContext region-parent subject mismatch")
    field = {row["id"]: row for row in facts["field_facts"]}.get("MetadataContext.current_region_stack")
    if field is None or field.get("transport") != "ArrayBox" or field.get("element_transport") != "i64":
        raise SystemExit("MetadataContext current_region_stack field transport mismatch")
    use = {row["id"]: row for row in facts["borrow_use_facts"]}.get("RegionObserver::parent_region")
    if use is None or use.get("consumer_kind") != "LastCopy" or use.get("escapes") is not False:
        raise SystemExit("MetadataContext region-parent borrow-use fact mismatch")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["MetadataContext.current_region_stack"].get("shape_rule") != "borrow_use.sequence_last_copy":
        raise SystemExit("MetadataContext region-parent must use borrow_use.sequence_last_copy")
    for op_name in ["current_parent_region_none", "push_region", "current_parent_region_some"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing MetadataContext region-parent oracle op: {op_name}")


def metadata_region_parent_spec() -> FamilyArtifactSpec:
    facts = extract_facts(METADATA_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "metadata-context-region-parent-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("borrow_use.sequence_last_copy", facts, plan)
    ]
    excluded = ["MetadataContext::current_region_stack", "MetadataContext::value_origin_callers"]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_metadata_region_parent_artifact.py",
        generator_version="metadata-context-region-parent-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_region_parent.artifact.json",
        family_comment="hakorune_mir_builder::metadata_context::MetadataContext.region_parent",
        using_module="",
        box=BoxSpec(name="MetadataContext", fields=[]),
        api_name="MetadataContextApi",
        api_methods=api_methods,
        main_operations=[
            op("NewArray", target="current_region_stack"),
            op("StaticCall", target="empty_parent", callee="MetadataContextApi.current_parent_region", args=["current_region_stack"]),
            op("AssertEq", left="empty_parent", right={"expr": "Option::None()"}, fail_message="metadata_region_parent_empty=fail", fail_code=1),
            op("StaticCall", callee="MetadataContextApi.push_region", args=["current_region_stack", "10"]),
            op("StaticCall", target="first_parent", callee="MetadataContextApi.current_parent_region", args=["current_region_stack"]),
            op("AssertEq", left="first_parent", right={"expr": "Option::Some(10)"}, fail_message="metadata_region_parent_first=fail", fail_code=2),
            op("StaticCall", callee="MetadataContextApi.push_region", args=["current_region_stack", "20"]),
            op("StaticCall", target="second_parent", callee="MetadataContextApi.current_parent_region", args=["current_region_stack"]),
            op("AssertEq", left="second_parent", right={"expr": "Option::Some(20)"}, fail_message="metadata_region_parent_second=fail", fail_code=3),
            op("Print", text="metadata_context_region_parent_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::metadata_context",
        state="DerivedShadow",
        source_rust_file=METADATA_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "metadata_context_region_parent.hako",
        facts_path=FIXTURES / "metadata-context-region-parent-facts-v0.json",
        plan_path=FIXTURES / "metadata-context-region-parent-plan-v0.json",
        oracle_path=FIXTURES / "metadata-context-region-parent-oracle-v0.json",
        recipe_path=FIXTURES / "metadata-context-region-parent-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "metadata-context-region-parent-derived-artifact-verifier-result-v0.json",
        pilot_scope="MetadataContext_region_parent_only",
        recipe_subject="hakorune_mir_builder::metadata_context::MetadataContext.region_parent",
        selected_body_count="region_parent_methods_only",
        methods=[
            BehaviorMethodSpec(id="MetadataContext::push_region", rust_operation="Vec::push", hako_operation="SequencePush", emits="MetadataContextApi.push_region(current_region_stack, region_id)"),
            BehaviorMethodSpec(id="RegionObserver::parent_region", rust_operation="current_region_stack().last().copied()", hako_operation="SequenceLastOption", emits="MetadataContextApi.current_parent_region(current_region_stack)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_metadata_context_claim": 0, "generic_metadata_context_claim": 0, "runtime_fallback": 0, "rust_bootstrap_retained": 1},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "borrow_use.sequence_last_copy", "borrow_lowering_decision": "ElideToLeafProjection", "raw_aggregate_return": 0, "read_lease_claim": 0, "unresolved_call_targets": 0, "needs_semantic_capsule": 0},
        verified_operations=["SequencePush", "SequenceLastOption"],
        transport_notes={"field_transport": "ArrayBox", "element_transport": "i64", "standalone_current_region_stack": "Deny(ReturnedReadBorrow)"},
        denied_boundaries=["MetadataContext::current_region_stack"],
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_metadata_region_parent_artifact_generator(*, check: bool) -> None:
    spec = metadata_region_parent_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [(spec.recipe_path, recipe_text), (spec.verifier_path, verifier_text), (spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)]
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_metadata_context_region_parent_artifact=unchanged",
        load_facts=lambda: extract_facts(METADATA_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_metadata_region_parent,
        outputs_factory=lambda: outputs,
    )
