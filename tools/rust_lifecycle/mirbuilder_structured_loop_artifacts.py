#!/usr/bin/env python3
"""Spec and validation for structured loop direct conversion."""

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
SOURCE = FIXTURES / "structured-loop-without-carried-state-source.rs"
FACTS = FIXTURES / "structured-loop-without-carried-state-facts-v0.json"
PLAN = FIXTURES / "structured-loop-without-carried-state-plan-v0.json"
ORACLE = FIXTURES / "structured-loop-without-carried-state-oracle-v0.json"


def extract_facts() -> dict[str, Any]:
    return read_json(FACTS)


def validate_structured_loop(
    facts: dict[str, Any],
    plan: dict[str, Any],
    oracle: dict[str, Any],
) -> None:
    subject = "hakorune_mir_builder::structured_loop_without_carried_state"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected structured-loop fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("structured-loop subject mismatch")
    body = {row["id"]: row for row in facts.get("body_facts", [])}
    method = body.get("StructuredLoopPilot::copy_values")
    if method is None:
        raise SystemExit("missing structured-loop body fact")
    if method.get("operation") != "StructuredLoopWithoutCarriedState":
        raise SystemExit("structured-loop operation mismatch")
    for field in ["break_count", "continue_count", "early_return_count"]:
        if method.get(field) != 0:
            raise SystemExit(f"structured loop must not contain {field}")
    if method.get("phi_required") is not False or method.get("loop_carried_state") is not False:
        raise SystemExit("structured loop must not require PHI or carried state")
    plans = {row["id"]: row for row in plan.get("plans", [])}
    shape = plans.get("StructuredLoopPilot::copy_values")
    if shape is None or shape.get("shape_rule") != "control.structured_loop_without_carried_state":
        raise SystemExit("structured-loop shape plan mismatch")
    if shape.get("raw_hako_body") is not False:
        raise SystemExit("structured-loop raw Hako body must be disabled")


def structured_loop_spec() -> FamilyArtifactSpec:
    facts = extract_facts()
    plan = read_json(PLAN)
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("control.structured_loop_without_carried_state", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_structured_loop_artifact.py",
        generator_version="structured-loop-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/structured_loop_without_carried_state.artifact.json",
        family_comment="hakorune_mir_builder::structured_loop_without_carried_state",
        using_module="",
        box=BoxSpec(name="StructuredLoopPilot", fields=[]),
        main_operations=[
            op("NewArray", target="values"),
            op("NewArray", target="out"),
            op("ArrayPush", target="values", value=3),
            op("ArrayPush", target="values", value=5),
            op("ArrayPush", target="values", value=8),
            op("StaticCall", target="status", callee="StructuredLoopPilotApi.copy_values", args=["values", "out"]),
            op("AssertEq", left="status", right=0, fail_message="structured_loop_status=fail", fail_code=1),
            op("AssertArrayValueEq", array="out", index=0, expected=3, fail_message="structured_loop_value_0=fail", fail_code=2),
            op("AssertArrayValueEq", array="out", index=1, expected=5, fail_message="structured_loop_value_1=fail", fail_code=3),
            op("AssertArrayValueEq", array="out", index=2, expected=8, fail_message="structured_loop_value_2=fail", fail_code=4),
            op("Print", text="structured_loop_without_carried_state_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::structured_loop_without_carried_state",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "structured_loop_without_carried_state.hako",
        facts_path=FACTS,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=FIXTURES / "structured-loop-without-carried-state-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "structured-loop-without-carried-state-derived-artifact-verifier-result-v0.json",
        pilot_scope="StructuredLoop_without_carried_state_only",
        recipe_subject="hakorune_mir_builder::structured_loop_without_carried_state",
        selected_body_count="structured_loop_without_carried_state_only",
        static_boxes=[StaticBoxSpec(name="StructuredLoopPilotApi", methods=api_methods)],
        methods=[
            BehaviorMethodSpec(
                id="StructuredLoopPilot::copy_values",
                rust_operation="while over slice without break/continue/PHI",
                hako_operation="StructuredLoop",
                emits="StructuredLoopPilotApi.copy_values(values, out)",
            )
        ],
        excluded_methods=[],
        claims={
            "generated_hako_manual_edit": 0,
            "mainline_selected": 0,
            "full_mirbuilder_crate_claim": 0,
            "runtime_fallback": 0,
            "phi_claim": 0,
            "loop_carried_state_claim": 0,
        },
        verifier_checks={
            "rust_facts_input": "fixture_verified",
            "direct_shape_rule": "control.structured_loop_without_carried_state",
            "raw_hako_body": 0,
            "break_continue": 0,
            "early_return": 0,
            "phi_required": 0,
            "loop_carried_state": 0,
        },
        verified_operations=["LocalI64", "StructuredLoop", "ArrayPush", "Assign", "ReturnI64"],
        denied_boundaries=["UnstructuredControlFlow", "LoopCarriedStateRequired", "PhiJoinRequired"],
    )


def run_structured_loop_artifact_generator(*, check: bool) -> None:
    spec = structured_loop_spec()
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
        unchanged_label="generated_structured_loop_artifact=unchanged",
        load_facts=extract_facts,
        plan_path=PLAN,
        oracle_path=ORACLE,
        validate_inputs=validate_structured_loop,
        outputs_factory=lambda: outputs,
    )
