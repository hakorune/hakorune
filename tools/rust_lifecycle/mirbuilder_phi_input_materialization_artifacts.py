#!/usr/bin/env python3
"""Generate the bounded PHI input materialization Hako artifact."""

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
SOURCE = ROOT / "src/mir/builder/ssa/phi_input_materializer.rs"
PLAN = FIXTURES / "mirbuilder-phi-input-materialization-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-phi-input-materialization-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-phi-input-materialization-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-phi-input-materialization-derived-hako-verifier-result-v0.json"

MATERIALIZATION_STEPS = [
    "PruneUnusedPhiInstructions",
    "CompleteMissingSelfCarriedPhiInputs",
    "CollectPhiInputWorklist",
    "BuildDefBlocksAndDominators",
    "RematerializeIncomingPerPredWithMemo",
    "RewritePhiInputSlots",
    "ReturnChangedCount",
]


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderPhiInputMaterializationDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module phi_input_materializer::materialize_all_phi_inputs",
        "vectors": [
            {
                "name": "prepared_phi_input_materialization_records_step_order_and_changed_count",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "context": "finalize_module",
                    "initial_next_value_id": 10,
                },
                "expect": {
                    "ok": 1,
                    "materialization_steps": len(MATERIALIZATION_STEPS),
                    "changed_count": 1,
                    "function_blocks_mutated": 1,
                    "function_next_value_id_mutated": 1,
                    "dev_birth_verification": 0,
                },
            }
        ],
        "non_claims": {
            "dev_birth_verification": 0,
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
            "semantic_refresh": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderPhiInputMaterializationPlanV1":
        raise ValueError("wrong PHI input materialization plan kind")
    if "PhiInputMaterialization" not in (plan.get("available_capabilities") or []):
        raise ValueError("PHI input materialization plan lacks PhiInputMaterialization capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "context": "finalize_module",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"PHI input materialization profile drift: {key}")
    if plan.get("materialization_steps") != MATERIALIZATION_STEPS:
        raise ValueError(f"PHI input materialization step drift: {plan.get('materialization_steps')}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "phi_input_materializer::materialize_all_phi_inputs":
        raise ValueError("PHI input materialization entrypoint drift")
    if result.get("minimal_path_expected_result") != "Result<usize, String>":
        raise ValueError("PHI input materialization result drift")
    if result.get("mutates") != ["function.blocks", "function.next_value_id"]:
        raise ValueError("PHI input materialization mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"PHI input materialization non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::phi_input_materialization",
        method_universe=("PhiInputMaterialization::run",),
        selected_method_ids=("PhiInputMaterialization::run",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "context": "finalize_module",
            "materialization_steps": tuple(MATERIALIZATION_STEPS),
            "dev_birth_verification": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::phi_input_materialization",
            api_name="PhiInputMaterializationApi",
            pilot_scope="PhiInputMaterialization_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.artifact.json",
        ),
        selected_body_count_label="phi_input_materialization_minimal_literal_profile_only",
        expected_fields=(
            "materialization_steps",
            "changed_count",
            "function_blocks_mutated",
            "function_next_value_id_mutated",
            "dev_birth_verification",
        ),
    )


def phi_input_materialization_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderPhiInputMaterializationDerivedHakoOracleV1":
        raise ValueError("PHI input materialization oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="PhiInputMaterialization::run",
            rust_operation="phi_input_materializer::materialize_all_phi_inputs",
            hako_operation="MaterializationStepFlags + SetField + ReturnValue",
            emits="PhiInputMaterializationApi.run(fn_state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-phi-input-materialization"
        ),
        generator_version="mirbuilder-phi-input-materialization-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="PhiInputMaterializationKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="PhiInputFunctionShellBox",
                fields=[
                    FieldSpec(name="prune_unused_phi_instructions", field_type="i64", initializer="0"),
                    FieldSpec(name="complete_missing_self_carried_phi_inputs", field_type="i64", initializer="0"),
                    FieldSpec(name="collect_phi_input_worklist", field_type="i64", initializer="0"),
                    FieldSpec(name="build_def_blocks_and_dominators", field_type="i64", initializer="0"),
                    FieldSpec(name="rematerialize_incoming_per_pred_with_memo", field_type="i64", initializer="0"),
                    FieldSpec(name="rewrite_phi_input_slots", field_type="i64", initializer="0"),
                    FieldSpec(name="return_changed_count", field_type="i64", initializer="0"),
                    FieldSpec(name="materialization_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="changed_count", field_type="i64", initializer="0"),
                    FieldSpec(name="function_blocks_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="function_next_value_id_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="dev_birth_verification", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="PhiInputMaterializationResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="changed_count", field_type="i64", initializer="0"),
                    FieldSpec(name="materialization_steps", field_type="i64", initializer="0"),
                    FieldSpec(name="function_blocks_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="function_next_value_id_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="dev_birth_verification", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="PhiInputMaterializationApi",
                methods=[
                    ApiMethodSpec(
                        signature="run(fn_state): PhiInputMaterializationResultBox",
                        operations=[
                            op("SetField", target="fn_state", field="prune_unused_phi_instructions", value=1).to_json(),
                            op("SetField", target="fn_state", field="complete_missing_self_carried_phi_inputs", value=1).to_json(),
                            op("SetField", target="fn_state", field="collect_phi_input_worklist", value=1).to_json(),
                            op("SetField", target="fn_state", field="build_def_blocks_and_dominators", value=1).to_json(),
                            op("SetField", target="fn_state", field="rematerialize_incoming_per_pred_with_memo", value=1).to_json(),
                            op("SetField", target="fn_state", field="rewrite_phi_input_slots", value=1).to_json(),
                            op("SetField", target="fn_state", field="return_changed_count", value=1).to_json(),
                            op("SetField", target="fn_state", field="materialization_steps", value=len(MATERIALIZATION_STEPS)).to_json(),
                            op("SetField", target="fn_state", field="changed_count", value=1).to_json(),
                            op("SetField", target="fn_state", field="function_blocks_mutated", value=1).to_json(),
                            op("SetField", target="fn_state", field="function_next_value_id_mutated", value=1).to_json(),
                            op("SetField", target="fn_state", field="dev_birth_verification", value=0).to_json(),
                            op("NewBox", target="result", box="PhiInputMaterializationResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="changed_count", value=1).to_json(),
                            op("SetField", target="result", field="materialization_steps", value=len(MATERIALIZATION_STEPS)).to_json(),
                            op("SetField", target="result", field="function_blocks_mutated", value=1).to_json(),
                            op("SetField", target="result", field="function_next_value_id_mutated", value=1).to_json(),
                            op("SetField", target="result", field="dev_birth_verification", value=0).to_json(),
                            op("SetField", target="result", field="full_finalize_module", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="PhiInputFunctionShellBox"),
            op("StaticCall", target="result", callee="PhiInputMaterializationApi.run", args=["fn_state"]),
            op("AssertEq", left="result.ok", right=1, fail_message="phi_input_materialization_ok=fail", fail_code=1),
            op("AssertEq", left="result.changed_count", right=1, fail_message="phi_input_materialization_changed=fail", fail_code=2),
            op("AssertEq", left="result.materialization_steps", right=len(MATERIALIZATION_STEPS), fail_message="phi_input_materialization_steps=fail", fail_code=3),
            op("AssertEq", left="result.function_blocks_mutated", right=1, fail_message="phi_input_materialization_blocks=fail", fail_code=4),
            op("AssertEq", left="result.function_next_value_id_mutated", right=1, fail_message="phi_input_materialization_next_value=fail", fail_code=5),
            op("AssertEq", left="result.dev_birth_verification", right=0, fail_message="phi_input_materialization_dev_birth=fail", fail_code=6),
            op("AssertEq", left="result.full_finalize_module", right=0, fail_message="phi_input_materialization_full_finalize=fail", fail_code=7),
            op("AssertEq", left="fn_state.prune_unused_phi_instructions", right=1, fail_message="phi_input_materialization_prune=fail", fail_code=8),
            op("AssertEq", left="fn_state.complete_missing_self_carried_phi_inputs", right=1, fail_message="phi_input_materialization_self_carried=fail", fail_code=9),
            op("AssertEq", left="fn_state.collect_phi_input_worklist", right=1, fail_message="phi_input_materialization_worklist=fail", fail_code=10),
            op("AssertEq", left="fn_state.build_def_blocks_and_dominators", right=1, fail_message="phi_input_materialization_analysis=fail", fail_code=11),
            op("AssertEq", left="fn_state.rematerialize_incoming_per_pred_with_memo", right=1, fail_message="phi_input_materialization_remat=fail", fail_code=12),
            op("AssertEq", left="fn_state.rewrite_phi_input_slots", right=1, fail_message="phi_input_materialization_rewrite=fail", fail_code=13),
            op("AssertEq", left="fn_state.return_changed_count", right=1, fail_message="phi_input_materialization_return_count=fail", fail_code=14),
            op("Print", text="mirbuilder_phi_input_materialization_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_phi_input_materialization.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.phi_input_materialization",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "phi_input_materialization": 1,
            "dev_birth_verification": 0,
            "module_function_insertion": 0,
            "condition_fn_injection": 0,
            "all_functions_phi_materialization": 0,
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
                "phi_input_materialization_only": 1,
                "entrypoint": "phi_input_materializer::materialize_all_phi_inputs",
                "materialization_steps": MATERIALIZATION_STEPS,
                "function_transport": "MirFunctionPreparedMain",
                "context": "finalize_module",
                "minimal_path_expected_result": "Result<usize, String>",
                "dev_birth_verification": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "PhiInputMaterializationShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "mutation_frame": ("function.blocks", "function.next_value_id"),
                "dev_birth_verification": "unselected",
                "module_function_insertion": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "dev_birth_verification",
            "module_function_insertion",
            "condition_fn_injection",
            "all_functions_phi_materialization",
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
    spec = phi_input_materialization_spec()
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


def run_phi_input_materialization_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_phi_input_materialization_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_phi_input_materialization_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
