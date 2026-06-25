#!/usr/bin/env python3
"""Generate the bounded TypedValueDefinitionVerification Hako artifact."""

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
SOURCE = ROOT / "src/mir/builder/emission/value_lifecycle.rs"
PLAN = FIXTURES / "mirbuilder-typed-value-verification-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-typed-value-verification-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-typed-value-verification-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-typed-value-verification-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderTypedValueVerificationDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module verify typed values are defined",
        "vectors": [
            {
                "name": "minimal_literal_result_value_is_defined",
                "inputs": {
                    "result_value": 7,
                    "typed_value_id": 7,
                    "defined_value_id": 7,
                    "param_value_id": 0,
                    "strict_gate": 1,
                },
                "expect": {
                    "verified": 1,
                    "missing_count": 0,
                    "fatal_missing": 0,
                    "stale_cleanup_count": 0,
                },
            }
        ],
        "non_claims": {
            "current_function_take": 0,
            "type_propagation": 0,
            "type_hint_provision": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_metadata_publication": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderTypedValueVerificationPlanV1":
        raise ValueError("wrong typed-value verification plan kind")
    if "TypedValueDefinitionVerification" not in (plan.get("available_capabilities") or []):
        raise ValueError("typed-value verification plan lacks TypedValueDefinitionVerification capability")
    contract = plan.get("verification_contract") or {}
    expected = {
        "typed_values": "builder.type_ctx.value_types",
        "definition_sources": ["compute_def_blocks(func)", "func.params"],
        "excluded_value": "ValueId::INVALID",
        "fail_fast_tag": "[freeze:contract][value_lifecycle/typed_without_def]",
    }
    for key, value in expected.items():
        if contract.get(key) != value:
            raise ValueError(f"typed-value verification contract drift: {key}")
    result = plan.get("result_contract") or {}
    if result.get("minimal_path_expected_result") != "Ok":
        raise ValueError("typed-value verification minimal path result drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"typed-value verification non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::typed_value_verification",
        method_universe=("TypedValueVerification::verify",),
        selected_method_ids=("TypedValueVerification::verify",),
        denials=(),
        semantic_transports={
            "typed_values_transport": "TypedValueSetShell",
            "definition_sources": "DefBlocksPlusParamsShell",
            "result_value_transport": "ValueIdAsI64",
            "excluded_value": "ValueId::INVALID",
            "current_function_take": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::typed_value_verification",
            api_name="TypedValueVerificationApi",
            pilot_scope="TypedValueVerification_minimal_literal_success_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json",
        ),
        selected_body_count_label="typed_value_verification_minimal_literal_success_only",
        expected_fields=("verified", "missing_count", "fatal_missing", "stale_cleanup_count"),
    )


def typed_value_verification_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderTypedValueVerificationDerivedHakoOracleV1":
        raise ValueError("typed-value verification oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="TypedValueVerification::verify",
            rust_operation="verify_typed_values_are_defined(self, \"finalize_module\")",
            hako_operation="ValidateTypedValueDefined + ReturnValue",
            emits="TypedValueVerificationApi.verify(result_value, typed_value, defined_value, param_value)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-typed-value-verification"
        ),
        generator_version="mirbuilder-typed-value-verification-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="TypedValueVerificationKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="TypedValueVerificationResultBox",
                fields=[
                    FieldSpec(name="verified", field_type="i64", initializer="0"),
                    FieldSpec(name="missing_count", field_type="i64", initializer="0"),
                    FieldSpec(name="fatal_missing", field_type="i64", initializer="0"),
                    FieldSpec(name="stale_cleanup_count", field_type="i64", initializer="0"),
                ],
            )
        ],
        static_boxes=[
            StaticBoxSpec(
                name="TypedValueVerificationApi",
                methods=[
                    ApiMethodSpec(
                        signature="verify(result_value, typed_value, defined_value, param_value): TypedValueVerificationResultBox",
                        operations=[
                            op(
                                "IfElse",
                                condition={"kind": "EqI64", "left": {"kind": "Var", "name": "typed_value"}, "right": {"kind": "Var", "name": "defined_value"}},
                                then_body=[
                                    op("NewBox", target="result", box="TypedValueVerificationResultBox").to_json(),
                                    op("SetField", target="result", field="verified", value=1).to_json(),
                                    op("SetField", target="result", field="missing_count", value=0).to_json(),
                                    op("SetField", target="result", field="fatal_missing", value=0).to_json(),
                                    op("SetField", target="result", field="stale_cleanup_count", value=0).to_json(),
                                    op("ReturnValue", value="result").to_json(),
                                ],
                                else_body=[
                                    op("NewBox", target="result", box="TypedValueVerificationResultBox").to_json(),
                                    op("SetField", target="result", field="verified", value=0).to_json(),
                                    op("SetField", target="result", field="missing_count", value=1).to_json(),
                                    op("SetField", target="result", field="fatal_missing", value=1).to_json(),
                                    op("SetField", target="result", field="stale_cleanup_count", value=0).to_json(),
                                    op("ReturnValue", value="result").to_json(),
                                ],
                            ).to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("StaticCall", target="verified", callee="TypedValueVerificationApi.verify", args=[7, 7, 7, 0]),
            op("AssertEq", left="verified.verified", right=1, fail_message="typed_value_verified=fail", fail_code=1),
            op("AssertEq", left="verified.missing_count", right=0, fail_message="typed_value_missing=fail", fail_code=2),
            op("AssertEq", left="verified.fatal_missing", right=0, fail_message="typed_value_fatal=fail", fail_code=3),
            op("AssertEq", left="verified.stale_cleanup_count", right=0, fail_message="typed_value_stale_cleanup=fail", fail_code=4),
            op("Print", text="mirbuilder_typed_value_verification_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_typed_value_verification.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.typed_value_verification",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "typed_value_verification": 1,
            "current_function_take": 0,
            "type_propagation": 0,
            "type_hint_provision": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_metadata_publication": 0,
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
                "typed_value_verification_only": 1,
                "typed_values": "builder.type_ctx.value_types",
                "definition_sources": ["compute_def_blocks(func)", "func.params"],
                "excluded_value": "ValueId::INVALID",
                "fail_fast_tag": "[freeze:contract][value_lifecycle/typed_without_def]",
                "minimal_path_expected_result": "Ok",
                "current_function_take": 0,
                "type_propagation": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "IfElse",
            "SetField",
            "ReturnValue",
            "TypedValueDefinitionVerificationShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "current_function_take": "unselected",
                "type_propagation": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "current_function_take",
            "type_propagation",
            "type_hint_provision",
            "phi_return_type_inference",
            "phi_input_materialization",
            "module_metadata_publication",
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
    spec = typed_value_verification_spec()
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


def run_typed_value_verification_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_typed_value_verification_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_typed_value_verification_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
