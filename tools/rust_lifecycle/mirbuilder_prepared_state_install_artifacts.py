#!/usr/bin/env python3
"""Generate the derived Hako artifact for prepared MirBuilder state install."""

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
SOURCE = ROOT / "src/mir/builder/module_lifecycle.rs"
PLAN = FIXTURES / "mirbuilder-prepared-state-install-plan-v0.json"
CURRENT_MODULE_TAKE = FIXTURES / "mirbuilder-current-module-take-plan-v0.json"
CURRENT_FUNCTION_TAKE = FIXTURES / "mirbuilder-current-function-take-plan-v0.json"
MODULE_MANIFEST = OUT_DIR / "mir_module_minimal_shell.artifact.json"
FUNCTION_MANIFEST = OUT_DIR / "mir_function_constructor_shell.artifact.json"
ORACLE = FIXTURES / "mirbuilder-prepared-state-install-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-prepared-state-install-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-prepared-state-install-derived-hako-verifier-result-v0.json"


def _validate_source_markers() -> None:
    text = SOURCE.read_text()
    markers = [
        "let mut module = MirModule::new(\"main\".to_string());",
        "let entry_block = self.next_block_id();",
        "let mut main_function = self.new_function_with_metadata(main_signature, entry_block);",
        "main_function.metadata.is_entry_point = true;",
        "self.current_module = Some(module);",
        "self.scope_ctx.current_function = Some(main_function);",
        "self.current_block = Some(entry_block);",
    ]
    last = -1
    for marker in markers:
        index = text.find(marker)
        if index < 0:
            raise ValueError(f"missing prepare_module marker: {marker}")
        if index <= last:
            raise ValueError(f"prepare_module marker order drift: {marker}")
        last = index


def _validate_dependencies() -> None:
    current_module = read_json(CURRENT_MODULE_TAKE)
    if current_module.get("kind") != "MirBuilderCurrentModuleTakePlanV1":
        raise ValueError("current-module take plan kind drift")
    module_contract = current_module.get("result_contract") or {}
    if module_contract.get("taken_value") != "MirModuleMinimalShell":
        raise ValueError("current-module take value drift")
    if current_module.get("non_claims", {}).get("generated_hako_artifact") != 0:
        raise ValueError("current-module take must remain PlanOnly")

    current_function = read_json(CURRENT_FUNCTION_TAKE)
    if current_function.get("kind") != "MirBuilderCurrentFunctionTakePlanV1":
        raise ValueError("current-function take plan kind drift")
    function_contract = current_function.get("result_contract") or {}
    if function_contract.get("taken_value") != "MirFunctionPreparedMain":
        raise ValueError("current-function take value drift")
    if current_function.get("non_claims", {}).get("generated_hako_artifact") != 0:
        raise ValueError("current-function take must remain PlanOnly")

    module_manifest = read_json(MODULE_MANIFEST)
    if module_manifest.get("family_id") != "hakorune_mir::MirModuleMinimalShell":
        raise ValueError("MirModule shell artifact dependency drift")
    if module_manifest.get("state") != "DerivedShadow":
        raise ValueError("MirModule shell dependency must remain DerivedShadow")

    function_manifest = read_json(FUNCTION_MANIFEST)
    if function_manifest.get("family_id") != "hakorune_mir::MirFunctionConstructorShell":
        raise ValueError("MirFunction constructor artifact dependency drift")
    if function_manifest.get("state") != "DerivedShadow":
        raise ValueError("MirFunction constructor dependency must remain DerivedShadow")


def build_plan() -> dict[str, Any]:
    _validate_source_markers()
    _validate_dependencies()
    return {
        "schema_version": 0,
        "kind": "MirBuilderPreparedStateInstallPlanV1",
        "subject": "MirBuilder::prepare_module prepared current state install",
        "source_authority": {
            "prepare": "src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module",
            "module_shell_artifact": "mir_module_minimal_shell.artifact.json",
            "function_constructor_artifact": "mir_function_constructor_shell.artifact.json",
            "current_module_take_plan": CURRENT_MODULE_TAKE.name,
            "current_function_take_plan": CURRENT_FUNCTION_TAKE.name,
        },
        "inputs": {
            "module": "MirModuleMinimalShell",
            "function": "MirFunctionConstructorShell",
            "entry_block": "BasicBlockIdAsI64",
        },
        "install_sequence": [
            {
                "step": "install_current_module",
                "source": "prepare_module",
                "operation": "self.current_module = Some(module)",
                "transport": "PresenceTaggedMirModuleHandle",
            },
            {
                "step": "install_current_function",
                "source": "prepare_module",
                "operation": "self.scope_ctx.current_function = Some(main_function)",
                "transport": "PresenceTaggedMirFunctionHandle",
            },
            {
                "step": "install_current_block",
                "source": "prepare_module",
                "operation": "self.current_block = Some(entry_block)",
                "transport": "PresenceTaggedBasicBlockIdAsI64",
            },
        ],
        "available_capabilities": ["PreparedStateInstall"],
        "result_contract": {
            "state_transport": "PreparedMirBuilderStateShell",
            "current_module": "Present",
            "current_function": "Present",
            "current_block": "Present",
            "module_transport": "MirModuleMinimalShell",
            "function_transport": "MirFunctionConstructorShell",
            "block_transport": "BasicBlockIdAsI64",
        },
        "non_claims": {
            "current_module_take": 0,
            "current_function_take": 0,
            "lower_root": 0,
            "literal_integer_lowering": 0,
            "return_emission": 0,
            "finalize_module": 0,
            "full_mirbuilder_object_transport": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderPreparedStateInstallDerivedHakoOracleV1",
        "subject": "MirBuilder::prepare_module prepared state install",
        "vectors": [
            {
                "name": "install_main_module_function_and_block",
                "inputs": {"module_name": "main", "function_name": "main", "entry_block": 0},
                "expect": {
                    "state_initial_current_module_present": 0,
                    "state_initial_current_function_present": 0,
                    "state_initial_current_block_present": 0,
                    "state_after_install_current_module_present": 1,
                    "state_after_install_current_function_present": 1,
                    "state_after_install_current_block_present": 1,
                },
            }
        ],
        "non_claims": {
            "current_module_take": 0,
            "current_function_take": 0,
            "lower_root": 0,
            "finalize_module": 0,
            "mainline_selected": 0,
        },
    }


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderPreparedStateInstallPlanV1":
        raise ValueError("wrong prepared-state install plan kind")
    capabilities = set(plan.get("available_capabilities") or [])
    if "PreparedStateInstall" not in capabilities:
        raise ValueError("missing PreparedStateInstall capability")
    result = plan.get("result_contract") or {}
    expected = {
        "state_transport": "PreparedMirBuilderStateShell",
        "current_module": "Present",
        "current_function": "Present",
        "current_block": "Present",
        "module_transport": "MirModuleMinimalShell",
        "function_transport": "MirFunctionConstructorShell",
        "block_transport": "BasicBlockIdAsI64",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"prepared-state result contract drift: {key}")
    operations = [row.get("operation") for row in plan.get("install_sequence") or []]
    required_operations = [
        "self.current_module = Some(module)",
        "self.scope_ctx.current_function = Some(main_function)",
        "self.current_block = Some(entry_block)",
    ]
    if operations != required_operations:
        raise ValueError("prepared-state install operation order drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"prepared-state non-claim must remain 0: {key}")


def _contract(plan: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::prepared_state_install",
        method_universe=(
            "PreparedScopeContextShell::new",
            "PreparedMirBuilderStateShell::empty",
            "PreparedMirBuilderStateShell::install",
        ),
        selected_method_ids=(
            "PreparedScopeContextShell::new",
            "PreparedMirBuilderStateShell::empty",
            "PreparedMirBuilderStateShell::install",
        ),
        denials=(),
        semantic_transports={
            "state_transport": "PreparedMirBuilderStateShell",
            "current_module_transport": "PresenceTaggedMirModuleHandle",
            "current_function_transport": "PresenceTaggedMirFunctionHandle",
            "current_block_transport": "PresenceTaggedBasicBlockIdAsI64",
            "current_module_take": 0,
            "current_function_take": 0,
            "full_mirbuilder_object_transport": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::prepared_state_install",
            api_name="PreparedMirBuilderStateShellApi",
            pilot_scope="PreparedStateInstall_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json",
        ),
        selected_body_count_label="prepared_state_install_only",
        expected_fields=("current_module", "scope_ctx", "current_block"),
    )


def prepared_state_install_contract() -> VerifiedFamilyArtifactContractV1:
    return _contract(read_json(PLAN))


def _mir_module_boxes() -> list[BoxSpec]:
    return [
        BoxSpec(
            name="ModuleMetadataDefaultShell",
            fields=[FieldSpec(name="source_file", field_type="Option<StringBox>", initializer="Option::None()")],
        ),
        BoxSpec(
            name="MirModuleMinimalShellBox",
            fields=[
                FieldSpec(name="name", field_type="StringBox", initializer="null"),
                FieldSpec(name="functions", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="globals", field_type="OrderedMapBox", initializer_operation={"kind": "NewOrderedMap"}),
                FieldSpec(name="metadata", field_type="ModuleMetadataDefaultShell", initializer="new ModuleMetadataDefaultShell()"),
            ],
        ),
    ]


def _mir_function_boxes() -> list[BoxSpec]:
    return [
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
    ]


def _state_boxes() -> list[BoxSpec]:
    return [
        BoxSpec(
            name="PreparedScopeContextShellBox",
            fields=[
                FieldSpec(name="current_function", field_type="MirFunctionConstructorShellBox", initializer="null"),
                FieldSpec(name="current_function_present", field_type="i64", initializer="0"),
            ],
        ),
        BoxSpec(
            name="PreparedMirBuilderStateShellBox",
            fields=[
                FieldSpec(name="current_module", field_type="MirModuleMinimalShellBox", initializer="null"),
                FieldSpec(name="current_module_present", field_type="i64", initializer="0"),
                FieldSpec(name="scope_ctx", field_type="PreparedScopeContextShellBox", initializer="new PreparedScopeContextShellBox()"),
                FieldSpec(name="current_block", field_type="i64", initializer="0"),
                FieldSpec(name="current_block_present", field_type="i64", initializer="0"),
            ],
        ),
    ]


def _function_constructor_operations() -> list[dict[str, Any]]:
    return [
        op("NewBox", target="func", box="MirFunctionConstructorShellBox").to_json(),
        op("SetField", target="func", field="signature", value="signature").to_json(),
        op("SetField", target="func", field="entry_block", value="entry_block").to_json(),
        op("SetField", target="func", field="blocks", value={"kind": "CallStatic", "callee": "FunctionBlockTableShellApi.create", "args": ["entry_block"]}).to_json(),
        op("SetField", target="func", field="locals", value={"kind": "NewArray"}).to_json(),
        op("SetField", target="func", field="params", value={"kind": "NewArray"}).to_json(),
        op("LocalI64", target="param_index", value={"kind": "I64", "value": 0}).to_json(),
        op(
            "StructuredLoop",
            condition={"kind": "LtI64", "left": {"kind": "Var", "name": "param_index"}, "right": "signature.param_count"},
            body=[
                op("ArrayPush", target="func.params", value={"kind": "Var", "name": "param_index"}).to_json(),
                op(
                    "Assign",
                    target="param_index",
                    value={"kind": "AddI64", "left": {"kind": "Var", "name": "param_index"}, "right": {"kind": "I64", "value": 1}},
                ).to_json(),
            ],
        ).to_json(),
        op("SetField", target="func", field="next_value_id", value="signature.param_count").to_json(),
        op(
            "IfElse",
            condition={"kind": "LtI64", "left": "func.next_value_id", "right": {"kind": "I64", "value": 1}},
            then_body=[op("SetField", target="func", field="next_value_id", value={"kind": "I64", "value": 1}).to_json()],
        ).to_json(),
        op("SetField", target="func", field="metadata", value={"kind": "NewBoxExpr", "box": "FunctionMetadataDefaultShell"}).to_json(),
        op("ReturnValue", value="func").to_json(),
    ]


def prepared_state_install_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="PreparedScopeContextShell::new",
            rust_operation="ScopeContext prepared current_function state shell",
            hako_operation="NewBox + ReturnValue",
            emits="PreparedScopeContextShellApi.create()",
        ),
        BehaviorMethodSpec(
            id="PreparedMirBuilderStateShell::empty",
            rust_operation="prepared MirBuilder state before prepare_module install",
            hako_operation="NewBox + SetField + ReturnValue",
            emits="PreparedMirBuilderStateShellApi.empty()",
        ),
        BehaviorMethodSpec(
            id="PreparedMirBuilderStateShell::install",
            rust_operation="prepare_module current_module/current_function/current_block install",
            hako_operation="SetSome + ReturnValue",
            emits="PreparedMirBuilderStateShellApi.install(state, module, function, entry_block)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-prepared-state-install",
        generator_version="mirbuilder-prepared-state-install-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="PreparedStateInstallHarness", fields=[]),
        additional_boxes=[*_mir_module_boxes(), *_mir_function_boxes(), *_state_boxes()],
        static_boxes=[
            StaticBoxSpec(
                name="MirModuleMinimalShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(name): MirModuleMinimalShellBox",
                        operations=[
                            op("NewBox", target="module", box="MirModuleMinimalShellBox").to_json(),
                            op("SetField", target="module", field="name", value="name").to_json(),
                            op("ReturnValue", value="module").to_json(),
                        ],
                    )
                ],
            ),
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
                                value={"kind": "CallStatic", "callee": "BasicBlockConstructorShellApi.create", "args": ["entry_block"]},
                            ).to_json(),
                            op("ReturnValue", value="blocks").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="MirFunctionConstructorShellApi",
                methods=[ApiMethodSpec(signature="create(signature, entry_block): MirFunctionConstructorShellBox", operations=_function_constructor_operations())],
            ),
            StaticBoxSpec(
                name="PreparedScopeContextShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="create(): PreparedScopeContextShellBox",
                        operations=[
                            op("NewBox", target="scope", box="PreparedScopeContextShellBox").to_json(),
                            op("ReturnValue", value="scope").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="PreparedMirBuilderStateShellApi",
                methods=[
                    ApiMethodSpec(
                        signature="empty(): PreparedMirBuilderStateShellBox",
                        operations=[
                            op("NewBox", target="state", box="PreparedMirBuilderStateShellBox").to_json(),
                            op("SetField", target="state", field="scope_ctx", value={"kind": "CallStatic", "callee": "PreparedScopeContextShellApi.create", "args": []}).to_json(),
                            op("ReturnValue", value="state").to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="install(state, module, func, entry_block): PreparedMirBuilderStateShellBox",
                        operations=[
                            op("SetField", target="state", field="current_module", value="module").to_json(),
                            op("SetField", target="state", field="current_module_present", value={"kind": "I64", "value": 1}).to_json(),
                            op("SetField", target="state.scope_ctx", field="current_function", value="func").to_json(),
                            op("SetField", target="state.scope_ctx", field="current_function_present", value={"kind": "I64", "value": 1}).to_json(),
                            op("SetField", target="state", field="current_block", value="entry_block").to_json(),
                            op("SetField", target="state", field="current_block_present", value={"kind": "I64", "value": 1}).to_json(),
                            op("ReturnValue", value="state").to_json(),
                        ],
                    ),
                ],
            ),
        ],
        main_operations=[
            op("StaticCall", target="state", callee="PreparedMirBuilderStateShellApi.empty", args=[]),
            op("AssertEq", left="state.current_module_present", right=0, fail_message="prepared_state_initial_module=fail", fail_code=1),
            op("AssertEq", left="state.scope_ctx.current_function_present", right=0, fail_message="prepared_state_initial_function=fail", fail_code=2),
            op("AssertEq", left="state.current_block_present", right=0, fail_message="prepared_state_initial_block=fail", fail_code=3),
            op("StaticCall", target="module", callee="MirModuleMinimalShellApi.create", args=[{"literal": "main"}]),
            op("StaticCall", target="signature", callee="FunctionSignaturePreparedApi.create", args=[{"literal": "main"}, 0]),
            op("StaticCall", target="main_func", callee="MirFunctionConstructorShellApi.create", args=["signature", 0]),
            op("StaticCall", target="installed", callee="PreparedMirBuilderStateShellApi.install", args=["state", "module", "main_func", 0]),
            op("AssertEq", left="installed.current_module_present", right=1, fail_message="prepared_state_module_some=fail", fail_code=4),
            op("AssertEq", left="installed.current_module.name", right={"literal": "main"}, fail_message="prepared_state_module_payload=fail", fail_code=5),
            op("AssertEq", left="installed.scope_ctx.current_function_present", right=1, fail_message="prepared_state_function_some=fail", fail_code=6),
            op("AssertEq", left="installed.scope_ctx.current_function.signature.name", right={"literal": "main"}, fail_message="prepared_state_function_payload=fail", fail_code=7),
            op("AssertEq", left="installed.current_block_present", right=1, fail_message="prepared_state_block_some=fail", fail_code=8),
            op("AssertEq", left="installed.current_block", right=0, fail_message="prepared_state_block_payload=fail", fail_code=9),
            op("StaticCall", target="other", callee="PreparedMirBuilderStateShellApi.empty", args=[]),
            op("AssertEq", left="other.current_module_present", right=0, fail_message="prepared_state_other_module_alias=fail", fail_code=10),
            op("AssertEq", left="other.scope_ctx.current_function_present", right=0, fail_message="prepared_state_other_function_alias=fail", fail_code=11),
            op("Print", text="mirbuilder_prepared_state_install_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_prepared_state_install.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="MirBuilder::prepare_module.prepared_state_install",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "prepared_state_install": 1,
            "current_module_take": 0,
            "current_function_take": 0,
            "lower_root": 0,
            "literal_integer_lowering": 0,
            "return_emission": 0,
            "finalize_module": 0,
            "full_mirbuilder_object_transport": 0,
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
                "prepared_state_install_only": 1,
                "current_module_installed": 1,
                "current_function_installed": 1,
                "current_block_installed": 1,
                "fresh_state_identity": 1,
                "current_module_take": 0,
                "current_function_take": 0,
                "lower_root": 0,
                "finalize_module": 0,
                "backend_behavior_changed": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "NewBox",
            "SetField",
            "PresenceTaggedSet",
            "ReturnValue",
            "PreparedStateInstall",
        ],
        transport_notes=contract.transport_notes(
            {
                "semantic_capability": "PreparedStateInstall",
                "module_transport": "MirModuleMinimalShell",
                "function_transport": "MirFunctionConstructorShell",
                "block_transport": "BasicBlockIdAsI64",
                "observation": "install only; take and finalize remain non-claims",
            }
        ),
        denied_boundaries=list(plan["non_claims"].keys()),
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan_text = stable_json(build_plan())
    if not PLAN.exists():
        raise FileNotFoundError(f"{PLAN} must be written before manifest hashing")
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle_text = stable_json(build_oracle())
    spec = prepared_state_install_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = [(PLAN, plan_text), (ORACLE, oracle_text)]
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


def run_prepared_state_install_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(PLAN, stable_json(build_plan()))
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_mirbuilder_prepared_state_install_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_prepared_state_install_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
