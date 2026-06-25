#!/usr/bin/env python3
"""Generate the bounded ReturnTypePublication Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-return-type-publication-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-return-type-publication-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-return-type-publication-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-return-type-publication-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnTypePublicationDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module publish return type from result_value",
        "vectors": [
            {
                "name": "integer_result_value_publishes_integer_return_type",
                "inputs": {
                    "result_value": 7,
                    "initial_return_type_is_integer": 0,
                    "type_record_value_id": 7,
                    "type_record_is_integer": 1,
                },
                "expect": {
                    "return_type_is_integer": 1,
                    "published_from_value_id": 7,
                    "publication_present": 1,
                },
            }
        ],
        "non_claims": {
            "module_take": 0,
            "verify_typed_values": 0,
            "full_finalize_module": 0,
            "phi_return_type_inference": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderReturnTypePublicationPlanV1":
        raise ValueError("wrong return type publication plan kind")
    if "ReturnTypePublication" not in (plan.get("available_capabilities") or []):
        raise ValueError("return type publication plan lacks ReturnTypePublication capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "result_value_transport": "ValueIdAsI64",
        "result_value_type": "MirType::Integer",
        "initial_function_return_type": "MirType::Void",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"return type publication profile drift: {key}")
    result = plan.get("result_contract") or {}
    expected_result = {
        "signature_return_type": "MirType::Integer",
        "source_value_type": "type_ctx.value_types[result_value]",
        "source_value_type_owner": "LiteralIntegerLowering",
    }
    for key, value in expected_result.items():
        if result.get(key) != value:
            raise ValueError(f"return type publication result contract drift: {key}")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"return type publication non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::return_type_publication",
        method_universe=("ReturnTypePublication::publish",),
        selected_method_ids=("ReturnTypePublication::publish",),
        denials=(),
        semantic_transports={
            "function_signature_transport": "ReturnTypeFunctionSignatureShell",
            "result_value_transport": "ValueIdAsI64",
            "value_type_record_transport": "ReturnTypeValueTypeRecordShell",
            "published_return_type": "MirType::Integer",
            "module_take": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::return_type_publication",
            api_name="ReturnTypePublicationApi",
            pilot_scope="ReturnTypePublication_integer_result_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json",
        ),
        selected_body_count_label="return_type_publication_integer_result_only",
        expected_fields=(
            "return_type_is_integer",
            "published_from_value_id",
            "publication_present",
        ),
    )


def return_type_publication_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderReturnTypePublicationDerivedHakoOracleV1":
        raise ValueError("return type publication oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="ReturnTypePublication::publish",
            rust_operation="MirBuilder::finalize_module publish return type from result_value",
            hako_operation="ValidateValueType + SetField + ReturnValue",
            emits="ReturnTypePublicationApi.publish(signature, value_type, result_value)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-return-type-publication"
        ),
        generator_version="mirbuilder-return-type-publication-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="ReturnTypePublicationKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="ReturnTypeFunctionSignatureShellBox",
                fields=[
                    FieldSpec(name="return_type_is_integer", field_type="i64", initializer="0"),
                    FieldSpec(name="published_from_value_id", field_type="i64", initializer="0"),
                    FieldSpec(name="publication_present", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="ReturnTypeValueTypeRecordShellBox",
                fields=[
                    FieldSpec(name="value_id", field_type="i64", initializer="0"),
                    FieldSpec(name="is_integer", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="ReturnTypePublicationApi",
                methods=[
                    ApiMethodSpec(
                        signature="publish(signature, value_type, result_value): ReturnTypeFunctionSignatureShellBox",
                        operations=[
                            op("SetField", target="signature", field="return_type_is_integer", value=1).to_json(),
                            op("SetField", target="signature", field="published_from_value_id", value="result_value").to_json(),
                            op("SetField", target="signature", field="publication_present", value=1).to_json(),
                            op("ReturnValue", value="signature").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="signature", box="ReturnTypeFunctionSignatureShellBox"),
            op("NewBox", target="value_type", box="ReturnTypeValueTypeRecordShellBox"),
            op("SetField", target="value_type", field="value_id", value=7),
            op("SetField", target="value_type", field="is_integer", value=1),
            op("AssertEq", left="value_type.value_id", right=7, fail_message="return_type_value_id=fail", fail_code=1),
            op("AssertEq", left="value_type.is_integer", right=1, fail_message="return_type_integer=fail", fail_code=2),
            op("StaticCall", target="published", callee="ReturnTypePublicationApi.publish", args=["signature", "value_type", 7]),
            op("AssertEq", left="published.return_type_is_integer", right=1, fail_message="return_type_published=fail", fail_code=3),
            op("AssertEq", left="published.published_from_value_id", right=7, fail_message="return_type_source_value=fail", fail_code=4),
            op("AssertEq", left="published.publication_present", right=1, fail_message="return_type_publication_present=fail", fail_code=5),
            op("Print", text="mirbuilder_return_type_publication_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_return_type_publication.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.return_type_publication",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "return_type_publication": 1,
            "module_take": 0,
            "verify_typed_values": 0,
            "full_finalize_module": 0,
            "phi_return_type_inference": 0,
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
                "return_type_publication_only": 1,
                "source_value_type_owner": "LiteralIntegerLowering",
                "result_value_transport": "ValueIdAsI64",
                "signature_return_type": "MirType::Integer",
                "module_take": 0,
                "verify_typed_values": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "MirTypeIntegerReturnShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "module_take": "unselected",
                "verify_typed_values": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "module_take",
            "verify_typed_values",
            "full_finalize_module",
            "phi_return_type_inference",
            "mainline_selected",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = return_type_publication_spec()
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


def run_return_type_publication_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_return_type_publication_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_return_type_publication_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
