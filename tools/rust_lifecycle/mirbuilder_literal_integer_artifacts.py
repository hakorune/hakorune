#!/usr/bin/env python3
"""Generate the derived Hako artifact for bounded literal integer lowering."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from family_artifact_builders import (
    build_family_artifact_hako_text,
    build_family_artifact_manifest_text,
    build_family_artifact_recipe_text,
    build_family_artifact_verifier_text,
)
from family_artifact_spec import ApiMethodSpec, BehaviorMethodSpec, BoxSpec, FieldSpec, FamilyArtifactSpec, StaticBoxSpec
from mirbuilder_core_context_artifacts import core_context_spec
from mirbuilder_next_value_id_prepared_state_kernel_artifacts import _policy_kernel_operations
from shared_family_generator import read_json, run_family_generator, rust_manifest_file_entry, stable_json, write_if_changed
from verified_family_artifact_contract import ArtifactIdentity, VerifiedFamilyArtifactContractV1
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
SOURCE = ROOT / "src/mir/builder/builder_build.rs"
PLAN = FIXTURES / "mirbuilder-literal-integer-lowering-plan-v0.json"
ALLOCATION_MANIFEST = OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
ORACLE = FIXTURES / "mirbuilder-literal-integer-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-literal-integer-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-literal-integer-derived-hako-verifier-result-v0.json"


def _var(name: str) -> dict[str, str]:
    return {"kind": "Var", "name": name}


def _call(callee: str, args: list[Any]) -> dict[str, Any]:
    return {"kind": "CallStatic", "callee": callee, "args": args}


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderLiteralIntegerDerivedHakoOracleV1",
        "subject": "MirBuilder::build_literal LiteralValue::Integer prepared-state projection",
        "vectors": [
            {
                "name": "literal_zero_then_reserved_gap_literal_42",
                "initial_function_counter": 1,
                "initial_core_context_value_counter": 100,
                "reserved_values": [2],
                "inputs": [0, 42],
                "expect": {
                    "first_result_value": 1,
                    "first_const_value": 0,
                    "first_published_type_integer": 1,
                    "second_result_value": 3,
                    "second_const_value": 42,
                    "second_published_type_integer": 1,
                    "final_function_counter": 4,
                    "final_core_context_value_counter": 100,
                },
            }
        ],
        "non_claims": {
            "return_emission": 0,
            "finalize_module": 0,
            "full_expression_lowering": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderLiteralIntegerLoweringPlanV1":
        raise ValueError("wrong literal integer lowering plan kind")
    if "LiteralIntegerLowering" not in (plan.get("available_capabilities") or []):
        raise ValueError("literal plan lacks LiteralIntegerLowering capability")
    result = plan.get("result_contract") or {}
    expected = {
        "result_value": "ValueIdAsI64",
        "emitted_instruction": "ConstValue::Integer",
        "published_type": "MirType::Integer",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"literal result contract drift: {key}")
    non_claims = plan.get("non_claims") or {}
    shape = plan.get("selected_source_shape") or {}
    if shape.get("literal_payload_transport") != "ScalarI64":
        raise ValueError("literal payload transport drift")
    for key, value in non_claims.items():
        if value != 0:
            raise ValueError(f"literal non-claim must remain 0: {key}")
    if non_claims.get("generated_hako_artifact") != 0:
        raise ValueError("source lowering plan must remain analysis-only")


def _validate_dependencies() -> None:
    manifest = read_json(ALLOCATION_MANIFEST)
    if manifest.get("family_id") != "hakorune_mir_builder::next_value_id_prepared_state_kernel":
        raise ValueError("allocation policy artifact dependency drift")
    if manifest.get("state") != "DerivedMainline":
        raise ValueError("allocation policy dependency must remain DerivedMainline")
    claims = manifest.get("claims") or {}
    if claims.get("prepared_state_policy_kernel") != 1 or claims.get("mainline_selected") != 1:
        raise ValueError("allocation policy dependency is not selected for mainline")
    if claims.get("runtime_fallback") != 0:
        raise ValueError("allocation policy dependency must not claim runtime fallback")


def _contract(plan: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    _validate_dependencies()
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::literal_integer_lowering",
        method_universe=(
            "FunctionValueIdCounterState::next",
            "ReservedValueIdMembershipView::add",
            "ReservedValueIdMembershipView::has",
            "MirBuilderAllocationPolicy::next_value_id",
            "LiteralIntegerLowering::lower",
        ),
        selected_method_ids=(
            "FunctionValueIdCounterState::next",
            "ReservedValueIdMembershipView::add",
            "ReservedValueIdMembershipView::has",
            "MirBuilderAllocationPolicy::next_value_id",
            "LiteralIntegerLowering::lower",
        ),
        denials=(),
        semantic_transports={
            "literal_payload_transport": "ScalarI64",
            "result_transport": "ValueIdAsI64",
            "instruction_transport": "ConstIntegerInstructionShell",
            "published_type_transport": "MirTypeIntegerPublicationShell",
            "allocation_policy_dependency": "MirBuilderAllocationPolicy.prepared_state_next_value_id",
            "return_emission": 0,
            "finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::literal_integer_lowering",
            api_name="LiteralIntegerLoweringApi",
            pilot_scope="LiteralIntegerLowering_prepared_state_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json",
        ),
        selected_body_count_label="literal_integer_lowering_prepared_state_only",
        expected_fields=("result_value", "instruction", "published_type"),
    )


def literal_integer_lowering_contract() -> VerifiedFamilyArtifactContractV1:
    return _contract(read_json(PLAN))


def _literal_lowering_operations() -> list[dict[str, Any]]:
    return [
        op(
            "LocalI64",
            target="dst",
            value=_call(
                "MirBuilderAllocationPolicyApi.next_value_id",
                ["current_function_present", "function_state", "core_context", "reserved_membership"],
            ),
        ).to_json(),
        op("NewBox", target="instruction", box="ConstIntegerInstructionShellBox").to_json(),
        op("SetField", target="instruction", field="dst", value=_var("dst")).to_json(),
        op("SetField", target="instruction", field="value", value=_var("literal_value")).to_json(),
        op("NewBox", target="published", box="PublishedIntegerTypeShellBox").to_json(),
        op("SetField", target="published", field="value_id", value=_var("dst")).to_json(),
        op("SetField", target="published", field="is_integer", value=1).to_json(),
        op("NewBox", target="result", box="LiteralIntegerLoweringResultBox").to_json(),
        op("SetField", target="result", field="result_value", value=_var("dst")).to_json(),
        op("SetField", target="result", field="instruction", value=_var("instruction")).to_json(),
        op("SetField", target="result", field="published_type", value=_var("published")).to_json(),
        op("ReturnValue", value=_var("result")).to_json(),
    ]


def literal_integer_lowering_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    oracle = build_oracle()
    contract = _contract(plan)
    if oracle.get("kind") != "MirBuilderLiteralIntegerDerivedHakoOracleV1":
        raise ValueError("literal integer oracle kind drift")
    core_spec = core_context_spec()
    core_next_value = [
        method for method in core_spec.api_methods if method.signature == "next_value(ctx): i64"
    ]
    if len(core_next_value) != 1:
        raise ValueError("missing CoreContextApi.next_value method")
    methods = [
        BehaviorMethodSpec(
            id="FunctionValueIdCounterState::next",
            rust_operation="MirFunction::next_value_id prepared-state counter",
            hako_operation="TakeThenIncrementI64",
            emits="FunctionValueIdCounterStateApi.next(state)",
        ),
        BehaviorMethodSpec(
            id="ReservedValueIdMembershipView::add",
            rust_operation="test-only prepared reserved membership setup",
            hako_operation="ValueIdOrderedMapBox.set",
            emits="ReservedValueIdMembershipViewApi.add(view, value_id)",
        ),
        BehaviorMethodSpec(
            id="ReservedValueIdMembershipView::has",
            rust_operation="MembershipOnly reserved ValueId observation",
            hako_operation="ValueIdOrderedMapBox.has",
            emits="ReservedValueIdMembershipViewApi.has(view, candidate)",
        ),
        BehaviorMethodSpec(
            id="MirBuilderAllocationPolicy::next_value_id",
            rust_operation="ResolvedValueAllocationPolicyV1 prepared-state execution",
            hako_operation="StructuredLoop + IfElse + CallStatic + ReturnValue",
            emits="MirBuilderAllocationPolicyApi.next_value_id(...)",
        ),
        BehaviorMethodSpec(
            id="LiteralIntegerLowering::lower",
            rust_operation="MirBuilder::build_literal LiteralValue::Integer",
            hako_operation="AllocateValueId + ConstIntegerInstructionShell + MirTypeIntegerPublicationShell",
            emits="LiteralIntegerLoweringApi.lower(...)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-literal-integer-lowering"
        ),
        generator_version="mirbuilder-literal-integer-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment="hakorune_mir_builder::literal_integer_lowering",
        using_module="",
        extra_using_modules=["apps.lib.collections.value_id_ordered_map as ValueIdOrderedMap"],
        box=BoxSpec(name="LiteralIntegerLoweringKernel", fields=[]),
        additional_boxes=[
            core_spec.box,
            BoxSpec(
                name="FunctionValueIdCounterState",
                fields=[FieldSpec(name="next_value_id", field_type="i64", initializer="0")],
            ),
            BoxSpec(
                name="ReservedValueIdMembershipView",
                fields=[
                    FieldSpec(
                        name="storage",
                        field_type="ValueIdOrderedMapBox",
                        initializer_operation={"kind": "NewValueIdOrderedMap"},
                    )
                ],
            ),
            BoxSpec(
                name="ConstIntegerInstructionShellBox",
                fields=[
                    FieldSpec(name="dst", field_type="i64", initializer="0"),
                    FieldSpec(name="value", field_type="i64", initializer="0"),
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
                    FieldSpec(
                        name="instruction",
                        field_type="ConstIntegerInstructionShellBox",
                        initializer="new ConstIntegerInstructionShellBox()",
                    ),
                    FieldSpec(
                        name="published_type",
                        field_type="PublishedIntegerTypeShellBox",
                        initializer="new PublishedIntegerTypeShellBox()",
                    ),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(name="CoreContextApi", methods=core_next_value),
            StaticBoxSpec(
                name="FunctionValueIdCounterStateApi",
                methods=[
                    ApiMethodSpec(
                        signature="next(state): i64",
                        operations=[op("TakeThenIncrementI64", source="state.next_value_id").to_json()],
                    )
                ],
            ),
            StaticBoxSpec(
                name="ReservedValueIdMembershipViewApi",
                methods=[
                    ApiMethodSpec(
                        signature="add(view, value_id): i64",
                        operations=[
                            op(
                                "MapSet",
                                source="view.storage",
                                key="value_id",
                                value="1",
                                storage="ValueIdOrderedMapBox",
                            ).to_json()
                        ],
                    ),
                    ApiMethodSpec(
                        signature="has(view, candidate): i64",
                        operations=[
                            op(
                                "MapHas",
                                source="view.storage",
                                key="candidate",
                                storage="ValueIdOrderedMapBox",
                            ).to_json()
                        ],
                    ),
                ],
            ),
            StaticBoxSpec(
                name="MirBuilderAllocationPolicyApi",
                methods=[
                    ApiMethodSpec(
                        signature=(
                            "next_value_id(current_function_present, function_state, "
                            "core_context, reserved_membership): i64"
                        ),
                        operations=_policy_kernel_operations(),
                    )
                ],
            ),
            StaticBoxSpec(
                name="LiteralIntegerLoweringApi",
                methods=[
                    ApiMethodSpec(
                        signature=(
                            "lower(current_function_present, function_state, core_context, "
                            "reserved_membership, literal_value): LiteralIntegerLoweringResultBox"
                        ),
                        operations=_literal_lowering_operations(),
                    )
                ],
            ),
        ],
        main_operations=[
            op("NewBox", target="function_state", box="FunctionValueIdCounterState"),
            op("SetField", target="function_state", field="next_value_id", value=1),
            op("NewBox", target="core_context", box="CoreContext"),
            op("SetField", target="core_context", field="value_next_id", value=100),
            op("NewBox", target="reserved", box="ReservedValueIdMembershipView"),
            op("StaticCall", callee="ReservedValueIdMembershipViewApi.add", args=["reserved", 2]),
            op(
                "StaticCall",
                target="first",
                callee="LiteralIntegerLoweringApi.lower",
                args=[1, "function_state", "core_context", "reserved", 0],
            ),
            op("AssertEq", left="first.result_value", right=1, fail_message="literal_integer_first_result=fail", fail_code=1),
            op("AssertEq", left="first.instruction.dst", right=1, fail_message="literal_integer_first_dst=fail", fail_code=2),
            op("AssertEq", left="first.instruction.value", right=0, fail_message="literal_integer_first_value=fail", fail_code=3),
            op("AssertEq", left="first.published_type.value_id", right=1, fail_message="literal_integer_first_type_value=fail", fail_code=4),
            op("AssertEq", left="first.published_type.is_integer", right=1, fail_message="literal_integer_first_type_integer=fail", fail_code=5),
            op(
                "StaticCall",
                target="second",
                callee="LiteralIntegerLoweringApi.lower",
                args=[1, "function_state", "core_context", "reserved", 42],
            ),
            op("AssertEq", left="second.result_value", right=3, fail_message="literal_integer_second_result=fail", fail_code=6),
            op("AssertEq", left="second.instruction.dst", right=3, fail_message="literal_integer_second_dst=fail", fail_code=7),
            op("AssertEq", left="second.instruction.value", right=42, fail_message="literal_integer_second_value=fail", fail_code=8),
            op("AssertEq", left="second.published_type.value_id", right=3, fail_message="literal_integer_second_type_value=fail", fail_code=9),
            op("AssertEq", left="second.published_type.is_integer", right=1, fail_message="literal_integer_second_type_integer=fail", fail_code=10),
            op("AssertEq", left="function_state.next_value_id", right=4, fail_message="literal_integer_function_counter_final=fail", fail_code=11),
            op("AssertEq", left="core_context.value_next_id", right=100, fail_message="literal_integer_core_counter_final=fail", fail_code=12),
            op("Print", text="mirbuilder_literal_integer_lowering_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_literal_integer_lowering.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.build_literal.integer.prepared_state_projection",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "literal_integer_lowering": 1,
            "allocation_policy_prepared_state_dependency": 1,
            "typed_integer_literal": 0,
            "float_literal": 0,
            "bool_literal": 0,
            "string_literal": 0,
            "null_literal": 0,
            "void_literal": 0,
            "full_expression_lowering": 0,
            "return_emission": 0,
            "finalize_module": 0,
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
                "literal_integer_lowering_only": 1,
                "allocation_policy_dependency_verified": 1,
                "allocates_result_value": 1,
                "emits_const_integer_instruction_shell": 1,
                "publishes_mir_type_integer_shell": 1,
                "returns_value_id": 1,
                "reserved_candidate_consumed": 1,
                "return_emission": 0,
                "finalize_module": 0,
                "backend_behavior_changed": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "TakeThenIncrementI64",
            "ValueIdOrderedMapBox.set",
            "ValueIdOrderedMapBox.has",
            "StructuredLoop",
            "IfElse",
            "CallStatic",
            "ReturnValue",
            "ConstIntegerInstructionShell",
            "MirTypeIntegerPublicationShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "allocation_policy_dependency": str(ALLOCATION_MANIFEST.relative_to(ROOT)),
                "type_context_map_transport": "unselected",
                "return_terminator_transport": "unselected",
            }
        ),
        denied_boundaries=[
            "typed_integer_literal",
            "float_literal",
            "bool_literal",
            "string_literal",
            "null_literal",
            "void_literal",
            "full_expression_lowering",
            "return_emission",
            "finalize_module",
            "backend_route_changed",
            "abi_changed",
            "runtime_fallback",
        ],
        extra_manifest_fields={
            **contract.manifest_extra_fields(),
            "dependency_artifacts": {
                "allocation_policy": rust_manifest_file_entry(path=ALLOCATION_MANIFEST, root=ROOT)
            },
        },
    )


def _outputs() -> list[tuple[Path, str]]:
    oracle_text = stable_json(build_oracle())
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    spec = literal_integer_lowering_spec()
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


def run_literal_integer_lowering_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_literal_integer_lowering_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_literal_integer_lowering_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
