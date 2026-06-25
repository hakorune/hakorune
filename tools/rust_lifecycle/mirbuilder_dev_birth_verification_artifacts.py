#!/usr/bin/env python3
"""Generate the bounded dev birth verification Hako artifact."""

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
SOURCE = ROOT / "src/mir/builder/module_lifecycle.rs"
PLAN = FIXTURES / "mirbuilder-dev-birth-verification-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-dev-birth-verification-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-dev-birth-verification-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-dev-birth-verification-derived-hako-verifier-result-v0.json"

GUARD_CONDITIONS = [
    "using_is_dev",
    "stageb_dev_verify_enabled",
    "cli_verbose_enabled",
]

VERIFICATION_STEPS = [
    "IterateFunctionBlocks",
    "ScanNewBoxInstructions",
    "SkipStageBDriverBox",
    "SkipStringBox",
    "ExpectBirthTailByBoxTypeAndArity",
    "LookAheadThreeInstructions",
    "AcceptMethodBirthOnSameReceiver",
    "AcceptConstStringGlobalCompatibilityPath",
    "WarnOnMissingBirth",
    "WarnSummaryWhenAnyMissing",
]


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderDevBirthVerificationDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module dev NewBox birth verification",
        "vectors": [
            {
                "name": "prepared_dev_birth_verification_records_disabled_warning_pass",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "context": "finalize_module",
                    "using_is_dev": 0,
                    "stageb_dev_verify_enabled": 0,
                    "cli_verbose_enabled": 0,
                },
                "expect": {
                    "ok": 1,
                    "guard_conditions": len(GUARD_CONDITIONS),
                    "verification_steps": len(VERIFICATION_STEPS),
                    "warnings": 0,
                    "mutates_function": 0,
                    "module_function_insertion": 0,
                },
            }
        ],
        "non_claims": {
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderDevBirthVerificationPlanV1":
        raise ValueError("wrong dev birth verification plan kind")
    if "DevBirthVerification" not in (plan.get("available_capabilities") or []):
        raise ValueError("dev birth verification plan lacks DevBirthVerification capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "context": "finalize_module",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"dev birth verification profile drift: {key}")
    if plan.get("guard_conditions") != GUARD_CONDITIONS:
        raise ValueError(f"dev birth verification guard condition drift: {plan.get('guard_conditions')}")
    if plan.get("verification_steps") != VERIFICATION_STEPS:
        raise ValueError(f"dev birth verification step drift: {plan.get('verification_steps')}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "MirBuilder::finalize_module dev birth verification block":
        raise ValueError("dev birth verification entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("dev birth verification result drift")
    if result.get("mutates") != []:
        raise ValueError("dev birth verification mutation frame drift")
    if result.get("side_effect") != "dev_warning_only":
        raise ValueError("dev birth verification side effect drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"dev birth verification non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::dev_birth_verification",
        method_universe=("DevBirthVerification::run",),
        selected_method_ids=("DevBirthVerification::run",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "context": "finalize_module",
            "guard_conditions": tuple(GUARD_CONDITIONS),
            "verification_steps": tuple(VERIFICATION_STEPS),
            "side_effect": "dev_warning_only",
            "mutation_frame": (),
            "module_function_insertion": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::dev_birth_verification",
            api_name="DevBirthVerificationApi",
            pilot_scope="DevBirthVerification_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.artifact.json",
        ),
        selected_body_count_label="dev_birth_verification_minimal_literal_profile_only",
        expected_fields=(
            "guard_conditions",
            "verification_steps",
            "warnings",
            "mutates_function",
            "module_function_insertion",
        ),
    )


def dev_birth_verification_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderDevBirthVerificationDerivedHakoOracleV1":
        raise ValueError("dev birth verification oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="DevBirthVerification::run",
            rust_operation="MirBuilder::finalize_module dev birth verification block",
            hako_operation="GuardConditionFlags + VerificationStepCount + ReturnValue",
            emits="DevBirthVerificationApi.run(fn_state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-dev-birth-verification"
        ),
        generator_version="mirbuilder-dev-birth-verification-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="DevBirthVerificationKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="DevBirthFunctionShellBox",
                fields=[
                    FieldSpec(name="using_is_dev", field_type="i64", initializer="0"),
                    FieldSpec(name="stageb_dev_verify_enabled", field_type="i64", initializer="0"),
                    FieldSpec(name="cli_verbose_enabled", field_type="i64", initializer="0"),
                    FieldSpec(name="guard_conditions", field_type="i64", initializer="0"),
                    FieldSpec(name="verification_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="warnings", field_type="i64", initializer="0"),
                    FieldSpec(name="mutates_function", field_type="i64", initializer="0"),
                    FieldSpec(name="module_function_insertion", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="DevBirthVerificationResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="guard_conditions", field_type="i64", initializer="0"),
                    FieldSpec(name="verification_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="warnings", field_type="i64", initializer="0"),
                    FieldSpec(name="mutates_function", field_type="i64", initializer="0"),
                    FieldSpec(name="module_function_insertion", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="DevBirthVerificationApi",
                methods=[
                    ApiMethodSpec(
                        signature="run(fn_state): DevBirthVerificationResultBox",
                        operations=[
                            op("SetField", target="fn_state", field="using_is_dev", value=0).to_json(),
                            op("SetField", target="fn_state", field="stageb_dev_verify_enabled", value=0).to_json(),
                            op("SetField", target="fn_state", field="cli_verbose_enabled", value=0).to_json(),
                            op("SetField", target="fn_state", field="guard_conditions", value=len(GUARD_CONDITIONS)).to_json(),
                            op("SetField", target="fn_state", field="verification_steps", value=len(VERIFICATION_STEPS)).to_json(),
                            op("SetField", target="fn_state", field="warnings", value=0).to_json(),
                            op("SetField", target="fn_state", field="mutates_function", value=0).to_json(),
                            op("SetField", target="fn_state", field="module_function_insertion", value=0).to_json(),
                            op("NewBox", target="result", box="DevBirthVerificationResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="guard_conditions", value=len(GUARD_CONDITIONS)).to_json(),
                            op("SetField", target="result", field="verification_steps", value=len(VERIFICATION_STEPS)).to_json(),
                            op("SetField", target="result", field="warnings", value=0).to_json(),
                            op("SetField", target="result", field="mutates_function", value=0).to_json(),
                            op("SetField", target="result", field="module_function_insertion", value=0).to_json(),
                            op("SetField", target="result", field="full_finalize_module", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="DevBirthFunctionShellBox"),
            op("StaticCall", target="result", callee="DevBirthVerificationApi.run", args=["fn_state"]),
            op("AssertEq", left="result.ok", right=1, fail_message="dev_birth_verification_ok=fail", fail_code=1),
            op("AssertEq", left="result.guard_conditions", right=len(GUARD_CONDITIONS), fail_message="dev_birth_verification_guards=fail", fail_code=2),
            op("AssertEq", left="result.verification_steps", right=len(VERIFICATION_STEPS), fail_message="dev_birth_verification_steps=fail", fail_code=3),
            op("AssertEq", left="result.warnings", right=0, fail_message="dev_birth_verification_warnings=fail", fail_code=4),
            op("AssertEq", left="result.mutates_function", right=0, fail_message="dev_birth_verification_mutation=fail", fail_code=5),
            op("AssertEq", left="result.module_function_insertion", right=0, fail_message="dev_birth_verification_module_insert=fail", fail_code=6),
            op("AssertEq", left="result.full_finalize_module", right=0, fail_message="dev_birth_verification_full_finalize=fail", fail_code=7),
            op("Print", text="mirbuilder_dev_birth_verification_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_dev_birth_verification.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.dev_birth_verification",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "dev_birth_verification": 1,
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
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
                "dev_birth_verification_only": 1,
                "entrypoint": "MirBuilder::finalize_module dev birth verification block",
                "guard_conditions": GUARD_CONDITIONS,
                "verification_steps": VERIFICATION_STEPS,
                "function_transport": "MirFunctionPreparedMain",
                "context": "finalize_module",
                "minimal_path_expected_result": "NoErrorReturn",
                "mutation_frame": [],
                "side_effect": "dev_warning_only",
                "module_function_insertion": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "DevBirthVerificationShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "guard_conditions": tuple(GUARD_CONDITIONS),
                "verification_steps": tuple(VERIFICATION_STEPS),
                "mutation_frame": (),
                "module_function_insertion": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "module_function_insertion",
            "condition_fn_injection",
            "all_functions_phi_materialization",
            "region_stack_pop",
            "slot_registry_release",
            "metadata_publication",
            "semantic_refresh",
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
    spec = dev_birth_verification_spec()
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


def run_dev_birth_verification_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_dev_birth_verification_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_dev_birth_verification_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
