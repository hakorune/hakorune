#!/usr/bin/env python3
"""Spec and validation for single-scalar loop carrier direct conversion."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_direct_shape_lowerer import lower_direct_shape_methods
from shared_family_generator import read_json, run_validated_family_generator
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
SOURCE = FIXTURES / "single-scalar-loop-carrier-source.rs"
FACTS = FIXTURES / "single-scalar-loop-carrier-facts-v0.json"
PLAN = FIXTURES / "single-scalar-loop-carrier-plan-v0.json"
ORACLE = FIXTURES / "single-scalar-loop-carrier-oracle-v0.json"


def extract_facts() -> dict[str, Any]:
    return read_json(FACTS)


def validate_single_scalar_loop_carrier(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::single_scalar_loop_carrier"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected single-scalar-loop fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("single-scalar-loop subject mismatch")
    body = {row["id"]: row for row in facts.get("body_facts", [])}
    method = body.get("SingleScalarLoopCarrierPilot::sum_values")
    if method is None or method.get("operation") != "SingleScalarLoopCarrier":
        raise SystemExit("single-scalar-loop operation mismatch")
    for field in ["break_count", "continue_count", "early_return_count"]:
        if method.get(field) != 0:
            raise SystemExit(f"single scalar loop must not contain {field}")
    carrier = method.get("carrier")
    if not isinstance(carrier, dict) or carrier.get("name") != "sum" or carrier.get("type") != "i64":
        raise SystemExit("single scalar loop carrier mismatch")
    if method.get("phi_required") is not False or method.get("loop_carried_state") != "single_scalar":
        raise SystemExit("single scalar loop must declare exactly one scalar carrier")
    plans = {row["id"]: row for row in plan.get("plans", [])}
    shape = plans.get("SingleScalarLoopCarrierPilot::sum_values")
    if shape is None or shape.get("shape_rule") != "control.single_scalar_loop_carrier":
        raise SystemExit("single-scalar-loop shape plan mismatch")
    if shape.get("raw_hako_body") is not False:
        raise SystemExit("single-scalar-loop raw Hako body must be disabled")


def single_scalar_loop_carrier_spec() -> FamilyArtifactSpec:
    facts = extract_facts()
    plan = read_json(PLAN)
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("control.single_scalar_loop_carrier", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_single_scalar_loop_carrier_artifact.py",
        generator_version="single-scalar-loop-carrier-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/single_scalar_loop_carrier.artifact.json",
        family_comment="hakorune_mir_builder::single_scalar_loop_carrier",
        using_module="",
        box=BoxSpec(name="SingleScalarLoopCarrierPilot", fields=[]),
        main_operations=[
            op("NewArray", target="values"),
            op("ArrayPush", target="values", value=3),
            op("ArrayPush", target="values", value=5),
            op("ArrayPush", target="values", value=8),
            op("StaticCall", target="sum", callee="SingleScalarLoopCarrierPilotApi.sum_values", args=["values"]),
            op("AssertEq", left="sum", right=16, fail_message="single_scalar_loop_sum=fail", fail_code=1),
            op("Print", text="single_scalar_loop_carrier_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::single_scalar_loop_carrier",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "single_scalar_loop_carrier.hako",
        facts_path=FACTS,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=FIXTURES / "single-scalar-loop-carrier-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "single-scalar-loop-carrier-derived-artifact-verifier-result-v0.json",
        pilot_scope="SingleScalarLoopCarrier_only",
        recipe_subject="hakorune_mir_builder::single_scalar_loop_carrier",
        selected_body_count="single_scalar_loop_carrier_only",
        static_boxes=[StaticBoxSpec(name="SingleScalarLoopCarrierPilotApi", methods=api_methods)],
        methods=[
            BehaviorMethodSpec(
                id="SingleScalarLoopCarrierPilot::sum_values",
                rust_operation="while over slice with one i64 carrier",
                hako_operation="StructuredLoop",
                emits="SingleScalarLoopCarrierPilotApi.sum_values(values)",
            )
        ],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_mirbuilder_crate_claim": 0, "runtime_fallback": 0, "phi_claim": 0, "multi_carrier_claim": 0},
        verifier_checks={"rust_facts_input": "fixture_verified", "direct_shape_rule": "control.single_scalar_loop_carrier", "raw_hako_body": 0, "single_scalar_carrier": 1, "break_continue": 0, "early_return": 0, "phi_required": 0},
        verified_operations=["LocalI64", "StructuredLoop", "Assign", "ReturnSource"],
        denied_boundaries=["UnstructuredControlFlow", "PhiJoinRequired", "CarrierSensitiveAlias"],
    )


def run_single_scalar_loop_carrier_artifact_generator(*, check: bool) -> None:
    spec = single_scalar_loop_carrier_spec()
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
        unchanged_label="generated_single_scalar_loop_carrier_artifact=unchanged",
        load_facts=extract_facts,
        plan_path=PLAN,
        oracle_path=ORACLE,
        validate_inputs=validate_single_scalar_loop_carrier,
        outputs_factory=lambda: outputs,
    )
