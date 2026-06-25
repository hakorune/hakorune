#!/usr/bin/env python3
"""Generate the bounded CurrentFunctionTake Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-current-function-take-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-current-function-take-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-current-function-take-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-current-function-take-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderCurrentFunctionTakeDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module scope_ctx.current_function.take().unwrap()",
        "vectors": [
            {
                "name": "present_function_is_taken_and_state_cleared",
                "inputs": {
                    "initial_current_function_present": 1,
                    "function_name": "main",
                    "entry_block": 0,
                    "next_value_id": 1,
                },
                "expect": {
                    "current_function_present": 0,
                    "taken_function_present": 1,
                    "function_name_is_main": 1,
                    "entry_block": 0,
                    "next_value_id": 1,
                },
            }
        ],
        "non_claims": {
            "type_propagation": 0,
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
    if plan.get("kind") != "MirBuilderCurrentFunctionTakePlanV1":
        raise ValueError("wrong current function take plan kind")
    if "CurrentFunctionTake" not in (plan.get("available_capabilities") or []):
        raise ValueError("current function take plan lacks CurrentFunctionTake capability")
    profile = plan.get("execution_profile") or {}
    if profile.get("function_transport") != "MirFunctionPreparedMain":
        raise ValueError("current function take function transport drift")
    result = plan.get("result_contract") or {}
    expected = {
        "taken_value": "MirFunctionPreparedMain",
        "source_state": "self.scope_ctx.current_function",
        "post_take_state": "None",
        "local_binding": "function",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"current function take result contract drift: {key}")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"current function take non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::current_function_take",
        method_universe=("CurrentFunctionTake::take",),
        selected_method_ids=("CurrentFunctionTake::take",),
        denials=(),
        semantic_transports={
            "state_transport": "PreparedCurrentFunctionPresenceShell",
            "function_transport": "MirFunctionPreparedMain",
            "post_take_state": "None",
            "type_propagation": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::current_function_take",
            api_name="CurrentFunctionTakeApi",
            pilot_scope="CurrentFunctionTake_present_function_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json",
        ),
        selected_body_count_label="current_function_take_present_function_only",
        expected_fields=(
            "current_function_present",
            "taken_function_present",
            "signature",
            "entry_block",
            "next_value_id",
        ),
    )


def current_function_take_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderCurrentFunctionTakeDerivedHakoOracleV1":
        raise ValueError("current function take oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="CurrentFunctionTake::take",
            rust_operation="MirBuilder::finalize_module scope_ctx.current_function.take().unwrap()",
            hako_operation="ClearPresence + MarkTaken + ReturnValue",
            emits="CurrentFunctionTakeApi.take(scope)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-current-function-take"
        ),
        generator_version="mirbuilder-current-function-take-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="CurrentFunctionTakeKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="FunctionSignaturePrepared",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="param_count", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="FunctionMetadataDefaultShell",
                fields=[],
            ),
            BoxSpec(
                name="MirFunctionPreparedMainBox",
                fields=[
                    FieldSpec(name="signature", field_type="FunctionSignaturePrepared", initializer="null"),
                    FieldSpec(name="entry_block", field_type="i64", initializer="0"),
                    FieldSpec(name="params", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="next_value_id", field_type="i64", initializer="1"),
                    FieldSpec(name="metadata", field_type="FunctionMetadataDefaultShell", initializer="new FunctionMetadataDefaultShell()"),
                ],
            ),
            BoxSpec(
                name="PreparedCurrentFunctionStateShellBox",
                fields=[
                    FieldSpec(name="current_function", field_type="MirFunctionPreparedMainBox", initializer="null"),
                    FieldSpec(name="current_function_present", field_type="i64", initializer="1"),
                    FieldSpec(name="taken_function_present", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="FunctionSignaturePreparedApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(name, param_count): FunctionSignaturePrepared",
                        operations=[
                            op("NewBox", target="signature", box="FunctionSignaturePrepared").to_json(),
                            op("SetField", target="signature", field="name", value={"kind": "Var", "name": "name"}).to_json(),
                            op("SetField", target="signature", field="param_count", value={"kind": "Var", "name": "param_count"}).to_json(),
                            op("ReturnValue", value="signature").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="MirFunctionPreparedMainApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(signature, entry_block): MirFunctionPreparedMainBox",
                        operations=[
                            op("NewBox", target="func", box="MirFunctionPreparedMainBox").to_json(),
                            op("SetField", target="func", field="signature", value={"kind": "Var", "name": "signature"}).to_json(),
                            op("SetField", target="func", field="entry_block", value={"kind": "Var", "name": "entry_block"}).to_json(),
                            op("SetField", target="func", field="params", value={"kind": "NewBoxExpr", "box": "ArrayBox"}).to_json(),
                            op("SetField", target="func", field="next_value_id", value=1).to_json(),
                            op("SetField", target="func", field="metadata", value={"kind": "NewBoxExpr", "box": "FunctionMetadataDefaultShell"}).to_json(),
                            op("ReturnValue", value="func").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="PreparedCurrentFunctionStateShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(func): PreparedCurrentFunctionStateShellBox",
                        operations=[
                            op("NewBox", target="state", box="PreparedCurrentFunctionStateShellBox").to_json(),
                            op("SetField", target="state", field="current_function", value={"kind": "Var", "name": "func"}).to_json(),
                            op("SetField", target="state", field="current_function_present", value=1).to_json(),
                            op("SetField", target="state", field="taken_function_present", value=0).to_json(),
                            op("ReturnValue", value="state").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="CurrentFunctionTakeApi",
                methods=[
                    ApiMethodSpec(
                        signature="take(state): MirFunctionPreparedMainBox",
                        operations=[
                            op("SetField", target="state", field="current_function_present", value=0).to_json(),
                            op("SetField", target="state", field="taken_function_present", value=1).to_json(),
                            op("ReturnValue", value="state.current_function").to_json(),
                        ],
                    )
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="signature", callee="FunctionSignaturePreparedApi.create", args=[{"literal": "main"}, 0]),
            op("StaticCall", target="main_func", callee="MirFunctionPreparedMainApi.create", args=["signature", 0]),
            op("StaticCall", target="state", callee="PreparedCurrentFunctionStateShellApi.create", args=["main_func"]),
            op("StaticCall", target="taken", callee="CurrentFunctionTakeApi.take", args=["state"]),
            op("AssertEq", left="state.current_function_present", right=0, fail_message="current_function_present=fail", fail_code=1),
            op("AssertEq", left="state.taken_function_present", right=1, fail_message="taken_function_present=fail", fail_code=2),
            op("AssertEq", left="taken.signature.name", right={"literal": "main"}, fail_message="function_name=fail", fail_code=3),
            op("AssertEq", left="taken.entry_block", right=0, fail_message="function_entry=fail", fail_code=4),
            op("AssertEq", left="taken.next_value_id", right=1, fail_message="function_next_value=fail", fail_code=5),
            op("AssertEq", left="taken.params.length()", right=0, fail_message="function_params=fail", fail_code=6),
            op("Print", text="mirbuilder_current_function_take_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_current_function_take.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.current_function_take",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "current_function_take": 1,
            "type_propagation": 0,
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
                "current_function_take_only": 1,
                "function_transport": "MirFunctionPreparedMain",
                "post_take_state": "None",
                "taken_function_present": 1,
                "type_propagation": 0,
                "type_hint_provision": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "MirFunctionPreparedMain",
            "PreparedCurrentFunctionPresenceShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "type_propagation": "unselected",
                "type_hint_provision": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "type_propagation",
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
    spec = current_function_take_spec()
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


def run_current_function_take_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_current_function_take_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_current_function_take_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
