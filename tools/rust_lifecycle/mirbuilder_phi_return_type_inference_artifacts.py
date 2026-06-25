#!/usr/bin/env python3
"""Generate the bounded PHI return-type inference Hako artifact."""

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
SOURCE = ROOT / "src/mir/builder/phi_type_inference.rs"
PLAN = FIXTURES / "mirbuilder-phi-return-type-inference-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-phi-return-type-inference-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-phi-return-type-inference-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-phi-return-type-inference-derived-hako-verifier-result-v0.json"

RESOLVER_CHAIN = [
    "SkipConcreteReturnType",
    "TerminatorReturnOnly",
    "DirectValueTypesLookup",
    "TypeHintPolicyExtract",
    "MethodReturnHintBox",
    "PhiTypeResolver",
    "GenericTypeResolver",
    "UnknownFallbackOutsideDebug",
]


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiReturnTypeInferenceDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module phi_type_inference::infer_return_type_from_phi",
        "vectors": [
            {
                "name": "unknown_return_type_infers_integer_from_phi_resolver_chain",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "builder_type_context": "self.type_ctx.value_types",
                    "initial_return_type": "MirType::Unknown",
                    "phi_value_type": "MirType::Integer",
                },
                "expect": {
                    "ok": 1,
                    "inferred": 1,
                    "return_type_is_integer": 1,
                    "resolver_steps": len(RESOLVER_CHAIN),
                    "phi_input_materialization": 0,
                },
            }
        ],
        "non_claims": {
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderPhiReturnTypeInferencePlanV1":
        raise ValueError("wrong PHI return-type inference plan kind")
    if "PhiReturnTypeInference" not in (plan.get("available_capabilities") or []):
        raise ValueError("PHI return-type inference plan lacks PhiReturnTypeInference capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "builder_type_context": "self.type_ctx.value_types",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"PHI return-type inference profile drift: {key}")
    if plan.get("resolver_chain") != RESOLVER_CHAIN:
        raise ValueError(f"PHI return-type inference resolver chain drift: {plan.get('resolver_chain')}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "phi_type_inference::infer_return_type_from_phi":
        raise ValueError("PHI return-type inference entrypoint drift")
    if result.get("minimal_path_expected_result") != "Option<MirType>":
        raise ValueError("PHI return-type inference result drift")
    if result.get("mutates") != ["function.signature.return_type"]:
        raise ValueError("PHI return-type inference mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"PHI return-type inference non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::phi_return_type_inference",
        method_universe=("PhiReturnTypeInference::infer",),
        selected_method_ids=("PhiReturnTypeInference::infer",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "builder_type_context": "self.type_ctx.value_types",
            "result_transport": "OptionMirType",
            "resolver_chain": tuple(RESOLVER_CHAIN),
            "phi_input_materialization": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::phi_return_type_inference",
            api_name="PhiReturnTypeInferenceApi",
            pilot_scope="PhiReturnTypeInference_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.artifact.json",
        ),
        selected_body_count_label="phi_return_type_inference_minimal_literal_profile_only",
        expected_fields=(
            "signature_return_type_is_integer",
            "inferred_return_type_present",
            "resolver_steps",
            "phi_input_materialization",
        ),
    )


def phi_return_type_inference_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderPhiReturnTypeInferenceDerivedHakoOracleV1":
        raise ValueError("PHI return-type inference oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="PhiReturnTypeInference::infer",
            rust_operation="phi_type_inference::infer_return_type_from_phi",
            hako_operation="ResolverChainFlags + SetField + ReturnValue",
            emits="PhiReturnTypeInferenceApi.infer(builder_state, fn_state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-phi-return-type-inference"
        ),
        generator_version="mirbuilder-phi-return-type-inference-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="PhiReturnTypeInferenceKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="PhiReturnBuilderShellBox",
                fields=[
                    FieldSpec(name="direct_value_type_lookup", field_type="i64", initializer="0"),
                    FieldSpec(name="type_hint_policy_checked", field_type="i64", initializer="0"),
                    FieldSpec(name="method_return_hint_checked", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_type_resolver_checked", field_type="i64", initializer="0"),
                    FieldSpec(name="generic_type_resolver_checked", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="PhiReturnFunctionShellBox",
                fields=[
                    FieldSpec(name="current_return_type_unknown", field_type="i64", initializer="1"),
                    FieldSpec(name="terminator_return_seen", field_type="i64", initializer="0"),
                    FieldSpec(name="signature_return_type_is_integer", field_type="i64", initializer="0"),
                    FieldSpec(name="inferred_return_type_present", field_type="i64", initializer="0"),
                    FieldSpec(name="resolver_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_input_materialization", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="PhiReturnTypeInferenceResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="inferred", field_type="i64", initializer="0"),
                    FieldSpec(name="return_type_is_integer", field_type="i64", initializer="0"),
                    FieldSpec(name="resolver_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_input_materialization", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="PhiReturnTypeInferenceApi",
                methods=[
                    ApiMethodSpec(
                        signature="infer(builder_state, fn_state): PhiReturnTypeInferenceResultBox",
                        operations=[
                            op("SetField", target="builder_state", field="direct_value_type_lookup", value=1).to_json(),
                            op("SetField", target="builder_state", field="type_hint_policy_checked", value=1).to_json(),
                            op("SetField", target="builder_state", field="method_return_hint_checked", value=1).to_json(),
                            op("SetField", target="builder_state", field="phi_type_resolver_checked", value=1).to_json(),
                            op("SetField", target="builder_state", field="generic_type_resolver_checked", value=1).to_json(),
                            op("SetField", target="fn_state", field="terminator_return_seen", value=1).to_json(),
                            op("SetField", target="fn_state", field="signature_return_type_is_integer", value=1).to_json(),
                            op("SetField", target="fn_state", field="inferred_return_type_present", value=1).to_json(),
                            op("SetField", target="fn_state", field="resolver_steps", value=len(RESOLVER_CHAIN)).to_json(),
                            op("SetField", target="fn_state", field="phi_input_materialization", value=0).to_json(),
                            op("NewBox", target="result", box="PhiReturnTypeInferenceResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="inferred", value=1).to_json(),
                            op("SetField", target="result", field="return_type_is_integer", value=1).to_json(),
                            op("SetField", target="result", field="resolver_steps", value=len(RESOLVER_CHAIN)).to_json(),
                            op("SetField", target="result", field="phi_input_materialization", value=0).to_json(),
                            op("SetField", target="result", field="full_finalize_module", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="builder_state", box="PhiReturnBuilderShellBox"),
            op("NewBox", target="fn_state", box="PhiReturnFunctionShellBox"),
            op("StaticCall", target="result", callee="PhiReturnTypeInferenceApi.infer", args=["builder_state", "fn_state"]),
            op("AssertEq", left="result.ok", right=1, fail_message="phi_return_type_ok=fail", fail_code=1),
            op("AssertEq", left="result.inferred", right=1, fail_message="phi_return_type_inferred=fail", fail_code=2),
            op("AssertEq", left="result.return_type_is_integer", right=1, fail_message="phi_return_type_integer=fail", fail_code=3),
            op("AssertEq", left="result.resolver_steps", right=len(RESOLVER_CHAIN), fail_message="phi_return_type_resolver_steps=fail", fail_code=4),
            op("AssertEq", left="result.phi_input_materialization", right=0, fail_message="phi_return_type_input_materialization=fail", fail_code=5),
            op("AssertEq", left="result.full_finalize_module", right=0, fail_message="phi_return_type_full_finalize=fail", fail_code=6),
            op("AssertEq", left="builder_state.direct_value_type_lookup", right=1, fail_message="phi_return_type_direct_lookup=fail", fail_code=7),
            op("AssertEq", left="builder_state.type_hint_policy_checked", right=1, fail_message="phi_return_type_hint_policy=fail", fail_code=8),
            op("AssertEq", left="builder_state.method_return_hint_checked", right=1, fail_message="phi_return_type_method_hint=fail", fail_code=9),
            op("AssertEq", left="builder_state.phi_type_resolver_checked", right=1, fail_message="phi_return_type_phi_resolver=fail", fail_code=10),
            op("AssertEq", left="builder_state.generic_type_resolver_checked", right=1, fail_message="phi_return_type_generic_resolver=fail", fail_code=11),
            op("AssertEq", left="fn_state.terminator_return_seen", right=1, fail_message="phi_return_type_return_seen=fail", fail_code=12),
            op("AssertEq", left="fn_state.signature_return_type_is_integer", right=1, fail_message="phi_return_type_signature=fail", fail_code=13),
            op("AssertEq", left="fn_state.inferred_return_type_present", right=1, fail_message="phi_return_type_present=fail", fail_code=14),
            op("AssertEq", left="fn_state.phi_input_materialization", right=0, fail_message="phi_return_type_phi_input=fail", fail_code=15),
            op("Print", text="mirbuilder_phi_return_type_inference_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_phi_return_type_inference.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.phi_return_type_inference",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "phi_return_type_inference": 1,
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
                "phi_return_type_inference_only": 1,
                "entrypoint": "phi_type_inference::infer_return_type_from_phi",
                "resolver_chain": RESOLVER_CHAIN,
                "function_transport": "MirFunctionPreparedMain",
                "builder_type_context": "self.type_ctx.value_types",
                "minimal_path_expected_result": "Option<MirType>",
                "phi_input_materialization": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "PhiReturnTypeInferenceShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "result_transport": "OptionMirType",
                "phi_input_materialization": "unselected",
                "module_function_insertion": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
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
    spec = phi_return_type_inference_spec()
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


def run_phi_return_type_inference_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_phi_return_type_inference_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_phi_return_type_inference_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
