#!/usr/bin/env python3
"""Generate the derived Hako artifact for MirBuilder record/packed layout refresh."""

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
SOURCE = ROOT / "src/mir/semantic_refresh.rs"
PLAN = FIXTURES / "mirbuilder-record-packed-layout-refresh-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-record-packed-layout-refresh-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-record-packed-layout-refresh-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-record-packed-layout-refresh-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-record-packed-layout-refresh-derived-hako-verifier-result-v0.json"


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


def _record_packed_layout_candidate(plan: dict[str, Any]) -> dict[str, Any]:
    mutates = list((plan.get("result_contract") or {}).get("mutates") or [])
    steps = list((plan.get("refresh_policy") or {}).get("steps") or [])
    return {
        "schema_version": 0,
        "kind": "RecordPackedLayoutRefreshShadowCandidateV1",
        "family_id": "hakorune_mir_builder::record_packed_layout_refresh",
        "stage_id": "record_packed_layout_refresh",
        "subject": plan["subject"],
        "source_authority": plan["source_authority"]["finalize"],
        "refresh_timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
        "publication_targets": mutates,
        "projected_fields": steps,
        "publication_target_count": len(mutates),
        "projected_field_count": len(steps),
        "mutation_target_count": len(mutates),
        "step_count": len(steps),
        "entrypoint": plan["result_contract"]["entrypoint"],
    }


def _record_packed_layout_payload_box(name: str, data: dict[str, Any]) -> BoxSpec:
    return BoxSpec(
        name=name,
        fields=[
            FieldSpec(name="payload", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            FieldSpec(name="shadow_json", field_type="StringBox", initializer='""'),
        ],
    )


def _payload_method_ops(target: str, payload: dict[str, Any], box_name: str = "RecordPackedLayoutRefreshPayloadBox") -> list[dict[str, Any]]:
    return _build_map_ops(target, payload, target) + [
        op("NewBox", target="result", box=box_name).to_json(),
        op("SetField", target="result", field="payload", value=target).to_json(),
        op("SetField", target="result", field="shadow_json", value=_literal(stable_json(payload))).to_json(),
        op("ReturnValue", value="result").to_json(),
    ]


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    if plan.get("kind") != "MirBuilderRecordPackedLayoutRefreshPlanV1":
        raise ValueError("wrong record/packed layout refresh plan kind")
    if "RecordAndPackedLayoutRefresh" not in (plan.get("available_capabilities") or []):
        raise ValueError("record/packed layout plan lacks RecordAndPackedLayoutRefresh capability")
    result_contract = plan.get("result_contract") or {}
    return {
        "schema_version": 0,
        "kind": "RecordPackedLayoutRefreshExecutionProjectionV1",
        "source_plan": "MirBuilderRecordPackedLayoutRefreshPlanV1",
        "execution_scope": "PreparedRecordPackedLayoutRefreshState",
        "source": "refresh_module_record_and_packed_layout_plans(module)",
        "build_provider": "semantic_refresh::refresh_module_record_and_packed_layout_plans",
        "target": "module.metadata",
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
        "result_transport": "RecordPackedLayoutRefreshResultBox",
        "result_semantics": "CompositeBoundarySummary",
        "directability": {
            "prepared_shadow_projection": "Allow",
            "host_env_lookup": "Deny",
            "full_metadata_context": "Deny",
            "record_packed_layout_refresh": "Allow",
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
            "record_packed_layout_field_value_type_refresh": 0,
            "record_packed_layout_collection_field_element_refresh": 0,
            "typed_object_plan_refresh": 0,
            "direct_state_plan_refresh": 0,
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
    candidate = _record_packed_layout_candidate(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderRecordPackedLayoutRefreshDerivedHakoOracleV1",
        "subject": plan["subject"],
        "vectors": [
            {
                "name": "record_packed_layout_shadow_candidate_canonical_parity",
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
            "record_packed_layout_field_value_type_refresh": 0,
            "record_packed_layout_collection_field_element_refresh": 0,
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
    if plan.get("kind") != "MirBuilderRecordPackedLayoutRefreshPlanV1":
        raise ValueError("wrong record/packed layout refresh plan kind")
    if "RecordAndPackedLayoutRefresh" not in (plan.get("available_capabilities") or []):
        raise ValueError("missing RecordAndPackedLayoutRefresh capability")
    if plan.get("subject") != "MirBuilder::finalize_module record/packed layout refresh":
        raise ValueError("record/packed layout plan subject drift")
    source_authority = plan.get("source_authority") or {}
    expected_source = {
        "finalize": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module",
        "refresh_entrypoint": "src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans",
        "predecessor_plan": "mirbuilder-module-metadata-publication-plan-v0.json",
    }
    for key, value in expected_source.items():
        if source_authority.get(key) != value:
            raise ValueError(f"record/packed layout source authority drift: {key}")
    execution_profile = plan.get("execution_profile") or {}
    if execution_profile.get("input") != "ASTNode::Literal(Integer(0))":
        raise ValueError("record/packed layout execution input drift")
    if execution_profile.get("context") != "finalize_module":
        raise ValueError("record/packed layout execution context drift")
    if execution_profile.get("module_transport") != "MirModuleMinimalShell":
        raise ValueError("record/packed layout module transport drift")
    refresh = plan.get("refresh_policy") or {}
    expected_refresh = {
        "entrypoint": "refresh_module_record_and_packed_layout_plans",
        "timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
        "module_arg": "&mut MirModule",
    }
    for key, value in expected_refresh.items():
        if refresh.get(key) != value:
            raise ValueError(f"record/packed layout refresh policy drift: {key}")
    expected_steps = [
        "refresh_module_record_layout_plans",
        "refresh_module_array_record_storage_plans",
        "refresh_module_array_record_autouse_eligibility_plans",
        "refresh_module_array_record_materialization_boundary_plans",
        "refresh_module_array_record_packed_autouse_pilot_plans",
        "refresh_module_source_packed_array_autouse_pilot_plans",
        "refresh_module_source_packed_array_direct_read_consumption_plans",
        "refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans",
        "refresh_module_hako_alloc_huge_page_packed_store_pilot_plans",
    ]
    if refresh.get("steps") != expected_steps:
        raise ValueError("record/packed layout refresh step order drift")
    if (plan.get("available_capabilities") or []).count("RecordAndPackedLayoutRefresh") != 1:
        raise ValueError("record/packed layout capability should appear exactly once")
    result = plan.get("result_contract") or {}
    expected_mutates = [
        "module.metadata.record_layout_plans",
        "module.metadata.array_record_storage_plans",
        "module.metadata.array_record_autouse_eligibility_plans",
        "module.metadata.array_record_materialization_boundary_plans",
        "module.metadata.array_record_packed_autouse_pilot_plans",
        "module.metadata.source_packed_array_autouse_pilot_plans",
        "module.metadata.source_packed_array_direct_read_consumption_plans",
        "module.metadata.hako_alloc_aligned_small_packed_store_pilot_plans",
        "module.metadata.hako_alloc_huge_page_packed_store_pilot_plans",
    ]
    if result.get("mutates") != expected_mutates:
        raise ValueError("record/packed layout mutation frame drift")
    if result.get("entrypoint") != "semantic_refresh::refresh_module_record_and_packed_layout_plans":
        raise ValueError("record/packed layout entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("record/packed layout expected result drift")
    non_claims = plan.get("non_claims") or {}
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"record/packed layout non-claim must remain 0: {key}")


def _contract(plan: dict[str, Any], projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::record_packed_layout_refresh",
        method_universe=("RecordPackedLayoutRefreshApi::project_shadow_record",),
        selected_method_ids=("RecordPackedLayoutRefreshApi::project_shadow_record",),
        denials=(),
        semantic_transports={
            "plan_transport": "MapBox",
            "python_oracle_transport": "MapBox",
            "hako_shadow_transport": "MapBox",
            "parity_gate_transport": "MapBox",
            "promotion_token_transport": "MapBox",
            "retirement_token_transport": "MapBox",
            "result_transport": "RecordPackedLayoutRefreshResultBox",
            "shadow_json_transport": "StringBox",
            "projection_contract": "RecordPackedLayoutRefreshHakoProjector",
            "publication_target_count": 9,
            "projected_field_count": 9,
            "mutation_target_count": 9,
            "entrypoint": "semantic_refresh::refresh_module_record_and_packed_layout_plans",
            "refresh_timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::record_packed_layout_refresh",
            api_name="RecordPackedLayoutRefreshApi",
            pilot_scope="RecordPackedLayoutRefresh_prepared_record_packed_layout_refresh_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_record_packed_layout_refresh.artifact.json",
        ),
        selected_body_count_label="record_packed_layout_refresh_prepared_record_packed_layout_refresh_only",
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


def record_packed_layout_refresh_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle(plan)
    _validate_plan(plan)
    contract = _contract(plan, projection)
    methods = [
        BehaviorMethodSpec(
            id="RecordPackedLayoutRefreshApi::project_shadow_record",
            rust_operation="MirBuilder::finalize_module record/packed layout refresh",
            hako_operation="StaticCall + SetField + ReturnValue",
            emits=(
                "RecordPackedLayoutRefreshApi.project_shadow_record(plan, python_oracle, "
                "hako_shadow, parity_gate, promotion_token, retirement_token)"
            ),
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    candidate = _record_packed_layout_candidate(plan)
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-record-packed-layout-refresh"
        ),
        generator_version="mirbuilder-record-packed-layout-refresh-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="RecordPackedLayoutRefreshKernel", fields=[]),
        additional_boxes=[
            _payload_box("RecordPackedLayoutRefreshPayloadBox"),
            BoxSpec(
                name="RecordPackedLayoutRefreshResultBox",
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
                name="RecordPackedLayoutRefreshFixtureApi",
                methods=[
                    ApiMethodSpec(
                        signature="build_plan(): RecordPackedLayoutRefreshPayloadBox",
                        operations=_payload_method_ops("plan", plan),
                    ),
                    ApiMethodSpec(
                        signature="build_shadow_candidate(): RecordPackedLayoutRefreshPayloadBox",
                        operations=_payload_method_ops("candidate", candidate),
                    ),
                    ApiMethodSpec(
                        signature="build_python_oracle(): RecordPackedLayoutRefreshPayloadBox",
                        operations=[
                            op("StaticCall", target="oracle", callee="RecordPackedLayoutRefreshFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="oracle").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_hako_shadow(): RecordPackedLayoutRefreshPayloadBox",
                        operations=[
                            op("StaticCall", target="shadow", callee="RecordPackedLayoutRefreshFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="shadow").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_parity_gate(): RecordPackedLayoutRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "parity_gate",
                            {
                                "schema_version": 0,
                                "kind": "RecordPackedLayoutRefreshParityGateV1",
                                "family_id": "hakorune_mir_builder::record_packed_layout_refresh",
                                "stage_id": "record_packed_layout_refresh",
                                "comparison": "canonical_json",
                                "expected_match": 1,
                            },
                        ),
                    ),
                    ApiMethodSpec(
                        signature="build_promotion_token(): RecordPackedLayoutRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "promotion_token",
                            {
                                "schema_version": 0,
                                "kind": "RecordPackedLayoutRefreshPromotionTokenV1",
                                "family_id": "hakorune_mir_builder::record_packed_layout_refresh",
                                "stage_id": "record_packed_layout_refresh",
                                "value": "promotion",
                            },
                        ),
                    ),
                    ApiMethodSpec(
                        signature="build_retirement_token(): RecordPackedLayoutRefreshPayloadBox",
                        operations=_payload_method_ops(
                            "retirement_token",
                            {
                                "schema_version": 0,
                                "kind": "RecordPackedLayoutRefreshRetirementTokenV1",
                                "family_id": "hakorune_mir_builder::record_packed_layout_refresh",
                                "stage_id": "record_packed_layout_refresh",
                                "value": "retirement",
                            },
                        ),
                    ),
                ],
            ),
            StaticBoxSpec(
                name="RecordPackedLayoutRefreshApi",
                methods=[
                    ApiMethodSpec(
                        signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): RecordPackedLayoutRefreshResultBox",
                        operations=[
                            op("NewBox", target="result", box="RecordPackedLayoutRefreshResultBox").to_json(),
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
            op("StaticCall", target="plan", callee="RecordPackedLayoutRefreshFixtureApi.build_plan", args=[]),
            op("StaticCall", target="python_oracle", callee="RecordPackedLayoutRefreshFixtureApi.build_python_oracle", args=[]),
            op("StaticCall", target="hako_shadow", callee="RecordPackedLayoutRefreshFixtureApi.build_hako_shadow", args=[]),
            op("StaticCall", target="parity_gate", callee="RecordPackedLayoutRefreshFixtureApi.build_parity_gate", args=[]),
            op("StaticCall", target="promotion_token", callee="RecordPackedLayoutRefreshFixtureApi.build_promotion_token", args=[]),
            op("StaticCall", target="retirement_token", callee="RecordPackedLayoutRefreshFixtureApi.build_retirement_token", args=[]),
            op("AssertEq", left="python_oracle.shadow_json", right="hako_shadow.shadow_json", fail_message="record_packed_layout_oracle_shadow_parity=fail", fail_code=1),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "kind")'}, right={"literal": "MirBuilderRecordPackedLayoutRefreshPlanV1"}, fail_message="record_packed_layout_plan_kind=fail", fail_code=2),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "subject")'}, right={"literal": "MirBuilder::finalize_module record/packed layout refresh"}, fail_message="record_packed_layout_plan_subject=fail", fail_code=3),
            op(
                "StaticCall",
                target="result",
                callee="RecordPackedLayoutRefreshApi.project_shadow_record",
                args=[
                    "plan",
                    "python_oracle",
                    "hako_shadow",
                    "parity_gate",
                    "promotion_token",
                    "retirement_token",
                ],
            ),
            op("AssertEq", left="result.err", right=0, fail_message="record_packed_layout_err=fail", fail_code=4),
            op("AssertEq", left="result.err_line", right={"literal": ""}, fail_message="record_packed_layout_err_line=fail", fail_code=5),
            op("AssertEq", left="result.shadow_json", right="hako_shadow.shadow_json", fail_message="record_packed_layout_shadow_json=fail", fail_code=6),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "kind")'}, right={"literal": "RecordPackedLayoutRefreshShadowCandidateV1"}, fail_message="record_packed_layout_shadow_record_kind=fail", fail_code=7),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "family_id")'}, right={"literal": "hakorune_mir_builder::record_packed_layout_refresh"}, fail_message="record_packed_layout_shadow_record_family=fail", fail_code=8),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "stage_id")'}, right={"literal": "record_packed_layout_refresh"}, fail_message="record_packed_layout_shadow_record_stage=fail", fail_code=9),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "source_authority")'}, right={"literal": "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module"}, fail_message="record_packed_layout_shadow_record_authority=fail", fail_code=10),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "refresh_timing")'}, right={"literal": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh"}, fail_message="record_packed_layout_shadow_record_timing=fail", fail_code=11),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "publication_target_count")'}, right=9, fail_message="record_packed_layout_shadow_record_publication_target_count=fail", fail_code=12),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "projected_field_count")'}, right=9, fail_message="record_packed_layout_shadow_record_projected_field_count=fail", fail_code=13),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "mutation_target_count")'}, right=9, fail_message="record_packed_layout_shadow_record_mutation_count=fail", fail_code=14),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "step_count")'}, right=9, fail_message="record_packed_layout_shadow_record_step_count=fail", fail_code=15),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 0)'}, right={"literal": "module.metadata.record_layout_plans"}, fail_message="record_packed_layout_shadow_record_target0=fail", fail_code=16),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 8)'}, right={"literal": "module.metadata.hako_alloc_huge_page_packed_store_pilot_plans"}, fail_message="record_packed_layout_shadow_record_target8=fail", fail_code=17),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 0)'}, right={"literal": "refresh_module_record_layout_plans"}, fail_message="record_packed_layout_shadow_record_field0=fail", fail_code=18),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "projected_fields"), 8)'}, right={"literal": "refresh_module_hako_alloc_huge_page_packed_store_pilot_plans"}, fail_message="record_packed_layout_shadow_record_field8=fail", fail_code=19),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "entrypoint")'}, right={"literal": "semantic_refresh::refresh_module_record_and_packed_layout_plans"}, fail_message="record_packed_layout_shadow_record_entrypoint=fail", fail_code=20),
            op("Print", text="mirbuilder_record_packed_layout_refresh_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_record_packed_layout_refresh.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.record_packed_layout_refresh",
        selected_body_count=contract.selected_body_count_label,
        api_name=contract.artifact.api_name,
        api_methods=[
            ApiMethodSpec(
                signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): RecordPackedLayoutRefreshResultBox",
                operations=[
                    op(
                        "StaticCall",
                        target="projected",
                        callee="RecordPackedLayoutRefreshHakoProjector.project_shadow_record",
                        args=[
                            "plan",
                            "python_oracle",
                            "hako_shadow",
                            "parity_gate",
                            "promotion_token",
                            "retirement_token",
                        ],
                    ).to_json(),
                    op("NewBox", target="result", box="RecordPackedLayoutRefreshResultBox").to_json(),
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
            "record_packed_layout_refresh": 1,
            "record_packed_layout_field_value_type_refresh": 0,
            "record_packed_layout_collection_field_element_refresh": 0,
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
                "record_packed_layout_refresh_only": 1,
                "canonical_json_parity": 1,
                "plan_kind": "MirBuilderRecordPackedLayoutRefreshPlanV1",
                "plan_subject": "MirBuilder::finalize_module record/packed layout refresh",
                "publication_target_count": 9,
                "projected_field_count": 9,
                "mutation_target_count": 9,
                "entrypoint": "semantic_refresh::refresh_module_record_and_packed_layout_plans",
                "refresh_timing": "AfterModuleMetadataPublicationBeforeTypedObjectRefresh",
                "record_packed_layout_refresh": 1,
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
                "result_transport": "RecordPackedLayoutRefreshResultBox",
                "shadow_json_transport": "StringBox",
                "projection_contract": "RecordPackedLayoutRefreshHakoProjector",
                "publication_target_count": 9,
                "projected_field_count": 9,
                "mutation_target_count": 9,
            }
        ),
        denied_boundaries=[
            "record_packed_layout_field_value_type_refresh",
            "record_packed_layout_collection_field_element_refresh",
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
    spec = record_packed_layout_refresh_spec()
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


def run_record_packed_layout_refresh_artifact_generator(*, check: bool) -> None:
    if not check:
        plan = read_json(PLAN)
        write_if_changed(PROJECTION, stable_json(build_execution_projection(plan)))
        write_if_changed(ORACLE, stable_json(build_oracle(plan)))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_record_packed_layout_refresh_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_record_packed_layout_refresh_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
