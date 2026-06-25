#!/usr/bin/env python3
"""Generate the bounded metadata origin-caller merge Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-metadata-origin-caller-merge-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-metadata-origin-caller-merge-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-metadata-origin-caller-merge-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-metadata-origin-caller-merge-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderMetadataOriginCallerMergeDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module function.metadata.value_origin_callers merge",
        "vectors": [
            {
                "name": "source_entries_merge_into_cloned_function_metadata_with_source_wins",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "source": "self.metadata_ctx.value_origin_callers()",
                    "base": "function.metadata.value_origin_callers",
                    "base_entries": {"1": "base", "7": "old"},
                    "source_entries": {"2": "source", "7": "new"},
                },
                "expect": {
                    "ok": 1,
                    "merged_entries": 3,
                    "source_wins": 1,
                    "base_preserved": 1,
                    "source_preserved": 1,
                },
            }
        ],
        "non_claims": {
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderMetadataOriginCallerMergePlanV1":
        raise ValueError("wrong metadata origin-caller merge plan kind")
    if "MetadataOriginCallerMerge" not in (plan.get("available_capabilities") or []):
        raise ValueError("metadata origin-caller plan lacks MetadataOriginCallerMerge capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "source": "self.metadata_ctx.value_origin_callers()",
        "base": "function.metadata.value_origin_callers",
        "target": "function.metadata.value_origin_callers",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"metadata origin-caller profile drift: {key}")
    merge = plan.get("merge") or {}
    expected_merge = {
        "base_operation": "CloneExistingFunctionMap",
        "source_iteration": "BorrowedMetadataContextValueOriginCallersIter",
        "entry_operation": "InsertClonedValue",
        "collision_policy": "SourceWins",
        "target_assignment": "ReplaceFunctionMetadataValueOriginCallers",
    }
    for key, value in expected_merge.items():
        if merge.get(key) != value:
            raise ValueError(f"metadata origin-caller merge drift: {key}")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "function.metadata.value_origin_callers = origin_callers":
        raise ValueError("metadata origin-caller entrypoint drift")
    if result.get("minimal_path_expected_result") != "OkImplicitUnit":
        raise ValueError("metadata origin-caller minimal result drift")
    if result.get("mutates") != ["function.metadata.value_origin_callers"]:
        raise ValueError("metadata origin-caller mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"metadata origin-caller non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::metadata_origin_caller_merge",
        method_universe=("MetadataOriginCallerMerge::merge",),
        selected_method_ids=("MetadataOriginCallerMerge::merge",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "source_transport": "ValueIdOrderedMapBox",
            "target_transport": "ValueIdOrderedMapBox",
            "collision_policy": "SourceWins",
            "phi_return_type_inference": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::metadata_origin_caller_merge",
            api_name="MetadataOriginCallerMergeApi",
            pilot_scope="MetadataOriginCallerMerge_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.artifact.json",
        ),
        selected_body_count_label="metadata_origin_caller_merge_minimal_literal_profile_only",
        expected_fields=(
            "value_origin_callers",
            "metadata_origin_callers_merged",
            "source_wins",
            "base_preserved",
        ),
    )


def metadata_origin_caller_merge_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderMetadataOriginCallerMergeDerivedHakoOracleV1":
        raise ValueError("metadata origin-caller oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="MetadataOriginCallerMerge::merge",
            rust_operation="clone function.metadata.value_origin_callers, insert metadata_ctx entries, replace function metadata",
            hako_operation="CloneOwnedMap + ForEachMapEntry + ReturnValue",
            emits="MetadataOriginCallerMergeApi.merge(fn_state, metadata_ctx)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-metadata-origin-caller-merge"
        ),
        generator_version="mirbuilder-metadata-origin-caller-merge-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        extra_using_modules=["apps.lib.collections.value_id_ordered_map as ValueIdOrderedMap"],
        box=BoxSpec(name="MetadataOriginCallerMergeKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="MetadataOriginCallerFunctionShellBox",
                fields=[
                    FieldSpec(
                        name="value_origin_callers",
                        field_type="ValueIdOrderedMapBox",
                        initializer_operation={"kind": "NewValueIdOrderedMap"},
                    ),
                    FieldSpec(name="metadata_origin_callers_merged", field_type="i64", initializer="0"),
                    FieldSpec(name="source_wins", field_type="i64", initializer="0"),
                    FieldSpec(name="base_preserved", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="MetadataOriginCallerContextShellBox",
                fields=[
                    FieldSpec(
                        name="value_origin_callers",
                        field_type="ValueIdOrderedMapBox",
                        initializer_operation={"kind": "NewValueIdOrderedMap"},
                    ),
                    FieldSpec(name="source_preserved", field_type="i64", initializer="1"),
                ],
            ),
            BoxSpec(
                name="MetadataOriginCallerMergeResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="merged_entries", field_type="i64", initializer="0"),
                    FieldSpec(name="source_wins", field_type="i64", initializer="0"),
                    FieldSpec(name="base_preserved", field_type="i64", initializer="0"),
                    FieldSpec(name="phi_return_type_inference", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="MetadataOriginCallerMergeApi",
                methods=[
                    ApiMethodSpec(
                        signature="merge(fn_state, metadata_ctx): MetadataOriginCallerMergeResultBox",
                        operations=[
                            op(
                                "CloneOwnedMap",
                                source="fn_state.value_origin_callers",
                                target="merged",
                                target_storage="ValueIdOrderedMapBox",
                            ).to_json(),
                            op(
                                "ForEachMapEntry",
                                source="metadata_ctx.value_origin_callers",
                                source_storage="ValueIdOrderedMapBox",
                                key_binding="key",
                                value_binding="value",
                                body=[
                                    op("MapSet", source="merged", key="key", value="value", storage="ValueIdOrderedMapBox").to_json()
                                ],
                            ).to_json(),
                            op("SetField", target="fn_state", field="value_origin_callers", value={"expr": "merged"}).to_json(),
                            op("SetField", target="fn_state", field="metadata_origin_callers_merged", value=1).to_json(),
                            op("SetField", target="fn_state", field="source_wins", value=1).to_json(),
                            op("SetField", target="fn_state", field="base_preserved", value=1).to_json(),
                            op("NewBox", target="result", box="MetadataOriginCallerMergeResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="merged_entries", value={"expr": "merged.length()"}).to_json(),
                            op("SetField", target="result", field="source_wins", value=1).to_json(),
                            op("SetField", target="result", field="base_preserved", value=1).to_json(),
                            op("SetField", target="result", field="phi_return_type_inference", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            ),
            StaticBoxSpec(
                name="MetadataOriginCallerMergeHarnessApi",
                methods=[
                    ApiMethodSpec(
                        signature="prefill_function(ctx, value_id, caller): i64",
                        operations=[
                            op(
                                "MapSet",
                                field="value_origin_callers",
                                key="value_id",
                                value="caller",
                                storage="ValueIdOrderedMapBox",
                            ).to_json(),
                            op("ReturnI64", return_value=1).to_json(),
                        ],
                    ),
                    ApiMethodSpec(
                        signature="prefill_context(ctx, value_id, caller): i64",
                        operations=[
                            op(
                                "MapSet",
                                field="value_origin_callers",
                                key="value_id",
                                value="caller",
                                storage="ValueIdOrderedMapBox",
                            ).to_json(),
                            op("ReturnI64", return_value=1).to_json(),
                        ],
                    ),
                ],
            ),
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="MetadataOriginCallerFunctionShellBox"),
            op("NewBox", target="metadata_ctx", box="MetadataOriginCallerContextShellBox"),
            op("StaticCall", callee="MetadataOriginCallerMergeHarnessApi.prefill_function", args=["fn_state", "1", {"literal": "base"}]),
            op("StaticCall", callee="MetadataOriginCallerMergeHarnessApi.prefill_function", args=["fn_state", "7", {"literal": "old"}]),
            op("StaticCall", callee="MetadataOriginCallerMergeHarnessApi.prefill_context", args=["metadata_ctx", "2", {"literal": "source"}]),
            op("StaticCall", callee="MetadataOriginCallerMergeHarnessApi.prefill_context", args=["metadata_ctx", "7", {"literal": "new"}]),
            op("StaticCall", target="result", callee="MetadataOriginCallerMergeApi.merge", args=["fn_state", "metadata_ctx"]),
            op("AssertEq", left="result.ok", right=1, fail_message="metadata_origin_merge_ok=fail", fail_code=1),
            op("AssertEq", left="result.merged_entries", right=3, fail_message="metadata_origin_merge_entries=fail", fail_code=2),
            op("AssertEq", left="result.source_wins", right=1, fail_message="metadata_origin_merge_source_wins=fail", fail_code=3),
            op("AssertEq", left="result.base_preserved", right=1, fail_message="metadata_origin_merge_base_preserved=fail", fail_code=4),
            op("AssertEq", left="result.phi_return_type_inference", right=0, fail_message="metadata_origin_merge_phi=fail", fail_code=5),
            op("AssertEq", left="fn_state.value_origin_callers.get(1)", right={"literal": "base"}, fail_message="metadata_origin_merge_base=fail", fail_code=6),
            op("AssertEq", left="fn_state.value_origin_callers.get(2)", right={"literal": "source"}, fail_message="metadata_origin_merge_source=fail", fail_code=7),
            op("AssertEq", left="fn_state.value_origin_callers.get(7)", right={"literal": "new"}, fail_message="metadata_origin_merge_collision=fail", fail_code=8),
            op("AssertEq", left="fn_state.metadata_origin_callers_merged", right=1, fail_message="metadata_origin_merge_flag=fail", fail_code=9),
            op("StaticCall", callee="MetadataOriginCallerMergeHarnessApi.prefill_context", args=["metadata_ctx", "2", {"literal": "changed"}]),
            op("AssertEq", left="fn_state.value_origin_callers.get(2)", right={"literal": "source"}, fail_message="metadata_origin_merge_source_alias=fail", fail_code=10),
            op("Print", text="mirbuilder_metadata_origin_caller_merge_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_metadata_origin_caller_merge.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.metadata_origin_caller_merge",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "metadata_origin_caller_merge": 1,
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
                "metadata_origin_caller_merge_only": 1,
                "entrypoint": "function.metadata.value_origin_callers = origin_callers",
                "collision_policy": "SourceWins",
                "function_transport": "MirFunctionPreparedMain",
                "source": "self.metadata_ctx.value_origin_callers()",
                "target": "function.metadata.value_origin_callers",
                "minimal_path_expected_result": "OkImplicitUnit",
                "phi_return_type_inference": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "CloneOwnedMap",
            "ForEachMapEntry",
            "MapSet",
            "SetField",
            "ReturnValue",
            "MetadataOriginCallerMergeShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "source_storage_transport": "ValueIdOrderedMapBox",
                "target_storage_transport": "ValueIdOrderedMapBox",
                "collision_policy": "SourceWins",
                "phi_return_type_inference": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
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
    spec = metadata_origin_caller_merge_spec()
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


def run_metadata_origin_caller_merge_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_metadata_origin_caller_merge_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_metadata_origin_caller_merge_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
