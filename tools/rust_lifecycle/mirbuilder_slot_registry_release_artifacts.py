#!/usr/bin/env python3
"""Generate the derived SlotRegistryRelease Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-slot-registry-release-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-slot-registry-release-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-slot-registry-release-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-slot-registry-release-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-slot-registry-release-derived-hako-verifier-result-v0.json"


def _var(name: str) -> dict[str, str]:
    return {"kind": "Var", "name": name}


def _i64(value: int) -> dict[str, int | str]:
    return {"kind": "I64", "value": value}


def _eq(left: Any, right: Any) -> dict[str, Any]:
    return {"kind": "EqI64", "left": left, "right": right}


def _read_json(path: Path) -> dict[str, Any]:
    import json

    return json.loads(path.read_text())


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    if plan.get("kind") != "MirBuilderSlotRegistryReleasePlanV1":
        raise ValueError("wrong slot registry release plan kind")
    if "SlotRegistryRelease" not in (plan.get("available_capabilities") or []):
        raise ValueError("slot registry release plan lacks SlotRegistryRelease capability")
    return {
        "schema_version": 0,
        "kind": "SlotRegistryReleaseExecutionProjectionV1",
        "source_plan": "MirBuilderSlotRegistryReleasePlanV1",
        "execution_scope": "PreparedSlotRegistryReleaseState",
        "inputs": {
            "slot_registry_transport": "FunctionSlotRegistryPreparedBox",
            "slot_registry_presence_transport": "I64BoolV0",
        },
        "methods": {
            "release": "Assign + SetField + ReturnValue",
            "apply": "StaticCall + ReturnI64",
        },
        "behavior": {
            "prepared_slot_registry": "Present",
            "release_clears_state": "Yes",
            "released_value_published": "Yes",
        },
        "result_transport": "ScalarI64",
        "result_semantics": "Unit",
        "directability": {
            "prepared_slot_registry_projection": "Allow",
            "host_env_lookup": "Deny",
            "full_metadata_context": "Deny",
        },
        "mutation_frame": {
            "current_slot_registry": "exclusive",
            "released_registry_present": "published by release",
            "slot_registry_released": "published by release",
        },
        "non_claims": {
            "slot_metadata_classification": 0,
            "function_region_stack_pop": 0,
            "module_metadata_publication": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderSlotRegistryReleasePlanV1":
        raise ValueError("wrong slot registry release plan kind")
    if "SlotRegistryRelease" not in (plan.get("available_capabilities") or []):
        raise ValueError("missing SlotRegistryRelease capability")
    profile = plan.get("execution_profile") or {}
    if profile.get("context") != "finalize_module":
        raise ValueError("slot registry release context drift")
    if profile.get("prepared_slot_registry") != "Some(FunctionSlotRegistry)":
        raise ValueError("prepared slot registry drift")
    release = plan.get("release_policy") or {}
    expected_release = {
        "lifecycle_owner": "CompilationContext.current_slot_registry",
        "init_operation": "Some(FunctionSlotRegistry::new())",
        "release_operation": "current_slot_registry = None",
        "release_timing": "AfterFunctionRegionStackPopBeforeModuleMetadataPublication",
        "released_value": "FunctionSlotRegistry",
    }
    for key, value in expected_release.items():
        if release.get(key) != value:
            raise ValueError(f"slot registry release policy drift: {key}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "MirBuilder::finalize_module current_slot_registry release":
        raise ValueError("slot registry release entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("slot registry release result drift")
    mutates = result.get("mutates") or []
    if mutates != ["builder.comp_ctx.current_slot_registry"]:
        raise ValueError("slot registry release mutation drift")
    observed = plan.get("observed_source_order") or {}
    prepare_module = observed.get("prepare_module") or []
    finalize_module = observed.get("finalize_module") or []
    expected_prepare = [
        "self.comp_ctx.current_slot_registry =",
        "Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());",
        "crate::mir::region::observer::observe_function_region(self);",
    ]
    expected_finalize = [
        "crate::mir::region::observer::pop_function_region(self);",
        "self.comp_ctx.current_slot_registry = None;",
        "module.metadata.user_box_decls = self.comp_ctx.user_defined_boxes.clone();",
    ]
    for rows, expected, label in [
        (prepare_module, expected_prepare, "prepare_module"),
        (finalize_module, expected_finalize, "finalize_module"),
    ]:
        if len(rows) != len(expected):
            raise ValueError(f"slot registry release observed source order drift: {label}")
        for index, marker in enumerate(expected):
            if rows[index].get("marker") != marker:
                raise ValueError(f"slot registry release observed source order drift: {label}[{index}]")
    non_claims = plan.get("non_claims") or {}
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"slot registry release non-claim must remain 0: {key}")


def _validate_projection(projection: dict[str, Any], plan: dict[str, Any]) -> None:
    if projection.get("kind") != "SlotRegistryReleaseExecutionProjectionV1":
        raise ValueError("wrong slot registry release projection kind")
    if projection.get("source_plan") != "MirBuilderSlotRegistryReleasePlanV1":
        raise ValueError("projection source plan drift")
    if projection.get("execution_scope") != "PreparedSlotRegistryReleaseState":
        raise ValueError("projection scope drift")
    inputs = projection.get("inputs") or {}
    if inputs.get("slot_registry_transport") != "FunctionSlotRegistryPreparedBox":
        raise ValueError("slot registry transport drift")
    if inputs.get("slot_registry_presence_transport") != "I64BoolV0":
        raise ValueError("slot registry presence transport drift")
    methods = projection.get("methods") or {}
    if methods.get("release") != "Assign + SetField + ReturnValue":
        raise ValueError("slot registry release method drift")
    if methods.get("apply") != "StaticCall + ReturnI64":
        raise ValueError("slot registry apply method drift")
    behavior = projection.get("behavior") or {}
    if behavior.get("prepared_slot_registry") != "Present":
        raise ValueError("prepared slot registry behavior drift")
    if behavior.get("release_clears_state") != "Yes":
        raise ValueError("slot registry release clear behavior drift")
    if behavior.get("released_value_published") != "Yes":
        raise ValueError("released value behavior drift")
    if projection.get("result_transport") != "ScalarI64":
        raise ValueError("slot registry release result transport drift")
    if projection.get("result_semantics") != "Unit":
        raise ValueError("slot registry release result semantics drift")
    directability = projection.get("directability") or {}
    if directability.get("prepared_slot_registry_projection") != "Allow":
        raise ValueError("prepared slot registry projection must be allowed")
    if directability.get("host_env_lookup") != "Deny":
        raise ValueError("host env lookup must remain denied")
    if directability.get("full_metadata_context") != "Deny":
        raise ValueError("full metadata context must remain denied")
    mutation_frame = projection.get("mutation_frame") or {}
    if mutation_frame.get("current_slot_registry") != "exclusive":
        raise ValueError("current slot registry mutation drift")
    if mutation_frame.get("released_registry_present") != "published by release":
        raise ValueError("released registry publication drift")
    if mutation_frame.get("slot_registry_released") != "published by release":
        raise ValueError("slot registry released publication drift")
    if (projection.get("non_claims") or {}).get("generated_hako_artifact") != 0:
        raise ValueError("projection may not claim generated hako artifact")
    _validate_plan(plan)


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderSlotRegistryReleaseDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module current_slot_registry = None",
        "vectors": [
            {
                "name": "release_clears_current_slot_registry",
                "inputs": {
                    "initial_current_slot_registry_present": 1,
                    "initial_released_registry_present": 0,
                    "initial_slot_registry_released": 0,
                },
                "expect": {
                    "current_slot_registry_present": 0,
                    "released_registry_present": 1,
                    "slot_registry_released": 1,
                    "current_slot_registry_is_null": 1,
                },
            },
            {
                "name": "apply_clears_current_slot_registry",
                "inputs": {
                    "initial_current_slot_registry_present": 1,
                    "initial_released_registry_present": 0,
                    "initial_slot_registry_released": 0,
                },
                "expect": {
                    "return_value": 0,
                    "current_slot_registry_present": 0,
                    "released_registry_present": 1,
                    "slot_registry_released": 1,
                    "current_slot_registry_is_null": 1,
                },
            },
        ],
        "non_claims": {
            "slot_metadata_classification": 0,
            "function_region_stack_pop": 0,
            "module_metadata_publication": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "runtime_fallback": 0,
        },
    }


def _contract(plan: dict[str, Any], projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_projection(projection, plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::slot_registry_release",
        method_universe=(
            "SlotRegistryReleaseApi::release",
            "SlotRegistryReleaseApi::apply",
        ),
        selected_method_ids=(
            "SlotRegistryReleaseApi::release",
            "SlotRegistryReleaseApi::apply",
        ),
        denials=(),
        semantic_transports={
            "state_transport": "PreparedSlotRegistryStateShell",
            "slot_registry_transport": "FunctionSlotRegistryPreparedBox",
            "release_result_transport": "FunctionSlotRegistryPreparedBox",
            "apply_result_transport": "ScalarI64",
            "apply_result_semantics": "Unit",
            "slot_metadata_classification": 0,
            "function_region_stack_pop": 0,
            "module_metadata_publication": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::slot_registry_release",
            api_name="SlotRegistryReleaseApi",
            pilot_scope="SlotRegistryRelease_prepared_slot_registry_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_slot_registry_release.artifact.json",
        ),
        selected_body_count_label="slot_registry_release_prepared_slot_registry_only",
        expected_fields=(
            "current_slot_registry",
            "current_slot_registry_present",
            "released_registry_present",
            "slot_registry_released",
        ),
    )


def _release_operations() -> list[dict[str, Any]]:
    return [
        op("Assign", target="registry", value="state.current_slot_registry", declaration="local").to_json(),
        op("SetField", target="state", field="current_slot_registry", value={"expr": "null"}).to_json(),
        op("SetField", target="state", field="current_slot_registry_present", value=0).to_json(),
        op("SetField", target="state", field="released_registry_present", value=1).to_json(),
        op("SetField", target="state", field="slot_registry_released", value=1).to_json(),
        op("ReturnValue", value="registry").to_json(),
    ]


def slot_registry_release_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderSlotRegistryReleaseDerivedHakoOracleV1":
        raise ValueError("slot registry release oracle kind drift")
    contract = _contract(plan, projection)
    methods = [
        BehaviorMethodSpec(
            id="SlotRegistryReleaseApi::release",
            rust_operation="MirBuilder::finalize_module current_slot_registry = None",
            hako_operation="Assign + SetField + ReturnValue",
            emits="SlotRegistryReleaseApi.release(state)",
        ),
        BehaviorMethodSpec(
            id="SlotRegistryReleaseApi::apply",
            rust_operation="MirBuilder::finalize_module current_slot_registry = None",
            hako_operation="StaticCall + ReturnI64",
            emits="SlotRegistryReleaseApi.apply(state)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-slot-registry-release"
        ),
        generator_version="mirbuilder-slot-registry-release-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="SlotRegistryReleaseKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="FunctionSlotRegistryPreparedBox",
                fields=[],
            ),
            BoxSpec(
                name="PreparedSlotRegistryStateShellBox",
                fields=[
                    FieldSpec(name="current_slot_registry", field_type="FunctionSlotRegistryPreparedBox", initializer="new FunctionSlotRegistryPreparedBox()"),
                    FieldSpec(name="current_slot_registry_present", field_type="i64", initializer="1"),
                    FieldSpec(name="released_registry_present", field_type="i64", initializer="0"),
                    FieldSpec(name="slot_registry_released", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="SlotRegistryReleaseApi",
                methods=[
                    ApiMethodSpec(
                        signature="release(state): FunctionSlotRegistryPreparedBox",
                        operations=_release_operations(),
                    ),
                    ApiMethodSpec(
                        signature="apply(state): i64",
                        operations=[
                            op("StaticCall", callee="SlotRegistryReleaseApi.release", args=["state"]).to_json(),
                            op("ReturnI64", return_value=0).to_json(),
                        ],
                    ),
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="direct_state", box="PreparedSlotRegistryStateShellBox"),
            op("Assign", target="pre_release_registry", value="direct_state.current_slot_registry"),
            op("StaticCall", target="released_registry", callee="SlotRegistryReleaseApi.release", args=["direct_state"]),
            op("AssertEq", left="released_registry", right="pre_release_registry", fail_message="slot_registry_release_return=fail", fail_code=1),
            op("AssertEq", left="direct_state.current_slot_registry", right={"expr": "null"}, fail_message="slot_registry_release_current_slot_registry=fail", fail_code=2),
            op("AssertEq", left="direct_state.current_slot_registry_present", right=0, fail_message="slot_registry_release_current_present=fail", fail_code=3),
            op("AssertEq", left="direct_state.released_registry_present", right=1, fail_message="slot_registry_release_released_present=fail", fail_code=4),
            op("AssertEq", left="direct_state.slot_registry_released", right=1, fail_message="slot_registry_release_flag=fail", fail_code=5),
            op("NewBox", target="apply_state", box="PreparedSlotRegistryStateShellBox"),
            op("StaticCall", target="apply_result", callee="SlotRegistryReleaseApi.apply", args=["apply_state"]),
            op("AssertEq", left="apply_result", right=0, fail_message="slot_registry_release_apply_return=fail", fail_code=6),
            op("AssertEq", left="apply_state.current_slot_registry", right={"expr": "null"}, fail_message="slot_registry_release_apply_current_slot_registry=fail", fail_code=7),
            op("AssertEq", left="apply_state.current_slot_registry_present", right=0, fail_message="slot_registry_release_apply_current_present=fail", fail_code=8),
            op("AssertEq", left="apply_state.released_registry_present", right=1, fail_message="slot_registry_release_apply_released_present=fail", fail_code=9),
            op("AssertEq", left="apply_state.slot_registry_released", right=1, fail_message="slot_registry_release_apply_flag=fail", fail_code=10),
            op("Print", text="mirbuilder_slot_registry_release_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_slot_registry_release.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.current_slot_registry_release",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "slot_registry_release": 1,
            "module_metadata_publication": 0,
            "metadata_publication": 0,
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
                "slot_registry_release_only": 1,
                "current_slot_registry_transport": "FunctionSlotRegistryPreparedBox",
                "release_result_transport": "FunctionSlotRegistryPreparedBox",
                "apply_result_transport": "ScalarI64",
                "apply_result_semantics": "Unit",
                "current_slot_registry_cleared": 1,
                "released_registry_present": 1,
                "slot_registry_released": 1,
                "module_metadata_publication": 0,
                "semantic_refresh": 0,
                "all_functions_phi_materialization": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "Assign",
            "SetField",
            "ReturnValue",
            "StaticCall",
            "FunctionSlotRegistryPreparedBox",
            "PreparedSlotRegistryStateShellBox",
        ],
        transport_notes=contract.transport_notes(
            {
                "module_metadata_publication": "unselected",
                "semantic_refresh": "unselected",
                "all_functions_phi_materialization": 0,
            }
        ),
        denied_boundaries=[
            "module_metadata_publication",
            "metadata_publication",
            "semantic_refresh",
            "all_functions_phi_materialization",
            "full_finalize_module",
            "mainline_selected",
            "runtime_fallback",
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
    spec = slot_registry_release_spec()
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
    outputs.extend(
        [
            (spec.hako_path, hako_text),
            (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text),
        ]
    )
    return outputs


def run_slot_registry_release_artifact_generator(*, check: bool) -> None:
    if not check:
        plan = read_json(PLAN)
        write_if_changed(PROJECTION, stable_json(build_execution_projection(plan)))
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_slot_registry_release_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_slot_registry_release_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
