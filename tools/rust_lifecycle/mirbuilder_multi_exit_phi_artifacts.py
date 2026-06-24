#!/usr/bin/env python3
"""Spec and validation for explicit multi-carrier exit PHI conversion."""

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
SOURCE = FIXTURES / "multi-carrier-exit-phi-source.rs"
FACTS = FIXTURES / "multi-carrier-exit-phi-facts-v0.json"
PLAN = FIXTURES / "multi-carrier-exit-phi-plan-v0.json"
ORACLE = FIXTURES / "multi-carrier-exit-phi-oracle-v0.json"


def extract_facts() -> dict[str, Any]:
    return read_json(FACTS)


def validate_multi_exit_phi(facts: dict[str, Any], plan: dict[str, Any], oracle: dict[str, Any]) -> None:
    subject = "hakorune_mir_builder::multi_carrier_exit_phi"
    if facts.get("kind") != "RustLifecycleFacts" or plan.get("kind") != "HakoLifecyclePlan" or oracle.get("kind") != "RustOracleVectors":
        raise SystemExit("unexpected multi-exit-PHI fixture kind")
    if facts.get("subject") != subject or plan.get("subject") != subject or oracle.get("subject") != subject:
        raise SystemExit("multi-exit-PHI subject mismatch")
    method = {row["id"]: row for row in facts.get("body_facts", [])}.get("MultiCarrierExitPhiPilot::project_exit_carriers")
    if method is None or method.get("operation") != "MultiCarrierExitPhi":
        raise SystemExit("multi-exit-PHI operation mismatch")
    if [row["kind"] for row in method.get("exits", [])] != ["break", "continue", "early_return"]:
        raise SystemExit("multi-exit-PHI exit coverage mismatch")
    if len(method.get("carriers", [])) != 2:
        raise SystemExit("multi-exit-PHI carrier count mismatch")
    default_exit = method.get("default_exit")
    if not isinstance(default_exit, dict) or default_exit.get("kind") != "default":
        raise SystemExit("multi-exit-PHI default exit missing")
    default_values = default_exit.get("values")
    if not isinstance(default_values, list) or len(default_values) != len(method.get("carriers", [])):
        raise SystemExit("multi-exit-PHI default carrier count mismatch")
    if [row.get("kind") for row in default_values if isinstance(row, dict)] != ["I64", "I64"]:
        raise SystemExit("multi-exit-PHI default carriers must be i64")
    plans = {row["id"]: row for row in plan.get("plans", [])}
    shape = plans.get("MultiCarrierExitPhiPilot::project_exit_carriers")
    if shape is None or shape.get("shape_rule") != "control.multi_carrier_exit_phi":
        raise SystemExit("multi-exit-PHI shape plan mismatch")
    if shape.get("raw_hako_body") is not False:
        raise SystemExit("multi-exit-PHI raw Hako body must be disabled")


def _check_ops() -> list[Any]:
    oracle = read_json(ORACLE)
    ops: list[Any] = []
    for offset, vector in enumerate(oracle.get("vectors", [])):
        expect = vector.get("expect")
        if not isinstance(expect, list) or len(expect) != 2:
            raise SystemExit("multi-exit-PHI oracle expects two carriers")
        exit_kind = vector.get("exit_kind")
        target = f"exit_{exit_kind}"
        ops.append(op("StaticCall", target=target, callee="MultiCarrierExitPhiPilotApi.project_exit_carriers", args=[str(exit_kind)]))
        ops.append(op("AssertArrayValueEq", array=target, index=0, expected=expect[0], fail_message=f"multi_exit_phi_{exit_kind}_carrier0=fail", fail_code=1 + (offset * 10)))
        ops.append(op("AssertArrayValueEq", array=target, index=1, expected=expect[1], fail_message=f"multi_exit_phi_{exit_kind}_carrier1=fail", fail_code=2 + (offset * 10)))
    return ops


def multi_exit_phi_spec() -> FamilyArtifactSpec:
    facts = extract_facts()
    plan = read_json(PLAN)
    api_methods = [
        ApiMethodSpec(signature=method.signature, operations=[operation.to_json() for operation in method.operations])
        for method in lower_direct_shape_methods("control.multi_carrier_exit_phi", facts, plan)
    ]
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/generate_multi_exit_phi_artifact.py",
        generator_version="multi-carrier-exit-phi-direct-artifact-v0",
        artifact_manifest="lang/generated/rust_derived/hakorune_mir_builder/multi_carrier_exit_phi.artifact.json",
        family_comment="hakorune_mir_builder::multi_carrier_exit_phi",
        using_module="",
        box=BoxSpec(name="MultiCarrierExitPhiPilot", fields=[]),
        main_operations=[*_check_ops(), op("Print", text="multi_carrier_exit_phi_direct_artifact=ok"), op("ReturnI64", return_value=0)],
        family_id="hakorune_mir_builder::multi_carrier_exit_phi",
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "multi_carrier_exit_phi.hako",
        facts_path=FACTS,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=FIXTURES / "multi-carrier-exit-phi-behavior-recipe-v0.json",
        verifier_path=FIXTURES / "multi-carrier-exit-phi-derived-artifact-verifier-result-v0.json",
        pilot_scope="MultiCarrierExitPhi_only",
        recipe_subject="hakorune_mir_builder::multi_carrier_exit_phi",
        selected_body_count="multi_carrier_exit_phi_only",
        static_boxes=[StaticBoxSpec(name="MultiCarrierExitPhiPilotApi", methods=api_methods)],
        methods=[BehaviorMethodSpec(id="MultiCarrierExitPhiPilot::project_exit_carriers", rust_operation="explicit break/continue/early-return/default carrier table", hako_operation="ExplicitMultiExitPhiI64Array", emits="MultiCarrierExitPhiPilotApi.project_exit_carriers(exit_kind)")],
        claims={"generated_hako_manual_edit": 0, "mainline_selected": 0, "full_mirbuilder_crate_claim": 0, "runtime_fallback": 0, "inferred_phi_claim": 0},
        verifier_checks={"rust_facts_input": "fixture_verified", "direct_shape_rule": "control.multi_carrier_exit_phi", "raw_hako_body": 0, "exit_kinds": ["break", "continue", "early_return"], "default_exit": [0, 0], "carrier_count": 2},
        verified_operations=["ExplicitMultiExitPhiI64Array", "ReturnSource"],
        denied_boundaries=["UnstructuredControlFlow", "PhiJoinRequired", "CarrierSensitiveAlias"],
    )


def run_multi_exit_phi_artifact_generator(*, check: bool) -> None:
    spec = multi_exit_phi_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(spec, hako_text=hako_text, recipe_text=recipe_text, verifier_text=verifier_text)
    outputs = [(spec.recipe_path, recipe_text), (spec.verifier_path, verifier_text), (spec.hako_path, hako_text), (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text)]
    run_validated_family_generator(check=check, root=ROOT, unchanged_label="generated_multi_carrier_exit_phi_artifact=unchanged", load_facts=extract_facts, plan_path=PLAN, oracle_path=ORACLE, validate_inputs=validate_multi_exit_phi, outputs_factory=lambda: outputs)
