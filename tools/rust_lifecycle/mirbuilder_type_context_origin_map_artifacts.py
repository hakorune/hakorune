#!/usr/bin/env python3
"""Spec and validation for TypeContext.value_origin_newbox conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_type_context_origin_map_facts import SOURCE as TYPE_CONTEXT_SOURCE, extract_facts
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


def validate_type_context_origin_map(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::type_context::TypeContext.value_origin_newbox"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected TypeContext origin-map fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("TypeContext origin-map subject mismatch")
    field = {row["id"]: row for row in facts["field_facts"]}.get("TypeContext.value_origin_newbox")
    if field is None or field.get("rust_type") != "BTreeMap<ValueId, String>":
        raise SystemExit("TypeContext.value_origin_newbox field fact mismatch")
    if field.get("key_transport") != "ValueIdAsI64":
        raise SystemExit("TypeContext.value_origin_newbox key transport mismatch")
    if field.get("value_transport") != "ImmutableStringAtom":
        raise SystemExit("TypeContext.value_origin_newbox value transport mismatch")
    if field.get("map_identity_escapes") is not False:
        raise SystemExit("TypeContext.value_origin_newbox map identity must not escape")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["TypeContext.value_origin_newbox"].get("shape_rule") != "map.optional_immutable_atom":
        raise SystemExit("TypeContext.value_origin_newbox must use map.optional_immutable_atom")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    for method, operation in {
        "TypeContext::new": "NewMap",
        "TypeContext::get_origin_box": "MapGetOption",
        "TypeContext::set_origin_box": "MapSet",
        "TypeContext::clear_origin_boxes": "MapClear",
    }.items():
        if body_facts.get(method, {}).get("operation") != operation:
            raise SystemExit(f"unexpected TypeContext origin-map body fact: {method}")
    for op_name in ["new", "set_origin_box", "get_origin_box_some", "clear_origin_boxes", "get_origin_box_none"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing TypeContext origin-map oracle op: {op_name}")


def type_context_origin_map_spec() -> FamilyArtifactSpec:
    facts = extract_facts(TYPE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "type-context-origin-map-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("map.optional_immutable_atom", facts, plan)
    ]
    excluded = [
        "TypeContext::get_type",
        "TypeContext::set_type",
        "TypeContext::try_get_kind",
        "TypeContext::get_kind",
        "TypeContext::set_kind",
        "TypeContext::take_snapshot",
        "TypeContext::restore_snapshot",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family type_context_origin_map",
        generator_version="type-context-origin-map-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/type_context_origin_map.artifact.json",
        family_comment="hakorune_mir_builder::type_context::TypeContext.value_origin_newbox",
        using_module="",
        extra_using_modules=[],
        box=BoxSpec(
            name="TypeContext",
            fields=[
                FieldSpec(name="value_origin_newbox", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            ],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="TypeContext"),
            op("StaticCall", callee="TypeContextApi.set_origin_box", args=["ctx", "7", {"literal": "ArrayBox"}]),
            op("StaticCall", target="origin", callee="TypeContextApi.get_origin_box", args=["ctx", "7"]),
            op("AssertEq", left="origin", right={"expr": "Option::Some(\"ArrayBox\")"}, fail_message="type_context_get_origin_box_some=fail", fail_code=1),
            op("StaticCall", callee="TypeContextApi.clear_origin_boxes", args=["ctx"]),
            op("StaticCall", target="missing", callee="TypeContextApi.get_origin_box", args=["ctx", "7"]),
            op("AssertEq", left="missing", right={"expr": "Option::None()"}, fail_message="type_context_get_origin_box_none=fail", fail_code=2),
            op("Print", text="type_context_origin_map_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::type_context",
        state="DerivedShadow",
        source_rust_file=TYPE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "type_context_origin_map.hako",
        facts_path=FIXTURES / "type-context-origin-map-facts-v0.json",
        plan_path=FIXTURES / "type-context-origin-map-plan-v0.json",
        oracle_path=FIXTURES / "type-context-origin-map-oracle-v0.json",
        recipe_path=FIXTURES / "type-context-origin-map-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "type-context-origin-map-derived-artifact-verifier-result-v0.json",
        pilot_scope="TypeContext_value_origin_newbox_only",
        recipe_subject="hakorune_mir_builder::type_context::TypeContext.value_origin_newbox",
        selected_body_count="origin_map_methods_only",
        api_name="TypeContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="TypeContext::get_origin_box", rust_operation="BTreeMap::get map as_str", hako_operation="MapGetOption", emits="TypeContextApi.get_origin_box(ctx, value_id)"),
            BehaviorMethodSpec(id="TypeContext::set_origin_box", rust_operation="BTreeMap::insert String", hako_operation="MapSet", emits="TypeContextApi.set_origin_box(ctx, value_id, class_name)"),
            BehaviorMethodSpec(id="TypeContext::clear_origin_boxes", rust_operation="BTreeMap::clear", hako_operation="MapClear", emits="TypeContextApi.clear_origin_boxes(ctx)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_type_context_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "map.optional_immutable_atom", "selected_body_count": "origin_map_methods_only", "returned_aggregate_alias": 0, "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0, "borrow_escape_analysis_claim": 0, "needs_semantic_capsule": 0},
        verified_operations=["NewMap", "MapGetOption", "MapSet", "MapClear"],
        transport_notes={"key_transport": "ValueIdAsI64", "value_transport": "ImmutableStringAtom", "return_transport": "OptionStringBox"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_type_context_origin_map_artifact_generator(*, check: bool) -> None:
    spec = type_context_origin_map_spec()
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
        unchanged_label="generated_type_context_origin_map_artifact=unchanged",
        load_facts=lambda: extract_facts(TYPE_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_type_context_origin_map,
        outputs_factory=lambda: outputs,
    )
