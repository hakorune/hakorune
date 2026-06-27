#!/usr/bin/env python3
"""Generate the bounded ReturnEmission Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-return-emission-plan-v0.json"
SHADOW_RESULT = FIXTURES / "mirbuilder-return-emission-hako-shadow-result-v0.json"
ORACLE = FIXTURES / "mirbuilder-return-emission-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-return-emission-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-return-emission-derived-hako-verifier-result-v0.json"


def _shadow_candidate(plan: dict[str, object]) -> dict[str, object]:
    execution_profile = plan.get("execution_profile") or {}
    result_contract = plan.get("result_contract") or {}
    return {
        "schema_version": 0,
        "kind": "ReturnEmissionShadowCandidateV1",
        "family_id": "hakorune_mir_builder::return_emission",
        "stage_id": "return_emission",
        "subject": plan["subject"],
        "source_authority": plan["source_authority"]["finalize"],
        "execution_profile": {
            "input": execution_profile.get("input"),
            "current_block": execution_profile.get("current_block"),
            "current_function": execution_profile.get("current_function"),
            "target_block": execution_profile.get("target_block"),
            "target_block_terminated": execution_profile.get("target_block_terminated"),
            "result_value_transport": execution_profile.get("result_value_transport"),
        },
        "emission_sequence": plan["emission_sequence"],
        "available_capabilities": plan["available_capabilities"],
        "result_contract": {
            "terminator": result_contract.get("terminator"),
            "value": result_contract.get("value"),
            "value_transport": result_contract.get("value_transport"),
            "successors": result_contract.get("successors"),
        },
        "non_claims": {
            "return_type_publication": 0,
            "full_finalize_module": 0,
            "already_terminated_block_behavior": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnEmissionDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module append Return(result_value)",
        "vectors": [
            {
                "name": "unterminated_block_gets_return_terminator",
                "inputs": {"block_id": 0, "result_value": 7, "initial_terminated": 0},
                "expect": {
                    "terminated": 1,
                    "return_value": 7,
                    "return_value_present": 1,
                    "successors_empty": 1,
                },
            }
        ],
        "non_claims": {
            "return_type_publication": 0,
            "full_finalize_module": 0,
            "already_terminated_block_behavior": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def build_shadow_result() -> dict[str, object]:
    plan = read_json(PLAN)
    candidate = _shadow_candidate(plan)
    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnEmissionDerivedHakoShadowResultV1",
        "subject": plan["subject"],
        "result": {
            "err": 0,
            "err_line": "",
            "shadow_record": candidate,
            "shadow_json": stable_json(candidate),
        },
        "non_claims": {
            "return_type_publication": 0,
            "full_finalize_module": 0,
            "already_terminated_block_behavior": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderReturnEmissionPlanV1":
        raise ValueError("wrong return emission plan kind")
    if "ReturnEmission" not in (plan.get("available_capabilities") or []):
        raise ValueError("return emission plan lacks ReturnEmission capability")
    result = plan.get("result_contract") or {}
    expected = {
        "terminator": "MirInstruction::Return",
        "value": "Some(result_value)",
        "value_transport": "ValueIdAsI64",
        "successors": "Empty",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"return emission result contract drift: {key}")
    profile = plan.get("execution_profile") or {}
    if profile.get("target_block_terminated") is not False:
        raise ValueError("return emission artifact only supports unterminated target block profile")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"return emission non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::return_emission",
        method_universe=("ReturnEmission::emit",),
        selected_method_ids=("ReturnEmission::emit",),
        denials=(),
        semantic_transports={
            "block_transport": "ReturnEmissionBasicBlockShell",
            "result_value_transport": "ValueIdAsI64",
            "terminator_transport": "MirInstructionReturnShell",
            "successor_transport": "EmptySuccessorShell",
            "return_type_publication": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::return_emission",
            api_name="ReturnEmissionApi",
            pilot_scope="ReturnEmission_unterminated_block_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json",
        ),
        selected_body_count_label="return_emission_unterminated_block_only",
        expected_fields=("terminated", "return_value", "return_value_present", "successors_empty"),
    )


def return_emission_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderReturnEmissionDerivedHakoOracleV1":
        raise ValueError("return emission oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="ReturnEmission::emit",
            rust_operation="MirBuilder::finalize_module append Return(result_value)",
            hako_operation="SetField + ReturnValue",
            emits="ReturnEmissionApi.emit(block, result_value)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-return-emission"
        ),
        generator_version="mirbuilder-return-emission-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="ReturnEmissionKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="ReturnEmissionBasicBlockShellBox",
                fields=[
                    FieldSpec(name="id", field_type="i64", initializer="0"),
                    FieldSpec(name="terminated", field_type="i64", initializer="0"),
                    FieldSpec(name="return_value", field_type="i64", initializer="0"),
                    FieldSpec(name="return_value_present", field_type="i64", initializer="0"),
                    FieldSpec(name="successors_empty", field_type="i64", initializer="1"),
                ],
            )
        ],
        static_boxes=[
            StaticBoxSpec(
                name="ReturnEmissionApi",
                methods=[
                    ApiMethodSpec(
                        signature="emit(block, result_value): ReturnEmissionBasicBlockShellBox",
                        operations=[
                            op("SetField", target="block", field="terminated", value=1).to_json(),
                            op("SetField", target="block", field="return_value", value="result_value").to_json(),
                            op("SetField", target="block", field="return_value_present", value=1).to_json(),
                            op("SetField", target="block", field="successors_empty", value=1).to_json(),
                            op("ReturnValue", value="block").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="block", box="ReturnEmissionBasicBlockShellBox"),
            op("SetField", target="block", field="id", value=0),
            op("StaticCall", target="emitted", callee="ReturnEmissionApi.emit", args=["block", 7]),
            op("AssertEq", left="emitted.terminated", right=1, fail_message="return_emission_terminated=fail", fail_code=1),
            op("AssertEq", left="emitted.return_value", right=7, fail_message="return_emission_value=fail", fail_code=2),
            op("AssertEq", left="emitted.return_value_present", right=1, fail_message="return_emission_value_present=fail", fail_code=3),
            op("AssertEq", left="emitted.successors_empty", right=1, fail_message="return_emission_successors=fail", fail_code=4),
            op("Print", text="mirbuilder_return_emission_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_return_emission.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.return_emission",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "return_emission": 1,
            "return_type_publication": 0,
            "full_finalize_module": 0,
            "other_terminator_shapes": 0,
            "already_terminated_block_behavior": 0,
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
                "return_emission_only": 1,
                "terminator_is_return": 1,
                "return_value_some": 1,
                "value_transport": "ValueIdAsI64",
                "successors_empty": 1,
                "return_type_publication": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "MirInstructionReturnShell",
            "EmptySuccessorShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "already_terminated_block_behavior": "unselected",
                "return_type_publication": "unselected",
            }
        ),
        denied_boundaries=[
            "return_type_publication",
            "full_finalize_module",
            "other_terminator_shapes",
            "already_terminated_block_behavior",
            "mainline_selected",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    shadow_result = build_shadow_result()
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    if not SHADOW_RESULT.exists():
        raise FileNotFoundError(f"{SHADOW_RESULT} must be written before manifest hashing")
    spec = return_emission_spec()
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
    outputs.append((SHADOW_RESULT, stable_json(shadow_result)))
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


def run_return_emission_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(SHADOW_RESULT, stable_json(build_shadow_result()))
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_return_emission_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_return_emission_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
