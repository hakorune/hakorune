#!/usr/bin/env python3
"""Generate the bounded TypePropagationPipeline Hako artifact."""

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
SOURCE = ROOT / "src/mir/type_propagation/pipeline.rs"
PLAN = FIXTURES / "mirbuilder-type-propagation-pipeline-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-type-propagation-pipeline-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-type-propagation-pipeline-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-type-propagation-pipeline-derived-hako-verifier-result-v0.json"

PIPELINE_STEPS = [
    "seed_declared_field_types",
    "copy_propagation_initial",
    "binop_repropagation",
    "copy_propagation_after_binop",
    "phi_type_inference",
]


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderTypePropagationPipelineDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module TypePropagationPipeline::run",
        "vectors": [
            {
                "name": "minimal_literal_pipeline_runs_all_steps",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "value_types_transport": "self.type_ctx.value_types",
                    "initial_steps_run": 0,
                },
                "expect": {
                    "ok": 1,
                    "steps_run": 5,
                    "function_mutated": 1,
                    "value_types_mutated": 1,
                    "phi_type_inference": 1,
                },
            }
        ],
        "non_claims": {
            "type_hint_provision": 0,
            "metadata_value_type_publication": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderTypePropagationPipelinePlanV1":
        raise ValueError("wrong type propagation pipeline plan kind")
    if "TypePropagationPipelineExecution" not in (plan.get("available_capabilities") or []):
        raise ValueError("type propagation plan lacks TypePropagationPipelineExecution capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "value_types": "self.type_ctx.value_types",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"type propagation profile drift: {key}")
    if plan.get("pipeline_steps") != PIPELINE_STEPS:
        raise ValueError("type propagation pipeline step order drift")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "TypePropagationPipeline::run":
        raise ValueError("type propagation entrypoint drift")
    if result.get("minimal_path_expected_result") != "Ok":
        raise ValueError("type propagation minimal result drift")
    if result.get("mutates") != ["function", "self.type_ctx.value_types"]:
        raise ValueError("type propagation mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"type propagation non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::type_propagation_pipeline",
        method_universe=("TypePropagationPipeline::run",),
        selected_method_ids=("TypePropagationPipeline::run",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "value_types_transport": "TypeContextValueTypesShell",
            "pipeline_steps": tuple(PIPELINE_STEPS),
            "type_hint_provision": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::type_propagation_pipeline",
            api_name="TypePropagationPipelineApi",
            pilot_scope="TypePropagationPipeline_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.artifact.json",
        ),
        selected_body_count_label="type_propagation_pipeline_minimal_literal_profile_only",
        expected_fields=(
            "seed_declared_field_types",
            "copy_propagation_initial",
            "binop_repropagation",
            "copy_propagation_after_binop",
            "phi_type_inference",
            "steps_run",
        ),
    )


def type_propagation_pipeline_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderTypePropagationPipelineDerivedHakoOracleV1":
        raise ValueError("type propagation oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="TypePropagationPipeline::run",
            rust_operation="TypePropagationPipeline::run(&mut function, &mut value_types)",
            hako_operation="Set pipeline step fields + ReturnValue",
            emits="TypePropagationPipelineApi.run(fn_state, value_types)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-type-propagation-pipeline"
        ),
        generator_version="mirbuilder-type-propagation-pipeline-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="TypePropagationPipelineKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="TypePropagationFunctionShellBox",
                fields=[
                    FieldSpec(name="seed_declared_field_types", field_type="i64", initializer="0"),
                    FieldSpec(name="copy_propagation_initial", field_type="i64", initializer="0"),
                    FieldSpec(name="binop_repropagation", field_type="i64", initializer="0"),
                    FieldSpec(name="copy_propagation_after_binop", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_type_inference", field_type="i64", initializer="0"),
                    FieldSpec(name="propagation_complete", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="TypePropagationValueTypesShellBox",
                fields=[
                    FieldSpec(name="mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="copy_entries", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_entries", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="TypePropagationPipelineResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="steps_run", field_type="i64", initializer="0"),
                    FieldSpec(name="function_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="value_types_mutated", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="TypePropagationPipelineApi",
                methods=[
                    ApiMethodSpec(
                        signature="run(fn_state, value_types): TypePropagationPipelineResultBox",
                        operations=[
                            op("SetField", target="fn_state", field="seed_declared_field_types", value=1).to_json(),
                            op("SetField", target="fn_state", field="copy_propagation_initial", value=1).to_json(),
                            op("SetField", target="fn_state", field="binop_repropagation", value=1).to_json(),
                            op("SetField", target="fn_state", field="copy_propagation_after_binop", value=1).to_json(),
                            op("SetField", target="fn_state", field="phi_type_inference", value=1).to_json(),
                            op("SetField", target="fn_state", field="propagation_complete", value=1).to_json(),
                            op("SetField", target="value_types", field="mutated", value=1).to_json(),
                            op("SetField", target="value_types", field="copy_entries", value=2).to_json(),
                            op("SetField", target="value_types", field="phi_entries", value=1).to_json(),
                            op("NewBox", target="result", box="TypePropagationPipelineResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="steps_run", value=5).to_json(),
                            op("SetField", target="result", field="function_mutated", value=1).to_json(),
                            op("SetField", target="result", field="value_types_mutated", value=1).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="TypePropagationFunctionShellBox"),
            op("NewBox", target="value_types", box="TypePropagationValueTypesShellBox"),
            op("StaticCall", target="result", callee="TypePropagationPipelineApi.run", args=["fn_state", "value_types"]),
            op("AssertEq", left="result.ok", right=1, fail_message="type_propagation_ok=fail", fail_code=1),
            op("AssertEq", left="result.steps_run", right=5, fail_message="type_propagation_steps=fail", fail_code=2),
            op("AssertEq", left="fn_state.seed_declared_field_types", right=1, fail_message="type_propagation_seed=fail", fail_code=3),
            op("AssertEq", left="fn_state.copy_propagation_initial", right=1, fail_message="type_propagation_copy1=fail", fail_code=4),
            op("AssertEq", left="fn_state.binop_repropagation", right=1, fail_message="type_propagation_binop=fail", fail_code=5),
            op("AssertEq", left="fn_state.copy_propagation_after_binop", right=1, fail_message="type_propagation_copy2=fail", fail_code=6),
            op("AssertEq", left="fn_state.phi_type_inference", right=1, fail_message="type_propagation_phi=fail", fail_code=7),
            op("AssertEq", left="fn_state.propagation_complete", right=1, fail_message="type_propagation_complete=fail", fail_code=8),
            op("AssertEq", left="value_types.mutated", right=1, fail_message="type_propagation_value_types=fail", fail_code=9),
            op("AssertEq", left="value_types.copy_entries", right=2, fail_message="type_propagation_copy_entries=fail", fail_code=10),
            op("AssertEq", left="value_types.phi_entries", right=1, fail_message="type_propagation_phi_entries=fail", fail_code=11),
            op("Print", text="mirbuilder_type_propagation_pipeline_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_type_propagation_pipeline.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.type_propagation_pipeline",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "type_propagation": 1,
            "type_hint_provision": 0,
            "metadata_value_type_publication": 0,
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
                "type_propagation_only": 1,
                "entrypoint": "TypePropagationPipeline::run",
                "pipeline_steps": PIPELINE_STEPS,
                "function_transport": "MirFunctionPreparedMain",
                "value_types": "self.type_ctx.value_types",
                "minimal_path_expected_result": "Ok",
                "type_hint_provision": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "TypePropagationPipelineExecutionShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "type_hint_provision": "unselected",
                "metadata_value_type_publication": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "type_hint_provision",
            "metadata_value_type_publication",
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
    spec = type_propagation_pipeline_spec()
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


def run_type_propagation_pipeline_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_type_propagation_pipeline_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_type_propagation_pipeline_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
