#!/usr/bin/env python3
"""Spec and validation for canonical explicit PHI direct conversion."""

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
SOURCE = FIXTURES / "canonical-explicit-phi-source.rs"
FACTS = FIXTURES / "canonical-explicit-phi-facts-v0.json"
PLAN = FIXTURES / "canonical-explicit-phi-plan-v0.json"
ORACLE = FIXTURES / "canonical-explicit-phi-oracle-v0.json"


def extract_facts() -> dict[str, Any]:
    return read_json(FACTS)


def validate_explicit_phi(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::canonical_explicit_phi"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected explicit-PHI fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("explicit-PHI subject mismatch")
    body = {row["id"]: row for row in facts.get("body_facts", [])}
    method = body.get("CanonicalExplicitPhiPilot::choose_value")
    if method is None or method.get("operation") != "CanonicalExplicitPhi":
        raise SystemExit("explicit-PHI operation mismatch")
    if method.get("phi_kind") != "explicit" or method.get("predecessor_count") != 2:
        raise SystemExit("explicit-PHI must have exactly two explicit predecessors")
    if method.get("value_type") != "i64":
        raise SystemExit("explicit-PHI value transport mismatch")
    plans = {row["id"]: row for row in plan.get("plans", [])}
    shape = plans.get("CanonicalExplicitPhiPilot::choose_value")
    if shape is None or shape.get("shape_rule") != "control.canonical_explicit_phi":
        raise SystemExit("explicit-PHI shape plan mismatch")
    if shape.get("raw_hako_body") is not False:
        raise SystemExit("explicit-PHI raw Hako body must be disabled")


def explicit_phi_spec() -> FamilyArtifactSpec:
    facts = extract_facts()
    plan = read_json(PLAN)
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("control.canonical_explicit_phi", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family explicit_phi",
        generator_version="canonical-explicit-phi-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/canonical_explicit_phi.artifact.json",
        family_comment="hakorune_mir_builder::canonical_explicit_phi",
        using_module="",
        box=BoxSpec(name="CanonicalExplicitPhiPilot", fields=[]),
        main_operations=[
            op("StaticCall", target="then_value", callee="CanonicalExplicitPhiPilotApi.choose_value", args=["1"]),
            op("AssertEq", left="then_value", right=10, fail_message="explicit_phi_then=fail", fail_code=1),
            op("StaticCall", target="else_value", callee="CanonicalExplicitPhiPilotApi.choose_value", args=["0"]),
            op("AssertEq", left="else_value", right=20, fail_message="explicit_phi_else=fail", fail_code=2),
            op("Print", text="canonical_explicit_phi_direct_artifact=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id="hakorune_mir_builder::canonical_explicit_phi",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "canonical_explicit_phi.hako",
        facts_path=FACTS,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=FIXTURES / "canonical-explicit-phi-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "canonical-explicit-phi-derived-artifact-verifier-result-v0.json",
        pilot_scope="CanonicalExplicitPhi_only",
        recipe_subject="hakorune_mir_builder::canonical_explicit_phi",
        selected_body_count="canonical_explicit_phi_only",
        static_boxes=[StaticBoxSpec(name="CanonicalExplicitPhiPilotApi", methods=api_methods)],
        methods=[BehaviorMethodSpec(id="CanonicalExplicitPhiPilot::choose_value", rust_operation="two explicit predecessor values", hako_operation="ExplicitPhiI64", emits="CanonicalExplicitPhiPilotApi.choose_value(flag)")],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_mirbuilder_crate_claim": 0, "runtime_fallback": 0, "inferred_phi_claim": 0, "multi_predecessor_phi_claim": 0},
        verifier_checks={"rust_facts_input": "fixture_verified", "direct_shape_rule": "control.canonical_explicit_phi", "raw_hako_body": 0, "explicit_predecessor_count": 2, "value_transport": "i64"},
        verified_operations=["ExplicitPhiI64", "ReturnSource"],
        denied_boundaries=["PhiJoinRequired", "UnsupportedTypeTransport"],
    )


def run_explicit_phi_artifact_generator(*, check: bool) -> None:
    spec = explicit_phi_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [(spec.recipe_path, recipe_text), (spec.verifier_path, verifier_text), (spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)]
    run_validated_family_generator(check=check, root=ROOT, unchanged_label="generated_canonical_explicit_phi_artifact=unchanged", load_facts=extract_facts, plan_path=PLAN, oracle_path=ORACLE, validate_inputs=validate_explicit_phi, outputs_factory=lambda: outputs)
