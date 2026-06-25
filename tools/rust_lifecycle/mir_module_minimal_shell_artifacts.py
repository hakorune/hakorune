#!/usr/bin/env python3
"""Generate the derived Hako artifact for the minimal MirModule shell."""

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
SOURCE = ROOT / "src/mir/function/module_impl.rs"
PLAN = FIXTURES / "mir-module-minimal-shell-transport-plan-v0.json"
ORACLE = FIXTURES / "mir-module-minimal-shell-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mir-module-minimal-shell-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mir-module-minimal-shell-derived-hako-verifier-result-v0.json"


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirModuleMinimalShellTransportPlanV1":
        raise ValueError("wrong MirModule shell transport plan kind")
    directability = plan.get("directability") or {}
    if directability.get("capability") != "MirModuleMinimalShellTransport":
        raise ValueError("MirModule shell plan capability drift")
    fields = {row["field"]: row for row in plan.get("shell_fields", [])}
    expected = {
        "name": ("parameter:name", "ModuleNameStringAtom"),
        "functions": ("BTreeMap::new", "EmptyFunctionTable"),
        "globals": ("HashMap::new", "EmptyGlobalConstTable"),
        "metadata": ("ModuleMetadata::default", "ModuleMetadataDefaultShell"),
    }
    for field, (initializer, transport) in expected.items():
        if fields.get(field, {}).get("initializer") != initializer:
            raise ValueError(f"MirModule shell initializer drift: {field}")
        if fields.get(field, {}).get("transport") != transport:
            raise ValueError(f"MirModule shell transport drift: {field}")
    if plan.get("metadata_default_observations", {}).get("source_file") is not None:
        raise ValueError("MirModule source_file default must remain absent")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"MirModule shell non-claim must remain 0: {key}")


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirModuleMinimalShellDerivedHakoOracleV1",
        "subject": "hakorune_mir::MirModule::new",
        "vectors": [
            {
                "name": "fresh_module_shells",
                "inputs": {"first_name": "main", "second_name": "other"},
                "expect": {
                    "first_name": "main",
                    "second_name": "other",
                    "first_functions_initial": 0,
                    "first_globals_initial": 0,
                    "first_source_file": "Option::None",
                    "second_functions_after_first_mutation": 0,
                    "second_globals_after_first_mutation": 0,
                },
            }
        ],
        "non_claims": {
            "function_insertion": 0,
            "global_publication": 0,
            "metadata_publication": 0,
            "source_file_assignment": 0,
            "full_mir_module_conversion": 0,
            "mainline_selected": 0,
        },
    }


def _contract(plan: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir::MirModuleMinimalShell",
        method_universe=("MirModuleMinimalShell::new",),
        selected_method_ids=("MirModuleMinimalShell::new",),
        denials=(),
        semantic_transports={
            "module_transport": "MirModuleMinimalShell",
            "name_transport": "ModuleNameStringAtom",
            "function_table_transport": "EmptyFunctionTable",
            "global_table_transport": "EmptyGlobalConstTable",
            "metadata_transport": "ModuleMetadataDefaultShell",
            "source_file_transport": "AbsentSourceFile",
            "full_mir_module_conversion": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir::MirModuleMinimalShell",
            api_name="MirModuleMinimalShellApi",
            pilot_scope="MirModuleMinimalShell_new_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json",
        ),
        selected_body_count_label="mir_module_minimal_shell_constructor_only",
        expected_fields=("name", "functions", "globals", "metadata"),
    )


def mir_module_minimal_shell_contract() -> VerifiedFamilyArtifactContractV1:
    return _contract(read_json(PLAN))


def mir_module_minimal_shell_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="MirModuleMinimalShell::new",
            rust_operation="MirModule::new shell constructor",
            hako_operation="NewBox + SetField + ReturnValue",
            emits="MirModuleMinimalShellApi.create(name)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by="tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mir-module-minimal-shell",
        generator_version="mir-module-minimal-shell-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="MirModuleMinimalShellHarness", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="ModuleMetadataDefaultShell",
                fields=[
                    FieldSpec(
                        name="source_file",
                        field_type="Option<StringBox>",
                        initializer="Option::None()",
                    )
                ],
            ),
            BoxSpec(
                name="MirModuleMinimalShellBox",
                fields=[
                    FieldSpec(name="name", field_type="StringBox", initializer="null"),
                    FieldSpec(
                        name="functions",
                        field_type="OrderedMapBox",
                        initializer_operation={"kind": "NewOrderedMap"},
                    ),
                    FieldSpec(
                        name="globals",
                        field_type="OrderedMapBox",
                        initializer_operation={"kind": "NewOrderedMap"},
                    ),
                    FieldSpec(
                        name="metadata",
                        field_type="ModuleMetadataDefaultShell",
                        initializer="new ModuleMetadataDefaultShell()",
                    ),
                ],
            ),
        ],
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
                    ),
                ],
            )
        ],
        main_operations=[
            op("StaticCall", target="module_a", callee="MirModuleMinimalShellApi.create", args=[{"literal": "main"}]),
            op("StaticCall", target="module_b", callee="MirModuleMinimalShellApi.create", args=[{"literal": "other"}]),
            op("AssertEq", left="module_a.name", right={"literal": "main"}, fail_message="mir_module_shell_name=fail", fail_code=1),
            op("AssertEq", left="module_b.name", right={"literal": "other"}, fail_message="mir_module_shell_second_name=fail", fail_code=2),
            op("Assign", target="module_a_functions", value="module_a.functions.keys_value.length()"),
            op("AssertEq", left="module_a_functions", right=0, fail_message="mir_module_shell_functions_empty=fail", fail_code=3),
            op("Assign", target="module_a_globals", value="module_a.globals.keys_value.length()"),
            op("AssertEq", left="module_a_globals", right=0, fail_message="mir_module_shell_globals_empty=fail", fail_code=4),
            op("AssertEq", left="module_a.metadata.source_file", right={"expr": "Option::None()"}, fail_message="mir_module_shell_source_file_absent=fail", fail_code=5),
            op("MethodCall", receiver="module_a.functions.keys_value", method="push", args=[{"literal": "only_a"}]),
            op("MethodCall", receiver="module_a.functions.values_value", method="push", args=[1]),
            op("MethodCall", receiver="module_a.globals.keys_value", method="push", args=[{"literal": "global_a"}]),
            op("MethodCall", receiver="module_a.globals.values_value", method="push", args=[1]),
            op("Assign", target="module_a_functions_after", value="module_a.functions.keys_value.length()"),
            op("AssertEq", left="module_a_functions_after", right=1, fail_message="mir_module_shell_function_table_mutated=fail", fail_code=6),
            op("Assign", target="module_b_functions_after", value="module_b.functions.keys_value.length()"),
            op("AssertEq", left="module_b_functions_after", right=0, fail_message="mir_module_shell_function_table_alias=fail", fail_code=7),
            op("Assign", target="module_a_globals_after", value="module_a.globals.keys_value.length()"),
            op("AssertEq", left="module_a_globals_after", right=1, fail_message="mir_module_shell_global_table_mutated=fail", fail_code=8),
            op("Assign", target="module_b_globals_after", value="module_b.globals.keys_value.length()"),
            op("AssertEq", left="module_b_globals_after", right=0, fail_message="mir_module_shell_global_table_alias=fail", fail_code=9),
            op("Print", text="mir_module_minimal_shell_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mir_module_minimal_shell.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir::MirModule::new.minimal_shell",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "mir_module_minimal_shell": 1,
            "source_file_assignment": 0,
            "function_insertion": 0,
            "global_publication": 0,
            "metadata_publication": 0,
            "finalize_module": 0,
            "full_mir_module_conversion": 0,
            "full_mirbuilder_new": 0,
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
                "module_name_preserved": 1,
                "functions_initially_empty": 1,
                "globals_initially_empty": 1,
                "source_file_absent": 1,
                "fresh_function_table_identity": 1,
                "fresh_global_table_identity": 1,
                "function_insertion": 0,
                "global_publication": 0,
                "backend_behavior_changed": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "NewBox",
            "SetField",
            "ReturnValue",
            "OrderedMap.create",
            "OrderedMapBox.length",
            "Option::None",
        ],
        transport_notes=contract.transport_notes(
            {
                "semantic_capability": "MirModuleMinimalShellTransport",
                "function_table_observation": "empty + fresh identity only",
                "global_table_observation": "empty + fresh identity only",
            }
        ),
        denied_boundaries=list(plan["non_claims"].keys()),
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle_text = stable_json(build_oracle())
    spec = mir_module_minimal_shell_spec()
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


def run_mir_module_minimal_shell_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_mir_module_minimal_shell_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_mir_module_minimal_shell_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
