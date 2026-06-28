#!/usr/bin/env python3
"""Generate the derived Hako artifact for MirBuilder direct state plan refresh."""

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
SOURCE = ROOT / "src/mir/direct_state_plan.rs"
PLAN = FIXTURES / "mirbuilder-direct-state-plan-refresh-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-direct-state-plan-refresh-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-direct-state-plan-refresh-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-direct-state-plan-refresh-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-direct-state-plan-refresh-derived-hako-verifier-result-v0.json"


def _literal(value: str) -> dict[str, str]:
    return {"literal": value}


def _expr(value: str) -> dict[str, str]:
    return {"expr": value}


def _scalar_expr(value: Any) -> Any:
    if isinstance(value, bool):
        return _expr("true" if value else "false")
    if value is None:
        return _expr("null")
    if isinstance(value, str):
        return _literal(value)
    return value


def _sanitize(name: str) -> str:
    out = []
    for char in name:
        if char.isalnum() or char == "_":
            out.append(char)
        else:
            out.append("_")
    return "".join(out)


def _build_map_ops(target: str, data: dict[str, Any], prefix: str) -> list[dict[str, Any]]:
    ops: list[dict[str, Any]] = [op("NewLocalBox", target=target, box="MapBox").to_json()]
    for key, value in data.items():
        ops.extend(_append_value_ops(target, key, value, prefix))
    return ops


def _append_value_ops(parent: str, key: str, value: Any, prefix: str) -> list[dict[str, Any]]:
    if isinstance(value, dict):
        child = f"{prefix}_{_sanitize(key)}"
        ops = _build_map_ops(child, value, child)
        ops.append(op("MethodCall", receiver=parent, method="set", args=[_literal(key), child]).to_json())
        return ops
    if isinstance(value, list):
        child = f"{prefix}_{_sanitize(key)}"
        ops: list[dict[str, Any]] = [op("NewLocalArray", target=child).to_json()]
        for index, item in enumerate(value):
            if isinstance(item, dict):
                nested = f"{child}_{index}"
                ops.extend(_build_map_ops(nested, item, nested))
                ops.append(op("MethodCall", receiver=child, method="push", args=[nested]).to_json())
            else:
                ops.append(op("MethodCall", receiver=child, method="push", args=[_scalar_expr(item)]).to_json())
        ops.append(op("MethodCall", receiver=parent, method="set", args=[_literal(key), child]).to_json())
        return ops
    return [op("MethodCall", receiver=parent, method="set", args=[_literal(key), _scalar_expr(value)]).to_json()]


def _direct_state_candidate(plan: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "DirectStatePlanRefreshShadowCandidateV1",
        "family_id": "hakorune_mir_builder::direct_state_plan_refresh",
        "stage_id": "direct_state_plan_refresh",
        "subject": plan["subject"],
        "source_authority": plan["source_authority"]["finalize"],
        "refresh_timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
        "publication_targets": ["module.metadata.direct_state_plans"],
        "projected_fields": [
            "box_name",
            "state_repr",
            "field_decl_authority",
            "selected_field_count",
            "unsupported_field_count",
            "materialization_boundary_known",
            "positive_net_expected",
            "fields",
        ],
        "mutation_target_count": 1,
        "entrypoint": plan["result_contract"]["entrypoint"],
    }


def _direct_state_payload_box(name: str, data: dict[str, Any]) -> BoxSpec:
    return BoxSpec(
        name=name,
        fields=[
            FieldSpec(name="payload", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            FieldSpec(name="shadow_json", field_type="StringBox", initializer='""'),
        ],
    )


def _payload_method_ops(target: str, payload: dict[str, Any], box_name: str = "DirectStatePlanRefreshPayloadBox") -> list[dict[str, Any]]:
    return _build_map_ops(target, payload, target) + [
        op("NewBox", target="result", box=box_name).to_json(),
        op("SetField", target="result", field="payload", value=target).to_json(),
        op("SetField", target="result", field="shadow_json", value=_literal(stable_json(payload))).to_json(),
        op("ReturnValue", value="result").to_json(),
    ]


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    if plan.get("kind") != "MirBuilderDirectStatePlanRefreshPlanV1":
        raise ValueError("wrong direct state plan refresh plan kind")
    if "DirectStatePlanRefresh" not in (plan.get("available_capabilities") or []):
        raise ValueError("direct state plan lacks DirectStatePlanRefresh capability")
    result_contract = plan.get("result_contract") or {}
    return {
        "schema_version": 0,
        "kind": "DirectStatePlanRefreshExecutionProjectionV1",
        "source_plan": "MirBuilderDirectStatePlanRefreshPlanV1",
        "execution_scope": "PreparedDirectStatePlanRefreshState",
        "inputs": {
            "plan_transport": "MapBox",
            "python_oracle_transport": "MapBox",
            "hako_shadow_transport": "MapBox",
            "parity_gate_transport": "MapBox",
            "promotion_token_transport": "MapBox",
            "retirement_token_transport": "MapBox",
        },
        "methods": {
            "build_plan": "StaticCall + MethodCall + ReturnValue",
            "build_shadow_candidate": "StaticCall + MethodCall + ReturnValue",
            "project_shadow_record": "StaticCall + SetField + ReturnValue",
        },
        "behavior": {
            "shadow_projection": "CanonicalJsonParity",
            "projector_validation": "Yes",
            "shadow_result_published": "Yes",
        },
        "result_transport": "DirectStatePlanRefreshResultBox",
        "result_semantics": "ShadowRecord",
        "directability": {
            "prepared_shadow_projection": "Allow",
            "host_env_lookup": "Deny",
            "full_metadata_context": "Deny",
            "direct_state_plan_refresh": "Allow",
        },
        "mutation_frame": {
            "shadow_record": "exclusive local result",
            "shadow_json": "exclusive local result",
            "plan": "read-only",
            "oracle": "read-only",
            "hako_shadow": "read-only",
            "parity_gate": "read-only",
            "promotion_token": "read-only",
            "retirement_token": "read-only",
        },
        "non_claims": {
            "direct_state_field_value_type_refresh": 0,
            "direct_state_collection_field_element_refresh": 0,
            "full_semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_behavior_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
            "module_metadata_publication": 0,
        },
        "result_contract": {
            "mutates": result_contract.get("mutates", []),
            "entrypoint": result_contract.get("entrypoint"),
            "minimal_path_expected_result": result_contract.get("minimal_path_expected_result"),
        },
    }


def build_oracle(plan: dict[str, Any]) -> dict[str, Any]:
    candidate = _direct_state_candidate(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderDirectStatePlanRefreshDerivedHakoOracleV1",
        "subject": plan["subject"],
        "vectors": [
            {
                "name": "direct_state_shadow_candidate_canonical_parity",
                "inputs": {
                    "plan_subject": plan["subject"],
                    "publication_target_count": len(candidate["publication_targets"]),
                    "projected_field_count": len(candidate["projected_fields"]),
                    "entrypoint": candidate["entrypoint"],
                },
                "expect": {
                    "canonical_json_parity": 1,
                    "shadow_candidate_kind": candidate["kind"],
                    "shadow_candidate_family": candidate["family_id"],
                    "shadow_candidate_stage": candidate["stage_id"],
                    "mutation_target_count": candidate["mutation_target_count"],
                },
            }
        ],
        "non_claims": {
            "direct_state_field_value_type_refresh": 0,
            "direct_state_collection_field_element_refresh": 0,
            "full_semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_behavior_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
            "module_metadata_publication": 0,
        },
    }


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderDirectStatePlanRefreshPlanV1":
        raise ValueError("wrong direct state plan refresh plan kind")
    if "DirectStatePlanRefresh" not in (plan.get("available_capabilities") or []):
        raise ValueError("missing DirectStatePlanRefresh capability")
    if plan.get("subject") != "MirBuilder::finalize_module direct state plan refresh":
        raise ValueError("direct state plan subject drift")
    source_authority = plan.get("source_authority") or {}
    expected_source = {
        "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
        "refresh_entrypoint": "src/mir/direct_state_plan.rs::refresh_module_direct_state_plans",
        "predecessor_plan": "mirbuilder-typed-object-plan-refresh-plan-v0.json",
    }
    for key, value in expected_source.items():
        if source_authority.get(key) != value:
            raise ValueError(f"direct state source authority drift: {key}")
    execution_profile = plan.get("execution_profile") or {}
    if execution_profile.get("input") != "ASTNode::Literal(Integer(0))":
        raise ValueError("direct state execution input drift")
    if execution_profile.get("context") != "finalize_module":
        raise ValueError("direct state execution context drift")
    if execution_profile.get("module_transport") != "MirModuleMinimalShell":
        raise ValueError("direct state module transport drift")
    refresh = plan.get("refresh_policy") or {}
    expected_refresh = {
        "entrypoint": "refresh_module_direct_state_plans",
        "timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
        "operation": "AssignDirectStatePlans",
        "source": "build_direct_state_plans(module)",
        "build_provider": "direct_state_plan::build_direct_state_plans",
        "target": "module.metadata.direct_state_plans",
        "module_arg": "&mut MirModule",
    }
    for key, value in expected_refresh.items():
        if refresh.get(key) != value:
            raise ValueError(f"direct state refresh policy drift: {key}")
    if (plan.get("available_capabilities") or []).count("DirectStatePlanRefresh") != 1:
        raise ValueError("direct state capability should appear exactly once")
    result = plan.get("result_contract") or {}
    if result.get("mutates") != ["module.metadata.direct_state_plans"]:
        raise ValueError("direct state mutation frame drift")
    if result.get("entrypoint") != "direct_state_plan::refresh_module_direct_state_plans":
        raise ValueError("direct state entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("direct state expected result drift")
    non_claims = plan.get("non_claims") or {}
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"direct state non-claim must remain 0: {key}")


def _contract(plan: dict[str, Any], projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::direct_state_plan_refresh",
        method_universe=("DirectStatePlanRefreshApi::project_shadow_record",),
        selected_method_ids=("DirectStatePlanRefreshApi::project_shadow_record",),
        denials=(),
        semantic_transports={
            "plan_transport": "MapBox",
            "python_oracle_transport": "MapBox",
            "hako_shadow_transport": "MapBox",
            "parity_gate_transport": "MapBox",
            "promotion_token_transport": "MapBox",
            "retirement_token_transport": "MapBox",
            "result_transport": "DirectStatePlanRefreshResultBox",
            "shadow_json_transport": "StringBox",
            "projection_contract": "DirectStatePlanRefreshHakoProjector",
            "publication_target_count": 1,
            "projected_field_count": 8,
            "mutation_target_count": 1,
            "entrypoint": "direct_state_plan::refresh_module_direct_state_plans",
            "refresh_timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::direct_state_plan_refresh",
            api_name="DirectStatePlanRefreshApi",
            pilot_scope="DirectStatePlanRefresh_prepared_direct_state_plan_refresh_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_direct_state_plan_refresh.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_direct_state_plan_refresh.artifact.json",
        ),
        selected_body_count_label="direct_state_plan_refresh_prepared_direct_state_plan_refresh_only",
        expected_fields=("err", "err_line", "shadow_record", "shadow_json"),
    )


def _payload_box(name: str) -> BoxSpec:
    return BoxSpec(
        name=name,
        fields=[
            FieldSpec(name="payload", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            FieldSpec(name="shadow_json", field_type="StringBox", initializer='""'),
        ],
    )


def direct_state_plan_refresh_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle(plan)
    _validate_plan(plan)
    contract = _contract(plan, projection)
    methods = [
        BehaviorMethodSpec(
            id="DirectStatePlanRefreshApi::project_shadow_record",
            rust_operation="MirBuilder::finalize_module direct state plan refresh",
            hako_operation="StaticCall + SetField + ReturnValue",
            emits=(
                "DirectStatePlanRefreshApi.project_shadow_record(plan, python_oracle, "
                "hako_shadow, parity_gate, promotion_token, retirement_token)"
            ),
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    candidate = _direct_state_candidate(plan)
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-direct-state-plan-refresh"
        ),
        generator_version="mirbuilder-direct-state-plan-refresh-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="DirectStatePlanRefreshKernel", fields=[]),
        additional_boxes=[
            _payload_box("DirectStatePlanRefreshPayloadBox"),
            BoxSpec(
                name="DirectStatePlanRefreshResultBox",
                fields=[
                    FieldSpec(name="err", field_type="i64", initializer="0"),
                    FieldSpec(name="err_line", field_type="StringBox", initializer='""'),
                    FieldSpec(name="shadow_record", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
                    FieldSpec(name="shadow_json", field_type="StringBox", initializer='""'),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="DirectStatePlanRefreshFixtureApi",
                methods=[
                    ApiMethodSpec(
                        signature="build_plan(): DirectStatePlanRefreshPayloadBox",
                        operations=_payload_method_ops("plan", plan),
                    ),
                    ApiMethodSpec(
                        signature="build_shadow_candidate(): DirectStatePlanRefreshPayloadBox",
                        operations=_payload_method_ops("candidate", candidate),
                    ),
                    ApiMethodSpec(
                        signature="build_python_oracle(): DirectStatePlanRefreshPayloadBox",
                        operations=[
                            op("StaticCall", target="oracle", callee="DirectStatePlanRefreshFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="oracle").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_hako_shadow(): DirectStatePlanRefreshPayloadBox",
                        operations=[
                            op("StaticCall", target="shadow", callee="DirectStatePlanRefreshFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="shadow").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_parity_gate(): DirectStatePlanRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "parity_gate",
                            {
                                "schema_version": 0,
                                "kind": "DirectStatePlanRefreshParityGateV1",
                                "family_id": "hakorune_mir_builder::direct_state_plan_refresh",
                                "stage_id": "direct_state_plan_refresh",
                                "comparison": "canonical_json",
                                "expected_match": 1,
                            },
                        ),
                    ),
                    ApiMethodSpec(
                        signature="build_promotion_token(): DirectStatePlanRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "promotion_token",
                            {
                                "schema_version": 0,
                                "kind": "DirectStatePlanRefreshPromotionTokenV1",
                                "family_id": "hakorune_mir_builder::direct_state_plan_refresh",
                                "stage_id": "direct_state_plan_refresh",
                                "value": "promotion",
                            },
                        ),
                    ),
                    ApiMethodSpec(
                        signature="build_retirement_token(): DirectStatePlanRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "retirement_token",
                            {
                                "schema_version": 0,
                                "kind": "DirectStatePlanRefreshRetirementTokenV1",
                                "family_id": "hakorune_mir_builder::direct_state_plan_refresh",
                                "stage_id": "direct_state_plan_refresh",
                                "value": "retirement",
                            },
                        ),
                    ),
                ],
            ),
            StaticBoxSpec(
                name="DirectStatePlanRefreshApi",
                methods=[
                    ApiMethodSpec(
                        signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): DirectStatePlanRefreshResultBox",
                        operations=[
                            op("NewBox", target="result", box="DirectStatePlanRefreshResultBox").to_json(),
                            op("SetField", target="result", field="err", value=0).to_json(),
                            op("SetField", target="result", field="err_line", value=_literal("")).to_json(),
                            op("SetField", target="result", field="shadow_record", value="hako_shadow.payload").to_json(),
                            op("SetField", target="result", field="shadow_json", value="hako_shadow.shadow_json").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="plan", callee="DirectStatePlanRefreshFixtureApi.build_plan", args=[]),
            op("StaticCall", target="python_oracle", callee="DirectStatePlanRefreshFixtureApi.build_python_oracle", args=[]),
            op("StaticCall", target="hako_shadow", callee="DirectStatePlanRefreshFixtureApi.build_hako_shadow", args=[]),
            op("StaticCall", target="parity_gate", callee="DirectStatePlanRefreshFixtureApi.build_parity_gate", args=[]),
            op("StaticCall", target="promotion_token", callee="DirectStatePlanRefreshFixtureApi.build_promotion_token", args=[]),
            op("StaticCall", target="retirement_token", callee="DirectStatePlanRefreshFixtureApi.build_retirement_token", args=[]),
            op("AssertEq", left="python_oracle.shadow_json", right="hako_shadow.shadow_json", fail_message="direct_state_oracle_shadow_parity=fail", fail_code=1),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "kind")'}, right={"literal": "MirBuilderDirectStatePlanRefreshPlanV1"}, fail_message="direct_state_plan_kind=fail", fail_code=2),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "subject")'}, right={"literal": "MirBuilder::finalize_module direct state plan refresh"}, fail_message="direct_state_plan_subject=fail", fail_code=3),
            op(
                "StaticCall",
                target="result",
                callee="DirectStatePlanRefreshApi.project_shadow_record",
                args=[
                    "plan",
                    "python_oracle",
                    "hako_shadow",
                    "parity_gate",
                    "promotion_token",
                    "retirement_token",
                ],
            ),
            op("AssertEq", left="result.err", right=0, fail_message="direct_state_err=fail", fail_code=4),
            op("AssertEq", left="result.err_line", right={"literal": ""}, fail_message="direct_state_err_line=fail", fail_code=5),
            op("AssertEq", left="result.shadow_json", right="hako_shadow.shadow_json", fail_message="direct_state_shadow_json=fail", fail_code=6),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "kind")'}, right={"literal": "DirectStatePlanRefreshShadowCandidateV1"}, fail_message="direct_state_shadow_record_kind=fail", fail_code=7),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "family_id")'}, right={"literal": "hakorune_mir_builder::direct_state_plan_refresh"}, fail_message="direct_state_shadow_record_family=fail", fail_code=8),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "stage_id")'}, right={"literal": "direct_state_plan_refresh"}, fail_message="direct_state_shadow_record_stage=fail", fail_code=9),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "source_authority")'}, right={"literal": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module"}, fail_message="direct_state_shadow_record_authority=fail", fail_code=10),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "refresh_timing")'}, right={"literal": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization"}, fail_message="direct_state_shadow_record_timing=fail", fail_code=11),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 0)'}, right={"literal": "module.metadata.direct_state_plans"}, fail_message="direct_state_shadow_record_target0=fail", fail_code=12),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 0)'}, right={"literal": "box_name"}, fail_message="direct_state_shadow_record_field0=fail", fail_code=13),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 1)'}, right={"literal": "state_repr"}, fail_message="direct_state_shadow_record_field1=fail", fail_code=14),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 2)'}, right={"literal": "field_decl_authority"}, fail_message="direct_state_shadow_record_field2=fail", fail_code=15),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 3)'}, right={"literal": "selected_field_count"}, fail_message="direct_state_shadow_record_field3=fail", fail_code=16),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 4)'}, right={"literal": "unsupported_field_count"}, fail_message="direct_state_shadow_record_field4=fail", fail_code=17),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 5)'}, right={"literal": "materialization_boundary_known"}, fail_message="direct_state_shadow_record_field5=fail", fail_code=18),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 6)'}, right={"literal": "positive_net_expected"}, fail_message="direct_state_shadow_record_field6=fail", fail_code=19),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 7)'}, right={"literal": "fields"}, fail_message="direct_state_shadow_record_field7=fail", fail_code=20),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "mutation_target_count")'}, right=1, fail_message="direct_state_shadow_record_mutation_count=fail", fail_code=21),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "entrypoint")'}, right={"literal": "direct_state_plan::refresh_module_direct_state_plans"}, fail_message="direct_state_shadow_record_entrypoint=fail", fail_code=22),
            op("Print", text="mirbuilder_direct_state_plan_refresh_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_direct_state_plan_refresh.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.direct_state_plan_refresh",
        selected_body_count=contract.selected_body_count_label,
        api_name=contract.artifact.api_name,
        api_methods=[
            ApiMethodSpec(
                signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): DirectStatePlanRefreshResultBox",
                operations=[
                    op(
                        "StaticCall",
                        target="projected",
                        callee="DirectStatePlanRefreshHakoProjector.project_shadow_record",
                        args=[
                            "plan",
                            "python_oracle",
                            "hako_shadow",
                            "parity_gate",
                            "promotion_token",
                            "retirement_token",
                        ],
                    ).to_json(),
                    op("NewBox", target="result", box="DirectStatePlanRefreshResultBox").to_json(),
                    op("SetField", target="result", field="err", value="projected.err").to_json(),
                    op("SetField", target="result", field="err_line", value="projected.err_line").to_json(),
                    op("SetField", target="result", field="shadow_record", value="projected.shadow_record").to_json(),
                    op("SetField", target="result", field="shadow_json", value="projected.shadow_json").to_json(),
                    op("ReturnValue", value="result").to_json(),
                ],
            )
        ],
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "direct_state_plan_refresh": 1,
            "direct_state_field_value_type_refresh": 0,
            "direct_state_collection_field_element_refresh": 0,
            "module_metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
            "backend_behavior_changed": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
        },
        verifier_checks=contract.verifier_checks(
            {
                "direct_state_plan_refresh_only": 1,
                "canonical_json_parity": 1,
                "plan_kind": "MirBuilderDirectStatePlanRefreshPlanV1",
                "plan_subject": "MirBuilder::finalize_module direct state plan refresh",
                "publication_target_count": 1,
                "projected_field_count": 8,
                "mutation_target_count": 1,
                "entrypoint": "direct_state_plan::refresh_module_direct_state_plans",
                "refresh_timing": "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization",
                "direct_state_plan_refresh": 1,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "NewLocalBox",
            "NewLocalArray",
            "MethodCall",
            "NewBox",
            "SetField",
            "StaticCall",
            "ReturnValue",
        ],
        transport_notes=contract.transport_notes(
            {
                "plan_transport": "MapBox",
                "python_oracle_transport": "MapBox",
                "hako_shadow_transport": "MapBox",
                "parity_gate_transport": "MapBox",
                "promotion_token_transport": "MapBox",
                "retirement_token_transport": "MapBox",
                "result_transport": "DirectStatePlanRefreshResultBox",
                "shadow_json_transport": "StringBox",
                "projection_contract": "DirectStatePlanRefreshHakoProjector",
                "publication_target_count": 1,
                "projected_field_count": 8,
                "mutation_target_count": 1,
            }
        ),
        denied_boundaries=[
            "direct_state_field_value_type_refresh",
            "direct_state_collection_field_element_refresh",
            "module_metadata_publication",
            "semantic_refresh",
            "all_functions_phi_materialization",
            "full_finalize_module",
            "mainline_selected",
            "runtime_fallback",
            "new_backend_route",
            "new_abi",
            "new_canonical_mir_instruction",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle(plan)
    spec = direct_state_plan_refresh_spec()
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
        (PROJECTION, stable_json(projection)),
        (ORACLE, stable_json(oracle)),
    ]
    if recipe_text is not None:
        outputs.append((RECIPE, recipe_text))
    if verifier_text is not None:
        outputs.append((VERIFIER, verifier_text))
    outputs.extend(
        [
            (spec.hako_path, hako_text),
            (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text),
        ]
    )
    return outputs


def run_direct_state_plan_refresh_artifact_generator(*, check: bool) -> None:
    if not check:
        plan = read_json(PLAN)
        write_if_changed(PROJECTION, stable_json(build_execution_projection(plan)))
        write_if_changed(ORACLE, stable_json(build_oracle(plan)))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_direct_state_plan_refresh_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_direct_state_plan_refresh_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
