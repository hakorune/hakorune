#!/usr/bin/env python3
"""Generate the bounded CurrentModuleTake Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-current-module-take-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-current-module-take-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-current-module-take-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-current-module-take-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderCurrentModuleTakeDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module current_module.take().unwrap()",
        "vectors": [
            {
                "name": "present_module_is_taken_and_state_cleared",
                "inputs": {
                    "initial_current_module_present": 1,
                    "module_name_is_main": 1,
                    "functions_empty": 1,
                    "globals_empty": 1,
                },
                "expect": {
                    "current_module_present": 0,
                    "taken_module_present": 1,
                    "module_name_is_main": 1,
                    "functions_empty": 1,
                    "globals_empty": 1,
                },
            }
        ],
        "non_claims": {
            "verify_typed_values": 0,
            "current_function_take": 0,
            "full_finalize_module": 0,
            "module_metadata_publication": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderCurrentModuleTakePlanV1":
        raise ValueError("wrong current module take plan kind")
    if "CurrentModuleTake" not in (plan.get("available_capabilities") or []):
        raise ValueError("current module take plan lacks CurrentModuleTake capability")
    profile = plan.get("execution_profile") or {}
    if profile.get("module_transport") != "MirModuleMinimalShell":
        raise ValueError("current module take module transport drift")
    result = plan.get("result_contract") or {}
    expected = {
        "taken_value": "MirModuleMinimalShell",
        "source_state": "self.current_module",
        "post_take_state": "None",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"current module take result contract drift: {key}")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"current module take non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::current_module_take",
        method_universe=("CurrentModuleTake::take",),
        selected_method_ids=("CurrentModuleTake::take",),
        denials=(),
        semantic_transports={
            "state_transport": "PreparedCurrentModulePresenceShell",
            "module_transport": "MirModuleMinimalShell",
            "post_take_state": "None",
            "verify_typed_values": 0,
            "current_function_take": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::current_module_take",
            api_name="CurrentModuleTakeApi",
            pilot_scope="CurrentModuleTake_present_module_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json",
        ),
        selected_body_count_label="current_module_take_present_module_only",
        expected_fields=(
            "current_module_present",
            "taken_module_present",
            "module_name_is_main",
            "functions_empty",
            "globals_empty",
        ),
    )


def current_module_take_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderCurrentModuleTakeDerivedHakoOracleV1":
        raise ValueError("current module take oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="CurrentModuleTake::take",
            rust_operation="MirBuilder::finalize_module current_module.take().unwrap()",
            hako_operation="ClearPresence + MarkTaken + ReturnValue",
            emits="CurrentModuleTakeApi.take(state, module)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-current-module-take"
        ),
        generator_version="mirbuilder-current-module-take-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="CurrentModuleTakeKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="PreparedCurrentModuleStateShellBox",
                fields=[
                    FieldSpec(name="current_module_present", field_type="i64", initializer="1"),
                    FieldSpec(name="taken_module_present", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="CurrentModuleTakeModuleShellBox",
                fields=[
                    FieldSpec(name="name_is_main", field_type="i64", initializer="1"),
                    FieldSpec(name="functions_empty", field_type="i64", initializer="1"),
                    FieldSpec(name="globals_empty", field_type="i64", initializer="1"),
                    FieldSpec(name="metadata_default", field_type="i64", initializer="1"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="CurrentModuleTakeApi",
                methods=[
                    ApiMethodSpec(
                        signature="take(state, module): CurrentModuleTakeModuleShellBox",
                        operations=[
                            op("SetField", target="state", field="current_module_present", value=0).to_json(),
                            op("SetField", target="state", field="taken_module_present", value=1).to_json(),
                            op("ReturnValue", value="module").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="state", box="PreparedCurrentModuleStateShellBox"),
            op("NewBox", target="module", box="CurrentModuleTakeModuleShellBox"),
            op("StaticCall", target="taken", callee="CurrentModuleTakeApi.take", args=["state", "module"]),
            op("AssertEq", left="state.current_module_present", right=0, fail_message="current_module_present=fail", fail_code=1),
            op("AssertEq", left="state.taken_module_present", right=1, fail_message="taken_module_present=fail", fail_code=2),
            op("AssertEq", left="taken.name_is_main", right=1, fail_message="module_name=fail", fail_code=3),
            op("AssertEq", left="taken.functions_empty", right=1, fail_message="module_functions=fail", fail_code=4),
            op("AssertEq", left="taken.globals_empty", right=1, fail_message="module_globals=fail", fail_code=5),
            op("Print", text="mirbuilder_current_module_take_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_current_module_take.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.current_module_take",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "current_module_take": 1,
            "verify_typed_values": 0,
            "current_function_take": 0,
            "full_finalize_module": 0,
            "module_metadata_publication": 0,
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
                "current_module_take_only": 1,
                "module_transport": "MirModuleMinimalShell",
                "post_take_state": "None",
                "taken_module_present": 1,
                "verify_typed_values": 0,
                "current_function_take": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "MirModuleMinimalShell",
            "PreparedCurrentModulePresenceShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "verify_typed_values": "unselected",
                "current_function_take": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "verify_typed_values",
            "current_function_take",
            "full_finalize_module",
            "module_metadata_publication",
            "mainline_selected",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = current_module_take_spec()
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


def run_current_module_take_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_current_module_take_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_current_module_take_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
