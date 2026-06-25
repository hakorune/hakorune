#!/usr/bin/env python3
"""Generate the derived Hako artifact for the MirFunction constructor shell."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

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
SOURCE = ROOT / "src/mir/function/function_impl.rs"
PLAN = FIXTURES / "mir-function-constructor-composition-plan-v0.json"
ORACLE = FIXTURES / "mir-function-constructor-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mir-function-constructor-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mir-function-constructor-derived-hako-verifier-result-v0.json"


def _var(name: str) -> dict[str, str]:
    return {"kind": "Var", "name": name}


def _i64(value: int) -> dict[str, int | str]:
    return {"kind": "I64", "value": value}


def _lt(left: Any, right: Any) -> dict[str, Any]:
    return {"kind": "LtI64", "left": left, "right": right}


def _add(left: Any, right: Any) -> dict[str, Any]:
    return {"kind": "AddI64", "left": left, "right": right}


def _call(callee: str, args: list[Any]) -> dict[str, Any]:
    return {"kind": "CallStatic", "callee": callee, "args": args}


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirFunctionConstructorCompositionPlanV1":
        raise ValueError("wrong MirFunction constructor plan kind")
    capabilities = set(plan.get("available_capabilities") or [])
    if "MirFunctionConstructorTransport" not in capabilities:
        raise ValueError("missing MirFunctionConstructorTransport capability")
    composition = plan.get("composition") or {}
    if composition.get("entry_block_child", {}).get("constructor") != "BasicBlock::new":
        raise ValueError("entry block constructor drift")
    if composition.get("params", {}).get("range") != "[0, param_count)":
        raise ValueError("parameter prepopulation range drift")
    if composition.get("next_value_id", {}).get("seed") != "max(param_count, 1)":
        raise ValueError("next_value_id seed drift")
    defaults = plan.get("basic_block_defaults") or {}
    if defaults.get("effects") != "EffectMask::PURE":
        raise ValueError("BasicBlock default effect drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"MirFunction constructor non-claim must remain 0: {key}")


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirFunctionConstructorDerivedHakoOracleV1",
        "subject": "hakorune_mir::MirFunction::new",
        "vectors": [
            {
                "name": "zero_param_entry_block_zero",
                "inputs": {"name": "main", "param_count": 0, "entry_block": 0},
                "expect": {
                    "param_len": 0,
                    "next_value_id": 1,
                    "entry_block": 0,
                    "block_table_len": 1,
                    "entry_block_instruction_len": 0,
                    "entry_block_terminator": "Option::None",
                    "entry_block_reachable": 0,
                    "entry_block_sealed": 0,
                },
            },
            {
                "name": "three_params_entry_block_seven",
                "inputs": {"name": "with_params", "param_count": 3, "entry_block": 7},
                "expect": {
                    "param_len": 3,
                    "params": [0, 1, 2],
                    "next_value_id": 3,
                    "entry_block": 7,
                    "block_table_len": 1,
                },
            },
        ],
        "non_claims": {
            "separate_block_only_claim": 0,
            "function_body_lowering": 0,
            "instruction_emission": 0,
            "parameter_setup_compatibility_fallback": 0,
            "reserve_parameter_value_ids_call": 0,
            "function_finalization": 0,
            "mainline_selected": 0,
        },
    }


def _contract(plan: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir::MirFunctionConstructorShell",
        method_universe=(
            "FunctionSignaturePrepared::create",
            "BasicBlockConstructorShell::create",
            "FunctionBlockTableShell::create",
            "MirFunctionConstructorShell::create",
        ),
        selected_method_ids=(
            "FunctionSignaturePrepared::create",
            "BasicBlockConstructorShell::create",
            "FunctionBlockTableShell::create",
            "MirFunctionConstructorShell::create",
        ),
        denials=(),
        semantic_transports={
            "function_signature_transport": "FunctionSignaturePrepared",
            "entry_block_transport": "BasicBlockIdAsI64",
            "basic_block_transport": "BasicBlockConstructorShell",
            "block_table_transport": "EntryBlockOnlyFunctionBlockTable",
            "params_transport": "ValueIdAsI64Array",
            "next_value_id_transport": "ValueIdCounterAsI64",
            "function_transport": "MirFunctionConstructorShell",
            "full_mir_function_conversion": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir::MirFunctionConstructorShell",
            api_name="MirFunctionConstructorShellApi",
            pilot_scope="MirFunctionConstructorShell_new_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json",
        ),
        selected_body_count_label="mir_function_constructor_shell_only",
        expected_fields=("signature", "blocks", "entry_block", "locals", "params", "next_value_id", "metadata"),
    )


def mir_function_constructor_shell_contract() -> VerifiedFamilyArtifactContractV1:
    return _contract(read_json(PLAN))


def _constructor_operations() -> list[dict[str, Any]]:
    return [
        op("NewBox", target="func", box="MirFunctionConstructorShellBox").to_json(),
        op("SetField", target="func", field="signature", value="signature").to_json(),
        op("SetField", target="func", field="entry_block", value="entry_block").to_json(),
        op(
            "SetField",
            target="func",
            field="blocks",
            value=_call("FunctionBlockTableShellApi.create", ["entry_block"]),
        ).to_json(),
        op("SetField", target="func", field="locals", value={"kind": "NewArray"}).to_json(),
        op("SetField", target="func", field="params", value={"kind": "NewArray"}).to_json(),
        op("LocalI64", target="param_index", value=_i64(0)).to_json(),
        op(
            "StructuredLoop",
            condition=_lt(_var("param_index"), "signature.param_count"),
            body=[
                op("ArrayPush", target="func.params", value=_var("param_index")).to_json(),
                op("Assign", target="param_index", value=_add(_var("param_index"), _i64(1))).to_json(),
            ],
        ).to_json(),
        op("SetField", target="func", field="next_value_id", value="signature.param_count").to_json(),
        op(
            "IfElse",
            condition=_lt("func.next_value_id", _i64(1)),
            then_body=[op("SetField", target="func", field="next_value_id", value=_i64(1)).to_json()],
        ).to_json(),
        op("SetField", target="func", field="metadata", value={"kind": "NewBoxExpr", "box": "FunctionMetadataDefaultShell"}).to_json(),
        op("ReturnValue", value="func").to_json(),
    ]


def mir_function_constructor_shell_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="FunctionSignaturePrepared::create",
            rust_operation="prepared FunctionSignature input surface",
            hako_operation="NewBox + SetField + ReturnValue",
            emits="FunctionSignaturePreparedApi.create(name, param_count)",
        ),
        BehaviorMethodSpec(
            id="BasicBlockConstructorShell::create",
            rust_operation="BasicBlock::new constructor defaults",
            hako_operation="NewBox + SetField + ReturnValue",
            emits="BasicBlockConstructorShellApi.create(entry_block)",
        ),
        BehaviorMethodSpec(
            id="FunctionBlockTableShell::create",
            rust_operation="HashMap::new plus entry BasicBlock insertion",
            hako_operation="EntryBlockOnlyFunctionBlockTable",
            emits="FunctionBlockTableShellApi.create(entry_block)",
        ),
        BehaviorMethodSpec(
            id="MirFunctionConstructorShell::create",
            rust_operation="MirFunction::new constructor composition",
            hako_operation="StructuredLoop + NewBox + SetField + ReturnValue",
            emits="MirFunctionConstructorShellApi.create(signature, entry_block)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mir-function-constructor-shell",
        generator_version="mir-function-constructor-shell-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="MirFunctionConstructorHarness", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="FunctionSignaturePrepared",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(name="param_count", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="BasicBlockConstructorShellBox",
                fields=[
                    FieldSpec(name="id", field_type="i64", initializer="0"),
                    FieldSpec(name="instructions", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="instruction_spans", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="terminator", field_type="Option<StringBox>", initializer="Option::None()"),
                    FieldSpec(name="terminator_span", field_type="Option<i64>", initializer="Option::None()"),
                    FieldSpec(name="predecessors", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="successors", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="effects", field_type="i64", initializer="0"),
                    FieldSpec(name="reachable", field_type="i64", initializer="0"),
                    FieldSpec(name="sealed", field_type="i64", initializer="0"),
                    FieldSpec(name="return_env", field_type="Option<ArrayBox>", initializer="Option::None()"),
                    FieldSpec(name="return_env_layout", field_type="Option<StringBox>", initializer="Option::None()"),
                ],
            ),
            BoxSpec(
                name="FunctionBlockTableShell",
                fields=[
                    FieldSpec(name="entry_block_id", field_type="i64", initializer="0"),
                    FieldSpec(name="entry_block", field_type="BasicBlockConstructorShellBox", initializer="null"),
                ],
            ),
            BoxSpec(name="FunctionMetadataDefaultShell", fields=[]),
            BoxSpec(
                name="MirFunctionConstructorShellBox",
                fields=[
                    FieldSpec(name="signature", field_type="FunctionSignaturePrepared", initializer="null"),
                    FieldSpec(name="blocks", field_type="FunctionBlockTableShell", initializer="null"),
                    FieldSpec(name="entry_block", field_type="i64", initializer="0"),
                    FieldSpec(name="locals", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="params", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="next_value_id", field_type="i64", initializer="0"),
                    FieldSpec(name="metadata", field_type="FunctionMetadataDefaultShell", initializer="new FunctionMetadataDefaultShell()"),
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
                            op("SetField", target="signature", field="name", value="name").to_json(),
                            op("SetField", target="signature", field="param_count", value="param_count").to_json(),
                            op("ReturnValue", value="signature").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="BasicBlockConstructorShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(id): BasicBlockConstructorShellBox",
                        operations=[
                            op("NewBox", target="block", box="BasicBlockConstructorShellBox").to_json(),
                            op("SetField", target="block", field="id", value="id").to_json(),
                            op("ReturnValue", value="block").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="FunctionBlockTableShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(entry_block): FunctionBlockTableShell",
                        operations=[
                            op("NewBox", target="blocks", box="FunctionBlockTableShell").to_json(),
                            op("SetField", target="blocks", field="entry_block_id", value="entry_block").to_json(),
                            op(
                                "SetField",
                                target="blocks",
                                field="entry_block",
                                value=_call("BasicBlockConstructorShellApi.create", ["entry_block"]),
                            ).to_json(),
                            op("ReturnValue", value="blocks").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="MirFunctionConstructorShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(signature, entry_block): MirFunctionConstructorShellBox",
                        operations=_constructor_operations(),
                    )
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="sig0", callee="FunctionSignaturePreparedApi.create", args=[{"literal": "main"}, 0]),
            op("StaticCall", target="fn0", callee="MirFunctionConstructorShellApi.create", args=["sig0", 0]),
            op("AssertEq", left="fn0.signature.name", right={"literal": "main"}, fail_message="mir_function_shell_name=fail", fail_code=1),
            op("AssertEq", left="fn0.params.length()", right=0, fail_message="mir_function_shell_zero_params=fail", fail_code=2),
            op("AssertEq", left="fn0.next_value_id", right=1, fail_message="mir_function_shell_zero_counter=fail", fail_code=3),
            op("AssertEq", left="fn0.entry_block", right=0, fail_message="mir_function_shell_entry_id=fail", fail_code=4),
            op("AssertEq", left="fn0.blocks.entry_block.id", right=0, fail_message="mir_function_shell_entry_block_id=fail", fail_code=5),
            op("AssertEq", left="fn0.blocks.entry_block.instructions.length()", right=0, fail_message="mir_function_shell_block_instructions=fail", fail_code=6),
            op("AssertEq", left="fn0.blocks.entry_block.terminator", right={"expr": "Option::None()"}, fail_message="mir_function_shell_block_terminator=fail", fail_code=7),
            op("AssertEq", left="fn0.blocks.entry_block.reachable", right=0, fail_message="mir_function_shell_block_reachable=fail", fail_code=8),
            op("AssertEq", left="fn0.blocks.entry_block.sealed", right=0, fail_message="mir_function_shell_block_sealed=fail", fail_code=9),
            op("StaticCall", target="sig3", callee="FunctionSignaturePreparedApi.create", args=[{"literal": "with_params"}, 3]),
            op("StaticCall", target="fn3", callee="MirFunctionConstructorShellApi.create", args=["sig3", 7]),
            op("AssertEq", left="fn3.params.length()", right=3, fail_message="mir_function_shell_param_len=fail", fail_code=10),
            op("AssertArrayValueEq", array="fn3.params", index=0, expected=0, fail_message="mir_function_shell_param0=fail", fail_code=11),
            op("AssertArrayValueEq", array="fn3.params", index=1, expected=1, fail_message="mir_function_shell_param1=fail", fail_code=12),
            op("AssertArrayValueEq", array="fn3.params", index=2, expected=2, fail_message="mir_function_shell_param2=fail", fail_code=13),
            op("AssertEq", left="fn3.next_value_id", right=3, fail_message="mir_function_shell_param_counter=fail", fail_code=14),
            op("AssertEq", left="fn3.entry_block", right=7, fail_message="mir_function_shell_param_entry=fail", fail_code=15),
            op("AssertEq", left="fn3.blocks.entry_block.id", right=7, fail_message="mir_function_shell_param_block_id=fail", fail_code=16),
            op("MethodCall", receiver="fn0.params", method="push", args=[99]),
            op("AssertEq", left="fn0.params.length()", right=1, fail_message="mir_function_shell_param_mutation=fail", fail_code=17),
            op("AssertEq", left="fn3.params.length()", right=3, fail_message="mir_function_shell_param_alias=fail", fail_code=18),
            op("MethodCall", receiver="fn0.blocks.entry_block.instructions", method="push", args=[1]),
            op("AssertEq", left="fn0.blocks.entry_block.instructions.length()", right=1, fail_message="mir_function_shell_instruction_mutation=fail", fail_code=19),
            op("AssertEq", left="fn3.blocks.entry_block.instructions.length()", right=0, fail_message="mir_function_shell_instruction_alias=fail", fail_code=20),
            op("Print", text="mir_function_constructor_shell_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mir_function_constructor_shell.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir::MirFunction::new.constructor_shell",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "mir_function_constructor_shell": 1,
            "basic_block_constructor_shell": 1,
            "prepared_state_install": 0,
            "separate_block_only_claim": 0,
            "function_body_lowering": 0,
            "instruction_emission": 0,
            "parameter_setup_compatibility_fallback": 0,
            "reserve_parameter_value_ids_call": 0,
            "function_finalization": 0,
            "full_mir_function_conversion": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
            "backend_behavior_changed": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
        verifier_checks=contract.verifier_checks(
            {
                "source_plan_input": "verified",
                "constructor_shell_only": 1,
                "basic_block_child_constructor": 1,
                "entry_block_only_table": 1,
                "params_prepopulated": 1,
                "next_value_id_seed_max_param_count_1": 1,
                "fresh_params_identity": 1,
                "fresh_entry_block_instruction_identity": 1,
                "function_body_lowering": 0,
                "instruction_emission": 0,
                "backend_behavior_changed": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "NewBox",
            "SetField",
            "StructuredLoop",
            "ArrayPush",
            "ReturnValue",
            "BasicBlock::new defaults",
        ],
        transport_notes=contract.transport_notes(
            {
                "semantic_capability": "MirFunctionConstructorTransport",
                "function_signature_surface": "name + param_count only",
                "block_table_observation": "entry-block-only",
                "basic_block_observation": "constructor defaults only",
            }
        ),
        denied_boundaries=list(plan["non_claims"].keys()),
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle_text = stable_json(build_oracle())
    spec = mir_function_constructor_shell_spec()
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


def run_mir_function_constructor_shell_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_mir_function_constructor_shell_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_mir_function_constructor_shell_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
