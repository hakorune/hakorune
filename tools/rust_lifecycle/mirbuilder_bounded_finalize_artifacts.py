#!/usr/bin/env python3
"""Generate the bounded finalize_module composition Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-bounded-finalize-composition-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-bounded-finalize-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-bounded-finalize-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-bounded-finalize-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderBoundedFinalizeDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module prepared minimal profile",
        "vectors": [
            {
                "name": "literal_integer_module_finalization",
                "inputs": {
                    "result_value": 1,
                    "published_type_is_integer": 1,
                    "initial_current_module_present": 1,
                    "initial_current_function_present": 1,
                    "initial_current_block_present": 1,
                },
                "expect": {
                    "return_value": 1,
                    "main_function_return_type_integer": 1,
                    "main_block_terminated": 1,
                    "module_function_count": 2,
                    "condition_fn_present": 1,
                    "module_metadata_published": 1,
                    "semantic_refresh_subset_applied": 1,
                    "state_current_module_present": 0,
                    "state_current_function_present": 0,
                    "state_current_block_present": 0,
                },
            }
        ],
        "non_claims": {
            "full_finalize_module": 0,
            "full_build_module_execution": 0,
            "reusable_return_emission": 0,
            "reusable_type_publication": 0,
            "current_module_take_artifact": 0,
            "current_function_take_artifact": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderBoundedFinalizeCompositionPlanV1":
        raise ValueError("wrong bounded finalize plan kind")
    if "FinalizeModuleComposition" not in (plan.get("available_capabilities") or []):
        raise ValueError("bounded finalize plan lacks FinalizeModuleComposition capability")
    steps = [row.get("step") for row in plan.get("composition") or [] if isinstance(row, dict)]
    required = [
        "append_return_if_unterminated",
        "update_return_type_from_result",
        "take_module",
        "verify_typed_values",
        "take_function",
        "type_propagation",
        "publish_value_types",
        "module_add_main_function",
        "inject_condition_fn_if_missing",
        "publish_module_metadata",
        "refresh_module_plans_subset",
        "materialize_phi_inputs_all_functions",
        "return_module",
    ]
    cursor = -1
    for step in required:
        if step not in steps:
            raise ValueError(f"bounded finalize missing step: {step}")
        index = steps.index(step)
        if index <= cursor:
            raise ValueError(f"bounded finalize step order drift: {step}")
        cursor = index
    non_claims = plan.get("non_claims") or {}
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"bounded finalize non-claim must remain 0: {key}")
    profile = plan.get("execution_profile") or {}
    if not isinstance(profile, dict) or profile.get("condition_fn_initially_missing") is not True:
        raise ValueError("bounded finalize profile must require condition_fn injection")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::bounded_finalize_composition",
        method_universe=(
            "PreparedFinalizeStateShell::new",
            "BoundedFinalizeComposition::finalize",
        ),
        selected_method_ids=(
            "PreparedFinalizeStateShell::new",
            "BoundedFinalizeComposition::finalize",
        ),
        denials=(),
        semantic_transports={
            "state_transport": "PreparedFinalizeStateShell",
            "module_transport": "FinalizedMirModuleShell",
            "function_transport": "FinalizedMirFunctionShell",
            "block_transport": "FinalizedBasicBlockShell",
            "literal_result_transport": "LiteralIntegerLoweringResultShell",
            "result_transport": "FinalizedMirModuleShell",
            "full_finalize_module": 0,
            "mainline_selected": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::bounded_finalize_composition",
            api_name="BoundedFinalizeCompositionApi",
            pilot_scope="BoundedFinalizeComposition_prepared_minimal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json",
        ),
        selected_body_count_label="bounded_finalize_composition_prepared_minimal_profile_only",
        expected_fields=("current_module", "scope_ctx", "current_block"),
    )


def _boxes() -> list[BoxSpec]:
    return [
        BoxSpec(
            name="FinalizedBasicBlockShellBox",
            fields=[
                FieldSpec(name="id", field_type="i64", initializer="0"),
                FieldSpec(name="terminated", field_type="i64", initializer="0"),
                FieldSpec(name="return_value", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="FinalizedMirFunctionShellBox",
            fields=[
                FieldSpec(name="entry_block", field_type="FinalizedBasicBlockShellBox", initializer="new FinalizedBasicBlockShellBox()"),
                FieldSpec(name="return_type_is_integer", field_type="i64", initializer="0"),
                FieldSpec(name="value_type_count", field_type="i64", initializer="0"),
                FieldSpec(name="metadata_value_types_published", field_type="i64", initializer="0"),
                FieldSpec(name="metadata_origin_callers_merged", field_type="i64", initializer="0"),
                FieldSpec(name="phi_return_type_inferred", field_type="i64", initializer="0"),
                FieldSpec(name="phi_inputs_materialized", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="FinalizedMirModuleShellBox",
            fields=[
                FieldSpec(name="functions_count", field_type="i64", initializer="0"),
                FieldSpec(name="main_function", field_type="FinalizedMirFunctionShellBox", initializer="null"),
                FieldSpec(name="condition_fn_present", field_type="i64", initializer="0"),
                FieldSpec(name="region_popped", field_type="i64", initializer="0"),
                FieldSpec(name="slot_registry_released", field_type="i64", initializer="0"),
                FieldSpec(name="metadata_published", field_type="i64", initializer="0"),
                FieldSpec(name="record_packed_layout_refreshed", field_type="i64", initializer="0"),
                FieldSpec(name="typed_object_plan_refreshed", field_type="i64", initializer="0"),
                FieldSpec(name="direct_state_plan_refreshed", field_type="i64", initializer="0"),
                FieldSpec(name="all_functions_phi_materialized", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="PreparedFinalizeScopeShellBox",
            fields=[
                FieldSpec(name="current_function", field_type="FinalizedMirFunctionShellBox", initializer="null"),
                FieldSpec(name="current_function_present", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="PreparedFinalizeStateShellBox",
            fields=[
                FieldSpec(name="current_module", field_type="FinalizedMirModuleShellBox", initializer="null"),
                FieldSpec(name="current_module_present", field_type="i64", initializer="0"),
                FieldSpec(name="scope_ctx", field_type="PreparedFinalizeScopeShellBox", initializer="new PreparedFinalizeScopeShellBox()"),
                FieldSpec(name="current_block", field_type="i64", initializer="0"),
                FieldSpec(name="current_block_present", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="PublishedIntegerTypeShellBox",
            fields=[
                FieldSpec(name="value_id", field_type="i64", initializer="0"),
                FieldSpec(name="is_integer", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="LiteralIntegerLoweringResultBox",
            fields=[
                FieldSpec(name="result_value", field_type="i64", initializer="0"),
                FieldSpec(name="published_type", field_type="PublishedIntegerTypeShellBox", initializer="new PublishedIntegerTypeShellBox()"),
            ],
        ),
    ]


def _finalize_operations() -> list[dict[str, object]]:
    return [
        op("SetField", target="block", field="terminated", value=1).to_json(),
        op("SetField", target="block", field="return_value", value="result_value").to_json(),
        op("SetField", target="func", field="entry_block", value="block").to_json(),
        op("SetField", target="func", field="return_type_is_integer", value="published_type_is_integer").to_json(),
        op("SetField", target="func", field="value_type_count", value=1).to_json(),
        op("SetField", target="func", field="metadata_value_types_published", value=1).to_json(),
        op("SetField", target="func", field="metadata_origin_callers_merged", value=1).to_json(),
        op("SetField", target="func", field="phi_return_type_inferred", value=1).to_json(),
        op("SetField", target="func", field="phi_inputs_materialized", value=1).to_json(),
        op("SetField", target="module", field="main_function", value="func").to_json(),
        op("SetField", target="module", field="functions_count", value=2).to_json(),
        op("SetField", target="module", field="condition_fn_present", value=1).to_json(),
        op("SetField", target="module", field="region_popped", value=1).to_json(),
        op("SetField", target="module", field="slot_registry_released", value=1).to_json(),
        op("SetField", target="module", field="metadata_published", value=1).to_json(),
        op("SetField", target="module", field="record_packed_layout_refreshed", value=1).to_json(),
        op("SetField", target="module", field="typed_object_plan_refreshed", value=1).to_json(),
        op("SetField", target="module", field="direct_state_plan_refreshed", value=1).to_json(),
        op("SetField", target="module", field="all_functions_phi_materialized", value=1).to_json(),
        op("SetField", target="state", field="current_module_present", value=0).to_json(),
        op("SetField", target="state.scope_ctx", field="current_function_present", value=0).to_json(),
        op("SetField", target="state", field="current_block_present", value=0).to_json(),
        op("ReturnValue", value="module").to_json(),
    ]


def bounded_finalize_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderBoundedFinalizeDerivedHakoOracleV1":
        raise ValueError("bounded finalize oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="PreparedFinalizeStateShell::new",
            rust_operation="prepared minimal current module/function/block state",
            hako_operation="NewBox + SetField",
            emits="PreparedFinalizeStateShellBox birth + Main setup",
        ),
        BehaviorMethodSpec(
            id="BoundedFinalizeComposition::finalize",
            rust_operation="MirBuilder::finalize_module bounded minimal profile",
            hako_operation="SetField + ReturnValue",
            emits="BoundedFinalizeCompositionApi.finalize(state, module, func, block, result_value, published_type)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-bounded-finalize-composition"
        ),
        generator_version="mirbuilder-bounded-finalize-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="BoundedFinalizeCompositionKernel", fields=[]),
        additional_boxes=_boxes(),
        static_boxes=[
            StaticBoxSpec(
                name="BoundedFinalizeCompositionApi",
                methods=[
                    ApiMethodSpec(
                        signature=(
                            "finalize(state, module, func, block, result_value, published_type_is_integer): FinalizedMirModuleShellBox"
                        ),
                        operations=_finalize_operations(),
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="module", box="FinalizedMirModuleShellBox"),
            op("NewBox", target="func", box="FinalizedMirFunctionShellBox"),
            op("NewBox", target="block", box="FinalizedBasicBlockShellBox"),
            op("SetField", target="block", field="id", value=0),
            op("NewBox", target="state", box="PreparedFinalizeStateShellBox"),
            op("SetField", target="state", field="current_module", value="module"),
            op("SetField", target="state", field="current_module_present", value=1),
            op("SetField", target="state.scope_ctx", field="current_function", value="func"),
            op("SetField", target="state.scope_ctx", field="current_function_present", value=1),
            op("SetField", target="state", field="current_block", value=0),
            op("SetField", target="state", field="current_block_present", value=1),
            op("NewBox", target="literal_result", box="LiteralIntegerLoweringResultBox"),
            op("SetField", target="literal_result", field="result_value", value=1),
            op("SetField", target="literal_result.published_type", field="value_id", value=1),
            op("SetField", target="literal_result.published_type", field="is_integer", value=1),
            op(
                "StaticCall",
                target="finalized",
                callee="BoundedFinalizeCompositionApi.finalize",
                args=[
                    "state",
                    "module",
                    "func",
                    "block",
                    "literal_result.result_value",
                    "literal_result.published_type.is_integer",
                ],
            ),
            op("AssertEq", left="block.terminated", right=1, fail_message="bounded_finalize_return_terminated=fail", fail_code=1),
            op("AssertEq", left="block.return_value", right=1, fail_message="bounded_finalize_return_value=fail", fail_code=2),
            op("AssertEq", left="func.return_type_is_integer", right=1, fail_message="bounded_finalize_return_type=fail", fail_code=3),
            op("AssertEq", left="finalized.functions_count", right=2, fail_message="bounded_finalize_function_count=fail", fail_code=4),
            op("AssertEq", left="finalized.condition_fn_present", right=1, fail_message="bounded_finalize_condition_fn=fail", fail_code=5),
            op("AssertEq", left="finalized.metadata_published", right=1, fail_message="bounded_finalize_metadata=fail", fail_code=6),
            op("AssertEq", left="finalized.record_packed_layout_refreshed", right=1, fail_message="bounded_finalize_record_refresh=fail", fail_code=7),
            op("AssertEq", left="finalized.typed_object_plan_refreshed", right=1, fail_message="bounded_finalize_typed_refresh=fail", fail_code=8),
            op("AssertEq", left="finalized.direct_state_plan_refreshed", right=1, fail_message="bounded_finalize_direct_refresh=fail", fail_code=9),
            op("AssertEq", left="finalized.all_functions_phi_materialized", right=1, fail_message="bounded_finalize_all_phi=fail", fail_code=10),
            op("AssertEq", left="state.current_module_present", right=0, fail_message="bounded_finalize_module_taken=fail", fail_code=11),
            op("AssertEq", left="state.scope_ctx.current_function_present", right=0, fail_message="bounded_finalize_function_taken=fail", fail_code=12),
            op("AssertEq", left="state.current_block_present", right=0, fail_message="bounded_finalize_block_taken=fail", fail_code=13),
            op("Print", text="mirbuilder_bounded_finalize_composition_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_bounded_finalize_composition.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.prepared_minimal_profile",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "bounded_finalize_composition": 1,
            "full_finalize_module": 0,
            "full_build_module_execution": 0,
            "reusable_return_emission": 0,
            "reusable_type_publication": 0,
            "current_module_take_artifact": 0,
            "current_function_take_artifact": 0,
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
                "bounded_finalize_composition_only": 1,
                "return_instruction_shell_connected": 1,
                "return_type_integer_published": 1,
                "module_function_count_includes_condition_fn": 1,
                "condition_fn_injection_source_required": 1,
                "metadata_publication_shell": 1,
                "semantic_refresh_subset_shell": 1,
                "state_take_presence_tags_cleared": 1,
                "full_finalize_module": 0,
                "full_build_module_execution": 0,
                "mainline_selected": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "BoundedReturnInstructionShell",
            "BoundedConditionFnInjectionShell",
            "BoundedSemanticRefreshSubsetShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "current_module_take_transport": "presence tag cleared; payload not canonical Option",
                "current_function_take_transport": "presence tag cleared; payload not canonical Option",
                "return_emission_artifact": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "full_finalize_module",
            "full_build_module_execution",
            "reusable_return_emission",
            "reusable_type_publication",
            "current_module_take_artifact",
            "current_function_take_artifact",
            "mainline_selected",
            "source_selfhost_claim",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = bounded_finalize_spec()
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


def run_bounded_finalize_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_bounded_finalize_composition_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_bounded_finalize_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
