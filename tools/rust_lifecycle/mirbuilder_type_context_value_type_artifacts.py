#!/usr/bin/env python3
"""Spec and validation for TypeContext.value_types direct conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_type_context_value_type_facts import MIR_TYPE_VARIANTS, SOURCE as TYPE_CONTEXT_SOURCE, extract_facts
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


def _payload_variant_names(rows: list[dict[str, Any]]) -> set[str]:
    return {row["name"] for row in rows if "payload" in row}


def validate_type_context_value_type(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::type_context::TypeContext.value_types"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected TypeContext value-type fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("TypeContext value-type subject mismatch")

    field = {row["id"]: row for row in facts["field_facts"]}.get("TypeContext.value_types")
    if field is None or field.get("rust_type") != "BTreeMap<ValueId, MirType>":
        raise SystemExit("TypeContext.value_types field fact mismatch")
    if field.get("key_transport") != "ValueIdAsI64" or field.get("value_transport") != "OwnedRecursiveEnum":
        raise SystemExit("TypeContext.value_types transport mismatch")
    if field.get("map_identity_escapes") is not False:
        raise SystemExit("TypeContext.value_types map identity must not escape")

    type_facts = {row["id"]: row for row in facts["type_facts"]}
    mir_type = type_facts.get("MirType")
    if mir_type is None or mir_type.get("transport") != "OwnedRecursiveEnum" or mir_type.get("recursive") is not True:
        raise SystemExit("MirType must be an owned recursive enum transport")
    if [row["name"] for row in mir_type["variants"]] != [row["name"] for row in MIR_TYPE_VARIANTS]:
        raise SystemExit("MirType variant coverage mismatch")
    if _payload_variant_names(mir_type["variants"]) != {"Box", "Array", "Future"}:
        raise SystemExit("MirType payload variant coverage mismatch")

    plans = {row["id"]: row for row in plan["plans"]}
    if plans["TypeContext.value_types"].get("shape_rule") != "map.optional_owned_recursive_enum":
        raise SystemExit("TypeContext.value_types must use map.optional_owned_recursive_enum")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    for method, operation in {
        "TypeContext::new": "NewMap",
        "TypeContext::get_type": "MapGetOption",
        "TypeContext::set_type": "MapSet",
    }.items():
        if body_facts.get(method, {}).get("operation") != operation:
            raise SystemExit(f"unexpected TypeContext value-type body fact: {method}")
    if body_facts["TypeContext::get_type"].get("returned_borrow_projected_to_owned") is not True:
        raise SystemExit("TypeContext.get_type must project returned borrow to owned enum")

    for op_name in ["new", "set_type", "get_type_some", "get_type_none", "roundtrip_recursive_types"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing TypeContext value-type oracle op: {op_name}")


def _enum_declaration() -> list[dict[str, Any]]:
    variants: list[dict[str, Any] | str] = []
    for variant in MIR_TYPE_VARIANTS:
        payload = variant.get("payload")
        if payload is None:
            variants.append(variant["name"])
        elif variant["name"] == "Box":
            variants.append({"name": "Box", "payload": "StringBox"})
        else:
            variants.append({"name": variant["name"], "payload": "MirType"})
    return [{"name": "MirType", "variants": variants}]


def type_context_value_type_spec() -> FamilyArtifactSpec:
    facts = extract_facts(TYPE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "type-context-value-type-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("map.optional_owned_recursive_enum", facts, plan)
    ]
    excluded = [
        "TypeContext::try_get_kind",
        "TypeContext::get_kind",
        "TypeContext::set_kind",
        "TypeContext::get_origin_box",
        "TypeContext::set_origin_box",
        "TypeContext::clear_origin_boxes",
        "TypeContext::take_snapshot",
        "TypeContext::restore_snapshot",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family type_context_value_type",
        generator_version="type-context-value-type-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/type_context_value_type.artifact.json",
        family_comment="hakorune_mir_builder::type_context::TypeContext.value_types",
        using_module="",
        extra_using_modules=[],
        enum_declarations=_enum_declaration(),
        box=BoxSpec(
            name="TypeContext",
            fields=[FieldSpec(name="value_types", field_type="MapBox", initializer_operation={"kind": "NewMap"})],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="TypeContext"),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "7", {"expr": "MirType::Integer()"}]),
            op("StaticCall", target="integer_type", callee="TypeContextApi.get_type", args=["ctx", "7"]),
            op("AssertEq", left="integer_type", right={"expr": "Option::Some(MirType::Integer())"}, fail_message="type_context_get_type_integer=fail", fail_code=1),
            op("StaticCall", target="missing_type", callee="TypeContextApi.get_type", args=["ctx", "99"]),
            op("AssertEq", left="missing_type", right={"expr": "Option::None()"}, fail_message="type_context_get_type_missing=fail", fail_code=2),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "8", {"expr": "MirType::Box(\"ArrayBox\")"}]),
            op("StaticCall", target="box_type", callee="TypeContextApi.get_type", args=["ctx", "8"]),
            op("AssertEq", left="box_type", right={"expr": "Option::Some(MirType::Box(\"ArrayBox\"))"}, fail_message="type_context_get_type_box=fail", fail_code=3),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "9", {"expr": "MirType::Array(MirType::Integer())"}]),
            op("StaticCall", target="array_type", callee="TypeContextApi.get_type", args=["ctx", "9"]),
            op("AssertEq", left="array_type", right={"expr": "Option::Some(MirType::Array(MirType::Integer()))"}, fail_message="type_context_get_type_array=fail", fail_code=4),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "10", {"expr": "MirType::Future(MirType::Box(\"TaskBox\"))"}]),
            op("StaticCall", target="future_type", callee="TypeContextApi.get_type", args=["ctx", "10"]),
            op("AssertEq", left="future_type", right={"expr": "Option::Some(MirType::Future(MirType::Box(\"TaskBox\")))"}, fail_message="type_context_get_type_future=fail", fail_code=5),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "11", {"expr": "MirType::Float()"}]),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "12", {"expr": "MirType::Bool()"}]),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "13", {"expr": "MirType::String()"}]),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "14", {"expr": "MirType::WeakRef()"}]),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "15", {"expr": "MirType::Void()"}]),
            op("StaticCall", callee="TypeContextApi.set_type", args=["ctx", "16", {"expr": "MirType::Unknown()"}]),
            op("StaticCall", target="unknown_type", callee="TypeContextApi.get_type", args=["ctx", "16"]),
            op("AssertEq", left="unknown_type", right={"expr": "Option::Some(MirType::Unknown())"}, fail_message="type_context_get_type_unknown=fail", fail_code=6),
            op("Print", text="type_context_value_type_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::type_context",
        state="DerivedShadow",
        source_rust_file=TYPE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "type_context_value_type.hako",
        facts_path=FIXTURES / "type-context-value-type-facts-v0.json",
        plan_path=FIXTURES / "type-context-value-type-plan-v0.json",
        oracle_path=FIXTURES / "type-context-value-type-oracle-v0.json",
        recipe_path=FIXTURES / "type-context-value-type-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "type-context-value-type-derived-artifact-verifier-result-v0.json",
        pilot_scope="TypeContext_value_types_only",
        recipe_subject="hakorune_mir_builder::type_context::TypeContext.value_types",
        selected_body_count="value_type_methods_only",
        api_name="TypeContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="TypeContext::get_type", rust_operation="BTreeMap::get as owned MirType projection", hako_operation="MapGetOption", emits="TypeContextApi.get_type(ctx, value_id)"),
            BehaviorMethodSpec(id="TypeContext::set_type", rust_operation="BTreeMap::insert MirType", hako_operation="MapSet", emits="TypeContextApi.set_type(ctx, value_id, ty)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_type_context_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "map.optional_owned_recursive_enum", "selected_body_count": "value_type_methods_only", "returned_aggregate_alias": 0, "returned_borrow_projected_to_owned": 1, "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0, "needs_semantic_capsule": 0},
        verified_operations=["NewMap", "MapGetOption", "MapSet"],
        transport_notes={"key_transport": "ValueIdAsI64", "value_transport": "MirTypeOwnedRecursiveEnum", "return_transport": "OptionMirTypeOwned"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_type_context_value_type_artifact_generator(*, check: bool) -> None:
    spec = type_context_value_type_spec()
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
        unchanged_label="generated_type_context_value_type_artifact=unchanged",
        load_facts=lambda: extract_facts(TYPE_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_type_context_value_type,
        outputs_factory=lambda: outputs,
    )
