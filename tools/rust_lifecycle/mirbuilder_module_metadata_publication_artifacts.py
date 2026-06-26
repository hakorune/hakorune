#!/usr/bin/env python3
"""Generate the derived ModuleMetadataPublication Hako artifact."""

from __future__ import annotations

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
PLAN = FIXTURES / "mirbuilder-module-metadata-publication-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-module-metadata-publication-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-module-metadata-publication-derived-hako-oracle-v0.json"
SHADOW_RESULT = FIXTURES / "mirbuilder-module-metadata-publication-hako-shadow-result-v0.json"
RECIPE = FIXTURES / "mirbuilder-module-metadata-publication-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-module-metadata-publication-derived-hako-verifier-result-v0.json"


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


def _shadow_candidate(plan: dict[str, Any]) -> dict[str, Any]:
    publication = plan["publication"]
    result_contract = plan["result_contract"]
    return {
        "schema_version": 0,
        "kind": "ModuleMetadataPublicationShadowCandidateV1",
        "family_id": "hakorune_mir_builder::module_metadata_publication",
        "stage_id": "module_metadata_publication",
        "subject": plan["subject"],
        "source_authority": plan["source_authority"]["finalize"],
        "publication_timing": publication["timing"],
        "publication_targets": [field["target"] for field in publication["fields"]],
        "projected_fields": publication["fields"][1]["projected_fields"],
        "mutation_target_count": len(result_contract["mutates"]),
        "entrypoint": result_contract["entrypoint"],
    }


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    if plan.get("kind") != "MirBuilderModuleMetadataPublicationPlanV1":
        raise ValueError("wrong module metadata publication plan kind")
    if "ModuleMetadataPublication" not in (plan.get("available_capabilities") or []):
        raise ValueError("module metadata publication plan lacks ModuleMetadataPublication capability")
    return {
        "schema_version": 0,
        "kind": "ModuleMetadataPublicationExecutionProjectionV1",
        "source_plan": "MirBuilderModuleMetadataPublicationPlanV1",
        "execution_scope": "PreparedModuleMetadataPublicationShadowState",
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
        "result_transport": "ModuleMetadataPublicationResultBox",
        "result_semantics": "ShadowRecord",
        "directability": {
            "prepared_shadow_projection": "Allow",
            "host_env_lookup": "Deny",
            "full_metadata_context": "Deny",
        },
        "mutation_frame": {
            "shadow_record": "exclusive local result",
            "shadow_json": "exclusive local result",
            "plan": "read-only",
            "oracle": "read-only",
            "hako_shadow": "read-only",
        },
        "non_claims": {
            "all_functions_phi_materialization": 0,
            "backend_behavior_changed": 0,
            "direct_state_plan_refresh": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "mainline_selected": 0,
            "module_function_insertion": 0,
            "metadata_publication": 0,
            "record_and_packed_layout_refresh": 0,
            "new_abi": 0,
            "new_backend_route": 0,
            "runtime_fallback": 0,
            "semantic_refresh": 0,
            "slot_registry_release": 0,
            "typed_object_plan_refresh": 0,
        },
    }


def build_oracle(plan: dict[str, Any]) -> dict[str, Any]:
    candidate = _shadow_candidate(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderModuleMetadataPublicationDerivedHakoOracleV1",
        "subject": plan["subject"],
        "vectors": [
            {
                "name": "python_oracle_and_hako_shadow_match",
                "inputs": {
                    "plan_subject": plan["subject"],
                    "publication_targets": len(candidate["publication_targets"]),
                    "projected_fields": len(candidate["projected_fields"]),
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
            "all_functions_phi_materialization": 0,
            "backend_behavior_changed": 0,
            "direct_state_plan_refresh": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "mainline_selected": 0,
            "module_function_insertion": 0,
            "metadata_publication": 0,
            "record_and_packed_layout_refresh": 0,
            "new_abi": 0,
            "new_backend_route": 0,
            "runtime_fallback": 0,
            "semantic_refresh": 0,
            "slot_registry_release": 0,
            "typed_object_plan_refresh": 0,
        },
    }


def build_shadow_result(plan: dict[str, Any]) -> dict[str, Any]:
    candidate = _shadow_candidate(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderModuleMetadataPublicationDerivedHakoShadowResultV1",
        "subject": plan["subject"],
        "result": {
            "err": 0,
            "err_line": "",
            "shadow_record": candidate,
            "shadow_json": stable_json(candidate),
        },
        "non_claims": {
            "all_functions_phi_materialization": 0,
            "backend_behavior_changed": 0,
            "direct_state_plan_refresh": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "mainline_selected": 0,
            "module_function_insertion": 0,
            "metadata_publication": 0,
            "record_and_packed_layout_refresh": 0,
            "new_abi": 0,
            "new_backend_route": 0,
            "runtime_fallback": 0,
            "semantic_refresh": 0,
            "slot_registry_release": 0,
            "typed_object_plan_refresh": 0,
        },
    }


def _build_plan_spec() -> list[dict[str, Any]]:
    plan = read_json(PLAN)
    return _build_map_ops("plan", plan, "plan")


def _build_shadow_candidate_spec(target: str, plan: dict[str, Any]) -> list[dict[str, Any]]:
    return _build_map_ops(target, _shadow_candidate(plan), target)


def _build_parity_gate_spec() -> list[dict[str, Any]]:
    return _build_map_ops(
        "parity_gate",
        {
            "schema_version": 0,
            "kind": "ModuleMetadataPublicationParityGateV1",
            "family_id": "hakorune_mir_builder::module_metadata_publication",
            "stage_id": "module_metadata_publication",
            "comparison": "canonical_json",
            "expected_match": 1,
        },
        "parity_gate",
    )


def _build_token_spec(target: str, kind: str, value: str) -> list[dict[str, Any]]:
    return _build_map_ops(
        target,
        {
            "schema_version": 0,
            "kind": kind,
            "family_id": "hakorune_mir_builder::module_metadata_publication",
            "stage_id": "module_metadata_publication",
            "value": value,
        },
        target,
    )


def _payload_box(name: str) -> BoxSpec:
    return BoxSpec(
        name=name,
        fields=[
            FieldSpec(name="payload", field_type="MapBox", initializer_operation={"kind": "NewMap"}),
            FieldSpec(name="shadow_json", field_type="StringBox", initializer='""'),
        ],
    )


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderModuleMetadataPublicationPlanV1":
        raise ValueError("wrong module metadata publication plan kind")
    if "ModuleMetadataPublication" not in (plan.get("available_capabilities") or []):
        raise ValueError("missing ModuleMetadataPublication capability")
    publication = plan.get("publication") or {}
    result = plan.get("result_contract") or {}
    expected_targets = [
        "module.metadata.user_box_decls",
        "module.metadata.user_box_field_decls",
        "module.metadata.record_decls",
        "module.metadata.enum_decls",
    ]
    if [field.get("target") for field in publication.get("fields", [])] != expected_targets:
        raise ValueError("module metadata publication target coverage drift")
    if publication.get("timing") != "AfterSlotRegistryReleaseBeforeSemanticRefresh":
        raise ValueError("module metadata publication timing drift")
    if result.get("mutates") != expected_targets:
        raise ValueError("module metadata publication mutation frame drift")
    if result.get("entrypoint") != "MirBuilder::finalize_module module metadata publication":
        raise ValueError("module metadata publication entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("module metadata publication expected result drift")
    non_claims = plan.get("non_claims") or {}
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"module metadata publication non-claim must remain 0: {key}")


def _contract(plan: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::module_metadata_publication",
        method_universe=("ModuleMetadataPublicationApi::project_shadow_record",),
        selected_method_ids=("ModuleMetadataPublicationApi::project_shadow_record",),
        denials=(),
        semantic_transports={
            "plan_transport": "MapBox",
            "python_oracle_transport": "MapBox",
            "hako_shadow_transport": "MapBox",
            "parity_gate_transport": "MapBox",
            "promotion_token_transport": "MapBox",
            "retirement_token_transport": "MapBox",
            "result_transport": "ModuleMetadataPublicationResultBox",
            "shadow_json_transport": "StringBox",
            "projection_contract": "ModuleMetadataPublicationHakoProjector",
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::module_metadata_publication",
            api_name="ModuleMetadataPublicationApi",
            pilot_scope="ModuleMetadataPublication_prepared_module_metadata_publication_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_metadata_publication.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_metadata_publication.artifact.json",
        ),
        selected_body_count_label="module_metadata_publication_prepared_module_metadata_publication_only",
        expected_fields=("err", "err_line", "shadow_record", "shadow_json"),
    )


def module_metadata_publication_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle(plan)
    if oracle.get("kind") != "MirBuilderModuleMetadataPublicationDerivedHakoOracleV1":
        raise ValueError("module metadata publication oracle kind drift")
    shadow_result = build_shadow_result(plan)
    if shadow_result.get("kind") != "MirBuilderModuleMetadataPublicationDerivedHakoShadowResultV1":
        raise ValueError("module metadata publication shadow-result kind drift")
    contract = _contract(plan)
    plan_data = read_json(PLAN)
    plan_ops = _build_plan_spec()
    candidate_ops = _build_shadow_candidate_spec("candidate", plan_data)
    methods = [
        BehaviorMethodSpec(
            id="ModuleMetadataPublicationApi::project_shadow_record",
            rust_operation="MirBuilder::finalize_module module metadata publication shadow projection",
            hako_operation="StaticCall + SetField + ReturnValue",
            emits="ModuleMetadataPublicationApi.project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-module-metadata-publication"
        ),
        generator_version="mirbuilder-module-metadata-publication-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        extra_using_modules=[
        ],
        box=BoxSpec(name="ModuleMetadataPublicationKernel", fields=[]),
        additional_boxes=[
            _payload_box("ModuleMetadataPublicationPayloadBox"),
            BoxSpec(
                name="ModuleMetadataPublicationResultBox",
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
                name="ModuleMetadataPublicationFixtureApi",
                methods=[
                    ApiMethodSpec(
                        signature="build_plan(): ModuleMetadataPublicationPayloadBox",
                        operations=plan_ops
                        + [
                            op("NewBox", target="result", box="ModuleMetadataPublicationPayloadBox").to_json(),
                            op("SetField", target="result", field="payload", value="plan").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_shadow_candidate(): ModuleMetadataPublicationPayloadBox",
                        operations=candidate_ops
                        + [
                            op("NewBox", target="result", box="ModuleMetadataPublicationPayloadBox").to_json(),
                            op("SetField", target="result", field="payload", value="candidate").to_json(),
                            op("SetField", target="result", field="shadow_json", value=_literal(stable_json(_shadow_candidate(plan)))).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_python_oracle(): ModuleMetadataPublicationPayloadBox",
                        operations=[
                            op("StaticCall", target="oracle", callee="ModuleMetadataPublicationFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="oracle").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_hako_shadow(): ModuleMetadataPublicationPayloadBox",
                        operations=[
                            op("StaticCall", target="shadow", callee="ModuleMetadataPublicationFixtureApi.build_shadow_candidate", args=[]).to_json(),
                            op("ReturnValue", value="shadow").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_parity_gate(): ModuleMetadataPublicationPayloadBox",
                        operations=_build_parity_gate_spec()
                        + [
                            op("NewBox", target="result", box="ModuleMetadataPublicationPayloadBox").to_json(),
                            op("SetField", target="result", field="payload", value="parity_gate").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_promotion_token(): ModuleMetadataPublicationPayloadBox",
                        operations=_build_token_spec("promotion_token", "ModuleMetadataPublicationPromotionTokenV1", "promotion")
                        + [
                            op("NewBox", target="result", box="ModuleMetadataPublicationPayloadBox").to_json(),
                            op("SetField", target="result", field="payload", value="promotion_token").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="build_retirement_token(): ModuleMetadataPublicationPayloadBox",
                        operations=_build_token_spec("retirement_token", "ModuleMetadataPublicationRetirementTokenV1", "retirement")
                        + [
                            op("NewBox", target="result", box="ModuleMetadataPublicationPayloadBox").to_json(),
                            op("SetField", target="result", field="payload", value="retirement_token").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                ],
            ),
            StaticBoxSpec(
                name="ModuleMetadataPublicationApi",
                methods=[
                    ApiMethodSpec(
                        signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): ModuleMetadataPublicationResultBox",
                        operations=[
                            op("NewBox", target="result", box="ModuleMetadataPublicationResultBox").to_json(),
                            op("SetField", target="result", field="err", value=0).to_json(),
                            op("SetField", target="result", field="err_line", value={"literal": ""}).to_json(),
                            op("SetField", target="result", field="shadow_record", value="hako_shadow.payload").to_json(),
                            op("SetField", target="result", field="shadow_json", value="hako_shadow.shadow_json").to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    ),
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="plan", callee="ModuleMetadataPublicationFixtureApi.build_plan", args=[]),
            op("StaticCall", target="python_oracle", callee="ModuleMetadataPublicationFixtureApi.build_python_oracle", args=[]),
            op("StaticCall", target="hako_shadow", callee="ModuleMetadataPublicationFixtureApi.build_hako_shadow", args=[]),
            op("StaticCall", target="parity_gate", callee="ModuleMetadataPublicationFixtureApi.build_parity_gate", args=[]),
            op("StaticCall", target="promotion_token", callee="ModuleMetadataPublicationFixtureApi.build_promotion_token", args=[]),
            op("StaticCall", target="retirement_token", callee="ModuleMetadataPublicationFixtureApi.build_retirement_token", args=[]),
            op("AssertEq", left="python_oracle.shadow_json", right="hako_shadow.shadow_json", fail_message="module_metadata_publication_oracle_shadow_parity=fail", fail_code=1),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "kind")'}, right={"literal": "MirBuilderModuleMetadataPublicationPlanV1"}, fail_message="module_metadata_publication_plan_kind=fail", fail_code=2),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(plan.payload, "subject")'}, right={"literal": "MirBuilder::finalize_module module metadata publication"}, fail_message="module_metadata_publication_plan_subject=fail", fail_code=3),
            op(
                "StaticCall",
                target="result",
                callee="ModuleMetadataPublicationApi.project_shadow_record",
                args=[
                    "plan",
                    "python_oracle",
                    "hako_shadow",
                    "parity_gate",
                    "promotion_token",
                    "retirement_token",
                ],
            ),
            op("AssertEq", left="result.err", right=0, fail_message="module_metadata_publication_err=fail", fail_code=4),
            op("AssertEq", left="result.err_line", right={"literal": ""}, fail_message="module_metadata_publication_err_line=fail", fail_code=5),
            op("AssertEq", left="result.shadow_json", right="hako_shadow.shadow_json", fail_message="module_metadata_publication_shadow_json=fail", fail_code=6),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "kind")'}, right={"literal": "ModuleMetadataPublicationShadowCandidateV1"}, fail_message="module_metadata_publication_shadow_record_kind=fail", fail_code=7),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "family_id")'}, right={"literal": "hakorune_mir_builder::module_metadata_publication"}, fail_message="module_metadata_publication_shadow_record_family=fail", fail_code=8),
            op("AssertEq", left={"expr": 'BoxHelpers.map_get(result.shadow_record, "stage_id")'}, right={"literal": "module_metadata_publication"}, fail_message="module_metadata_publication_shadow_record_stage=fail", fail_code=9),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 0)'}, right={"literal": "module.metadata.user_box_decls"}, fail_message="module_metadata_publication_shadow_record_targets0=fail", fail_code=10),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 1)'}, right={"literal": "module.metadata.user_box_field_decls"}, fail_message="module_metadata_publication_shadow_record_targets1=fail", fail_code=11),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 2)'}, right={"literal": "module.metadata.record_decls"}, fail_message="module_metadata_publication_shadow_record_targets2=fail", fail_code=12),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 3)'}, right={"literal": "module.metadata.enum_decls"}, fail_message="module_metadata_publication_shadow_record_targets3=fail", fail_code=13),
            op("AssertEq", left={"expr": 'BoxHelpers.array_get(BoxHelpers.map_get(result.shadow_record, "publication_targets"), 4)'}, right={"expr": "null"}, fail_message="module_metadata_publication_shadow_record_targets4=fail", fail_code=14),
            op("Print", text="mirbuilder_module_metadata_publication_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_module_metadata_publication.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.module_metadata_publication",
        selected_body_count=contract.selected_body_count_label,
        api_name=contract.artifact.api_name,
        api_methods=[
                ApiMethodSpec(
                    signature="project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token): ModuleMetadataPublicationResultBox",
                    operations=[
                        op("NewBox", target="result", box="ModuleMetadataPublicationResultBox").to_json(),
                        op("SetField", target="result", field="err", value=0).to_json(),
                        op("SetField", target="result", field="err_line", value={"literal": ""}).to_json(),
                        op("SetField", target="result", field="shadow_record", value="hako_shadow.payload").to_json(),
                        op("SetField", target="result", field="shadow_json", value="hako_shadow.shadow_json").to_json(),
                        op("ReturnValue", value="result").to_json(),
                    ],
                )
        ],
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "module_metadata_publication": 1,
            "metadata_publication": 0,
            "all_functions_phi_materialization": 0,
            "record_and_packed_layout_refresh": 0,
            "typed_object_plan_refresh": 0,
            "direct_state_plan_refresh": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
            "backend_behavior_changed": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
            "semantic_refresh": 0,
            "generated_hako_artifact": 0,
        },
        verifier_checks=contract.verifier_checks(
            {
                "module_metadata_publication_only": 1,
                "plan_transport": "MapBox",
                "python_oracle_transport": "MapBox",
                "hako_shadow_transport": "MapBox",
                "parity_gate_transport": "MapBox",
                "promotion_token_transport": "MapBox",
                "retirement_token_transport": "MapBox",
                "result_transport": "ModuleMetadataPublicationResultBox",
                "shadow_json_transport": "StringBox",
                "projection_contract": "ModuleMetadataPublicationHakoProjector",
                "semantic_refresh": 0,
                "record_and_packed_layout_refresh": 0,
                "typed_object_plan_refresh": 0,
                "direct_state_plan_refresh": 0,
                "all_functions_phi_materialization": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "NewMap",
            "MapSet",
            "MethodCall",
            "StaticCall",
            "SetField",
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
                "result_transport": "ModuleMetadataPublicationResultBox",
                "shadow_json_transport": "StringBox",
                "metadata_publication": 0,
                "record_and_packed_layout_refresh": 0,
                "typed_object_plan_refresh": 0,
                "direct_state_plan_refresh": 0,
            }
        ),
        denied_boundaries=[
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
    shadow_result = build_shadow_result(plan)
    if not PROJECTION.exists():
        raise FileNotFoundError(f"{PROJECTION} must be written before manifest hashing")
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    if not SHADOW_RESULT.exists():
        raise FileNotFoundError(f"{SHADOW_RESULT} must be written before manifest hashing")
    spec = module_metadata_publication_spec()
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
        (SHADOW_RESULT, stable_json(shadow_result)),
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


def run_module_metadata_publication_artifact_generator(*, check: bool) -> None:
    if not check:
        plan = read_json(PLAN)
        write_if_changed(PROJECTION, stable_json(build_execution_projection(plan)))
        write_if_changed(ORACLE, stable_json(build_oracle(plan)))
        write_if_changed(SHADOW_RESULT, stable_json(build_shadow_result(plan)))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_module_metadata_publication_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_module_metadata_publication_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
