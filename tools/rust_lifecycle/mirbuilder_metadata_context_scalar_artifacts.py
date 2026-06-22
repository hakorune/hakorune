#!/usr/bin/env python3
"""Spec and validation for MetadataContext scalar/source-file conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_metadata_context_scalar_facts import SOURCE as METADATA_CONTEXT_SOURCE, extract_facts
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


def validate_metadata_context_scalar(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::metadata_context::MetadataContext.scalar_source_file"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected MetadataContext scalar/source-file fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("MetadataContext scalar/source-file subject mismatch")
    type_fact = {row["id"]: row for row in facts["type_facts"]}.get("MetadataContext")
    if type_fact is None or type_fact.get("selected_concrete_instantiation") != "MetadataContext<i64, i64>":
        raise SystemExit("MetadataContext concrete instantiation mismatch")
    if type_fact.get("generic_wide_claim") is not False:
        raise SystemExit("MetadataContext scalar/source-file must not claim wide generic support")
    fields = {row["id"]: row for row in facts["field_facts"]}
    if fields.get("MetadataContext.current_span", {}).get("transport") != "i64":
        raise SystemExit("MetadataContext current_span transport mismatch")
    if fields.get("MetadataContext.source_file", {}).get("transport") != "OptionStringBox":
        raise SystemExit("MetadataContext source_file transport mismatch")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["MetadataContext"].get("shape_rule") != "metadata.scalar_option_atom":
        raise SystemExit("MetadataContext must use metadata.scalar_option_atom")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    for method, operation in {
        "MetadataContext::new": "ConstructScalarOptionContext",
        "MetadataContext::current_span": "FieldGet",
        "MetadataContext::set_current_span": "FieldSet",
        "MetadataContext::set_source_file": "SetSome",
        "MetadataContext::clear_source_file": "ClearOption",
        "MetadataContext::current_source_file": "CloneImmutableString",
    }.items():
        if body_facts.get(method, {}).get("operation") != operation:
            raise SystemExit(f"unexpected MetadataContext body fact: {method}")
    for op_name in ["new", "current_span", "set_current_span", "set_source_file", "current_source_file_some", "clear_source_file", "current_source_file_none"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing MetadataContext oracle op: {op_name}")


def metadata_context_scalar_spec() -> FamilyArtifactSpec:
    facts = extract_facts(METADATA_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "metadata-context-scalar-source-file-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("metadata.scalar_option_atom", facts, plan)
    ]
    excluded = [
        "MetadataContext::hint_scope_enter",
        "MetadataContext::hint_scope_leave",
        "MetadataContext::hint_join_result",
        "MetadataContext::push_region",
        "MetadataContext::pop_region",
        "MetadataContext::current_region_stack",
        "MetadataContext::record_value_span",
        "MetadataContext::value_span",
        "MetadataContext::record_value_caller",
        "MetadataContext::value_caller",
        "MetadataContext::value_origin_callers",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_metadata_context_scalar_artifact.py",
        generator_version="metadata-context-scalar-source-file-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/metadata_context_scalar_source_file.artifact.json",
        family_comment="hakorune_mir_builder::metadata_context::MetadataContext.scalar_source_file",
        using_module="",
        extra_using_modules=[],
        box=BoxSpec(
            name="MetadataContext",
            fields=[
                FieldSpec(name="current_span", field_type="i64", initializer="0"),
                FieldSpec(name="source_file", field_type="Option<StringBox>", initializer="Option::None()"),
            ],
        ),
        main_operations=[
            op("StaticCall", target="ctx", callee="MetadataContextApi.create", args=["10"]),
            op("StaticCall", target="initial_span", callee="MetadataContextApi.current_span", args=["ctx"]),
            op("AssertEq", left="initial_span", right="10", fail_message="metadata_context_initial_span=fail", fail_code=1),
            op("StaticCall", callee="MetadataContextApi.set_current_span", args=["ctx", "20"]),
            op("StaticCall", target="next_span", callee="MetadataContextApi.current_span", args=["ctx"]),
            op("AssertEq", left="next_span", right="20", fail_message="metadata_context_set_span=fail", fail_code=2),
            op("StaticCall", callee="MetadataContextApi.set_source_file", args=["ctx", {"literal": "main.hako"}]),
            op("StaticCall", target="source_file", callee="MetadataContextApi.current_source_file", args=["ctx"]),
            op("AssertEq", left="source_file", right={"expr": "Option::Some(\"main.hako\")"}, fail_message="metadata_context_source_file_some=fail", fail_code=3),
            op("StaticCall", callee="MetadataContextApi.clear_source_file", args=["ctx"]),
            op("StaticCall", target="missing_source", callee="MetadataContextApi.current_source_file", args=["ctx"]),
            op("AssertEq", left="missing_source", right={"expr": "Option::None()"}, fail_message="metadata_context_source_file_none=fail", fail_code=4),
            op("Print", text="metadata_context_scalar_source_file_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::metadata_context",
        state="DerivedShadow",
        source_rust_file=METADATA_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "metadata_context_scalar_source_file.hako",
        facts_path=FIXTURES / "metadata-context-scalar-source-file-facts-v0.json",
        plan_path=FIXTURES / "metadata-context-scalar-source-file-plan-v0.json",
        oracle_path=FIXTURES / "metadata-context-scalar-source-file-oracle-v0.json",
        recipe_path=FIXTURES / "metadata-context-scalar-source-file-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "metadata-context-scalar-source-file-derived-artifact-verifier-result-v0.json",
        pilot_scope="MetadataContext_scalar_source_file_only",
        recipe_subject="hakorune_mir_builder::metadata_context::MetadataContext.scalar_source_file",
        selected_body_count="scalar_source_file_methods_only",
        api_name="MetadataContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="MetadataContext::new", rust_operation="MetadataContext::new", hako_operation="NewBoxWithFieldValues", emits="MetadataContextApi.create(current_span)"),
            BehaviorMethodSpec(id="MetadataContext::current_span", rust_operation="field read current_span", hako_operation="FieldGet", emits="MetadataContextApi.current_span(ctx)"),
            BehaviorMethodSpec(id="MetadataContext::set_current_span", rust_operation="field write current_span", hako_operation="FieldSet", emits="MetadataContextApi.set_current_span(ctx, span)"),
            BehaviorMethodSpec(id="MetadataContext::set_source_file", rust_operation="Option::Some(source.into())", hako_operation="SetSome", emits="MetadataContextApi.set_source_file(ctx, source)"),
            BehaviorMethodSpec(id="MetadataContext::clear_source_file", rust_operation="Option::None", hako_operation="ClearOption", emits="MetadataContextApi.clear_source_file(ctx)"),
            BehaviorMethodSpec(id="MetadataContext::current_source_file", rust_operation="Option<String>::clone", hako_operation="CloneImmutableString", emits="MetadataContextApi.current_source_file(ctx)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_metadata_context_claim": 0, "generic_metadata_context_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "metadata.scalar_option_atom", "selected_body_count": "scalar_source_file_methods_only", "returned_aggregate_alias": 0, "returned_slice_claim": 0, "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0, "borrow_escape_analysis_claim": 0, "needs_semantic_capsule": 0},
        verified_operations=["NewBoxWithFieldValues", "FieldGet", "FieldSet", "SetSome", "ClearOption", "CloneImmutableString"],
        transport_notes={"span_transport": "i64", "region_transport": "i64", "source_file_transport": "OptionStringBox", "generic_wide_claim": 0},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_metadata_context_scalar_artifact_generator(*, check: bool) -> None:
    spec = metadata_context_scalar_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [
        (spec.recipe_path, recipe_text),
        (spec.verifier_path, verifier_text),
        (spec.hako_path, hako_text),
        (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text),
    ]
    run_validated_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_metadata_context_scalar_source_file_artifact=unchanged",
        load_facts=lambda: extract_facts(METADATA_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_metadata_context_scalar,
        outputs_factory=lambda: outputs,
    )
