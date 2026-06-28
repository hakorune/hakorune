#!/usr/bin/env python3
"""Generate the composed-execution closure Hako artifact for the minimal MirBuilder path."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from shared_family_generator import read_json, run_family_generator, stable_json, write_if_changed
from verified_family_artifact_contract import ArtifactIdentity, VerifiedFamilyArtifactContractV1
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
SOURCE = ROOT / "src/mir/builder/module_lifecycle.rs"
PLAN = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-minimal-path-composed-execution-closure-derived-hako-verifier-result-v0.json"


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderMinimalPathComposedExecutionClosurePlanV1":
        raise ValueError("wrong composed execution closure plan kind")
    if "ComposedExecutionClosure" not in (plan.get("available_capabilities") or []):
        raise ValueError("composed execution closure plan lacks capability")
    source_authority = plan.get("source_authority") or {}
    for key in [
        "semantic_closure_report",
        "composed_execution_route",
        "frontier_resolution",
        "adoption_recheck",
    ]:
        if key not in source_authority:
            raise ValueError(f"composed execution closure plan missing source authority: {key}")
    execution_profile = plan.get("execution_profile") or {}
    if execution_profile.get("same_state_handoff") != "Observed":
        raise ValueError("same-state handoff drift")
    if execution_profile.get("selected_existing_contracts_consumed") != "Consumed":
        raise ValueError("selected existing contracts drift")
    if execution_profile.get("generated_hako_executable_closure") != "Open":
        raise ValueError("generated Hako executable closure should start open in the plan")
    closure_policy = plan.get("closure_policy") or {}
    expected_policy = {
        "callsite": "MinimalMirBuilderComposedExecutionClosure::seal",
        "operation": "set generated_hako_executable_closure_closed and route_chain_closed",
        "generated_hako_executable_closure": "Closed",
        "route_chain_closed": "Yes",
    }
    for key, value in expected_policy.items():
        if closure_policy.get(key) != value:
            raise ValueError(f"closure policy drift: {key}")
    result = plan.get("result_contract") or {}
    expected_result = {
        "same_state_handoff_observed": 1,
        "selected_existing_contracts_consumed": 1,
        "generated_hako_executable_closure": "Closed",
        "route_chain_closed": 1,
        "minimal_path_expected_result": "NoErrorReturn",
    }
    for key, value in expected_result.items():
        if result.get(key) != value:
            raise ValueError(f"closure result contract drift: {key}")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"closure plan non-claim must remain 0: {key}")


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    _validate_plan(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderMinimalPathComposedExecutionClosureExecutionProjectionV1",
        "source_plan": "MirBuilderMinimalPathComposedExecutionClosurePlanV1",
        "execution_scope": "PreparedMirBuilderStateShell",
        "inputs": {
            "same_state_handoff_transport": "PreparedMirBuilderStateShell",
            "selected_existing_contracts_transport": "RouteEvidenceOnly",
            "route_transport": "MinimalMirBuilderComposedExecutionRouteV1",
        },
        "methods": {
            "seal": "SetField + ReturnValue",
            "apply": "StaticCall + AssertEq + ReturnI64",
        },
        "behavior": {
            "same_state_handoff_observed": "Yes",
            "selected_existing_contracts_consumed": "Yes",
            "generated_hako_executable_closure": "Closed",
            "route_chain_closed": "Yes",
        },
        "result_transport": "ScalarI64",
        "result_semantics": "Unit",
        "directability": {
            "prepared_closure_projection": "Allow",
            "full_mirbuilder_object_method": "Deny",
        },
        "mutation_frame": {
            "generated_hako_executable_closure_closed": "exclusive",
            "route_chain_closed": "exclusive",
            "same_state_handoff_observed": "read-only",
            "selected_existing_contracts_consumed": "read-only",
        },
        "non_claims": {
            "semantic_plan_closure": 0,
            "full_minimal_path_mainline_selected": 0,
            "hako_adopted": 0,
            "rust_bootstrap_retirement": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "generated_hako_artifact": 0,
        },
    }


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderMinimalPathComposedExecutionClosureDerivedHakoOracleV1",
        "subject": "MinimalMirBuilder composed execution closure seal",
        "vectors": [
            {
                "name": "seal_closes_executable_closure",
                "inputs": {
                    "initial_same_state_handoff_observed": 1,
                    "initial_selected_existing_contracts_consumed": 1,
                    "initial_generated_hako_executable_closure_closed": 0,
                    "initial_route_chain_closed": 0,
                },
                "expect": {
                    "same_state_handoff_observed": 1,
                    "selected_existing_contracts_consumed": 1,
                    "generated_hako_executable_closure_closed": 1,
                    "route_chain_closed": 1,
                    "return_value": 0,
                },
            }
        ],
        "non_claims": {
            "semantic_plan_closure": 0,
            "full_minimal_path_mainline_selected": 0,
            "hako_adopted": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
        },
    }


def _contract(plan: dict[str, Any], projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    if projection.get("kind") != "MirBuilderMinimalPathComposedExecutionClosureExecutionProjectionV1":
        raise ValueError("wrong composed execution closure projection kind")
    if projection.get("source_plan") != "MirBuilderMinimalPathComposedExecutionClosurePlanV1":
        raise ValueError("projection source plan drift")
    if projection.get("execution_scope") != "PreparedMirBuilderStateShell":
        raise ValueError("projection scope drift")
    if projection.get("directability", {}).get("prepared_closure_projection") != "Allow":
        raise ValueError("prepared closure projection must be allowed")
    if projection.get("directability", {}).get("full_mirbuilder_object_method") != "Deny":
        raise ValueError("full MirBuilder object method must remain denied")
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::minimal_path_composed_execution_closure",
        method_universe=("MinimalPathComposedExecutionClosure::seal",),
        selected_method_ids=("MinimalPathComposedExecutionClosure::seal",),
        denials=(),
        semantic_transports={
            "same_state_handoff_transport": "PreparedMirBuilderStateShell",
            "selected_existing_contracts_transport": "RouteEvidenceOnly",
            "route_transport": "MinimalMirBuilderComposedExecutionRouteV1",
            "generated_hako_executable_closure": "Closed",
            "route_chain_closed": 1,
            "full_mirbuilder_object_method": "Deny",
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::minimal_path_composed_execution_closure",
            api_name="MinimalPathComposedExecutionClosureApi",
            pilot_scope="MinimalMirBuilderComposedExecutionClosure_prepared_state_only",
            artifact_path=(
                "lang/generated/rust_derived/hakorune_mir_builder/"
                "mirbuilder_minimal_path_composed_execution_closure.hako"
            ),
            manifest_path=(
                "lang/generated/rust_derived/hakorune_mir_builder/"
                "mirbuilder_minimal_path_composed_execution_closure.artifact.json"
            ),
        ),
        selected_body_count_label="minimal_path_composed_execution_closure_prepared_state_only",
        expected_fields=(
            "same_state_handoff_observed",
            "selected_existing_contracts_consumed",
            "generated_hako_executable_closure_closed",
            "route_chain_closed",
        ),
    )


def composed_execution_closure_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderMinimalPathComposedExecutionClosureDerivedHakoOracleV1":
        raise ValueError("closure oracle kind drift")
    contract = _contract(plan, projection)
    methods = [
        BehaviorMethodSpec(
            id="MinimalPathComposedExecutionClosure::seal",
            rust_operation="MinimalMirBuilder composed execution closure seal",
            hako_operation="SetField + ReturnValue",
            emits="MinimalPathComposedExecutionClosureApi.seal(state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-minimal-path-composed-execution-closure"
        ),
        generator_version="mirbuilder-minimal-path-composed-execution-closure-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="MinimalPathComposedExecutionClosureKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="PreparedComposedExecutionClosureStateShellBox",
                fields=[
                    FieldSpec(name="same_state_handoff_observed", field_type="i64", initializer="1"),
                    FieldSpec(name="selected_existing_contracts_consumed", field_type="i64", initializer="1"),
                    FieldSpec(name="generated_hako_executable_closure_closed", field_type="i64", initializer="0"),
                    FieldSpec(name="route_chain_closed", field_type="i64", initializer="0"),
                ],
            )
        ],
        static_boxes=[
            StaticBoxSpec(
                name="MinimalPathComposedExecutionClosureApi",
                methods=[
                    ApiMethodSpec(
                        signature="seal(state): PreparedComposedExecutionClosureStateShellBox",
                        operations=[
                            op(
                                "SetField",
                                target="state",
                                field="generated_hako_executable_closure_closed",
                                value=1,
                            ).to_json(),
                            op("SetField", target="state", field="route_chain_closed", value=1).to_json(),
                            op("ReturnValue", value="state").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="state", box="PreparedComposedExecutionClosureStateShellBox"),
            op(
                "StaticCall",
                target="sealed",
                callee="MinimalPathComposedExecutionClosureApi.seal",
                args=["state"],
            ),
            op(
                "AssertEq",
                left="sealed.same_state_handoff_observed",
                right=1,
                fail_message="composed_execution_closure_same_state=fail",
                fail_code=1,
            ),
            op(
                "AssertEq",
                left="sealed.selected_existing_contracts_consumed",
                right=1,
                fail_message="composed_execution_closure_contracts=fail",
                fail_code=2,
            ),
            op(
                "AssertEq",
                left="sealed.generated_hako_executable_closure_closed",
                right=1,
                fail_message="composed_execution_closure_closed=fail",
                fail_code=3,
            ),
            op(
                "AssertEq",
                left="sealed.route_chain_closed",
                right=1,
                fail_message="composed_execution_closure_route_chain=fail",
                fail_code=4,
            ),
            op("Print", text="mirbuilder_minimal_path_composed_execution_closure_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedMainline",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_minimal_path_composed_execution_closure.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MinimalMirBuilder.composed_execution_closure",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "generated_hako_change": 1,
            "same_state_handoff_observed": 1,
            "selected_existing_contracts_consumed": 1,
            "generated_hako_executable_closure": 1,
            "route_chain_closed": 1,
            "mainline_selected": 1,
            "source_selfhost_claim": 0,
            "backend_behavior_changed": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
        },
        verifier_checks=contract.verifier_checks(
            {
                "same_state_handoff_observed": 1,
                "selected_existing_contracts_consumed": 1,
                "generated_hako_executable_closure_closed": 1,
                "route_chain_closed": 1,
                "generated_hako_change": 1,
                "runtime_fallback": 0,
                "new_backend_route": 0,
                "new_abi": 0,
                "source_selfhost_claim": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "PreparedComposedExecutionClosureStateShellBox",
            "MinimalPathComposedExecutionClosureApi",
        ],
        transport_notes=contract.transport_notes(
            {
                "same_state_handoff_observed": "read-only",
                "selected_existing_contracts_consumed": "read-only",
                "generated_hako_executable_closure": "closed by seal",
            }
        ),
        denied_boundaries=[
            "semantic_plan_closure",
            "full_minimal_path_mainline_selected",
            "hako_adopted",
            "rust_bootstrap_retirement",
            "new_backend_route",
            "new_abi",
            "runtime_fallback",
            "source_selfhost_claim",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle_text = stable_json(build_oracle())
    projection_text = stable_json(projection)
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    if not PROJECTION.exists():
        raise FileNotFoundError(f"{PROJECTION} must be written before manifest hashing")
    spec = composed_execution_closure_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = [
        (PROJECTION, projection_text),
        (ORACLE, oracle_text),
    ]
    if recipe_text is not None:
        outputs.append((RECIPE, recipe_text))
    if verifier_text is not None:
        outputs.append((VERIFIER, verifier_text))
    outputs.append((Path(spec.hako_path), hako_text))
    outputs.append((Path(spec.artifact_manifest), manifest_text))
    return outputs


def run(*, check: bool) -> None:
    outputs = _outputs()
    for path, text in outputs:
        if check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise ValueError(f"{path} is stale")
        else:
            write_if_changed(path, text)

    print("output_contract=rust-lifecycle-mirbuilder-minimal-path-composed-execution-closure-v0")
    print("composed_execution_closure_guard=green")
    print("same_state_handoff_observed=1")
    print("selected_existing_contracts_consumed=1")
    print("generated_hako_executable_closure=Closed")
    print("route_chain_closed=1")
    print("generated_hako_change=1")
    print("runtime_fallback=0")
    print("new_backend_route=0")
    print("new_abi=0")
    print("source_selfhost_claim=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except ValueError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
