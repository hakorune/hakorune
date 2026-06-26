#!/usr/bin/env python3
"""Spec and validation for TypeContext.string_literals leaf projection."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from extract_type_context_string_literal_facts import SOURCE, extract_facts
from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from mirbuilder_optional_map_converter import require_immutable_leaf_projection_plan
from shared_family_generator import read_json, run_validated_family_generator, stable_json, write_if_changed
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"


def _oracle_ops(oracle: dict[str, Any]) -> set[str]:
    return {operation["op"] for vector in oracle["vectors"] for operation in vector["operations"]}


def validate_type_context_string_literal(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::type_context::TypeContext.string_literals"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected TypeContext string-literal fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("TypeContext string-literal subject mismatch")
    field = {row["id"]: row for row in facts["field_facts"]}.get("TypeContext.string_literals")
    if field is None or field.get("rust_type") != "BTreeMap<ValueId, String>":
        raise SystemExit("TypeContext.string_literals field fact mismatch")
    if field.get("key_transport") != "ValueIdAsI64" or field.get("value_transport") != "ImmutableStringAtom":
        raise SystemExit("TypeContext.string_literals transport mismatch")
    if field.get("iteration_observed") is not False or field.get("map_identity_escapes") is not False:
        raise SystemExit("TypeContext.string_literals must be point-lookup only")
    require_immutable_leaf_projection_plan(
        plan,
        plan_id="TypeContext.string_literals",
        error_type=SystemExit,
    )
    body_facts = {row["id"]: row for row in facts["body_facts"]}
    if body_facts.get("map_value::string_literal", {}).get("value_projection") != "ImmutableStringAtom":
        raise SystemExit("TypeContext string_literal projection mismatch")
    for op_name in ["prefill_string_literal", "string_literal_some", "string_literal_none", "owned_first_result"]:
        if op_name not in _oracle_ops(oracle):
            raise SystemExit(f"missing TypeContext string-literal oracle op: {op_name}")


def type_context_string_literal_spec() -> FamilyArtifactSpec:
    facts = extract_facts(SOURCE)
    plan = read_json(FIXTURES / "type-context-string-literal-plan-v0.json")
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("map.immutable_leaf_projection", facts, plan)
    ]
    excluded = [
        "emit_string",
        "TypeContext::take_snapshot",
        "TypeContext::restore_snapshot",
        "TypeContext.map_value_types",
        "TypeContext.map_literal_value_types",
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family type_context_string_literal",
        generator_version="type-context-string-literal-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/type_context_string_literal.artifact.json",
        family_comment="hakorune_mir_builder::type_context::TypeContext.string_literals",
        using_module="",
        extra_using_modules=[],
        box=BoxSpec(
            name="TypeContext",
            fields=[FieldSpec(name="string_literals", field_type="MapBox", initializer_operation={"kind": "NewMap"})],
        ),
        static_boxes=[
            StaticBoxSpec(name="TypeContextApi", methods=api_methods),
            StaticBoxSpec(
                name="TypeContextHarnessApi",
                methods=[
                    ApiMethodSpec(
                        signature="prefill_string_literal(ctx, value_id, text): i64",
                        operations=[op("MapSet", field="string_literals", key="value_id", value="text", storage="MapBox").to_json()],
                    )
                ],
            ),
        ],
        main_operations=[
            op("NewBox", target="ctx", box="TypeContext"),
            op("StaticCall", callee="TypeContextHarnessApi.prefill_string_literal", args=["ctx", "7", {"literal": "alpha"}]),
            op("StaticCall", target="first", callee="TypeContextApi.string_literal", args=["ctx", "7"]),
            op("AssertOptionSomeStringEq", source="first", value_name="first_initial_value", expected={"literal": "alpha"}, fail_message="type_context_string_literal_some=fail", fail_code=1),
            op("StaticCall", target="missing", callee="TypeContextApi.string_literal", args=["ctx", "99"]),
            op("AssertEq", left="missing", right={"expr": "Option::None()"}, fail_message="type_context_string_literal_none=fail", fail_code=2),
            op("StaticCall", callee="TypeContextHarnessApi.prefill_string_literal", args=["ctx", "7", {"literal": "beta"}]),
            op("AssertOptionSomeStringEq", source="first", value_name="first_after_update_value", expected={"literal": "alpha"}, fail_message="type_context_string_literal_owned_clone=fail", fail_code=3),
            op("StaticCall", target="updated", callee="TypeContextApi.string_literal", args=["ctx", "7"]),
            op("AssertOptionSomeStringEq", source="updated", expected={"literal": "beta"}, fail_message="type_context_string_literal_updated=fail", fail_code=4),
            op("StaticCall", callee="TypeContextHarnessApi.prefill_string_literal", args=["ctx", "8", {"literal": "gamma"}]),
            op("StaticCall", target="other", callee="TypeContextApi.string_literal", args=["ctx", "8"]),
            op("AssertOptionSomeStringEq", source="other", expected={"literal": "gamma"}, fail_message="type_context_string_literal_distinct_key=fail", fail_code=5),
            op("Print", text="type_context_string_literal_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::type_context",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "type_context_string_literal.hako",
        facts_path=FIXTURES / "type-context-string-literal-facts-v0.json",
        plan_path=FIXTURES / "type-context-string-literal-plan-v0.json",
        oracle_path=FIXTURES / "type-context-string-literal-oracle-v0.json",
        recipe_path=FIXTURES / "type-context-string-literal-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "type-context-string-literal-derived-artifact-verifier-result-v0.json",
        pilot_scope="TypeContext_string_literals_only",
        recipe_subject="hakorune_mir_builder::type_context::TypeContext.string_literals",
        selected_body_count="string_literal_helper_only",
        methods=[
            BehaviorMethodSpec(id="map_value::string_literal", rust_operation="BTreeMap::get cloned String", hako_operation="MapGetOption", emits="TypeContextApi.string_literal(ctx, value_id)")
        ],
        excluded_methods=excluded,
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_emit_string_claim": 0, "full_map_value_publication_claim": 0, "runtime_fallback": 0, "backend_behavior_changed": 0},
        verifier_checks={"rust_facts_input": "verified", "direct_shape_rule": "map.immutable_leaf_projection", "storage_access_normalized": 1, "borrow_lowering_decision": "ElideToLeafProjection", "selected_body_count": "string_literal_helper_only", "returned_aggregate_alias": 0, "order_observed": 0, "unmapped_mir_side_effects": 0},
        verified_operations=["MapGetOption"],
        transport_notes={"key_transport": "ValueIdAsI64", "value_transport": "ImmutableStringAtom", "return_transport": "OptionStringBox", "source_access_order": "Unobserved"},
        extra_manifest_fields={"excluded_methods": excluded},
    )


def run_type_context_string_literal_artifact_generator(*, check: bool) -> None:
    spec = type_context_string_literal_spec()
    facts = extract_facts(SOURCE)
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
        unchanged_label="generated_type_context_string_literal_artifact=unchanged",
        load_facts=lambda: extract_facts(SOURCE),
        plan_path=spec.plan_path,
        oracle_path=spec.oracle_path,
        validate_inputs=validate_type_context_string_literal,
        outputs_factory=lambda: outputs,
    )
