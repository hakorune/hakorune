#!/usr/bin/env python3
"""Generate the derived Hako artifact for MirBuilder function-region stack pop."""

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
PLAN = FIXTURES / "mirbuilder-function-region-stack-pop-plan-v0.json"
PROJECTION = FIXTURES / "mirbuilder-function-region-stack-pop-execution-projection-v0.json"
ORACLE = FIXTURES / "mirbuilder-function-region-stack-pop-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-function-region-stack-pop-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json"


def _var(name: str) -> dict[str, str]:
    return {"kind": "Var", "name": name}


def _i64(value: int) -> dict[str, int | str]:
    return {"kind": "I64", "value": value}


def _eq(left: Any, right: Any) -> dict[str, Any]:
    return {"kind": "EqI64", "left": left, "right": right}


def build_execution_projection(plan: dict[str, Any]) -> dict[str, Any]:
    if plan.get("kind") != "MirBuilderFunctionRegionStackPopPlanV1":
        raise ValueError("wrong function-region stack pop plan kind")
    if "FunctionRegionStackPop" not in (plan.get("available_capabilities") or []):
        raise ValueError("function-region stack pop plan lacks FunctionRegionStackPop capability")
    return {
        "schema_version": 0,
        "kind": "FunctionRegionStackPopExecutionProjectionV1",
        "source_plan": "MirBuilderFunctionRegionStackPopPlanV1",
        "execution_scope": "PreparedRegionTraceState",
        "inputs": {
            "trace_flag_transport": "RegionTraceEnabledI64BoolV0",
            "current_region_stack_transport": "ArrayBox",
            "current_region_stack_element_transport": "RegionIdAsI64",
        },
        "methods": {
            "pop_option": "SequencePopOption",
            "apply": "IfElse + StaticCall + MethodCall + SetField + ReturnI64",
        },
        "behavior": {
            "trace_disabled": "NoOp",
            "trace_enabled": "PopAndDiscard",
            "empty_stack": "SafeNoErrorReturn",
        },
        "result_transport": "ScalarI64",
        "result_semantics": "Unit",
        "directability": {
            "prepared_trace_projection": "Allow",
            "host_env_lookup": "Deny",
            "full_metadata_context": "Deny",
        },
        "mutation_frame": {
            "current_region_stack": "exclusive",
            "trace_enabled": "read-only guard input",
            "stack_size_before": "published by apply",
            "stack_size_after": "published by apply",
            "pop_attempted": "published by apply",
        },
        "non_claims": {
            "host_env_lookup": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
        },
    }


def build_oracle() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderFunctionRegionStackPopDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module trace-gated region stack pop",
        "vectors": [
            {
                "name": "direct_pop_option_lifo",
                "inputs": {
                    "initial_stack": [10, 20],
                },
                "expect": {
                    "first_pop": 20,
                    "second_pop": 10,
                    "third_pop_is_none": 1,
                    "final_stack_size": 0,
                },
            },
            {
                "name": "trace_disabled_apply_keeps_stack",
                "inputs": {
                    "trace_enabled": 0,
                    "initial_stack": [10, 20],
                },
                "expect": {
                    "stack_size_before": 2,
                    "stack_size_after": 2,
                    "pop_attempted": 0,
                    "return_value": 0,
                },
            },
            {
                "name": "trace_enabled_apply_pops_once",
                "inputs": {
                    "trace_enabled": 1,
                    "initial_stack": [10, 20],
                },
                "expect": {
                    "stack_size_before": 2,
                    "stack_size_after": 1,
                    "pop_attempted": 1,
                    "return_value": 0,
                },
            },
            {
                "name": "trace_enabled_empty_safe",
                "inputs": {
                    "trace_enabled": 1,
                    "initial_stack": [],
                },
                "expect": {
                    "stack_size_before": 0,
                    "stack_size_after": 0,
                    "pop_attempted": 1,
                    "return_value": 0,
                },
            },
        ],
        "non_claims": {
            "host_env_lookup": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderFunctionRegionStackPopPlanV1":
        raise ValueError("wrong function-region stack pop plan kind")
    if "FunctionRegionStackPop" not in (plan.get("available_capabilities") or []):
        raise ValueError("missing FunctionRegionStackPop capability")
    result = plan.get("result_contract") or {}
    expected = {
        "entrypoint": "region::observer::pop_function_region",
        "minimal_path_expected_result": "NoErrorReturn",
    }
    for key, value in expected.items():
        if result.get(key) != value:
            raise ValueError(f"function-region stack pop result contract drift: {key}")
    pop_policy = plan.get("pop_policy") or {}
    if pop_policy.get("callsite") != "region::observer::pop_function_region(self)":
        raise ValueError("function-region stack pop callsite drift")
    if pop_policy.get("guard") != "NYASH_REGION_TRACE == 1":
        raise ValueError("function-region stack pop guard drift")
    if pop_policy.get("operation") != "metadata_ctx.pop_region":
        raise ValueError("function-region stack pop operation drift")
    if pop_policy.get("result_ignored") is not True:
        raise ValueError("function-region stack pop must discard the popped value")
    if pop_policy.get("tracing_disabled_effect") != "NoOp":
        raise ValueError("function-region stack pop trace-disabled effect drift")
    if pop_policy.get("push_counterpart_observed") is not True:
        raise ValueError("function-region stack pop push counterpart drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"function-region stack pop non-claim must remain 0: {key}")


def _validate_projection(projection: dict[str, Any], plan: dict[str, Any]) -> None:
    if projection.get("kind") != "FunctionRegionStackPopExecutionProjectionV1":
        raise ValueError("wrong execution projection kind")
    if projection.get("source_plan") != "MirBuilderFunctionRegionStackPopPlanV1":
        raise ValueError("execution projection source plan drift")
    if projection.get("execution_scope") != "PreparedRegionTraceState":
        raise ValueError("execution scope drift")
    inputs = projection.get("inputs") or {}
    if inputs.get("trace_flag_transport") != "RegionTraceEnabledI64BoolV0":
        raise ValueError("trace flag transport drift")
    if inputs.get("current_region_stack_transport") != "ArrayBox":
        raise ValueError("current region stack transport drift")
    if inputs.get("current_region_stack_element_transport") != "RegionIdAsI64":
        raise ValueError("current region stack element transport drift")
    methods = projection.get("methods") or {}
    if methods.get("pop_option") != "SequencePopOption":
        raise ValueError("pop_option lowering drift")
    if "IfElse" not in str(methods.get("apply")):
        raise ValueError("apply lowering drift")
    if projection.get("result_transport") != "ScalarI64":
        raise ValueError("result transport drift")
    if projection.get("result_semantics") != "Unit":
        raise ValueError("result semantics drift")
    directability = projection.get("directability") or {}
    if directability.get("prepared_trace_projection") != "Allow":
        raise ValueError("prepared trace projection must be allowed")
    if directability.get("host_env_lookup") != "Deny":
        raise ValueError("host env lookup must remain denied")
    if directability.get("full_metadata_context") != "Deny":
        raise ValueError("full metadata context must remain denied")
    if (projection.get("non_claims") or {}).get("slot_registry_release") != 0:
        raise ValueError("slot registry release must remain unselected in projection")
    _validate_plan(plan)


def _contract(plan: dict[str, Any], projection: dict[str, Any]) -> VerifiedFamilyArtifactContractV1:
    _validate_projection(projection, plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::function_region_stack_pop",
        method_universe=(
            "FunctionRegionStackPopApi::pop_option",
            "FunctionRegionStackPopApi::apply",
        ),
        selected_method_ids=(
            "FunctionRegionStackPopApi::pop_option",
            "FunctionRegionStackPopApi::apply",
        ),
        denials=(),
        semantic_transports={
            "trace_flag_transport": "RegionTraceEnabledI64BoolV0",
            "current_region_stack_transport": "ArrayBox",
            "current_region_stack_element_transport": "RegionIdAsI64",
            "pop_option_transport": "Option<RegionIdAsI64>",
            "apply_result_transport": "ScalarI64",
            "apply_result_semantics": "Unit",
            "host_env_lookup": 0,
            "slot_registry_release": 0,
            "metadata_publication": 0,
            "semantic_refresh": 0,
            "all_functions_phi_materialization": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::function_region_stack_pop",
            api_name="FunctionRegionStackPopApi",
            pilot_scope="FunctionRegionStackPop_prepared_region_trace_only",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.artifact.json",
        ),
        selected_body_count_label="function_region_stack_pop_prepared_region_trace_only",
        expected_fields=(
            "trace_enabled",
            "current_region_stack",
            "stack_size_before",
            "stack_size_after",
            "pop_attempted",
        ),
    )


def _apply_body_operations() -> list[dict[str, Any]]:
    return [
        op("MethodCall", target="before", receiver="state.current_region_stack", method="length", args=[]).to_json(),
        op("SetField", target="state", field="stack_size_before", value="before").to_json(),
        op("SetField", target="state", field="pop_attempted", value=0).to_json(),
        op(
            "IfElse",
            condition=_eq(_var("state.trace_enabled"), _i64(1)),
            then_body=[
                op("SetField", target="state", field="pop_attempted", value=1).to_json(),
                op("StaticCall", callee="FunctionRegionStackPopApi.pop_option", args=["state.current_region_stack"]).to_json(),
            ],
        ).to_json(),
        op("MethodCall", target="after", receiver="state.current_region_stack", method="length", args=[]).to_json(),
        op("SetField", target="state", field="stack_size_after", value="after").to_json(),
        op("ReturnI64", return_value=0).to_json(),
    ]


def function_region_stack_pop_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle = build_oracle()
    _validate_projection(projection, plan)
    contract = _contract(plan, projection)
    methods = [
        BehaviorMethodSpec(
            id="FunctionRegionStackPopApi::pop_option",
            rust_operation="trace-gated region stack pop helper",
            hako_operation="SequencePopOption",
            emits="FunctionRegionStackPopApi.pop_option(stack)",
        ),
        BehaviorMethodSpec(
            id="FunctionRegionStackPopApi::apply",
            rust_operation="MirBuilder::finalize_module trace-gated region stack pop",
            hako_operation="MethodCall + SetField + IfElse + StaticCall + ReturnI64",
            emits="FunctionRegionStackPopApi.apply(state)",
        ),
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-function-region-stack-pop"
        ),
        generator_version="mirbuilder-function-region-stack-pop-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="FunctionRegionStackPopKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="PreparedRegionTraceStateBox",
                fields=[
                    FieldSpec(name="trace_enabled", field_type="i64", initializer="0"),
                    FieldSpec(name="current_region_stack", field_type="ArrayBox", initializer="new ArrayBox()"),
                    FieldSpec(name="stack_size_before", field_type="i64", initializer="0"),
                    FieldSpec(name="stack_size_after", field_type="i64", initializer="0"),
                    FieldSpec(name="pop_attempted", field_type="i64", initializer="0"),
                ],
            )
        ],
        static_boxes=[
            StaticBoxSpec(
                name="FunctionRegionStackPopApi",
                methods=[
                    ApiMethodSpec(
                        signature="pop_option(stack): Option<i64>",
                        operations=[op("SequencePopOption", source="stack").to_json()],
                    ),
                    ApiMethodSpec(
                        signature="apply(state): i64",
                        operations=_apply_body_operations(),
                    ),
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="direct_state", box="PreparedRegionTraceStateBox"),
            op("ArrayPush", target="direct_state.current_region_stack", value=10),
            op("ArrayPush", target="direct_state.current_region_stack", value=20),
            op(
                "StaticCall",
                target="direct_pop0",
                callee="FunctionRegionStackPopApi.pop_option",
                args=["direct_state.current_region_stack"],
            ),
            op("AssertOptionSomeI64Eq", source="direct_pop0", expected=20, fail_message="function_region_stack_pop_first=fail", fail_code=1),
            op(
                "StaticCall",
                target="direct_pop1",
                callee="FunctionRegionStackPopApi.pop_option",
                args=["direct_state.current_region_stack"],
            ),
            op("AssertOptionSomeI64Eq", source="direct_pop1", expected=10, fail_message="function_region_stack_pop_second=fail", fail_code=2),
            op(
                "StaticCall",
                target="direct_pop2",
                callee="FunctionRegionStackPopApi.pop_option",
                args=["direct_state.current_region_stack"],
            ),
            op("AssertEq", left="direct_pop2", right={"expr": "Option::None()"}, fail_message="function_region_stack_pop_third_none=fail", fail_code=3),
            op("AssertEq", left="direct_state.current_region_stack.length()", right=0, fail_message="function_region_stack_pop_direct_stack_empty=fail", fail_code=4),
            op("NewBox", target="trace_disabled", box="PreparedRegionTraceStateBox"),
            op("ArrayPush", target="trace_disabled.current_region_stack", value=10),
            op("ArrayPush", target="trace_disabled.current_region_stack", value=20),
            op("SetField", target="trace_disabled", field="trace_enabled", value=0),
            op("StaticCall", target="disabled_result", callee="FunctionRegionStackPopApi.apply", args=["trace_disabled"]),
            op("AssertEq", left="disabled_result", right=0, fail_message="function_region_stack_pop_disabled_return=fail", fail_code=5),
            op("AssertEq", left="trace_disabled.stack_size_before", right=2, fail_message="function_region_stack_pop_disabled_before=fail", fail_code=6),
            op("AssertEq", left="trace_disabled.stack_size_after", right=2, fail_message="function_region_stack_pop_disabled_after=fail", fail_code=7),
            op("AssertEq", left="trace_disabled.pop_attempted", right=0, fail_message="function_region_stack_pop_disabled_attempted=fail", fail_code=8),
            op("AssertEq", left="trace_disabled.current_region_stack.length()", right=2, fail_message="function_region_stack_pop_disabled_stack=fail", fail_code=9),
            op("NewBox", target="trace_enabled", box="PreparedRegionTraceStateBox"),
            op("ArrayPush", target="trace_enabled.current_region_stack", value=10),
            op("ArrayPush", target="trace_enabled.current_region_stack", value=20),
            op("SetField", target="trace_enabled", field="trace_enabled", value=1),
            op("StaticCall", target="enabled_result", callee="FunctionRegionStackPopApi.apply", args=["trace_enabled"]),
            op("AssertEq", left="enabled_result", right=0, fail_message="function_region_stack_pop_enabled_return=fail", fail_code=10),
            op("AssertEq", left="trace_enabled.stack_size_before", right=2, fail_message="function_region_stack_pop_enabled_before=fail", fail_code=11),
            op("AssertEq", left="trace_enabled.stack_size_after", right=1, fail_message="function_region_stack_pop_enabled_after=fail", fail_code=12),
            op("AssertEq", left="trace_enabled.pop_attempted", right=1, fail_message="function_region_stack_pop_enabled_attempted=fail", fail_code=13),
            op("AssertEq", left="trace_enabled.current_region_stack.length()", right=1, fail_message="function_region_stack_pop_enabled_stack=fail", fail_code=14),
            op("NewBox", target="trace_empty", box="PreparedRegionTraceStateBox"),
            op("SetField", target="trace_empty", field="trace_enabled", value=1),
            op("StaticCall", target="empty_result", callee="FunctionRegionStackPopApi.apply", args=["trace_empty"]),
            op("AssertEq", left="empty_result", right=0, fail_message="function_region_stack_pop_empty_return=fail", fail_code=15),
            op("AssertEq", left="trace_empty.stack_size_before", right=0, fail_message="function_region_stack_pop_empty_before=fail", fail_code=16),
            op("AssertEq", left="trace_empty.stack_size_after", right=0, fail_message="function_region_stack_pop_empty_after=fail", fail_code=17),
            op("AssertEq", left="trace_empty.pop_attempted", right=1, fail_message="function_region_stack_pop_empty_attempted=fail", fail_code=18),
            op("AssertEq", left="trace_empty.current_region_stack.length()", right=0, fail_message="function_region_stack_pop_empty_stack=fail", fail_code=19),
            op("Print", text="mirbuilder_function_region_stack_pop_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_function_region_stack_pop.hako",
        facts_path=PLAN,
        plan_path=PROJECTION,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.region_stack_pop",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "function_region_stack_pop": 1,
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
                "function_region_stack_pop_only": 1,
                "trace_flag_transport": "RegionTraceEnabledI64BoolV0",
                "stack_transport": "ArrayBox",
                "stack_element_transport": "RegionIdAsI64",
                "apply_result_transport": "ScalarI64",
                "apply_result_semantics": "Unit",
                "trace_disabled_noop": 1,
                "trace_enabled_pop_once": 1,
                "trace_enabled_empty_safe": 1,
                "result_discarded": 1,
                "host_env_lookup": 0,
                "slot_registry_release": 0,
                "metadata_publication": 0,
                "semantic_refresh": 0,
                "all_functions_phi_materialization": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SequencePopOption",
            "MethodCall",
            "IfElse",
            "StaticCall",
            "SetField",
            "ReturnI64",
            "AssertOptionSomeI64Eq",
        ],
        transport_notes=contract.transport_notes(
            {
                "host_env_lookup": "unselected",
                "stack_transport": "ArrayBox",
                "stack_element_transport": "RegionIdAsI64",
                "apply_result_semantics": "Unit",
            }
        ),
        denied_boundaries=[
            "host_env_lookup",
            "slot_registry_release",
            "metadata_publication",
            "semantic_refresh",
            "all_functions_phi_materialization",
            "full_finalize_module",
            "new_backend_route",
            "new_abi",
            "runtime_fallback",
        ],
        extra_manifest_fields=contract.manifest_extra_fields(),
    )


def _outputs() -> list[tuple[Path, str]]:
    plan = read_json(PLAN)
    projection = build_execution_projection(plan)
    oracle_text = stable_json(build_oracle())
    projection_text = stable_json(projection)
    if not ORACLE.exists():
        raise FileNotFoundError(f"{ORACLE} must be written before manifest hashing")
    if not PROJECTION.exists():
        raise FileNotFoundError(f"{PROJECTION} must be written before manifest hashing")
    spec = function_region_stack_pop_spec()
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


def run_function_region_stack_pop_artifact_generator(*, check: bool) -> None:
    if not check:
        plan = read_json(PLAN)
        write_if_changed(PROJECTION, stable_json(build_execution_projection(plan)))
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_function_region_stack_pop_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_function_region_stack_pop_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
