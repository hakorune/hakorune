#!/usr/bin/env python3
"""Generate the prepared-state Hako kernel for MirBuilder::next_value_id."""

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
from mirbuilder_core_context_artifacts import core_context_contract, core_context_spec
from shared_family_generator import read_json, run_family_generator, stable_json, write_if_changed
from verified_family_artifact_contract import ArtifactIdentity, VerifiedFamilyArtifactContractV1
from verified_hako_family_ir import op


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
ID_ALLOC = ROOT / "src/mir/builder/utils/id_alloc.rs"

COMPOSITION = FIXTURES / "mirbuilder-next-value-id-composition-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-next-value-id-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-next-value-id-prepared-state-oracle-v0.json"


def _var(name: str) -> dict[str, str]:
    return {"kind": "Var", "name": name}


def _i64(value: int) -> dict[str, int | str]:
    return {"kind": "I64", "value": value}


def _eq(left: Any, right: Any) -> dict[str, Any]:
    return {"kind": "EqI64", "left": left, "right": right}


def _call(callee: str, args: list[Any]) -> dict[str, Any]:
    return {"kind": "CallStatic", "callee": callee, "args": args}


def build_execution_projection(composition: dict[str, Any]) -> dict[str, Any]:
    if composition.get("kind") != "MirBuilderNextValueIdCompositionPlanV1":
        raise ValueError("wrong composition plan kind")
    return {
        "schema_version": 0,
        "kind": "MirBuilderNextValueIdExecutionProjectionV1",
        "source_plan": "MirBuilderNextValueIdCompositionPlanV1",
        "execution_scope": "PreparedStatePolicyKernel",
        "selector_transport": {
            "semantic": "CurrentFunctionPresence",
            "physical": "CurrentFunctionPresenceI64BoolV0",
            "canonical_values": {"absent": 0, "present": 1},
            "evaluation": "PerCandidateAttempt",
        },
        "function_state_transport": {
            "semantic": "MirFunction.next_value_id",
            "physical": "FunctionValueIdCounterState.next_value_id",
            "lane": "i64",
            "operation": "TakeThenIncrementI64",
            "nominal_result": "ValueIdAsI64",
        },
        "module_state_transport": {
            "semantic": "CoreContext.value_gen",
            "artifact_contract": "CoreContext VerifiedFamilyArtifactContractV1",
            "called_method": "CoreContextApi.next_value",
        },
        "exclusion_transport": {
            "semantic": "ReservedValueIdMembership",
            "access": "ReadOnly",
            "physical_pilot": "ReservedValueIdMembershipViewBox",
            "substrate": "ValueIdOrderedMapBox",
            "exposed_operation": "has",
        },
        "result_transport": composition["result_transport"],
        "directability": {
            "prepared_state_policy_kernel": "Allow",
            "full_mirbuilder_object_method": "Deny",
        },
        "observation_closure": {
            "current_function": "PresenceOnly",
            "function_state": "next_value_id field only",
            "core_context": "CoreContextApi.next_value only",
            "reserved_set": "MembershipOnly",
        },
        "mutation_frame": {
            "present": "function counter mutates, CoreContext value counter unchanged",
            "absent": "CoreContext value counter mutates, function counter unchanged",
            "rejected": "selected counter already consumed",
            "reserved_view": "read-only",
        },
        "progress_precondition": "At least one unreserved candidate before overflow boundary",
        "non_claims": {
            "full_option_mirfunction_transport": 0,
            "full_mirfunction_conversion": 0,
            "scope_context_conversion": 0,
            "compilation_context_conversion": 0,
            "parameter_compatibility_fallback": 0,
            "formal_invalid_sentinel_exclusion": 0,
            "overflow_parity": 0,
            "total_termination": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "PreparedStateKernelOracleV1",
        "subject": "MirBuilderNextValueIdExecutionProjectionV1",
        "vectors": [
            {
                "name": "present_branch_reserved_gap",
                "current_function_present": 1,
                "initial_function_counter": 1,
                "initial_core_context_value_counter": 100,
                "reserved_values": [2],
                "outputs": [1, 3, 4],
                "final_function_counter": 5,
                "final_core_context_value_counter": 100,
            },
            {
                "name": "absent_branch_reserved_gap",
                "current_function_present": 0,
                "initial_function_counter": 100,
                "initial_core_context_value_counter": 0,
                "reserved_values": [1],
                "outputs": [0, 2],
                "final_function_counter": 100,
                "final_core_context_value_counter": 3,
            },
        ],
    }


def validate_projection(projection: dict[str, Any], oracle: dict[str, Any]) -> None:
    if projection.get("kind") != "MirBuilderNextValueIdExecutionProjectionV1":
        raise ValueError("wrong execution projection kind")
    if projection.get("source_plan") != "MirBuilderNextValueIdCompositionPlanV1":
        raise ValueError("execution projection must consume composition plan")
    if projection["selector_transport"]["evaluation"] != "PerCandidateAttempt":
        raise ValueError("selector must stay per-candidate")
    if projection["function_state_transport"]["operation"] != "TakeThenIncrementI64":
        raise ValueError("function-state transport must use non-saturating take-then-increment")
    if projection["module_state_transport"]["called_method"] != "CoreContextApi.next_value":
        raise ValueError("module allocator must consume existing CoreContextApi.next_value")
    if projection["exclusion_transport"]["exposed_operation"] != "has":
        raise ValueError("reserved view must expose membership only")
    if projection["result_transport"] != "ValueIdAsI64":
        raise ValueError("result transport drift")
    if projection["directability"]["prepared_state_policy_kernel"] != "Allow":
        raise ValueError("prepared-state kernel must be allowed")
    if projection["directability"]["full_mirbuilder_object_method"] != "Deny":
        raise ValueError("full MirBuilder method must remain denied")
    for key, value in projection["non_claims"].items():
        if value != 0:
            raise ValueError(f"non-claim must stay 0: {key}")
    if oracle.get("kind") != "PreparedStateKernelOracleV1" or len(oracle.get("vectors", [])) != 2:
        raise ValueError("prepared-state oracle drift")


def _contract(projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    core_contract = core_context_contract()
    if core_contract.semantic_transports.get("reserved_value_id_skipping_claim") != 0:
        raise ValueError("CoreContext must not claim reserved skipping")
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::next_value_id_prepared_state_kernel",
        method_universe=(
            "FunctionValueIdCounterState::next",
            "ReservedValueIdMembershipView::add",
            "ReservedValueIdMembershipView::has",
            "MirBuilderAllocationPolicy::next_value_id",
        ),
        selected_method_ids=(
            "FunctionValueIdCounterState::next",
            "ReservedValueIdMembershipView::add",
            "ReservedValueIdMembershipView::has",
            "MirBuilderAllocationPolicy::next_value_id",
        ),
        denials=(),
        semantic_transports={
            "current_function_presence_transport": projection["selector_transport"]["physical"],
            "function_state_transport": "FunctionValueIdCounterAsI64",
            "module_state_transport": "CoreContext VerifiedFamilyArtifactContractV1",
            "reserved_membership_transport": "ReservedValueIdMembershipOnly",
            "result_transport": "ValueIdAsI64",
            "full_mirbuilder_object_method_directability": "Deny",
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::next_value_id_prepared_state_kernel",
            api_name="MirBuilderAllocationPolicyApi",
            pilot_scope="PreparedStateMirBuilderNextValueIdKernel",
            artifact_path=(
                "lang/generated/rust_derived/hakorune_mir_builder/"
                "mirbuilder_next_value_id_prepared_state_kernel.hako"
            ),
            manifest_path=(
                "lang/generated/rust_derived/hakorune_mir_builder/"
                "mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
            ),
        ),
        selected_body_count_label="prepared_state_policy_kernel_methods_only",
        expected_fields=("next_value_id", "storage"),
    )


def _policy_kernel_operations() -> list[dict[str, Any]]:
    return [
        op("LocalI64", target="candidate", value=_i64(0)).to_json(),
        op(
            "StructuredLoop",
            condition={"kind": "LtI64", "left": {"kind": "Var", "name": "candidate"}, "right": {"kind": "I64", "value": 4294967295}},
            body=[
                op(
                    "IfElse",
                    condition=_eq(_var("current_function_present"), _i64(1)),
                    then_body=[
                        op(
                            "Assign",
                            target="candidate",
                            value=_call("FunctionValueIdCounterStateApi.next", ["function_state"]),
                        ).to_json()
                    ],
                    else_body=[
                        op(
                            "Assign",
                            target="candidate",
                            value=_call("CoreContextApi.next_value", ["core_context"]),
                        ).to_json()
                    ],
                ).to_json(),
                op(
                    "LocalI64",
                    target="is_reserved",
                    value=_call("ReservedValueIdMembershipViewApi.has", ["reserved_membership", "candidate"]),
                ).to_json(),
                op(
                    "IfElse",
                    condition=_eq(_var("is_reserved"), _i64(0)),
                    then_body=[op("ReturnValue", value=_var("candidate")).to_json()],
                ).to_json(),
            ],
        ).to_json(),
        op("ReturnI64", return_value=0).to_json(),
    ]


def prepared_state_kernel_spec() -> FamilyArtifactSpec:
    composition = read_json(COMPOSITION)
    projection = build_execution_projection(composition)
    oracle = build_oracle()
    validate_projection(projection, oracle)
    contract = _contract(projection)
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
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-next-value-id-prepared-state-kernel"
        ),
        generator_version="mirbuilder-next-value-id-prepared-state-kernel-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment="hakorune_mir_builder::next_value_id_prepared_state_kernel",
        using_module="apps.lib.collections.ordered_map",
        box=BoxSpec(name="MirBuilderAllocationPolicyKernel", fields=[]),
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
                        field_type="OrderedMapBox",
                        initializer_operation={"kind": "NewValueIdOrderedMap"},
                    )
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
        ],
        main_operations=[
            op("NewBox", target="function_present", box="FunctionValueIdCounterState"),
            op("SetField", target="function_present", field="next_value_id", value=1),
            op("NewBox", target="core_present", box="CoreContext"),
            op("SetField", target="core_present", field="value_next_id", value=100),
            op("NewBox", target="reserved_present", box="ReservedValueIdMembershipView"),
            op("StaticCall", callee="ReservedValueIdMembershipViewApi.add", args=["reserved_present", 2]),
            op(
                "StaticCall",
                target="present0",
                callee="MirBuilderAllocationPolicyApi.next_value_id",
                args=[1, "function_present", "core_present", "reserved_present"],
            ),
            op("AssertEq", left="present0", right=1, fail_message="next_value_id_present0=fail", fail_code=1),
            op(
                "StaticCall",
                target="present1",
                callee="MirBuilderAllocationPolicyApi.next_value_id",
                args=[1, "function_present", "core_present", "reserved_present"],
            ),
            op("AssertEq", left="present1", right=3, fail_message="next_value_id_present1=fail", fail_code=2),
            op(
                "StaticCall",
                target="present2",
                callee="MirBuilderAllocationPolicyApi.next_value_id",
                args=[1, "function_present", "core_present", "reserved_present"],
            ),
            op("AssertEq", left="present2", right=4, fail_message="next_value_id_present2=fail", fail_code=3),
            op("AssertEq", left="function_present.next_value_id", right=5, fail_message="next_value_id_present_function_final=fail", fail_code=4),
            op("AssertEq", left="core_present.value_next_id", right=100, fail_message="next_value_id_present_core_final=fail", fail_code=5),
            op("NewBox", target="function_absent", box="FunctionValueIdCounterState"),
            op("SetField", target="function_absent", field="next_value_id", value=100),
            op("NewBox", target="core_absent", box="CoreContext"),
            op("SetField", target="core_absent", field="value_next_id", value=0),
            op("NewBox", target="reserved_absent", box="ReservedValueIdMembershipView"),
            op("StaticCall", callee="ReservedValueIdMembershipViewApi.add", args=["reserved_absent", 1]),
            op(
                "StaticCall",
                target="absent0",
                callee="MirBuilderAllocationPolicyApi.next_value_id",
                args=[0, "function_absent", "core_absent", "reserved_absent"],
            ),
            op("AssertEq", left="absent0", right=0, fail_message="next_value_id_absent0=fail", fail_code=6),
            op(
                "StaticCall",
                target="absent1",
                callee="MirBuilderAllocationPolicyApi.next_value_id",
                args=[0, "function_absent", "core_absent", "reserved_absent"],
            ),
            op("AssertEq", left="absent1", right=2, fail_message="next_value_id_absent1=fail", fail_code=7),
            op("AssertEq", left="function_absent.next_value_id", right=100, fail_message="next_value_id_absent_function_final=fail", fail_code=8),
            op("AssertEq", left="core_absent.value_next_id", right=3, fail_message="next_value_id_absent_core_final=fail", fail_code=9),
            op("Print", text="mirbuilder_next_value_id_prepared_state_kernel=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=ID_ALLOC,
        hako_path=OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.hako",
        facts_path=COMPOSITION,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=FIXTURES / "mirbuilder-next-value-id-prepared-state-recipe-v0.json",
        verifier_path=FIXTURES / "mirbuilder-next-value-id-prepared-state-verifier-result-v0.json",
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.next_value_id.prepared_state_kernel",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "prepared_state_policy_kernel": 1,
            "full_mirbuilder_object_method": 0,
            "mainline_selected": 0,
            "backend_behavior_changed": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
        verifier_checks=contract.verifier_checks(
            {
                "composition_plan_input": "verified",
                "execution_projection": "verified",
                "prepared_state_policy_kernel": 1,
                "full_mirbuilder_object_method": 0,
                "selector_per_candidate_attempt": 1,
                "reserved_membership_only": 1,
                "runtime_fallback": 0,
                "backend_behavior_changed": 0,
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
        ],
        transport_notes=contract.transport_notes(
            {
                "core_context_contract": "VerifiedFamilyArtifactContractV1",
                "raw_i64_truthiness": 0,
            }
        ),
        denied_boundaries=list(projection["non_claims"].keys()),
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _projection_and_oracle_text() -> tuple[str, str]:
    composition = read_json(COMPOSITION)
    projection = build_execution_projection(composition)
    oracle = build_oracle()
    validate_projection(projection, oracle)
    return stable_json(projection), stable_json(oracle)


def _ensure_projection_inputs(*, check: bool) -> None:
    projection_text, oracle_text = _projection_and_oracle_text()
    expected = [(PROJECTION, projection_text), (ORACLE, oracle_text)]
    if check:
        missing_or_changed = [
            str(path.relative_to(ROOT))
            for path, text in expected
            if not path.exists() or path.read_text() != text
        ]
        if missing_or_changed:
            raise SystemExit("generated files differ: " + ", ".join(missing_or_changed))
        return
    for path, text in expected:
        write_if_changed(path, text)


def _outputs() -> list[tuple[Path, str]]:
    projection_text, oracle_text = _projection_and_oracle_text()
    spec = prepared_state_kernel_spec()
    recipe_text = build_family_artifact_recipe_text(spec)
    verifier_text = build_family_artifact_verifier_text(spec)
    hako_text = build_family_artifact_hako_text(spec)
    manifest_text = build_family_artifact_manifest_text(
        spec,
        hako_text=hako_text,
        recipe_text=recipe_text,
        verifier_text=verifier_text,
    )
    outputs: list[tuple[Path, str]] = [
        (PROJECTION, projection_text),
        (ORACLE, oracle_text),
    ]
    if recipe_text is not None and spec.recipe_path is not None:
        outputs.append((spec.recipe_path, recipe_text))
    if verifier_text is not None and spec.verifier_path is not None:
        outputs.append((spec.verifier_path, verifier_text))
    outputs.extend(
        [
            (spec.hako_path, hako_text),
            (OUT_DIR / Path(spec.artifact_manifest).name, manifest_text),
        ]
    )
    return outputs


def run_prepared_state_kernel_generator(*, check: bool) -> None:
    _ensure_projection_inputs(check=check)
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="generated_mirbuilder_next_value_id_prepared_state_kernel=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_prepared_state_kernel_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
