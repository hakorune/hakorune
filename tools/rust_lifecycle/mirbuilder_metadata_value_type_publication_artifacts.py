#!/usr/bin/env python3
"""Generate the bounded metadata value-type publication Hako artifact."""

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
PLAN = FIXTURES / "mirbuilder-metadata-value-type-publication-plan-v0.json"
ORACLE = FIXTURES / "mirbuilder-metadata-value-type-publication-derived-hako-oracle-v0.json"
RECIPE = FIXTURES / "mirbuilder-metadata-value-type-publication-derived-hako-recipe-v0.json"
VERIFIER = FIXTURES / "mirbuilder-metadata-value-type-publication-derived-hako-verifier-result-v0.json"


def build_oracle() -> dict[str, object]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderMetadataValueTypePublicationDerivedHakoOracleV1",
        "subject": "MirBuilder::finalize_module function.metadata.value_types publication",
        "vectors": [
            {
                "name": "minimal_literal_value_types_are_cloned_into_function_metadata",
                "inputs": {
                    "function_transport": "MirFunctionPreparedMain",
                    "value_types_source": "self.type_ctx.value_types",
                    "metadata_target": "function.metadata.value_types",
                    "source_value_type_entries": 4,
                },
                "expect": {
                    "ok": 1,
                    "source_entries": 4,
                    "published_entries": 4,
                    "clone_owned": 1,
                    "source_preserved": 1,
                },
            }
        ],
        "non_claims": {
            "metadata_origin_caller_merge": 0,
            "phi_return_type_inference": 0,
            "phi_input_materialization": 0,
            "module_function_insertion": 0,
            "full_finalize_module": 0,
            "mainline_selected": 0,
            "runtime_fallback": 0,
        },
    }


def _validate_plan(plan: dict[str, object]) -> None:
    if plan.get("kind") != "MirBuilderMetadataValueTypePublicationPlanV1":
        raise ValueError("wrong metadata value-type publication plan kind")
    if "MetadataValueTypePublication" not in (plan.get("available_capabilities") or []):
        raise ValueError("metadata publication plan lacks MetadataValueTypePublication capability")
    profile = plan.get("execution_profile") or {}
    expected_profile = {
        "function_transport": "MirFunctionPreparedMain",
        "value_types_source": "self.type_ctx.value_types",
        "metadata_target": "function.metadata.value_types",
    }
    for key, value in expected_profile.items():
        if profile.get(key) != value:
            raise ValueError(f"metadata publication profile drift: {key}")
    publication = plan.get("publication") or {}
    if publication.get("operation") != "CloneOwnedMap":
        raise ValueError("metadata publication operation drift")
    if publication.get("timing") != "AfterTypeHintProvisionBeforeOriginCallerMerge":
        raise ValueError("metadata publication timing drift")
    result = plan.get("result_contract") or {}
    if result.get("entrypoint") != "function.metadata.value_types = self.type_ctx.value_types.clone()":
        raise ValueError("metadata publication entrypoint drift")
    if result.get("minimal_path_expected_result") != "OkImplicitUnit":
        raise ValueError("metadata publication minimal result drift")
    if result.get("mutates") != ["function.metadata.value_types"]:
        raise ValueError("metadata publication mutation frame drift")
    for key, value in (plan.get("non_claims") or {}).items():
        if value != 0:
            raise ValueError(f"metadata publication non-claim must remain 0: {key}")


def _contract(plan: dict[str, object]) -> VerifiedFamilyArtifactContractV1:
    _validate_plan(plan)
    return VerifiedFamilyArtifactContractV1(
        family_id="hakorune_mir_builder::metadata_value_type_publication",
        method_universe=("MetadataValueTypePublication::publish",),
        selected_method_ids=("MetadataValueTypePublication::publish",),
        denials=(),
        semantic_transports={
            "function_transport": "MirFunctionPreparedMain",
            "value_types_source": "TypeContextValueTypesShell",
            "metadata_target": "FunctionMetadataValueTypesShell",
            "publication_operation": "CloneOwnedMap",
            "metadata_origin_caller_merge": 0,
            "full_finalize_module": 0,
        },
        artifact=ArtifactIdentity(
            family_id="hakorune_mir_builder::metadata_value_type_publication",
            api_name="MetadataValueTypePublicationApi",
            pilot_scope="MetadataValueTypePublication_minimal_literal_profile",
            artifact_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.hako",
            manifest_path="lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.artifact.json",
        ),
        selected_body_count_label="metadata_value_type_publication_minimal_literal_profile_only",
        expected_fields=(
            "value_types_source_entries",
            "metadata_value_types_entries",
            "clone_owned_publication",
            "source_preserved",
        ),
    )


def metadata_value_type_publication_spec() -> FamilyArtifactSpec:
    plan = read_json(PLAN)
    _validate_plan(plan)
    oracle = build_oracle()
    if oracle.get("kind") != "MirBuilderMetadataValueTypePublicationDerivedHakoOracleV1":
        raise ValueError("metadata publication oracle kind drift")
    contract = _contract(plan)
    methods = [
        BehaviorMethodSpec(
            id="MetadataValueTypePublication::publish",
            rust_operation="function.metadata.value_types = self.type_ctx.value_types.clone()",
            hako_operation="CloneOwnedMap publication shell + ReturnValue",
            emits="MetadataValueTypePublicationApi.publish(fn_state, type_ctx)",
        )
    ]
    contract.require_selected_recipe_methods([method.id for method in methods])
    return FamilyArtifactSpec(
        root=ROOT,
        generated_by=(
            "tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py "
            "--family mirbuilder-metadata-value-type-publication"
        ),
        generator_version="mirbuilder-metadata-value-type-publication-derived-hako-v0",
        artifact_manifest=contract.artifact.manifest_path,
        family_comment=contract.family_id,
        using_module="",
        box=BoxSpec(name="MetadataValueTypePublicationKernel", fields=[]),
        additional_boxes=[
            BoxSpec(
                name="MetadataValueTypeFunctionShellBox",
                fields=[
                    FieldSpec(name="metadata_value_types_entries", field_type="i64", initializer="0"),
                    FieldSpec(name="metadata_value_types_mutated", field_type="i64", initializer="0"),
                    FieldSpec(name="metadata_origin_callers_mutated", field_type="i64", initializer="0"),
                ],
            ),
            BoxSpec(
                name="MetadataValueTypeContextShellBox",
                fields=[
                    FieldSpec(name="value_types_source_entries", field_type="i64", initializer="4"),
                    FieldSpec(name="value_types_source_preserved", field_type="i64", initializer="1"),
                ],
            ),
            BoxSpec(
                name="MetadataValueTypePublicationResultBox",
                fields=[
                    FieldSpec(name="ok", field_type="i64", initializer="0"),
                    FieldSpec(name="published_entries", field_type="i64", initializer="0"),
                    FieldSpec(name="clone_owned", field_type="i64", initializer="0"),
                    FieldSpec(name="source_preserved", field_type="i64", initializer="0"),
                    FieldSpec(name="origin_caller_merge", field_type="i64", initializer="0"),
                ],
            ),
        ],
        static_boxes=[
            StaticBoxSpec(
                name="MetadataValueTypePublicationApi",
                methods=[
                    ApiMethodSpec(
                        signature="publish(fn_state, type_ctx): MetadataValueTypePublicationResultBox",
                        operations=[
                            op("SetField", target="fn_state", field="metadata_value_types_entries", value={"expr": "type_ctx.value_types_source_entries"}).to_json(),
                            op("SetField", target="fn_state", field="metadata_value_types_mutated", value=1).to_json(),
                            op("SetField", target="fn_state", field="metadata_origin_callers_mutated", value=0).to_json(),
                            op("NewBox", target="result", box="MetadataValueTypePublicationResultBox").to_json(),
                            op("SetField", target="result", field="ok", value=1).to_json(),
                            op("SetField", target="result", field="published_entries", value={"expr": "type_ctx.value_types_source_entries"}).to_json(),
                            op("SetField", target="result", field="clone_owned", value=1).to_json(),
                            op("SetField", target="result", field="source_preserved", value={"expr": "type_ctx.value_types_source_preserved"}).to_json(),
                            op("SetField", target="result", field="origin_caller_merge", value=0).to_json(),
                            op("ReturnValue", value="result").to_json(),
                        ],
                    )
                ],
            )
        ],
        main_operations=[
            op("NewBox", target="fn_state", box="MetadataValueTypeFunctionShellBox"),
            op("NewBox", target="type_ctx", box="MetadataValueTypeContextShellBox"),
            op("StaticCall", target="result", callee="MetadataValueTypePublicationApi.publish", args=["fn_state", "type_ctx"]),
            op("AssertEq", left="result.ok", right=1, fail_message="metadata_value_type_ok=fail", fail_code=1),
            op("AssertEq", left="result.published_entries", right=4, fail_message="metadata_value_type_entries=fail", fail_code=2),
            op("AssertEq", left="result.clone_owned", right=1, fail_message="metadata_value_type_clone=fail", fail_code=3),
            op("AssertEq", left="result.source_preserved", right=1, fail_message="metadata_value_type_source=fail", fail_code=4),
            op("AssertEq", left="result.origin_caller_merge", right=0, fail_message="metadata_value_type_origin_merge=fail", fail_code=5),
            op("AssertEq", left="fn_state.metadata_value_types_entries", right=4, fail_message="metadata_value_type_fn_entries=fail", fail_code=6),
            op("AssertEq", left="fn_state.metadata_value_types_mutated", right=1, fail_message="metadata_value_type_mutation=fail", fail_code=7),
            op("AssertEq", left="fn_state.metadata_origin_callers_mutated", right=0, fail_message="metadata_value_type_origin_mutation=fail", fail_code=8),
            op("Print", text="mirbuilder_metadata_value_type_publication_derived_hako=ok"),
            op("ReturnI64", return_value=0),
        ],
        family_id=contract.family_id,
        state="DerivedShadow",
        source_rust_file=SOURCE,
        hako_path=OUT_DIR / "mirbuilder_metadata_value_type_publication.hako",
        facts_path=PLAN,
        plan_path=PLAN,
        oracle_path=ORACLE,
        recipe_path=RECIPE,
        verifier_path=VERIFIER,
        pilot_scope=contract.artifact.pilot_scope,
        recipe_subject="hakorune_mir_builder::MirBuilder.finalize_module.metadata_value_type_publication",
        selected_body_count=contract.selected_body_count_label,
        methods=methods,
        claims={
            "generated_hako_manual_edit": 0,
            "metadata_value_type_publication": 1,
            "metadata_origin_caller_merge": 0,
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
                "metadata_value_type_publication_only": 1,
                "entrypoint": "function.metadata.value_types = self.type_ctx.value_types.clone()",
                "publication_operation": "CloneOwnedMap",
                "function_transport": "MirFunctionPreparedMain",
                "value_types_source": "self.type_ctx.value_types",
                "metadata_target": "function.metadata.value_types",
                "minimal_path_expected_result": "OkImplicitUnit",
                "metadata_origin_caller_merge": 0,
                "full_finalize_module": 0,
                "runtime_fallback": 0,
            }
        ),
        verified_operations=[
            "SetField",
            "ReturnValue",
            "MetadataValueTypePublicationShell",
        ],
        transport_notes=contract.transport_notes(
            {
                "metadata_origin_caller_merge": "unselected",
                "phi_return_type_inference": "unselected",
                "full_finalize_module": 0,
            }
        ),
        denied_boundaries=[
            "metadata_origin_caller_merge",
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
    spec = metadata_value_type_publication_spec()
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


def run_metadata_value_type_publication_artifact_generator(*, check: bool) -> None:
    if not check:
        write_if_changed(ORACLE, stable_json(build_oracle()))
    run_family_generator(
        check=check,
        root=ROOT,
        unchanged_label="mirbuilder_metadata_value_type_publication_artifact=unchanged",
        outputs_factory=_outputs,
    )


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_metadata_value_type_publication_artifact_generator(check=args.check)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
