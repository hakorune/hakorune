#!/usr/bin/env python3
"""Spec and validation for TypeContext.value_kinds direct conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_type_context_value_kind_facts import SOURCE as TYPE_CONTEXT_SOURCE, extract_facts
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


def validate_type_context_value_kind(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::type_context::TypeContext.value_kinds"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected TypeContext value-kind fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("TypeContext value-kind subject mismatch")
    field = {row["id"]: row for row in facts["field_facts"]}.get("TypeContext.value_kinds")
    if field is None or field.get("rust_type") != "HashMap<ValueId, MirValueKind>":
        raise SystemExit("TypeContext.value_kinds field fact mismatch")
    if field.get("key_transport") != "ValueIdAsI64" or field.get("value_transport") != "DirectEnum":
        raise SystemExit("TypeContext.value_kinds transport mismatch")
    if field.get("iteration_observed") is not False:
        raise SystemExit("TypeContext.value_kinds must not claim iteration semantics")
    plans = {row["id"]: row for row in plan["plans"]}
    if plans["TypeContext.value_kinds"].get("shape_rule") != "map.optional_copy_default":
        raise SystemExit("TypeContext.value_kinds must use generic map.optional_copy_default")
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    for method, operation in {
        "TypeContext::new": "NewMap",
        "TypeContext::try_get_kind": "MapGetOption",
        "TypeContext::get_kind": "MapGetDefault",
        "TypeContext::set_kind": "MapSet",
    }.items():
        if body_facts.get(method, {}).get("operation") != operation:
            raise SystemExit(f"unexpected TypeContext body fact: {method}")
    for op_name in ["new", "set_kind", "try_get_kind_some", "try_get_kind_none", "get_kind"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing TypeContext oracle op: {op_name}")


def type_context_value_kind_spec() -> FamilyArtifactSpec:
    facts = extract_facts(TYPE_CONTEXT_SOURCE)
    plan = read_json(FIXTURES / "type-context-value-kind-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("map.optional_copy_default", facts, plan)
    ]
    excluded = [
        "TypeContext::get_type",
        "TypeContext::set_type",
        "TypeContext::get_origin_box",
        "TypeContext::set_origin_box",
        "TypeContext::clear_origin_boxes",
        "TypeContext::take_snapshot",
        "TypeContext::restore_snapshot",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_type_context_value_kind_artifact.py",
        generator_version="type-context-value-kind-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/type_context_value_kind.artifact.json",
        family_comment="hakorune_mir_builder::type_context::TypeContext.value_kinds",
        using_module="",
        extra_using_modules=[],
        enum_declarations=[
            {
                "name": "MirValueKind",
                "variants": [
                    {"name": "Parameter", "payload": "i64"},
                    {"name": "Local", "payload": "i64"},
                    "Constant",
                    "Temporary",
                    "Pinned",
                    "LoopCarrier",
                ],
            }
        ],
        box=BoxSpec(
            name="TypeContext",
            fields=[
                FieldSpec(name="value_kinds", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            ],
        ),
        main_operations=[
            op("NewBox", target="ctx", box="TypeContext"),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "7", {"expr": "MirValueKind::Parameter(2)"}]),
            op("StaticCall", target="try_param", callee="TypeContextApi.try_get_kind", args=["ctx", "7"]),
            op("AssertEq", left="try_param", right={"expr": "Option::Some(MirValueKind::Parameter(2))"}, fail_message="type_context_try_get_parameter=fail", fail_code=1),
            op("StaticCall", target="get_param", callee="TypeContextApi.get_kind", args=["ctx", "7"]),
            op("AssertEq", left="get_param", right={"expr": "MirValueKind::Parameter(2)"}, fail_message="type_context_get_parameter=fail", fail_code=2),
            op("StaticCall", target="try_missing", callee="TypeContextApi.try_get_kind", args=["ctx", "99"]),
            op("AssertEq", left="try_missing", right={"expr": "Option::None()"}, fail_message="type_context_try_get_missing=fail", fail_code=3),
            op("StaticCall", target="get_missing", callee="TypeContextApi.get_kind", args=["ctx", "99"]),
            op("AssertEq", left="get_missing", right={"expr": "MirValueKind::Temporary()"}, fail_message="type_context_get_missing_default=fail", fail_code=4),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "10", {"expr": "MirValueKind::Local(3)"}]),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "11", {"expr": "MirValueKind::Constant()"}]),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "12", {"expr": "MirValueKind::Temporary()"}]),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "13", {"expr": "MirValueKind::Pinned()"}]),
            op("StaticCall", callee="TypeContextApi.set_kind", args=["ctx", "14", {"expr": "MirValueKind::LoopCarrier()"}]),
            op("StaticCall", target="local_kind", callee="TypeContextApi.get_kind", args=["ctx", "10"]),
            op("AssertEq", left="local_kind", right={"expr": "MirValueKind::Local(3)"}, fail_message="type_context_local_roundtrip=fail", fail_code=5),
            op("StaticCall", target="constant_kind", callee="TypeContextApi.get_kind", args=["ctx", "11"]),
            op("AssertEq", left="constant_kind", right={"expr": "MirValueKind::Constant()"}, fail_message="type_context_constant_roundtrip=fail", fail_code=6),
            op("StaticCall", target="temporary_kind", callee="TypeContextApi.get_kind", args=["ctx", "12"]),
            op("AssertEq", left="temporary_kind", right={"expr": "MirValueKind::Temporary()"}, fail_message="type_context_temporary_roundtrip=fail", fail_code=7),
            op("StaticCall", target="pinned_kind", callee="TypeContextApi.get_kind", args=["ctx", "13"]),
            op("AssertEq", left="pinned_kind", right={"expr": "MirValueKind::Pinned()"}, fail_message="type_context_pinned_roundtrip=fail", fail_code=8),
            op("StaticCall", target="carrier_kind", callee="TypeContextApi.get_kind", args=["ctx", "14"]),
            op("AssertEq", left="carrier_kind", right={"expr": "MirValueKind::LoopCarrier()"}, fail_message="type_context_loop_carrier_roundtrip=fail", fail_code=9),
            op("Print", text="type_context_value_kind_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::type_context",
        state="DerivedShadow",
        source_rust_file=TYPE_CONTEXT_SOURCE,
        hako_path=OUT_DIR / "type_context_value_kind.hako",
        facts_path=FIXTURES / "type-context-value-kind-facts-v0.json",
        plan_path=FIXTURES / "type-context-value-kind-plan-v0.json",
        oracle_path=FIXTURES / "type-context-value-kind-oracle-v0.json",
        recipe_path=FIXTURES / "type-context-value-kind-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "type-context-value-kind-derived-artifact-verifier-result-v0.json",
        pilot_scope="TypeContext_value_kinds_only",
        recipe_subject="hakorune_mir_builder::type_context::TypeContext.value_kinds",
        selected_body_count="value_kind_methods_only",
        api_name="TypeContextApi",
        api_methods=api_methods,
        methods=[
            BehaviorMethodSpec(id="TypeContext::try_get_kind", rust_operation="HashMap::get copied", hako_operation="MapGetOption", emits="TypeContextApi.try_get_kind(ctx, value_id)"),
            BehaviorMethodSpec(id="TypeContext::get_kind", rust_operation="HashMap::get copied unwrap_or Temporary", hako_operation="ReturnDefaultIfMissing", emits="TypeContextApi.get_kind(ctx, value_id)"),
            BehaviorMethodSpec(id="TypeContext::set_kind", rust_operation="HashMap::insert", hako_operation="MapSet", emits="TypeContextApi.set_kind(ctx, value_id, kind)"),
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_type_context_claim": 0, "mirbuilder_wide_claim": 0, "rust_bootstrap_retained": 1, "backend_behavior_changed": 0, "source_selfhost_claim": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "map.optional_copy_default", "selected_body_count": "value_kind_methods_only", "unresolved_call_targets": 0, "unmapped_mir_side_effects": 0, "borrow_escape_analysis_claim": 0, "needs_semantic_capsule": 0},
        verified_operations=["NewMap", "MapGetOption", "MapSet", "ReturnDefaultIfMissing"],
        transport_notes={"key_transport": "ValueIdAsI64", "value_transport": "MirValueKindDirectEnum", "u32_payload_transport": "checked_i64"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_type_context_value_kind_artifact_generator(*, check: bool) -> None:
    spec = type_context_value_kind_spec()
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
        unchanged_label="generated_type_context_value_kind_artifact=unchanged",
        load_facts=lambda: extract_facts(TYPE_CONTEXT_SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_type_context_value_kind,
        outputs_factory=lambda: outputs,
    )
