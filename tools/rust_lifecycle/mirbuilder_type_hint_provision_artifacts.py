#!/usr/bin/env python3
"""Generate the bounded TypeHintProvision Hako artifact."""

from __future__ import annotations

from pathlib import Path

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
SOURCE = ROOT / "src/mir/builder/type_hint_providers.rs"
PLAN = FIXTURES / "mirbuilder-type-hint-provision-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-type-hint-provision-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-type-hint-provision-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-type-hint-provision-derived-hako-verifier-result-v0.json"

PROVIDER_CASES = [
    "Await",
    "Call(Global)",
    "Call(Constructor)",
    "Call(OtherOrMissingCallee)",
]


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderTypeHintProvisionDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module type_hint_providers::annotate_missing_result_types_from_calls_and_await",
        "vectors": [
            {
                "name": "minimal_literal_type_hints_cover_provider_cases",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "module_transport": "MirModulePreparedMain",
                    "value_types_transport": "self.type_ctx.value_types",
                    "preexisting_value_type_count": 1,
                },
                "expect": {
                    "ok": 1,
                    "provider_cases": 4,
                    "value_types_inserted": 4,
                    "value_origin_newbox_inserted": 1,
                    "existing_type_preserved": 1,
                },
            }
        ],
        "non_claims": {
            "metadata_value_type_publication": 0,
            "metadata_origin_caller_merge": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderTypeHintProvisionPlanV1":
        raise ValueError("wrong type hint provision plan kind")
    if "TypeHintProvision" not in (plan.get("available_capabilities") or []):
        raise ValueError("type hint plan lacks TypeHintProvision capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "module_transport": "MirModulePreparedMain",
        "value_types": "self.type_ctx.value_types",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"type hint profile drift: {key}")
    cases = [case.get("instruction") for case in (plan.get("provider_cases") or [])]
    if cases != PROVIDER_CASES:
        raise ValueError(f"type hint provider case order drift: {cases}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "type_hint_providers::annotate_missing_result_types_from_calls_and_await":
        raise ValueError("type hint entrypoint drift")
    if result.get("minimal_path_expected_result") != "OkImplicitUnit":
        raise ValueError("type hint minimal result drift")
    if result.get("mutates") != ["self.type_ctx.value_types", "self.type_ctx.value_origin_newbox"]:
        raise ValueError("type hint mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"type hint non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::type_hint_provision",
        method_universe=("TypeHintProvision::run",),
        selected_method_ids=("TypeHintProvision::run",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "module_transport": "MirModulePreparedMain",
            "value_types_transport": "TypeContextValueTypesShell",
            "value_origin_newbox_transport": "TypeContextValueOriginNewBoxShell",
            "provider_cases": tuple(PROVIDER_CASES),
            "metadata_value_type_publication": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::type_hint_provision",
            api_name="TypeHintProvisionApi",
            pilot_scope="TypeHintProvision_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.artifact.json",
        ),
        selected_body_count_label="type_hint_provision_minimal_literal_profile_only",
        expected_fields=(
            "await_case_seen",
            "call_global_case_seen",
            "call_constructor_case_seen",
            "call_unknown_case_seen",
            "value_types_inserted",
            "value_origin_newbox_inserted",
        ),
    )


def type_hint_provision_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderTypeHintProvisionDerivedHakoOracleV1":
        raise ValueError("type hint oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="TypeHintProvision::run",
            rust_operation="type_hint_providers::annotate_missing_result_types_from_calls_and_await",
            hako_operation="Set provider case fields + ReturnValue",
            emits="TypeHintProvisionApi.run(fn_state, module_state, type_ctx)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-type-hint-provision"
        ),
        generator_version="mirbuilder-type-hint-provision-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="TypeHintProvisionKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="TypeHintFunctionShellBox",
                fields=[
                    FieldSpec(name="await_case_seen", field_type="i64", initializer="0"),
                    FieldSpec(name="call_global_case_seen", field_type="i64", initializer="0"),
                    FieldSpec(name="call_constructor_case_seen", field_type="i64", initializer="0"),
                    FieldSpec(name="call_unknown_case_seen", field_type="i64", initializer="0"),
                    FieldSpec(name="existing_type_preserved", field_type="i64", initializer="0"),
                    FieldSpec(name="scan_complete", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="TypeHintModuleShellBox",
                fields=[
                    FieldSpec(name="global_signature_lookup", field_type="i64", initializer="0"),
                    FieldSpec(name="annotation_fallback_checked", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="TypeHintTypeContextShellBox",
                fields=[
                    FieldSpec(name="value_types_inserted", field_type="i64", initializer="0"),
                    FieldSpec(name="value_origin_newbox_inserted", field_type="i64", initializer="0"),
                    FieldSpec(name="future_inner_inferred", field_type="i64", initializer="0"),
                    FieldSpec(name="module_return_inferred", field_type="i64", initializer="0"),
                    FieldSpec(name="constructor_box_inferred", field_type="i64", initializer="0"),
                    FieldSpec(name="unknown_inferred", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="TypeHintProvisionResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="provider_cases", field_type="i64", initializer="0"),
                    FieldSpec(name="value_types_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="value_origin_newbox_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="metadata_publication", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="TypeHintProvisionApi",
                methods=[
                    ApiMethodSpec(
                        signature="run(fn_state, module_state, type_ctx): TypeHintProvisionResultBox",
                        operations=[
                            op("SetField", target="fn_state", field="await_case_seen", value=1).to_json(),
                            op("SetField", target="fn_state", field="call_global_case_seen", value=1).to_json(),
                            op("SetField", target="fn_state", field="call_constructor_case_seen", value=1).to_json(),
                            op("SetField", target="fn_state", field="call_unknown_case_seen", value=1).to_json(),
                            op("SetField", target="fn_state", field="existing_type_preserved", value=1).to_json(),
                            op("SetField", target="fn_state", field="scan_complete", value=1).to_json(),
                            op("SetField", target="module_state", field="global_signature_lookup", value=1).to_json(),
                            op("SetField", target="module_state", field="annotation_fallback_checked", value=1).to_json(),
                            op("SetField", target="type_ctx", field="future_inner_inferred", value=1).to_json(),
                            op("SetField", target="type_ctx", field="module_return_inferred", value=1).to_json(),
                            op("SetField", target="type_ctx", field="constructor_box_inferred", value=1).to_json(),
                            op("SetField", target="type_ctx", field="unknown_inferred", value=1).to_json(),
                            op("SetField", target="type_ctx", field="value_types_inserted", value=4).to_json(),
                            op("SetField", target="type_ctx", field="value_origin_newbox_inserted", value=1).to_json(),
                            op("NewBox", target="result", box="TypeHintProvisionResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="provider_cases", value=4).to_json(),
                            op("SetField", target="result", field="value_types_mutated", value=1).to_json(),
                            op("SetField", target="result", field="value_origin_newbox_mutated", value=1).to_json(),
                            op("SetField", target="result", field="metadata_publication", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="TypeHintFunctionShellBox"),
            op("NewBox", target="module_state", box="TypeHintModuleShellBox"),
            op("NewBox", target="type_ctx", box="TypeHintTypeContextShellBox"),
            op("StaticCall", target="result", callee="TypeHintProvisionApi.run", args=["fn_state", "module_state", "type_ctx"]),
            op("AssertEq", left="result.ok", right=1, fail_message="type_hint_ok=fail", fail_code=1),
            op("AssertEq", left="result.provider_cases", right=4, fail_message="type_hint_provider_cases=fail", fail_code=2),
            op("AssertEq", left="result.value_types_mutated", right=1, fail_message="type_hint_value_types=fail", fail_code=3),
            op("AssertEq", left="result.value_origin_newbox_mutated", right=1, fail_message="type_hint_newbox=fail", fail_code=4),
            op("AssertEq", left="result.metadata_publication", right=0, fail_message="type_hint_metadata_publication=fail", fail_code=5),
            op("AssertEq", left="fn_state.await_case_seen", right=1, fail_message="type_hint_await=fail", fail_code=6),
            op("AssertEq", left="fn_state.call_global_case_seen", right=1, fail_message="type_hint_global=fail", fail_code=7),
            op("AssertEq", left="fn_state.call_constructor_case_seen", right=1, fail_message="type_hint_constructor=fail", fail_code=8),
            op("AssertEq", left="fn_state.call_unknown_case_seen", right=1, fail_message="type_hint_unknown=fail", fail_code=9),
            op("AssertEq", left="fn_state.existing_type_preserved", right=1, fail_message="type_hint_existing=fail", fail_code=10),
            op("AssertEq", left="fn_state.scan_complete", right=1, fail_message="type_hint_scan=fail", fail_code=11),
            op("AssertEq", left="module_state.global_signature_lookup", right=1, fail_message="type_hint_module_lookup=fail", fail_code=12),
            op("AssertEq", left="type_ctx.value_types_inserted", right=4, fail_message="type_hint_value_types_inserted=fail", fail_code=13),
            op("AssertEq", left="type_ctx.value_origin_newbox_inserted", right=1, fail_message="type_hint_origin_newbox_inserted=fail", fail_code=14),
            op("Print", text="mirbuilder_type_hint_provision_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_type_hint_provision.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.type_hint_provision",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "type_hint_provision": 1,
            "metadata_value_type_publication": 0,
            "metadata_origin_caller_merge": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
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
                "type_hint_provision_only": 1,
                "entrypoint": "type_hint_providers::annotate_missing_result_types_from_calls_and_await",
                "provider_cases": PROVIDER_CASES,
                "function_transport": "MirFunctionPreparedMain",
                "module_transport": "MirModulePreparedMain",
                "value_types": "self.type_ctx.value_types",
                "minimal_path_expected_result": "OkImplicitUnit",
                "metadata_value_type_publication": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "TypeHintProvisionShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "metadata_value_type_publication": "unselected",
                "metadata_origin_caller_merge": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "metadata_value_type_publication",
            "metadata_origin_caller_merge",
            "phi_return_type_inference",
            "phi_input_materialization",
            "module_function_insertion",
            "full_finalize_module",
            "mainline_selected",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = type_hint_provision_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = [(ORACLE, oracle_text)]
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


def run_type_hint_provision_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_type_hint_provision_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_type_hint_provision_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
