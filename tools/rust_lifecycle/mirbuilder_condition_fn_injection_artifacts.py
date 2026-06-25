#!/usr/bin/env python3
"""Generate the bounded condition_fn injection Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-condition-fn-injection-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-condition-fn-injection-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-condition-fn-injection-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-condition-fn-injection-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderConditionFnInjectionDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module condition_fn injection",
        "vectors": [
            {
                "name": "prepared_module_injects_missing_condition_fn_stub",
                "inputs": {
                    "module_transport": "MirModuleMinimalShell",
                    "condition_fn_initially_missing": True,
                    "initial_function_count": 1,
                },
                "expect": {
                    "ok": 1,
                    "injected": 1,
                    "function_count": 2,
                    "function_name": "condition_fn",
                    "param_count": 1,
                    "return_type": "MirType::Integer",
                    "effects": "EffectMask::PURE",
                    "entry_block": 0,
                    "const_integer_value": 1,
                    "returns_value": 1,
                    "region_stack_pop": 0,
                    "full_finalize_module": 0,
                },
            },
            {
                "name": "existing_condition_fn_is_not_duplicated",
                "inputs": {
                    "module_transport": "MirModuleMinimalShell",
                    "condition_fn_initially_missing": False,
                    "initial_function_count": 2,
                },
                "expect": {
                    "ok": 1,
                    "injected": 0,
                    "function_count": 2,
                    "collision_policy": "NoDuplicateWhenPresent",
                },
            },
        ],
        "non_claims": {
            "condition_fn_policy_generalization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderConditionFnInjectionPlanV1":
        raise ValueError("wrong condition_fn injection plan kind")
    if "ConditionFnInjection" not in (plan.get("available_capabilities") or []):
        raise ValueError("condition_fn injection plan lacks ConditionFnInjection capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "module_transport": "MirModuleMinimalShell",
        "condition_fn_initially_missing": True,
        "context": "finalize_module",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"condition_fn injection profile drift: {key}")
    injection = plan.get("injection") or {}
    expected_injection = {
        "predicate": 'module.functions.get("condition_fn").is_none()',
        "function_name": "condition_fn",
        "return_type": "MirType::Integer",
        "effects": "EffectMask::PURE",
        "entry_block": "BasicBlockId(0)",
        "insert_operation": "module.add_function(f)",
        "required_by_source": True,
    }
    for key, value in expected_injection.items():
        if injection.get(key) != value:
            raise ValueError(f"condition_fn injection drift: {key}")
    if injection.get("params") != ["MirType::Integer"]:
        raise ValueError("condition_fn param drift")
    if injection.get("body") != ["ConstInteger(1)", "ReturnValue(one)"]:
        raise ValueError("condition_fn body drift")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "MirBuilder::finalize_module condition_fn injection block":
        raise ValueError("condition_fn entrypoint drift")
    if result.get("minimal_path_expected_result") != "NoErrorReturn":
        raise ValueError("condition_fn result drift")
    if result.get("mutates") != ["module.functions"]:
        raise ValueError("condition_fn mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"condition_fn non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::condition_fn_injection",
        method_universe=("ConditionFnInjection::inject_if_missing",),
        selected_method_ids=("ConditionFnInjection::inject_if_missing",),
        denials=(),
        semantic_transports={
            "module_transport": "MirModuleMinimalShell",
            "condition_fn_initially_missing": True,
            "function_name": "condition_fn",
            "param_count": 1,
            "return_type": "MirType::Integer",
            "effects": "EffectMask::PURE",
            "entry_block": 0,
            "const_integer_value": 1,
            "returns_value": 1,
            "hako_operation": "OrderedMapBox.has + OrderedMapBox.set",
            "region_stack_pop": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::condition_fn_injection",
            api_name="ConditionFnInjectionApi",
            pilot_scope="ConditionFnInjection_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.artifact.json",
        ),
        selected_body_count_label="condition_fn_injection_minimal_literal_profile_only",
        expected_fields=(
            "injected",
            "function_count",
            "function_name",
            "param_count",
            "return_type_integer",
            "effects_pure",
            "entry_block",
            "const_integer_value",
            "returns_value",
            "region_stack_pop",
            "full_finalize_module",
        ),
    )


def condition_fn_injection_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderConditionFnInjectionDerivedHakoOracleV1":
        raise ValueError("condition_fn oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="ConditionFnInjection::inject_if_missing",
            rust_operation="MirBuilder::finalize_module condition_fn injection block",
            hako_operation="OrderedMapBox.has + OrderedMapBox.set + ReturnValue",
            emits="ConditionFnInjectionApi.inject_if_missing(module_state)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-condition-fn-injection"
        ),
        generator_version="mirbuilder-condition-fn-injection-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="ConditionFnInjectionKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="ConditionFnInjectionModuleShellBox",
                fields=[
                    FieldSpec(
                        name="functions",
                        field_type="OrderedMapBox",
                        initializer_operation={"kind": "NewOrderedMap"},
                    ),
                    FieldSpec(name="function_count", field_type="i64", initializer="0"),
                    FieldSpec(name="region_stack_pop", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="ConditionFnInjectionFunctionShellBox",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="param_count", field_type="i64", initializer="0"),
                    FieldSpec(name="return_type_integer", field_type="i64", initializer="0"),
                    FieldSpec(name="effects_pure", field_type="i64", initializer="0"),
                    FieldSpec(name="entry_block", field_type="i64", initializer="0"),
                    FieldSpec(name="const_integer_value", field_type="i64", initializer="0"),
                    FieldSpec(name="returns_value", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="ConditionFnInjectionResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="injected", field_type="i64", initializer="0"),
                    FieldSpec(name="function_count", field_type="i64", initializer="0"),
                    FieldSpec(name="function_name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="param_count", field_type="i64", initializer="0"),
                    FieldSpec(name="return_type_integer", field_type="i64", initializer="0"),
                    FieldSpec(name="effects_pure", field_type="i64", initializer="0"),
                    FieldSpec(name="entry_block", field_type="i64", initializer="0"),
                    FieldSpec(name="const_integer_value", field_type="i64", initializer="0"),
                    FieldSpec(name="returns_value", field_type="i64", initializer="0"),
                    FieldSpec(name="region_stack_pop", field_type="i64", initializer="0"),
                    FieldSpec(name="full_finalize_module", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="ConditionFnInjectionFunctionShellApi",
                methods=[
                    ApiMethodSpec(
                        signature=(
                            "create(name, param_count, return_type_integer, effects_pure, "
                            "entry_block, const_integer_value, returns_value): ConditionFnInjectionFunctionShellBox"
                        ),
                        operations=[
                            op("NewBox", target="function_state", box="ConditionFnInjectionFunctionShellBox").to_json(),
                            op("SetField", target="function_state", field="name", value="name").to_json(),
                            op("SetField", target="function_state", field="param_count", value="param_count").to_json(),
                            op(
                                "SetField",
                                target="function_state",
                                field="return_type_integer",
                                value="return_type_integer",
                            ).to_json(),
                            op("SetField", target="function_state", field="effects_pure", value="effects_pure").to_json(),
                            op("SetField", target="function_state", field="entry_block", value="entry_block").to_json(),
                            op(
                                "SetField",
                                target="function_state",
                                field="const_integer_value",
                                value="const_integer_value",
                            ).to_json(),
                            op("SetField", target="function_state", field="returns_value", value="returns_value").to_json(),
                            op("ReturnValue", value="function_state").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="ConditionFnInjectionStubApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(): ConditionFnInjectionFunctionShellBox",
                        operations=[
                            op(
                                "StaticCall",
                                target="stub",
                                callee="ConditionFnInjectionFunctionShellApi.create",
                                args=[{"literal": "condition_fn"}, 1, 1, 1, 0, 1, 1],
                            ).to_json(),
                            op("ReturnValue", value="stub").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="ConditionFnInjectionApi",
                methods=[
                    ApiMethodSpec(
                        signature="inject_if_missing(module_state): ConditionFnInjectionResultBox",
                        operations=[
                            op("StaticCall", target="stub", callee="ConditionFnInjectionStubApi.create", args=[]).to_json(),
                            op(
                                "MethodCall",
                                target="has_condition_fn",
                                receiver="module_state.functions",
                                method="has",
                                args=["stub.name"],
                            ).to_json(),
                            op("LocalI64", target="injected", value=0).to_json(),
                            op(
                                "IfElse",
                                condition={"kind": "EqI64", "left": "has_condition_fn", "right": 0},
                                then_body=[
                                    op(
                                        "MethodCall",
                                        receiver="module_state.functions",
                                        method="set",
                                        args=["stub.name", "stub"],
                                    ).to_json(),
                                    op("SetField", target="module_state", field="function_count", value="module_state.functions.length()").to_json(),
                                    op("Assign", target="injected", value=1).to_json(),
                                ],
                            ).to_json(),
                            op("NewBox", target="result", box="ConditionFnInjectionResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="injected", value="injected").to_json(),
                            op("SetField", target="result", field="function_count", value="module_state.functions.length()").to_json(),
                            op("SetField", target="result", field="function_name", value="stub.name").to_json(),
                            op("SetField", target="result", field="param_count", value="stub.param_count").to_json(),
                            op(
                                "SetField",
                                target="result",
                                field="return_type_integer",
                                value="stub.return_type_integer",
                            ).to_json(),
                            op("SetField", target="result", field="effects_pure", value="stub.effects_pure").to_json(),
                            op("SetField", target="result", field="entry_block", value="stub.entry_block").to_json(),
                            op(
                                "SetField",
                                target="result",
                                field="const_integer_value",
                                value="stub.const_integer_value",
                            ).to_json(),
                            op("SetField", target="result", field="returns_value", value="stub.returns_value").to_json(),
                            op("SetField", target="result", field="region_stack_pop", value=0).to_json(),
                            op("SetField", target="result", field="full_finalize_module", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            ),
        ],
        main_operations=[
            op("NewBox", target="module_state", box="ConditionFnInjectionModuleShellBox"),
            op(
                "StaticCall",
                target="main_func",
                callee="ConditionFnInjectionFunctionShellApi.create",
                args=[{"literal": "main"}, 0, 1, 1, 0, 0, 1],
            ),
            op(
                "MethodCall",
                receiver="module_state.functions",
                method="set",
                args=["main_func.name", "main_func"],
            ),
            op("SetField", target="module_state", field="function_count", value="module_state.functions.length()"),
            op("StaticCall", target="result", callee="ConditionFnInjectionApi.inject_if_missing", args=["module_state"]),
            op("AssertEq", left="result.ok", right=1, fail_message="condition_fn_ok=fail", fail_code=1),
            op("AssertEq", left="result.injected", right=1, fail_message="condition_fn_injected=fail", fail_code=2),
            op("AssertEq", left="result.function_count", right=2, fail_message="condition_fn_count=fail", fail_code=3),
            op("AssertEq", left="result.function_name", right={"literal": "condition_fn"}, fail_message="condition_fn_name=fail", fail_code=4),
            op("AssertEq", left="result.param_count", right=1, fail_message="condition_fn_param=fail", fail_code=5),
            op("AssertEq", left="result.return_type_integer", right=1, fail_message="condition_fn_return_type=fail", fail_code=6),
            op("AssertEq", left="result.effects_pure", right=1, fail_message="condition_fn_effect=fail", fail_code=7),
            op("AssertEq", left="result.entry_block", right=0, fail_message="condition_fn_entry=fail", fail_code=8),
            op("AssertEq", left="result.const_integer_value", right=1, fail_message="condition_fn_const=fail", fail_code=9),
            op("AssertEq", left="result.returns_value", right=1, fail_message="condition_fn_return=fail", fail_code=10),
            op("AssertEq", left="result.region_stack_pop", right=0, fail_message="condition_fn_region=fail", fail_code=11),
            op("AssertEq", left="result.full_finalize_module", right=0, fail_message="condition_fn_finalize=fail", fail_code=12),
            op("StaticCall", target="second", callee="ConditionFnInjectionApi.inject_if_missing", args=["module_state"]),
            op("AssertEq", left="second.injected", right=0, fail_message="condition_fn_duplicate=fail", fail_code=13),
            op("AssertEq", left="second.function_count", right=2, fail_message="condition_fn_duplicate_count=fail", fail_code=14),
            op("Print", text="mirbuilder_condition_fn_injection_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_condition_fn_injection.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.condition_fn_injection",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "condition_fn_injection": 1,
            "condition_fn_policy_generalization": 0,
            "region_stack_pop": 0,
            "slot_registry_release": 0,
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
                "condition_fn_injection_only": 1,
                "entrypoint": "MirBuilder::finalize_module condition_fn injection block",
                "module_transport": "MirModuleMinimalShell",
                "context": "finalize_module",
                "predicate": 'module.functions.get("condition_fn").is_none()',
                "function_name": "condition_fn",
                "param_count": 1,
                "return_type": "MirType::Integer",
                "effects": "EffectMask::PURE",
                "entry_block": 0,
                "body": ["ConstInteger(1)", "ReturnValue(one)"],
                "hako_operation": "OrderedMapBox.has + OrderedMapBox.set",
                "mutation_frame": ["module.functions"],
                "region_stack_pop": 0,
                "slot_registry_release": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "OrderedMapBox.has",
            "OrderedMapBox.set",
            "SetField",
            "ReturnValue",
            "ConditionFnStubShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "module_transport": "MirModuleMinimalShell",
                "condition_fn_initially_missing": True,
                "function_name": "condition_fn",
                "condition_fn_stub_body": "ConstInteger(1) -> ReturnValue(one)",
                "condition_fn_policy_generalization": 0,
                "region_stack_pop": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "condition_fn_policy_generalization",
            "region_stack_pop",
            "slot_registry_release",
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
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = condition_fn_injection_spec()
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


def run_condition_fn_injection_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_condition_fn_injection_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_condition_fn_injection_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
