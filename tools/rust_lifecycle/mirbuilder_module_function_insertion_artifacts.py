#!/usr/bin/env python3
"""Generate the bounded module function insertion Hako artifact."""

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
SOURCE = ROOT / "src/mir/function/module_impl.rs"
PLAN = FIXTURES / "mirbuilder-module-function-insertion-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-module-function-insertion-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-module-function-insertion-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-module-function-insertion-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderModuleFunctionInsertionDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module module.add_function(function)",
        "vectors": [
            {
                "name": "prepared_module_inserts_main_function_by_signature_name",
                "inputs": {
                    "module_transport": "MirModuleMinimalShell",
                    "function_transport": "MirFunctionPreparedMain",
                    "function_name": "main",
                    "initial_function_count": 0,
                },
                "expect": {
                    "ok": 1,
                    "function_count": 1,
                    "inserted_name": "main",
                    "collision_policy": "ReplaceExistingByName",
                    "condition_fn_injection": 0,
                    "full_finalize_module": 0,
                },
            }
        ],
        "non_claims": {
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
    if plan.get("kind") != "MirBuilderModuleFunctionInsertionPlanV1":
        raise ValueError("wrong module function insertion plan kind")
    if "ModuleFunctionInsertion" not in (plan.get("available_capabilities") or []):
        raise ValueError("module function insertion plan lacks ModuleFunctionInsertion capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "module_transport": "MirModuleMinimalShell",
        "function_transport": "MirFunctionPreparedMain",
        "context": "finalize_module",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"module function insertion profile drift: {key}")
    insertion = plan.get("insertion") or {}
    expected_insertion = {
        "callsite": "module.add_function(function)",
        "inserted_function": "MirFunctionPreparedMain",
        "key_source": "function.signature.name.clone()",
        "container": "MirModule.functions",
        "container_operation": "BTreeMap::insert",
        "collision_policy": "ReplaceExistingByName",
    }
    for key, value in expected_insertion.items():
        if insertion.get(key) != value:
            raise ValueError(f"module function insertion drift: {key}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "MirModule::add_function":
        raise ValueError("module function insertion entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("module function insertion result drift")
    if result.get("mutates") != ["module.functions"]:
        raise ValueError("module function insertion mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"module function insertion non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::module_function_insertion",
        method_universe=("ModuleFunctionInsertion::insert",),
        selected_method_ids=("ModuleFunctionInsertion::insert",),
        denials=(),
        semantic_transports={
            "module_transport": "MirModuleMinimalShell",
            "function_transport": "MirFunctionPreparedMain",
            "container": "MirModule.functions",
            "container_operation": "BTreeMap::insert",
            "hako_operation": "OrderedMapBox.set",
            "collision_policy": "ReplaceExistingByName",
            "condition_fn_injection": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::module_function_insertion",
            api_name="ModuleFunctionInsertionApi",
            pilot_scope="ModuleFunctionInsertion_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.artifact.json",
        ),
        selected_body_count_label="module_function_insertion_minimal_literal_profile_only",
        expected_fields=(
            "function_count",
            "inserted_name",
            "collision_policy_replace",
            "condition_fn_injection",
            "full_finalize_module",
        ),
    )


def module_function_insertion_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderModuleFunctionInsertionDerivedHakoOracleV1":
        raise ValueError("module function insertion oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="ModuleFunctionInsertion::insert",
            rust_operation="MirModule::add_function",
            hako_operation="OrderedMapBox.set + ReturnValue",
            emits="ModuleFunctionInsertionApi.insert(module_state, function_state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-module-function-insertion"
        ),
        generator_version="mirbuilder-module-function-insertion-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="ModuleFunctionInsertionKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="ModuleFunctionInsertionModuleShellBox",
                fields=[
                    FieldSpec(
                        name="functions",
                        field_type="OrderedMapBox",
                        initializer_operation={"kind": "NewOrderedMap"},
                    ),
                    FieldSpec(name="function_count", field_type="i64", initializer="0"),
                    FieldSpec(name="condition_fn_injection", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="ModuleFunctionInsertionFunctionShellBox",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="inserted", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="ModuleFunctionInsertionResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="function_count", field_type="i64", initializer="0"),
                    FieldSpec(name="inserted_name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="collision_policy_replace", field_type="i64", initializer="0"),
                    FieldSpec(name="condition_fn_injection", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="ModuleFunctionInsertionFunctionShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(name): ModuleFunctionInsertionFunctionShellBox",
                        operations=[
                            op("NewBox", target="function_state", box="ModuleFunctionInsertionFunctionShellBox").to_json(),
                            op("SetField", target="function_state", field="name", value="name").to_json(),
                            op("ReturnValue", value="function_state").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="ModuleFunctionInsertionApi",
                methods=[
                    ApiMethodSpec(
                        signature="insert(module_state, function_state): ModuleFunctionInsertionResultBox",
                        operations=[
                            op(
                                "MethodCall",
                                receiver="module_state.functions",
                                method="set",
                                args=["function_state.name", "function_state"],
                            ).to_json(),
                            op("SetField", target="module_state", field="function_count", value=1).to_json(),
                            op("SetField", target="function_state", field="inserted", value=1).to_json(),
                            op("NewBox", target="result", box="ModuleFunctionInsertionResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="function_count", value=1).to_json(),
                            op("SetField", target="result", field="inserted_name", value="function_state.name").to_json(),
                            op("SetField", target="result", field="collision_policy_replace", value=1).to_json(),
                            op("SetField", target="result", field="condition_fn_injection", value=0).to_json(),
                            op("SetField", target="result", field="full_finalize_module", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="module_state", box="ModuleFunctionInsertionModuleShellBox"),
            op("StaticCall", target="function_state", callee="ModuleFunctionInsertionFunctionShellApi.create", args=[{"literal": "main"}]),
            op("StaticCall", target="result", callee="ModuleFunctionInsertionApi.insert", args=["module_state", "function_state"]),
            op("AssertEq", left="result.ok", right=1, fail_message="module_function_insertion_ok=fail", fail_code=1),
            op("AssertEq", left="result.function_count", right=1, fail_message="module_function_insertion_count=fail", fail_code=2),
            op("AssertEq", left="result.inserted_name", right={"literal": "main"}, fail_message="module_function_insertion_name=fail", fail_code=3),
            op("AssertEq", left="function_state.inserted", right=1, fail_message="module_function_insertion_flag=fail", fail_code=4),
            op("AssertEq", left="module_state.functions.length()", right=1, fail_message="module_function_insertion_map=fail", fail_code=5),
            op("AssertEq", left="result.collision_policy_replace", right=1, fail_message="module_function_insertion_collision=fail", fail_code=6),
            op("AssertEq", left="result.condition_fn_injection", right=0, fail_message="module_function_insertion_condition_fn=fail", fail_code=7),
            op("AssertEq", left="result.full_finalize_module", right=0, fail_message="module_function_insertion_full_finalize=fail", fail_code=8),
            op("Print", text="mirbuilder_module_function_insertion_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_module_function_insertion.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.module_function_insertion",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "module_function_insertion": 1,
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
                "module_function_insertion_only": 1,
                "entrypoint": "MirModule::add_function",
                "module_transport": "MirModuleMinimalShell",
                "function_transport": "MirFunctionPreparedMain",
                "context": "finalize_module",
                "container": "MirModule.functions",
                "container_operation": "BTreeMap::insert",
                "hako_operation": "OrderedMapBox.set",
                "collision_policy": "ReplaceExistingByName",
                "mutation_frame": ["module.functions"],
                "condition_fn_injection": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "OrderedMapBox.set",
            "SetField",
            "ReturnValue",
            "ModuleFunctionInsertionShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "module_transport": "MirModuleMinimalShell",
                "function_transport": "MirFunctionPreparedMain",
                "function_key_source": "function.signature.name.clone()",
                "collision_policy": "ReplaceExistingByName",
                "condition_fn_injection": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
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
    spec = module_function_insertion_spec()
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


def run_module_function_insertion_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_module_function_insertion_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_module_function_insertion_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
