#!/usr/bin/env python3
"""Derive the first unsupported edge for the minimal MirBuilder path.

This is an analysis-only selector. It does not generate Hako, routes, backend
code, or runtime behavior. The plan records live source order and required
capabilities; the frontier result is derived from explicit artifact contracts.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

BUILDER_BUILD = ROOT / "src/mir/builder/builder_build.rs"
MODULE_LIFECYCLE = ROOT / "src/mir/builder/module_lifecycle.rs"

BUNDLE_MANIFEST = (
    ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.artifact.json"
)
CORE_CONTEXT_MANIFEST = (
    ROOT / "lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json"
)
PREPARED_KERNEL_MANIFEST = (
    ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/"
    "mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
)
MODULE_SHELL_PLAN = (
    FIXTURES / "mir-module-minimal-shell-transport-plan-v0.json"
)
FUNCTION_CONSTRUCTOR_PLAN = (
    FIXTURES / "mir-function-constructor-composition-plan-v0.json"
)
LITERAL_INTEGER_PLAN = (
    FIXTURES / "mirbuilder-literal-integer-lowering-plan-v0.json"
)
BOUNDED_FINALIZE_PLAN = (
    FIXTURES / "mirbuilder-bounded-finalize-composition-plan-v0.json"
)
RETURN_EMISSION_PLAN = (
    FIXTURES / "mirbuilder-return-emission-plan-v0.json"
)
RETURN_TYPE_PUBLICATION_PLAN = (
    FIXTURES / "mirbuilder-return-type-publication-plan-v0.json"
)
CURRENT_MODULE_TAKE_PLAN = (
    FIXTURES / "mirbuilder-current-module-take-plan-v0.json"
)
TYPED_VALUE_VERIFICATION_PLAN = (
    FIXTURES / "mirbuilder-typed-value-verification-plan-v0.json"
)
CURRENT_FUNCTION_TAKE_PLAN = (
    FIXTURES / "mirbuilder-current-function-take-plan-v0.json"
)
TYPE_PROPAGATION_PIPELINE_PLAN = (
    FIXTURES / "mirbuilder-type-propagation-pipeline-plan-v0.json"
)
TYPE_HINT_PROVISION_PLAN = (
    FIXTURES / "mirbuilder-type-hint-provision-plan-v0.json"
)
METADATA_VALUE_TYPE_PUBLICATION_PLAN = (
    FIXTURES / "mirbuilder-metadata-value-type-publication-plan-v0.json"
)
METADATA_ORIGIN_CALLER_MERGE_PLAN = (
    FIXTURES / "mirbuilder-metadata-origin-caller-merge-plan-v0.json"
)
PHI_RETURN_TYPE_INFERENCE_PLAN = (
    FIXTURES / "mirbuilder-phi-return-type-inference-plan-v0.json"
)
PHI_INPUT_MATERIALIZATION_PLAN = (
    FIXTURES / "mirbuilder-phi-input-materialization-plan-v0.json"
)
DEV_BIRTH_VERIFICATION_PLAN = (
    FIXTURES / "mirbuilder-dev-birth-verification-plan-v0.json"
)
MODULE_FUNCTION_INSERTION_PLAN = (
    FIXTURES / "mirbuilder-module-function-insertion-plan-v0.json"
)
CONDITION_FN_INJECTION_PLAN = (
    FIXTURES / "mirbuilder-condition-fn-injection-plan-v0.json"
)
FUNCTION_REGION_STACK_POP_PLAN = (
    FIXTURES / "mirbuilder-function-region-stack-pop-plan-v0.json"
)
SLOT_REGISTRY_RELEASE_PLAN = (
    FIXTURES / "mirbuilder-slot-registry-release-plan-v0.json"
)
MODULE_METADATA_PUBLICATION_PLAN = (
    FIXTURES / "mirbuilder-module-metadata-publication-plan-v0.json"
)
RECORD_PACKED_LAYOUT_REFRESH_PLAN = (
    FIXTURES / "mirbuilder-record-packed-layout-refresh-plan-v0.json"
)
TYPED_OBJECT_PLAN_REFRESH_PLAN = (
    FIXTURES / "mirbuilder-typed-object-plan-refresh-plan-v0.json"
)
DIRECT_STATE_PLAN_REFRESH_PLAN = (
    FIXTURES / "mirbuilder-direct-state-plan-refresh-plan-v0.json"
)
ALL_FUNCTIONS_PHI_MATERIALIZATION_PLAN = (
    FIXTURES / "mirbuilder-all-functions-phi-materialization-plan-v0.json"
)
MINIMAL_SMOKE_RESULT = (
    FIXTURES / "mirbuilder-minimal-execution-path-smoke-result-v0.json"
)
MAINLINE_ROUTE = (
    ROOT
    / "lang/generated/rust_derived/hakorune_mir_builder/"
    "mirbuilder_next_value_id_prepared_state_kernel.route.json"
)

PLAN_PATH = FIXTURES / "minimal-mirbuilder-execution-path-plan-v0.json"
RESULT_PATH = FIXTURES / "minimal-mirbuilder-first-red-edge-result-v0.json"


class SelectionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def stable_json(data: dict[str, Any]) -> str:
    return json.dumps(data, ensure_ascii=False, indent=2, sort_keys=True) + "\n"


def require_order(text: str, needles: list[str], label: str) -> list[dict[str, Any]]:
    cursor = -1
    observed: list[dict[str, Any]] = []
    for needle in needles:
        idx = text.find(needle, cursor + 1)
        if idx < 0:
            raise SelectionError(f"{label}: missing or out-of-order source marker: {needle}")
        observed.append({"marker": needle, "byte_offset": idx})
        cursor = idx
    return observed


def extract_source_order_facts() -> dict[str, Any]:
    build_text = BUILDER_BUILD.read_text()
    lifecycle_text = MODULE_LIFECYCLE.read_text()

    build_order = require_order(
        build_text,
        [
            "self.prepare_module()?;",
            "let result_value = self.lower_root(ast)?;",
            "self.finalize_module(result_value)",
        ],
        "MirBuilder::build_module",
    )
    prepare_order = require_order(
        lifecycle_text,
        [
            'let mut module = MirModule::new("main".to_string());',
            "module.metadata.source_file = self.current_source_file();",
            "let entry_block = self.next_block_id();",
            "let mut main_function = self.new_function_with_metadata(main_signature, entry_block);",
            "self.current_module = Some(module);",
            "self.scope_ctx.current_function = Some(main_function);",
            "self.current_block = Some(entry_block);",
        ],
        "MirBuilder::prepare_module",
    )

    return {
        "kind": "MirBuilderMinimalPathSourceOrderFactsV1",
        "source_files": [
            {"path": rel(BUILDER_BUILD), "sha256": sha256_text(build_text)},
            {"path": rel(MODULE_LIFECYCLE), "sha256": sha256_text(lifecycle_text)},
        ],
        "build_module_order": build_order,
        "prepare_module_order": prepare_order,
    }


def contract_sources() -> list[dict[str, Any]]:
    bundle = read_json(BUNDLE_MANIFEST)
    core = read_json(CORE_CONTEXT_MANIFEST)
    prepared = read_json(PREPARED_KERNEL_MANIFEST)
    module_shell = read_json(MODULE_SHELL_PLAN)
    function_constructor = read_json(FUNCTION_CONSTRUCTOR_PLAN)
    literal_integer = read_json(LITERAL_INTEGER_PLAN)
    bounded_finalize = read_json(BOUNDED_FINALIZE_PLAN)
    return_emission = read_json(RETURN_EMISSION_PLAN)
    return_type_publication = read_json(RETURN_TYPE_PUBLICATION_PLAN)
    current_module_take = read_json(CURRENT_MODULE_TAKE_PLAN)
    typed_value_verification = read_json(TYPED_VALUE_VERIFICATION_PLAN)
    current_function_take = read_json(CURRENT_FUNCTION_TAKE_PLAN)
    type_propagation_pipeline = read_json(TYPE_PROPAGATION_PIPELINE_PLAN)
    type_hint_provision = read_json(TYPE_HINT_PROVISION_PLAN)
    metadata_value_type_publication = read_json(METADATA_VALUE_TYPE_PUBLICATION_PLAN)
    metadata_origin_caller_merge = read_json(METADATA_ORIGIN_CALLER_MERGE_PLAN)
    phi_return_type_inference = read_json(PHI_RETURN_TYPE_INFERENCE_PLAN)
    phi_input_materialization = read_json(PHI_INPUT_MATERIALIZATION_PLAN)
    dev_birth_verification = read_json(DEV_BIRTH_VERIFICATION_PLAN)
    module_function_insertion = read_json(MODULE_FUNCTION_INSERTION_PLAN)
    condition_fn_injection = read_json(CONDITION_FN_INJECTION_PLAN)
    function_region_stack_pop = read_json(FUNCTION_REGION_STACK_POP_PLAN)
    slot_registry_release = read_json(SLOT_REGISTRY_RELEASE_PLAN)
    module_metadata_publication = read_json(MODULE_METADATA_PUBLICATION_PLAN)
    record_packed_layout_refresh = read_json(RECORD_PACKED_LAYOUT_REFRESH_PLAN)
    typed_object_plan_refresh = read_json(TYPED_OBJECT_PLAN_REFRESH_PLAN)
    direct_state_plan_refresh = read_json(DIRECT_STATE_PLAN_REFRESH_PLAN)
    all_functions_phi_materialization = read_json(ALL_FUNCTIONS_PHI_MATERIALIZATION_PLAN)
    minimal_smoke = read_json(MINIMAL_SMOKE_RESULT)
    mainline_route = read_json(MAINLINE_ROUTE)

    if bundle.get("bundle_contract_model") != "membership_only_v1":
        raise SelectionError("ordered_map bundle is not membership_only_v1")
    bundle_members = set(bundle.get("bundle_members") or [])
    if "mirbuilder_next_value_id_prepared_state_kernel" not in bundle_members:
        raise SelectionError("bundle does not include prepared-state allocation kernel")
    exercised = set(bundle.get("exercised_capabilities") or [])
    required_exercised = {
        "CoreContext.scalar_counters_and_id_generators",
        "MirBuilderAllocationPolicy.prepared_state_next_value_id",
    }
    missing = sorted(required_exercised - exercised)
    if missing:
        raise SelectionError(f"bundle lacks exercised capabilities: {missing}")

    core_claims = core.get("claims") or {}
    if core_claims.get("core_context_full_claim") != 0:
        raise SelectionError("CoreContext manifest claims full CoreContext conversion")
    if core_claims.get("mirbuilder_wide_claim") != 0:
        raise SelectionError("CoreContext manifest claims wide MirBuilder conversion")
    if core_claims.get("source_selfhost_claim") != 0:
        raise SelectionError("CoreContext manifest claims source selfhost")

    prepared_claims = prepared.get("claims") or {}
    if prepared_claims.get("prepared_state_policy_kernel") != 1:
        raise SelectionError("prepared-state kernel manifest lacks policy-kernel claim")
    if prepared_claims.get("full_mirbuilder_object_method") != 0:
        raise SelectionError("prepared-state kernel claims full MirBuilder object method")
    if module_shell.get("kind") != "MirModuleMinimalShellTransportPlanV1":
        raise SelectionError("module shell transport plan has wrong kind")
    if module_shell.get("directability", {}).get("capability") != "MirModuleMinimalShellTransport":
        raise SelectionError("module shell plan does not provide MirModuleMinimalShellTransport")
    if module_shell.get("non_claims", {}).get("source_file_assignment") != 0:
        raise SelectionError("module shell plan must not claim source_file assignment")
    if function_constructor.get("kind") != "MirFunctionConstructorCompositionPlanV1":
        raise SelectionError("function constructor plan has wrong kind")
    function_caps = set(function_constructor.get("available_capabilities") or [])
    for capability in ["MirFunctionConstructorTransport", "PreparedStateInstall"]:
        if capability not in function_caps:
            raise SelectionError(f"function constructor plan lacks capability: {capability}")
    if function_constructor.get("non_claims", {}).get("separate_block_only_claim") != 0:
        raise SelectionError("function constructor plan must not split block-only claim")
    if literal_integer.get("kind") != "MirBuilderLiteralIntegerLoweringPlanV1":
        raise SelectionError("literal integer lowering plan has wrong kind")
    literal_caps = set(literal_integer.get("available_capabilities") or [])
    if "LiteralIntegerLowering" not in literal_caps:
        raise SelectionError("literal integer plan lacks LiteralIntegerLowering")
    literal_non_claims = literal_integer.get("non_claims") or {}
    if literal_non_claims.get("return_emission") != 0:
        raise SelectionError("literal integer plan must not claim return emission")
    if literal_non_claims.get("generated_hako_artifact") != 0:
        raise SelectionError("literal integer plan must not claim generated Hako")
    if bounded_finalize.get("kind") != "MirBuilderBoundedFinalizeCompositionPlanV1":
        raise SelectionError("bounded finalize composition plan has wrong kind")
    finalize_caps = set(bounded_finalize.get("available_capabilities") or [])
    if "FinalizeModuleComposition" not in finalize_caps:
        raise SelectionError("bounded finalize plan lacks FinalizeModuleComposition")
    finalize_non_claims = bounded_finalize.get("non_claims") or {}
    if finalize_non_claims.get("full_finalize_module") != 0:
        raise SelectionError("bounded finalize plan must not claim full finalize")
    if finalize_non_claims.get("generated_hako_artifact") != 0:
        raise SelectionError("bounded finalize plan must not claim generated Hako")
    if return_emission.get("kind") != "MirBuilderReturnEmissionPlanV1":
        raise SelectionError("return emission plan has wrong kind")
    return_caps = set(return_emission.get("available_capabilities") or [])
    if "ReturnEmission" not in return_caps:
        raise SelectionError("return emission plan lacks ReturnEmission")
    return_non_claims = return_emission.get("non_claims") or {}
    if return_non_claims.get("return_type_publication") != 0:
        raise SelectionError("return emission plan must not claim return type publication")
    if return_non_claims.get("full_finalize_module") != 0:
        raise SelectionError("return emission plan must not claim full finalize")
    if return_non_claims.get("generated_hako_artifact") != 0:
        raise SelectionError("return emission plan must not claim generated Hako")
    if return_type_publication.get("kind") != "MirBuilderReturnTypePublicationPlanV1":
        raise SelectionError("return type publication plan has wrong kind")
    return_type_caps = set(return_type_publication.get("available_capabilities") or [])
    if "ReturnTypePublication" not in return_type_caps:
        raise SelectionError("return type publication plan lacks ReturnTypePublication")
    return_type_non_claims = return_type_publication.get("non_claims") or {}
    if return_type_non_claims.get("module_take") != 0:
        raise SelectionError("return type publication plan must not claim module take")
    if return_type_non_claims.get("full_finalize_module") != 0:
        raise SelectionError("return type publication plan must not claim full finalize")
    if return_type_non_claims.get("generated_hako_artifact") != 0:
        raise SelectionError("return type publication plan must not claim generated Hako")
    if current_module_take.get("kind") != "MirBuilderCurrentModuleTakePlanV1":
        raise SelectionError("current module take plan has wrong kind")
    current_module_caps = set(current_module_take.get("available_capabilities") or [])
    if "CurrentModuleTake" not in current_module_caps:
        raise SelectionError("current module take plan lacks CurrentModuleTake")
    current_module_contract = current_module_take.get("result_contract") or {}
    if current_module_contract.get("taken_value") != "MirModuleMinimalShell":
        raise SelectionError("current module take must transport MirModuleMinimalShell")
    current_module_non_claims = current_module_take.get("non_claims") or {}
    for key in [
        "verify_typed_values",
        "current_function_take",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if current_module_non_claims.get(key) != 0:
            raise SelectionError(f"current module take plan must keep {key}=0")
    if typed_value_verification.get("kind") != "MirBuilderTypedValueVerificationPlanV1":
        raise SelectionError("typed-value verification plan has wrong kind")
    typed_value_caps = set(typed_value_verification.get("available_capabilities") or [])
    if "TypedValueDefinitionVerification" not in typed_value_caps:
        raise SelectionError("typed-value verification plan lacks TypedValueDefinitionVerification")
    typed_value_contract = typed_value_verification.get("verification_contract") or {}
    if typed_value_contract.get("typed_values") != "builder.type_ctx.value_types":
        raise SelectionError("typed-value verification plan has wrong typed_values source")
    if typed_value_contract.get("definition_sources") != ["compute_def_blocks(func)", "func.params"]:
        raise SelectionError("typed-value verification plan has wrong definition sources")
    typed_value_non_claims = typed_value_verification.get("non_claims") or {}
    for key in [
        "current_function_take",
        "type_propagation",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if typed_value_non_claims.get(key) != 0:
            raise SelectionError(f"typed-value verification plan must keep {key}=0")
    if current_function_take.get("kind") != "MirBuilderCurrentFunctionTakePlanV1":
        raise SelectionError("current function take plan has wrong kind")
    current_function_caps = set(current_function_take.get("available_capabilities") or [])
    if "CurrentFunctionTake" not in current_function_caps:
        raise SelectionError("current function take plan lacks CurrentFunctionTake")
    current_function_contract = current_function_take.get("result_contract") or {}
    if current_function_contract.get("taken_value") != "MirFunctionPreparedMain":
        raise SelectionError("current function take must transport MirFunctionPreparedMain")
    current_function_non_claims = current_function_take.get("non_claims") or {}
    for key in [
        "type_propagation",
        "type_hint_provision",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if current_function_non_claims.get(key) != 0:
            raise SelectionError(f"current function take plan must keep {key}=0")
    if type_propagation_pipeline.get("kind") != "MirBuilderTypePropagationPipelinePlanV1":
        raise SelectionError("type propagation pipeline plan has wrong kind")
    type_propagation_caps = set(type_propagation_pipeline.get("available_capabilities") or [])
    if "TypePropagationPipelineExecution" not in type_propagation_caps:
        raise SelectionError("type propagation plan lacks TypePropagationPipelineExecution")
    if type_propagation_pipeline.get("pipeline_steps") != [
        "seed_declared_field_types",
        "copy_propagation_initial",
        "binop_repropagation",
        "copy_propagation_after_binop",
        "phi_type_inference",
    ]:
        raise SelectionError("type propagation pipeline step order drift")
    type_propagation_non_claims = type_propagation_pipeline.get("non_claims") or {}
    for key in [
        "type_hint_provision",
        "metadata_value_type_publication",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if type_propagation_non_claims.get(key) != 0:
            raise SelectionError(f"type propagation pipeline plan must keep {key}=0")
    if type_hint_provision.get("kind") != "MirBuilderTypeHintProvisionPlanV1":
        raise SelectionError("type hint provision plan has wrong kind")
    type_hint_caps = set(type_hint_provision.get("available_capabilities") or [])
    if "TypeHintProvision" not in type_hint_caps:
        raise SelectionError("type hint provision plan lacks TypeHintProvision")
    type_hint_contract = type_hint_provision.get("result_contract") or {}
    if (
        type_hint_contract.get("entrypoint")
        != "type_hint_providers::annotate_missing_result_types_from_calls_and_await"
    ):
        raise SelectionError("type hint provision entrypoint drift")
    if [
        case.get("instruction") for case in type_hint_provision.get("provider_cases") or []
    ] != [
        "Await",
        "Call(Global)",
        "Call(Constructor)",
        "Call(OtherOrMissingCallee)",
    ]:
        raise SelectionError("type hint provision provider case order drift")
    type_hint_non_claims = type_hint_provision.get("non_claims") or {}
    for key in [
        "metadata_value_type_publication",
        "metadata_origin_caller_merge",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if type_hint_non_claims.get(key) != 0:
            raise SelectionError(f"type hint provision plan must keep {key}=0")
    if (
        metadata_value_type_publication.get("kind")
        != "MirBuilderMetadataValueTypePublicationPlanV1"
    ):
        raise SelectionError("metadata value-type publication plan has wrong kind")
    metadata_value_caps = set(
        metadata_value_type_publication.get("available_capabilities") or []
    )
    if "MetadataValueTypePublication" not in metadata_value_caps:
        raise SelectionError(
            "metadata value-type publication plan lacks MetadataValueTypePublication"
        )
    publication = metadata_value_type_publication.get("publication") or {}
    if publication.get("operation") != "CloneOwnedMap":
        raise SelectionError("metadata value-type publication operation drift")
    if publication.get("timing") != "AfterTypeHintProvisionBeforeOriginCallerMerge":
        raise SelectionError("metadata value-type publication timing drift")
    metadata_value_non_claims = metadata_value_type_publication.get("non_claims") or {}
    for key in [
        "metadata_origin_caller_merge",
        "phi_return_type_inference",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if metadata_value_non_claims.get(key) != 0:
            raise SelectionError(f"metadata value-type publication plan must keep {key}=0")
    if (
        metadata_origin_caller_merge.get("kind")
        != "MirBuilderMetadataOriginCallerMergePlanV1"
    ):
        raise SelectionError("metadata origin-caller merge plan has wrong kind")
    metadata_origin_caps = set(metadata_origin_caller_merge.get("available_capabilities") or [])
    if "MetadataOriginCallerMerge" not in metadata_origin_caps:
        raise SelectionError("metadata origin-caller merge plan lacks MetadataOriginCallerMerge")
    merge = metadata_origin_caller_merge.get("merge") or {}
    if merge.get("collision_policy") != "SourceWins":
        raise SelectionError("metadata origin-caller merge collision policy drift")
    metadata_origin_non_claims = metadata_origin_caller_merge.get("non_claims") or {}
    for key in [
        "phi_return_type_inference",
        "phi_input_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if metadata_origin_non_claims.get(key) != 0:
            raise SelectionError(f"metadata origin-caller merge plan must keep {key}=0")
    if phi_return_type_inference.get("kind") != "MirBuilderPhiReturnTypeInferencePlanV1":
        raise SelectionError("PHI return-type inference plan has wrong kind")
    phi_return_caps = set(phi_return_type_inference.get("available_capabilities") or [])
    if "PhiReturnTypeInference" not in phi_return_caps:
        raise SelectionError("PHI return-type inference plan lacks PhiReturnTypeInference")
    if phi_return_type_inference.get("resolver_chain") != [
        "SkipConcreteReturnType",
        "TerminatorReturnOnly",
        "DirectValueTypesLookup",
        "TypeHintPolicyExtract",
        "MethodReturnHintBox",
        "PhiTypeResolver",
        "GenericTypeResolver",
        "UnknownFallbackOutsideDebug",
    ]:
        raise SelectionError("PHI return-type inference resolver chain drift")
    phi_return_non_claims = phi_return_type_inference.get("non_claims") or {}
    for key in [
        "phi_input_materialization",
        "module_function_insertion",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if phi_return_non_claims.get(key) != 0:
            raise SelectionError(f"PHI return-type inference plan must keep {key}=0")
    if phi_input_materialization.get("kind") != "MirBuilderPhiInputMaterializationPlanV1":
        raise SelectionError("PHI input materialization plan has wrong kind")
    phi_input_caps = set(phi_input_materialization.get("available_capabilities") or [])
    if "PhiInputMaterialization" not in phi_input_caps:
        raise SelectionError("PHI input materialization plan lacks PhiInputMaterialization")
    if phi_input_materialization.get("materialization_steps") != [
        "PruneUnusedPhiInstructions",
        "CompleteMissingSelfCarriedPhiInputs",
        "CollectPhiInputWorklist",
        "BuildDefBlocksAndDominators",
        "RematerializeIncomingPerPredWithMemo",
        "RewritePhiInputSlots",
        "ReturnChangedCount",
    ]:
        raise SelectionError("PHI input materialization step order drift")
    phi_input_contract = phi_input_materialization.get("result_contract") or {}
    if (
        phi_input_contract.get("entrypoint")
        != "phi_input_materializer::materialize_all_phi_inputs"
    ):
        raise SelectionError("PHI input materialization entrypoint drift")
    phi_input_non_claims = phi_input_materialization.get("non_claims") or {}
    for key in [
        "dev_birth_verification",
        "module_function_insertion",
        "all_functions_phi_materialization",
        "semantic_refresh",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if phi_input_non_claims.get(key) != 0:
            raise SelectionError(f"PHI input materialization plan must keep {key}=0")
    if dev_birth_verification.get("kind") != "MirBuilderDevBirthVerificationPlanV1":
        raise SelectionError("dev birth verification plan has wrong kind")
    dev_birth_caps = set(dev_birth_verification.get("available_capabilities") or [])
    if "DevBirthVerification" not in dev_birth_caps:
        raise SelectionError("dev birth verification plan lacks DevBirthVerification")
    if dev_birth_verification.get("guard_conditions") != [
        "using_is_dev",
        "stageb_dev_verify_enabled",
        "cli_verbose_enabled",
    ]:
        raise SelectionError("dev birth verification guard condition drift")
    if dev_birth_verification.get("verification_steps") != [
        "IterateFunctionBlocks",
        "ScanNewBoxInstructions",
        "SkipStageBDriverBox",
        "SkipStringBox",
        "ExpectBirthTailByBoxTypeAndArity",
        "LookAheadThreeInstructions",
        "AcceptMethodBirthOnSameReceiver",
        "AcceptConstStringGlobalCompatibilityPath",
        "WarnOnMissingBirth",
        "WarnSummaryWhenAnyMissing",
    ]:
        raise SelectionError("dev birth verification step order drift")
    dev_birth_contract = dev_birth_verification.get("result_contract") or {}
    if dev_birth_contract.get("side_effect") != "dev_warning_only":
        raise SelectionError("dev birth verification side effect drift")
    dev_birth_non_claims = dev_birth_verification.get("non_claims") or {}
    for key in [
        "module_function_insertion",
        "condition_fn_injection",
        "all_functions_phi_materialization",
        "semantic_refresh",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if dev_birth_non_claims.get(key) != 0:
            raise SelectionError(f"dev birth verification plan must keep {key}=0")
    if module_function_insertion.get("kind") != "MirBuilderModuleFunctionInsertionPlanV1":
        raise SelectionError("module function insertion plan has wrong kind")
    module_function_caps = set(module_function_insertion.get("available_capabilities") or [])
    if "ModuleFunctionInsertion" not in module_function_caps:
        raise SelectionError("module function insertion plan lacks ModuleFunctionInsertion")
    insertion = module_function_insertion.get("insertion") or {}
    if insertion.get("callsite") != "module.add_function(function)":
        raise SelectionError("module function insertion callsite drift")
    if insertion.get("key_source") != "function.signature.name.clone()":
        raise SelectionError("module function insertion key source drift")
    if insertion.get("container_operation") != "BTreeMap::insert":
        raise SelectionError("module function insertion operation drift")
    if insertion.get("collision_policy") != "ReplaceExistingByName":
        raise SelectionError("module function insertion collision policy drift")
    module_function_non_claims = module_function_insertion.get("non_claims") or {}
    for key in [
        "condition_fn_injection",
        "all_functions_phi_materialization",
        "region_stack_pop",
        "slot_registry_release",
        "metadata_publication",
        "semantic_refresh",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if module_function_non_claims.get(key) != 0:
            raise SelectionError(f"module function insertion plan must keep {key}=0")
    if condition_fn_injection.get("kind") != "MirBuilderConditionFnInjectionPlanV1":
        raise SelectionError("condition_fn injection plan has wrong kind")
    condition_caps = set(condition_fn_injection.get("available_capabilities") or [])
    if "ConditionFnInjection" not in condition_caps:
        raise SelectionError("condition_fn injection plan lacks ConditionFnInjection")
    injection = condition_fn_injection.get("injection") or {}
    if injection.get("predicate") != 'module.functions.get("condition_fn").is_none()':
        raise SelectionError("condition_fn injection predicate drift")
    if injection.get("function_name") != "condition_fn":
        raise SelectionError("condition_fn injection function name drift")
    if injection.get("body") != ["ConstInteger(1)", "ReturnValue(one)"]:
        raise SelectionError("condition_fn injection body drift")
    if injection.get("insert_operation") != "module.add_function(f)":
        raise SelectionError("condition_fn injection insert operation drift")
    condition_non_claims = condition_fn_injection.get("non_claims") or {}
    for key in [
        "condition_fn_policy_generalization",
        "region_stack_pop",
        "slot_registry_release",
        "metadata_publication",
        "semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if condition_non_claims.get(key) != 0:
            raise SelectionError(f"condition_fn injection plan must keep {key}=0")
    if function_region_stack_pop.get("kind") != "MirBuilderFunctionRegionStackPopPlanV1":
        raise SelectionError("function-region stack pop plan has wrong kind")
    region_pop_caps = set(function_region_stack_pop.get("available_capabilities") or [])
    if "FunctionRegionStackPop" not in region_pop_caps:
        raise SelectionError("function-region stack pop plan lacks FunctionRegionStackPop")
    pop_policy = function_region_stack_pop.get("pop_policy") or {}
    if pop_policy.get("callsite") != "region::observer::pop_function_region(self)":
        raise SelectionError("function-region stack pop callsite drift")
    if pop_policy.get("guard") != "NYASH_REGION_TRACE == 1":
        raise SelectionError("function-region stack pop guard drift")
    if pop_policy.get("operation") != "metadata_ctx.pop_region":
        raise SelectionError("function-region stack pop operation drift")
    if pop_policy.get("tracing_disabled_effect") != "NoOp":
        raise SelectionError("function-region stack pop disabled effect drift")
    region_pop_contract = function_region_stack_pop.get("result_contract") or {}
    if region_pop_contract.get("entrypoint") != "region::observer::pop_function_region":
        raise SelectionError("function-region stack pop entrypoint drift")
    if region_pop_contract.get("mutates_when_guard_enabled") != [
        "builder.metadata_ctx.current_region_stack"
    ]:
        raise SelectionError("function-region stack pop mutation frame drift")
    region_pop_non_claims = function_region_stack_pop.get("non_claims") or {}
    for key in [
        "observe_function_region_claim",
        "slot_registry_release",
        "metadata_publication",
        "semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if region_pop_non_claims.get(key) != 0:
            raise SelectionError(f"function-region stack pop plan must keep {key}=0")
    if slot_registry_release.get("kind") != "MirBuilderSlotRegistryReleasePlanV1":
        raise SelectionError("SlotRegistry release plan has wrong kind")
    slot_caps = set(slot_registry_release.get("available_capabilities") or [])
    if "SlotRegistryRelease" not in slot_caps:
        raise SelectionError("SlotRegistry release plan lacks SlotRegistryRelease")
    release = slot_registry_release.get("release_policy") or {}
    if release.get("lifecycle_owner") != "CompilationContext.current_slot_registry":
        raise SelectionError("SlotRegistry release lifecycle owner drift")
    if release.get("init_operation") != "Some(FunctionSlotRegistry::new())":
        raise SelectionError("SlotRegistry release init operation drift")
    if release.get("release_operation") != "current_slot_registry = None":
        raise SelectionError("SlotRegistry release operation drift")
    if (
        release.get("release_timing")
        != "AfterFunctionRegionStackPopBeforeModuleMetadataPublication"
    ):
        raise SelectionError("SlotRegistry release timing drift")
    slot_contract = slot_registry_release.get("result_contract") or {}
    if slot_contract.get("entrypoint") != "MirBuilder::finalize_module current_slot_registry release":
        raise SelectionError("SlotRegistry release entrypoint drift")
    if slot_contract.get("mutates") != ["builder.comp_ctx.current_slot_registry"]:
        raise SelectionError("SlotRegistry release mutation frame drift")
    slot_non_claims = slot_registry_release.get("non_claims") or {}
    for key in [
        "slot_metadata_classification",
        "module_metadata_publication",
        "metadata_publication",
        "semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if slot_non_claims.get(key) != 0:
            raise SelectionError(f"SlotRegistry release plan must keep {key}=0")
    if (
        module_metadata_publication.get("kind")
        != "MirBuilderModuleMetadataPublicationPlanV1"
    ):
        raise SelectionError("module metadata publication plan has wrong kind")
    module_metadata_caps = set(module_metadata_publication.get("available_capabilities") or [])
    if "ModuleMetadataPublication" not in module_metadata_caps:
        raise SelectionError("module metadata publication plan lacks ModuleMetadataPublication")
    publication = module_metadata_publication.get("publication") or {}
    if publication.get("timing") != "AfterSlotRegistryReleaseBeforeSemanticRefresh":
        raise SelectionError("module metadata publication timing drift")
    fields = publication.get("fields") or []
    if [field.get("target") for field in fields] != [
        "module.metadata.user_box_decls",
        "module.metadata.user_box_field_decls",
        "module.metadata.record_decls",
        "module.metadata.enum_decls",
    ]:
        raise SelectionError("module metadata publication field order drift")
    if fields[1].get("projected_fields") != ["name", "declared_type_name", "is_weak"]:
        raise SelectionError("module metadata user-box field projection drift")
    module_metadata_non_claims = module_metadata_publication.get("non_claims") or {}
    for key in [
        "semantic_refresh",
        "record_and_packed_layout_refresh",
        "typed_object_plan_refresh",
        "direct_state_plan_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if module_metadata_non_claims.get(key) != 0:
            raise SelectionError(f"module metadata publication plan must keep {key}=0")
    if (
        record_packed_layout_refresh.get("kind")
        != "MirBuilderRecordPackedLayoutRefreshPlanV1"
    ):
        raise SelectionError("record/packed layout refresh plan has wrong kind")
    record_refresh_caps = set(record_packed_layout_refresh.get("available_capabilities") or [])
    if "RecordAndPackedLayoutRefresh" not in record_refresh_caps:
        raise SelectionError("record/packed layout refresh plan lacks RecordAndPackedLayoutRefresh")
    record_refresh = record_packed_layout_refresh.get("refresh_policy") or {}
    if record_refresh.get("entrypoint") != "refresh_module_record_and_packed_layout_plans":
        raise SelectionError("record/packed layout refresh entrypoint drift")
    if (
        record_refresh.get("timing")
        != "AfterModuleMetadataPublicationBeforeTypedObjectRefresh"
    ):
        raise SelectionError("record/packed layout refresh timing drift")
    if record_refresh.get("steps") != [
        "refresh_module_record_layout_plans",
        "refresh_module_array_record_storage_plans",
        "refresh_module_array_record_autouse_eligibility_plans",
        "refresh_module_array_record_materialization_boundary_plans",
        "refresh_module_array_record_packed_autouse_pilot_plans",
        "refresh_module_source_packed_array_autouse_pilot_plans",
        "refresh_module_source_packed_array_direct_read_consumption_plans",
        "refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans",
        "refresh_module_hako_alloc_huge_page_packed_store_pilot_plans",
    ]:
        raise SelectionError("record/packed layout refresh step order drift")
    record_refresh_non_claims = record_packed_layout_refresh.get("non_claims") or {}
    for key in [
        "typed_object_plan_refresh",
        "direct_state_plan_refresh",
        "full_semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if record_refresh_non_claims.get(key) != 0:
            raise SelectionError(f"record/packed layout refresh plan must keep {key}=0")
    if typed_object_plan_refresh.get("kind") != "MirBuilderTypedObjectPlanRefreshPlanV1":
        raise SelectionError("typed object plan refresh plan has wrong kind")
    typed_object_caps = set(typed_object_plan_refresh.get("available_capabilities") or [])
    if "TypedObjectPlanRefresh" not in typed_object_caps:
        raise SelectionError("typed object plan refresh plan lacks TypedObjectPlanRefresh")
    typed_object_refresh = typed_object_plan_refresh.get("refresh_policy") or {}
    if typed_object_refresh.get("entrypoint") != "refresh_module_typed_object_plans":
        raise SelectionError("typed object plan refresh entrypoint drift")
    if (
        typed_object_refresh.get("timing")
        != "AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh"
    ):
        raise SelectionError("typed object plan refresh timing drift")
    if typed_object_refresh.get("operation") != "AssignTypedObjectPlans":
        raise SelectionError("typed object plan refresh operation drift")
    if (
        typed_object_refresh.get("build_provider")
        != "storage_inference::build_typed_object_plans"
    ):
        raise SelectionError("typed object plan refresh build provider drift")
    if typed_object_refresh.get("target") != "module.metadata.typed_object_plans":
        raise SelectionError("typed object plan refresh target drift")
    typed_object_non_claims = typed_object_plan_refresh.get("non_claims") or {}
    for key in [
        "typed_object_field_value_type_refresh",
        "typed_object_collection_field_element_refresh",
        "direct_state_plan_refresh",
        "full_semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if typed_object_non_claims.get(key) != 0:
            raise SelectionError(f"typed object plan refresh plan must keep {key}=0")
    if direct_state_plan_refresh.get("kind") != "MirBuilderDirectStatePlanRefreshPlanV1":
        raise SelectionError("direct state plan refresh plan has wrong kind")
    direct_state_caps = set(direct_state_plan_refresh.get("available_capabilities") or [])
    if "DirectStatePlanRefresh" not in direct_state_caps:
        raise SelectionError("direct state plan refresh plan lacks DirectStatePlanRefresh")
    direct_state_refresh = direct_state_plan_refresh.get("refresh_policy") or {}
    if direct_state_refresh.get("entrypoint") != "refresh_module_direct_state_plans":
        raise SelectionError("direct state plan refresh entrypoint drift")
    if (
        direct_state_refresh.get("timing")
        != "AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization"
    ):
        raise SelectionError("direct state plan refresh timing drift")
    if direct_state_refresh.get("operation") != "AssignDirectStatePlans":
        raise SelectionError("direct state plan refresh operation drift")
    if (
        direct_state_refresh.get("build_provider")
        != "direct_state_plan::build_direct_state_plans"
    ):
        raise SelectionError("direct state plan refresh build provider drift")
    if direct_state_refresh.get("target") != "module.metadata.direct_state_plans":
        raise SelectionError("direct state plan refresh target drift")
    direct_state_builder = direct_state_plan_refresh.get("plan_builder_contract") or {}
    if direct_state_builder.get("input_authority") != "module.metadata.user_box_field_decls":
        raise SelectionError("direct state plan refresh input authority drift")
    if direct_state_builder.get("field_selection") != "TypedObjectFieldStorageUsesIntegerLaneAndNotWeak":
        raise SelectionError("direct state plan refresh field selection drift")
    if direct_state_builder.get("runtime_layout_created") != 0:
        raise SelectionError("direct state plan refresh must not claim runtime layout")
    if direct_state_builder.get("lowering_enabled") != 0:
        raise SelectionError("direct state plan refresh must not claim lowering")
    direct_state_non_claims = direct_state_plan_refresh.get("non_claims") or {}
    for key in [
        "all_functions_phi_materialization",
        "direct_state_lowering",
        "route_selection",
        "native_direct_guard",
        "full_semantic_refresh",
        "full_finalize_module",
        "generated_hako_artifact",
    ]:
        if direct_state_non_claims.get(key) != 0:
            raise SelectionError(f"direct state plan refresh plan must keep {key}=0")
    if (
        all_functions_phi_materialization.get("kind")
        != "MirBuilderAllFunctionsPhiMaterializationPlanV1"
    ):
        raise SelectionError("all-functions PHI materialization plan has wrong kind")
    all_functions_caps = set(all_functions_phi_materialization.get("available_capabilities") or [])
    if "AllFunctionsPhiMaterialization" not in all_functions_caps:
        raise SelectionError(
            "all-functions PHI materialization plan lacks AllFunctionsPhiMaterialization"
        )
    sweep = all_functions_phi_materialization.get("sweep_policy") or {}
    if sweep.get("iteration") != "for function in module.functions.values_mut()":
        raise SelectionError("all-functions PHI sweep iteration drift")
    if sweep.get("delegate") != "phi_input_materializer::materialize_all_phi_inputs":
        raise SelectionError("all-functions PHI sweep delegate drift")
    if sweep.get("delegate_context") != "finalize_module_all_functions":
        raise SelectionError("all-functions PHI sweep context drift")
    if sweep.get("delegate_capability") != "PhiInputMaterialization":
        raise SelectionError("all-functions PHI delegate capability drift")
    all_functions_non_claims = all_functions_phi_materialization.get("non_claims") or {}
    for key in [
        "full_finalize_module",
        "generated_hako_artifact",
        "backend_route_changed",
        "abi_changed",
        "runtime_fallback",
        "mainline_selected",
        "source_selfhost_claim",
    ]:
        if all_functions_non_claims.get(key) != 0:
            raise SelectionError(f"all-functions PHI plan must keep {key}=0")
    if minimal_smoke.get("kind") != "MinimalMirBuilderExecutionPathSmokeResultV1":
        raise SelectionError("minimal execution smoke result has wrong kind")
    smoke_caps = set(minimal_smoke.get("available_capabilities") or [])
    if "MinimalExecutionPathSmoke" not in smoke_caps:
        raise SelectionError("minimal execution smoke result lacks MinimalExecutionPathSmoke")
    smoke_claims = minimal_smoke.get("claims") or {}
    for key in [
        "full_mirbuilder_new_claim",
        "generated_hako_change",
        "mainline_selected",
        "new_backend_route",
        "new_abi",
        "runtime_fallback",
        "source_selfhost_claim",
    ]:
        if smoke_claims.get(key) != 0:
            raise SelectionError(f"minimal smoke result must keep {key}=0")
    if mainline_route.get("kind") != "DerivedMainlineRouteSelectionV1":
        raise SelectionError("mainline route has wrong kind")
    if (
        mainline_route.get("route_slot_id")
        != "hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1"
    ):
        raise SelectionError("mainline route has wrong route_slot_id")
    profiles = mainline_route.get("profiles") or {}
    if profiles.get("selfhost_mainline", {}).get("route") != "derived_hako":
        raise SelectionError("mainline route does not select derived_hako for selfhost")
    if profiles.get("rust_bootstrap", {}).get("route") != "rust_bootstrap":
        raise SelectionError("mainline route does not retain rust_bootstrap")
    if mainline_route.get("fallback_policy") != "Forbidden":
        raise SelectionError("mainline route fallback must be forbidden")
    route_claims = mainline_route.get("claims") or {}
    if route_claims.get("runtime_try_hako_then_rust_fallback") != 0:
        raise SelectionError("mainline route must not permit runtime fallback")

    return [
        {
            "capability": "MirModuleMinimalShellTransport",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir::MirModule",
            "manifest_path": rel(MODULE_SHELL_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "MirFunctionConstructorTransport",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir::MirFunction",
            "manifest_path": rel(FUNCTION_CONSTRUCTOR_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "PreparedStateInstall",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir::MirFunction",
            "manifest_path": rel(FUNCTION_CONSTRUCTOR_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "LiteralIntegerLowering",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::literal_integer",
            "manifest_path": rel(LITERAL_INTEGER_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "FinalizeModuleComposition",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::bounded_finalize",
            "manifest_path": rel(BOUNDED_FINALIZE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "ReturnEmission",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::return_emission",
            "manifest_path": rel(RETURN_EMISSION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "ReturnTypePublication",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::return_type_publication",
            "manifest_path": rel(RETURN_TYPE_PUBLICATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "CurrentModuleTake",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::current_module_take",
            "manifest_path": rel(CURRENT_MODULE_TAKE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "TypedValueDefinitionVerification",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::typed_value_verification",
            "manifest_path": rel(TYPED_VALUE_VERIFICATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "CurrentFunctionTake",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::current_function_take",
            "manifest_path": rel(CURRENT_FUNCTION_TAKE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "TypePropagationPipelineExecution",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::type_propagation_pipeline",
            "manifest_path": rel(TYPE_PROPAGATION_PIPELINE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "TypeHintProvision",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::type_hint_provision",
            "manifest_path": rel(TYPE_HINT_PROVISION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "MetadataValueTypePublication",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::metadata_value_type_publication",
            "manifest_path": rel(METADATA_VALUE_TYPE_PUBLICATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "MetadataOriginCallerMerge",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::metadata_origin_caller_merge",
            "manifest_path": rel(METADATA_ORIGIN_CALLER_MERGE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "PhiReturnTypeInference",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::phi_return_type_inference",
            "manifest_path": rel(PHI_RETURN_TYPE_INFERENCE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "PhiInputMaterialization",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::phi_input_materialization",
            "manifest_path": rel(PHI_INPUT_MATERIALIZATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "DevBirthVerification",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::dev_birth_verification",
            "manifest_path": rel(DEV_BIRTH_VERIFICATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "ModuleFunctionInsertion",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::module_function_insertion",
            "manifest_path": rel(MODULE_FUNCTION_INSERTION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "ConditionFnInjection",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::condition_fn_injection",
            "manifest_path": rel(CONDITION_FN_INJECTION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "FunctionRegionStackPop",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::function_region_stack_pop",
            "manifest_path": rel(FUNCTION_REGION_STACK_POP_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "SlotRegistryRelease",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::slot_registry_release",
            "manifest_path": rel(SLOT_REGISTRY_RELEASE_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "ModuleMetadataPublication",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::module_metadata_publication",
            "manifest_path": rel(MODULE_METADATA_PUBLICATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "RecordAndPackedLayoutRefresh",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::record_packed_layout_refresh",
            "manifest_path": rel(RECORD_PACKED_LAYOUT_REFRESH_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "TypedObjectPlanRefresh",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::typed_object_plan_refresh",
            "manifest_path": rel(TYPED_OBJECT_PLAN_REFRESH_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "DirectStatePlanRefresh",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::direct_state_plan_refresh",
            "manifest_path": rel(DIRECT_STATE_PLAN_REFRESH_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "AllFunctionsPhiMaterialization",
            "contract_kind": "SourceDerivedCapabilityPlanV1",
            "family_id": "hakorune_mir_builder::all_functions_phi_materialization",
            "manifest_path": rel(ALL_FUNCTIONS_PHI_MATERIALIZATION_PLAN),
            "artifact_state": "PlanOnly",
        },
        {
            "capability": "MinimalExecutionPathSmoke",
            "contract_kind": "SmokeResultV1",
            "family_id": "hakorune_mir_builder::minimal_execution_path",
            "manifest_path": rel(MINIMAL_SMOKE_RESULT),
            "artifact_state": "Observed",
        },
        {
            "capability": "MirBuilderAllocationPolicyMainlinePilot",
            "contract_kind": "DerivedMainlineRouteSelectionV1",
            "family_id": mainline_route.get("family_id"),
            "manifest_path": rel(MAINLINE_ROUTE),
            "artifact_state": "DerivedMainline",
        },
        {
            "capability": "CoreContext.scalar_counters_and_id_generators",
            "contract_kind": "VerifiedFamilyArtifactContractV1",
            "family_id": core.get("family_id"),
            "manifest_path": rel(CORE_CONTEXT_MANIFEST),
            "artifact_state": core.get("state"),
        },
        {
            "capability": "MirBuilderAllocationPolicy.prepared_state_next_value_id",
            "contract_kind": "VerifiedFamilyArtifactContractV1",
            "family_id": prepared.get("family_id"),
            "manifest_path": rel(PREPARED_KERNEL_MANIFEST),
            "artifact_state": prepared.get("state"),
        },
        {
            "capability": "MirBuilderBundle.membership_only",
            "contract_kind": "BundleMembershipOnlyV1",
            "family_id": bundle.get("family_id"),
            "manifest_path": rel(BUNDLE_MANIFEST),
            "artifact_state": bundle.get("state"),
        },
    ]


def provider_contract(capability: str, contracts: list[dict[str, Any]]) -> dict[str, Any] | None:
    for contract in contracts:
        if contract.get("capability") == capability:
            return contract
    return None


def build_plan() -> dict[str, Any]:
    sources = extract_source_order_facts()
    contracts = contract_sources()
    edges = [
        {
            "id": "entry.prepared_state_profile",
            "callsite": "PreparedMirBuilderStateV1",
            "required_capability": "PreparedMirBuilderStateV1",
            "provider": {"kind": "ExecutionProfile"},
        },
        {
            "id": "build_module.prepare_module",
            "callsite": "MirBuilder::build_module -> prepare_module",
            "required_capability": "RustSourceCallOrder",
            "provider": {"kind": "LiveSourceOrder", "facts": "build_module_order"},
        },
        {
            "id": "prepare_module.module_new",
            "callsite": "MirBuilder::prepare_module -> MirModule::new",
            "required_capability": "MirModuleMinimalShellTransport",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MirModuleMinimalShellTransport",
            },
            "unsupported": {
                "deny_reason": "UnsupportedTypeTransport",
                "deny_detail": "MirModuleMinimalShellTransportRequired",
                "semantic_owner": "MirModule::new",
                "next_slice_token": "MIR-MODULE-MINIMAL-SHELL-TRANSPORT-001",
            },
        },
        {
            "id": "prepare_module.source_file",
            "callsite": "MirBuilder::prepare_module -> current_source_file",
            "required_capability": "SourceFileOptionTransport",
            "provider": {
                "kind": "ProfileExcluded",
                "profile_key": "source_file",
                "profile_value": None,
            },
            "unsupported": {
                "deny_reason": "UnsupportedTypeTransport",
                "deny_detail": "SourceFileOptionTransportRequired",
                "semantic_owner": "MirBuilder::current_source_file",
                "next_slice_token": "MIRBUILDER-SOURCE-FILE-OPTION-TRANSPORT-001",
            },
        },
        {
            "id": "prepare_module.next_block",
            "callsite": "MirBuilder::prepare_module -> CoreContextApi.next_block",
            "required_capability": "CoreContext.scalar_counters_and_id_generators",
            "provider": {
                "kind": "ArtifactContract",
                "capability": "CoreContext.scalar_counters_and_id_generators",
            },
        },
        {
            "id": "prepare_module.function_new",
            "callsite": "MirBuilder::prepare_module -> MirFunction::new",
            "required_capability": "MirFunctionConstructorTransport",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MirFunctionConstructorTransport",
            },
            "unsupported": {
                "deny_reason": "UnsupportedTypeTransport",
                "deny_detail": "MirFunctionConstructorTransportRequired",
                "semantic_owner": "MirFunction::new",
                "next_slice_token": "MIR-FUNCTION-CONSTRUCTOR-COMPOSITION-001",
            },
        },
        {
            "id": "prepare_module.state_install",
            "callsite": "MirBuilder::prepare_module -> current state install",
            "required_capability": "PreparedStateInstall",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "PreparedStateInstall",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "PreparedStateInstallRequired",
                "semantic_owner": "MirBuilder prepared state",
                "next_slice_token": "MIRBUILDER-PREPARED-STATE-INSTALL-001",
            },
        },
        {
            "id": "lower_root.literal_integer",
            "callsite": "MirBuilder::lower_root(ASTNode::Literal(Integer(0)))",
            "required_capability": "LiteralIntegerLowering",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "LiteralIntegerLowering",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "LiteralIntegerLoweringRequired",
                "semantic_owner": "MirBuilder::build_literal",
                "next_slice_token": "MIRBUILDER-LITERAL-INTEGER-LOWERING-001",
            },
        },
        {
            "id": "finalize_module.composition",
            "callsite": "MirBuilder::finalize_module",
            "required_capability": "FinalizeModuleComposition",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "FinalizeModuleComposition",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "FinalizeModuleCompositionRequired",
                "semantic_owner": "MirBuilder::finalize_module",
                "next_slice_token": "MIRBUILDER-BOUNDED-FINALIZE-COMPOSITION-001",
            },
        },
        {
            "id": "minimal_execution_path.smoke",
            "callsite": "PreparedMirBuilderStateV1 build_module(ASTNode::Literal(Integer(0))) smoke",
            "required_capability": "MinimalExecutionPathSmoke",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MinimalExecutionPathSmoke",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MinimalExecutionPathSmokeRequired",
                "semantic_owner": "minimal MirBuilder execution path",
                "next_slice_token": "MIRBUILDER-MINIMAL-EXECUTION-PATH-SMOKE-001",
            },
        },
        {
            "id": "mainline_pilot.selection",
            "callsite": "MirBuilder allocation policy mainline pilot selection",
            "required_capability": "MirBuilderAllocationPolicyMainlinePilot",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MirBuilderAllocationPolicyMainlinePilot",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MainlineSelectionRequired",
                "semantic_owner": "MirBuilder allocation policy mainline pilot",
                "next_slice_token": "MIRBUILDER-ALLOCATION-POLICY-MAINLINE-PILOT-001",
            },
        },
        {
            "id": "finalize_module.return_emission",
            "callsite": "MirBuilder::finalize_module -> append Return(result_value)",
            "required_capability": "ReturnEmission",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "ReturnEmission",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "ReturnEmissionRequired",
                "semantic_owner": "MirBuilder::finalize_module return emission",
                "next_slice_token": "MIRBUILDER-RETURN-EMISSION-001",
            },
        },
        {
            "id": "finalize_module.return_type_publication",
            "callsite": "MirBuilder::finalize_module -> publish return type from result_value",
            "required_capability": "ReturnTypePublication",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "ReturnTypePublication",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "ReturnTypePublicationRequired",
                "semantic_owner": "MirBuilder::finalize_module return type publication",
                "next_slice_token": "MIRBUILDER-RETURN-TYPE-PUBLICATION-001",
            },
        },
        {
            "id": "finalize_module.take_module",
            "callsite": "MirBuilder::finalize_module -> take current_module",
            "required_capability": "CurrentModuleTake",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "CurrentModuleTake",
            },
            "unsupported": {
                "deny_reason": "UnsupportedTypeTransport",
                "deny_detail": "CurrentModuleTakeRequired",
                "semantic_owner": "MirBuilder::finalize_module current_module take",
                "next_slice_token": "MIRBUILDER-CURRENT-MODULE-TAKE-001",
            },
        },
        {
            "id": "finalize_module.verify_typed_values",
            "callsite": "MirBuilder::finalize_module -> verify typed values are defined",
            "required_capability": "TypedValueDefinitionVerification",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "TypedValueDefinitionVerification",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "TypedValueVerificationRequired",
                "semantic_owner": "MirBuilder::finalize_module typed value verification",
                "next_slice_token": "MIRBUILDER-TYPED-VALUE-VERIFICATION-001",
            },
        },
        {
            "id": "finalize_module.take_current_function",
            "callsite": "MirBuilder::finalize_module -> take current_function",
            "required_capability": "CurrentFunctionTake",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "CurrentFunctionTake",
            },
            "unsupported": {
                "deny_reason": "UnsupportedTypeTransport",
                "deny_detail": "CurrentFunctionTakeRequired",
                "semantic_owner": "MirBuilder::finalize_module current_function take",
                "next_slice_token": "MIRBUILDER-CURRENT-FUNCTION-TAKE-001",
            },
        },
        {
            "id": "finalize_module.type_propagation",
            "callsite": "MirBuilder::finalize_module -> TypePropagationPipeline::run",
            "required_capability": "TypePropagationPipelineExecution",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "TypePropagationPipelineExecution",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "TypePropagationPipelineRequired",
                "semantic_owner": "MirBuilder::finalize_module type propagation",
                "next_slice_token": "MIRBUILDER-TYPE-PROPAGATION-PIPELINE-001",
            },
        },
        {
            "id": "finalize_module.type_hint_provision",
            "callsite": "MirBuilder::finalize_module -> annotate missing call/await result types",
            "required_capability": "TypeHintProvision",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "TypeHintProvision",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "TypeHintProvisionRequired",
                "semantic_owner": "MirBuilder::finalize_module type hint provision",
                "next_slice_token": "MIRBUILDER-TYPE-HINT-PROVISION-001",
            },
        },
        {
            "id": "finalize_module.metadata_value_type_publication",
            "callsite": "MirBuilder::finalize_module -> publish function.metadata.value_types",
            "required_capability": "MetadataValueTypePublication",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MetadataValueTypePublication",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MetadataValueTypePublicationRequired",
                "semantic_owner": "MirBuilder::finalize_module metadata value type publication",
                "next_slice_token": "MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-001",
            },
        },
        {
            "id": "finalize_module.metadata_origin_caller_merge",
            "callsite": "MirBuilder::finalize_module -> merge function.metadata.value_origin_callers",
            "required_capability": "MetadataOriginCallerMerge",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "MetadataOriginCallerMerge",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MetadataOriginCallerMergeRequired",
                "semantic_owner": "MirBuilder::finalize_module metadata origin-caller merge",
                "next_slice_token": "MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-001",
            },
        },
        {
            "id": "finalize_module.phi_return_type_inference",
            "callsite": "MirBuilder::finalize_module -> infer return type from PHI",
            "required_capability": "PhiReturnTypeInference",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "PhiReturnTypeInference",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "PhiReturnTypeInferenceRequired",
                "semantic_owner": "MirBuilder::finalize_module PHI return type inference",
                "next_slice_token": "MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-001",
            },
        },
        {
            "id": "finalize_module.phi_input_materialization",
            "callsite": "MirBuilder::finalize_module -> materialize all PHI inputs",
            "required_capability": "PhiInputMaterialization",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "PhiInputMaterialization",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "PhiInputMaterializationRequired",
                "semantic_owner": "MirBuilder::finalize_module PHI input materialization",
                "next_slice_token": "MIRBUILDER-PHI-INPUT-MATERIALIZATION-001",
            },
        },
        {
            "id": "finalize_module.dev_birth_verification",
            "callsite": "MirBuilder::finalize_module -> dev NewBox birth verification",
            "required_capability": "DevBirthVerification",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "DevBirthVerification",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "DevBirthVerificationRequired",
                "semantic_owner": "MirBuilder::finalize_module dev birth verification",
                "next_slice_token": "MIRBUILDER-DEV-BIRTH-VERIFICATION-001",
            },
        },
        {
            "id": "finalize_module.module_function_insertion",
            "callsite": "MirBuilder::finalize_module -> module.add_function(function)",
            "required_capability": "ModuleFunctionInsertion",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "ModuleFunctionInsertion",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "ModuleFunctionInsertionRequired",
                "semantic_owner": "MirBuilder::finalize_module module function insertion",
                "next_slice_token": "MIRBUILDER-MODULE-FUNCTION-INSERTION-001",
            },
        },
        {
            "id": "finalize_module.condition_fn_injection",
            "callsite": "MirBuilder::finalize_module -> inject condition_fn when missing",
            "required_capability": "ConditionFnInjection",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "ConditionFnInjection",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "ConditionFnInjectionRequired",
                "semantic_owner": "MirBuilder::finalize_module condition_fn injection",
                "next_slice_token": "MIRBUILDER-CONDITION-FN-INJECTION-001",
            },
        },
        {
            "id": "finalize_module.region_stack_pop",
            "callsite": "MirBuilder::finalize_module -> region::observer::pop_function_region",
            "required_capability": "FunctionRegionStackPop",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "FunctionRegionStackPop",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "FunctionRegionStackPopRequired",
                "semantic_owner": "MirBuilder::finalize_module function region cleanup",
                "next_slice_token": "MIRBUILDER-FUNCTION-REGION-STACK-POP-001",
            },
        },
        {
            "id": "finalize_module.slot_registry_release",
            "callsite": "MirBuilder::finalize_module -> current_slot_registry = None",
            "required_capability": "SlotRegistryRelease",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "SlotRegistryRelease",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "SlotRegistryReleaseRequired",
                "semantic_owner": "MirBuilder::finalize_module slot registry cleanup",
                "next_slice_token": "MIRBUILDER-SLOT-REGISTRY-RELEASE-001",
            },
        },
        {
            "id": "finalize_module.module_metadata_publication",
            "callsite": "MirBuilder::finalize_module -> publish module metadata",
            "required_capability": "ModuleMetadataPublication",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "ModuleMetadataPublication",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "ModuleMetadataPublicationRequired",
                "semantic_owner": "MirBuilder::finalize_module module metadata publication",
                "next_slice_token": "MIRBUILDER-MODULE-METADATA-PUBLICATION-001",
            },
        },
        {
            "id": "finalize_module.record_packed_layout_refresh",
            "callsite": "MirBuilder::finalize_module -> refresh_module_record_and_packed_layout_plans",
            "required_capability": "RecordAndPackedLayoutRefresh",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "RecordAndPackedLayoutRefresh",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "RecordAndPackedLayoutRefreshRequired",
                "semantic_owner": "MirBuilder::finalize_module record/packed layout refresh",
                "next_slice_token": "MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001",
            },
        },
        {
            "id": "finalize_module.typed_object_plan_refresh",
            "callsite": "MirBuilder::finalize_module -> refresh_module_typed_object_plans",
            "required_capability": "TypedObjectPlanRefresh",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "TypedObjectPlanRefresh",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "TypedObjectPlanRefreshRequired",
                "semantic_owner": "MirBuilder::finalize_module typed object plan refresh",
                "next_slice_token": "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001",
            },
        },
        {
            "id": "finalize_module.direct_state_plan_refresh",
            "callsite": "MirBuilder::finalize_module -> refresh_module_direct_state_plans",
            "required_capability": "DirectStatePlanRefresh",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "DirectStatePlanRefresh",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "DirectStatePlanRefreshRequired",
                "semantic_owner": "MirBuilder::finalize_module direct state plan refresh",
                "next_slice_token": "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001",
            },
        },
        {
            "id": "finalize_module.all_functions_phi_materialization",
            "callsite": "MirBuilder::finalize_module -> materialize_all_phi_inputs for all functions",
            "required_capability": "AllFunctionsPhiMaterialization",
            "provider": {
                "kind": "CapabilityPlan",
                "capability": "AllFunctionsPhiMaterialization",
            },
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "AllFunctionsPhiMaterializationRequired",
                "semantic_owner": "MirBuilder::finalize_module all-functions PHI materialization",
                "next_slice_token": "MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001",
            },
        },
        {
            "id": "minimal_path.completion_design_stop",
            "callsite": "MinimalMirBuilderExecutionPath -> post-finalize completion design stop",
            "required_capability": "MinimalExecutionPathCompletionDesignReview",
            "provider": None,
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MinimalExecutionPathCompletionDesignReviewRequired",
                "semantic_owner": "Minimal MirBuilder execution path completion review",
                "next_slice_token": "MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001",
            },
        },
    ]

    return {
        "kind": "MinimalMirBuilderExecutionPathPlanV1",
        "source_entry": "MirBuilder::build_module",
        "input_profile": {
            "ast": "ASTNode::Literal(Integer(0))",
        },
        "execution_profile": {
            "kind": "PreparedMirBuilderStateV1",
            "current_module": "Absent",
            "current_function": "Absent",
            "current_block": "Absent",
            "reserved_value_ids": "Empty",
            "source_file": None,
            "builder_safepoint_entry": False,
            "dev_birth_verification": False,
            "runtime_fallback": False,
        },
        "entry_preconditions": [
            "prepared generated contexts",
            "no full MirBuilder::new claim",
            "bundle membership is not capability proof by itself",
        ],
        "source_order_facts": sources,
        "contract_sources": contracts,
        "ordered_source_edges": edges,
        "explicit_non_claims": {
            "full_mirbuilder_new_claim": 0,
            "generated_hako_change": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
            "bundle_size_as_proof": 0,
        },
    }


def analyze_frontier(plan: dict[str, Any]) -> dict[str, Any]:
    contracts = plan["contract_sources"]
    reached_prefix: list[dict[str, Any]] = []
    profile_excluded: list[dict[str, Any]] = []
    not_reached: list[dict[str, Any]] = []
    first_unsupported: dict[str, Any] | None = None

    for edge in plan["ordered_source_edges"]:
        if first_unsupported is not None:
            not_reached.append(
                {
                    "edge_id": edge["id"],
                    "callsite": edge["callsite"],
                    "status": "NotReached",
                }
            )
            continue

        provider = edge.get("provider")
        if provider is None:
            unsupported = edge["unsupported"]
            first_unsupported = {
                "edge_id": edge["id"],
                "callsite": edge["callsite"],
                "status": "Unsupported",
                "required_capability": edge["required_capability"],
                "deny_reason": unsupported["deny_reason"],
                "deny_detail": unsupported["deny_detail"],
                "semantic_owner": unsupported["semantic_owner"],
                "next_slice_token": unsupported["next_slice_token"],
            }
            reached_prefix.append(first_unsupported)
            continue

        if provider["kind"] in {"ArtifactContract", "CapabilityPlan"}:
            contract = provider_contract(provider["capability"], contracts)
            if contract is None:
                raise SelectionError(
                    f"edge {edge['id']} marks artifact contract available without a contract"
                )
            reached_prefix.append(
                {
                    "edge_id": edge["id"],
                    "callsite": edge["callsite"],
                    "status": "Available",
                    "required_capability": edge["required_capability"],
                    "contract_reference": contract,
                }
            )
            continue

        if provider["kind"] == "ProfileExcluded":
            key = provider["profile_key"]
            if plan["execution_profile"].get(key) != provider["profile_value"]:
                unsupported = edge["unsupported"]
                first_unsupported = {
                    "edge_id": edge["id"],
                    "callsite": edge["callsite"],
                    "status": "Unsupported",
                    "required_capability": edge["required_capability"],
                    "deny_reason": unsupported["deny_reason"],
                    "deny_detail": unsupported["deny_detail"],
                    "semantic_owner": unsupported["semantic_owner"],
                    "next_slice_token": unsupported["next_slice_token"],
                }
                reached_prefix.append(first_unsupported)
                continue
            row = {
                "edge_id": edge["id"],
                "callsite": edge["callsite"],
                "status": "ProfileExcluded",
                "required_capability": edge["required_capability"],
                "profile_key": key,
                "profile_value": provider["profile_value"],
            }
            reached_prefix.append(row)
            profile_excluded.append(row)
            continue

        if provider["kind"] in {"ExecutionProfile", "LiveSourceOrder"}:
            reached_prefix.append(
                {
                    "edge_id": edge["id"],
                    "callsite": edge["callsite"],
                    "status": "Available",
                    "required_capability": edge["required_capability"],
                    "provider": provider,
                }
            )
            continue

        raise SelectionError(f"unsupported provider kind: {provider}")

    if first_unsupported is None:
        raise SelectionError("frontier analysis unexpectedly found no unsupported edge")

    return {
        "kind": "MinimalMirBuilderFirstRedEdgeResultV1",
        "source_entry": plan["source_entry"],
        "input_profile": plan["input_profile"],
        "execution_profile": plan["execution_profile"],
        "reached_prefix": reached_prefix,
        "profile_excluded_edges": profile_excluded,
        "first_unsupported_edge": first_unsupported,
        "not_reached_edges": not_reached,
        "claims": {
            "entry_is_prepared_state": 1,
            "full_mirbuilder_new_claim": 0,
            "first_edge_result_is_derived": 1,
            "generated_hako_change": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "mainline_selected": 0,
            "source_selfhost_claim": 0,
        },
    }


def verify_result(plan: dict[str, Any], result: dict[str, Any]) -> None:
    if "first_unsupported_edge" in plan:
        raise SelectionError("plan must not duplicate first_unsupported_edge")
    if plan["execution_profile"]["kind"] != "PreparedMirBuilderStateV1":
        raise SelectionError("entry profile must be PreparedMirBuilderStateV1")
    if plan["explicit_non_claims"].get("bundle_size_as_proof") != 0:
        raise SelectionError("bundle size must not be a capability proof")
    first = result["first_unsupported_edge"]
    # Derive the expected frontier from the first provider=None edge (SSOT),
    # so this check self-tracks edge changes. The guard holds the independent pin.
    frontier_edge = next(
        (e for e in plan["ordered_source_edges"] if e.get("provider") is None), None
    )
    if frontier_edge is None:
        raise SelectionError("ordered_source_edges has no provider=None frontier edge")
    unsupported = frontier_edge["unsupported"]
    expected = {
        "callsite": frontier_edge["callsite"],
        "deny_reason": unsupported["deny_reason"],
        "deny_detail": unsupported["deny_detail"],
        "semantic_owner": unsupported["semantic_owner"],
        "next_slice_token": unsupported["next_slice_token"],
    }
    for key, value in expected.items():
        if first.get(key) != value:
            raise SelectionError(f"first unsupported edge expected {key}={value}, got {first.get(key)}")
    statuses = [row["status"] for row in result["reached_prefix"]]
    if statuses != [
        "Available",
        "Available",
        "Available",
        "ProfileExcluded",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Available",
        "Unsupported",
    ]:
        raise SelectionError(f"unexpected reached frontier statuses: {statuses}")
    for row in result["not_reached_edges"]:
        if row.get("status") != "NotReached":
            raise SelectionError("all edges after first Unsupported must be NotReached")


def run(check: bool) -> None:
    plan = build_plan()
    result = analyze_frontier(plan)
    verify_result(plan, result)

    plan_text = stable_json(plan)
    result_text = stable_json(result)
    if check:
        if not PLAN_PATH.exists() or not RESULT_PATH.exists():
            raise SelectionError("selection fixtures missing; run without --check")
        if PLAN_PATH.read_text() != plan_text:
            raise SelectionError(f"{rel(PLAN_PATH)} is stale")
        if RESULT_PATH.read_text() != result_text:
            raise SelectionError(f"{rel(RESULT_PATH)} is stale")
    else:
        FIXTURES.mkdir(parents=True, exist_ok=True)
        PLAN_PATH.write_text(plan_text)
        RESULT_PATH.write_text(result_text)

    first = result["first_unsupported_edge"]
    print("output_contract=rust-lifecycle-mirbuilder-minimal-execution-path-selection-v0")
    print("entry_is_prepared_state=1")
    print("full_mirbuilder_new_claim=0")
    print(f"first_unsupported_edge={first['callsite']}")
    print(f"deny_reason={first['deny_reason']}")
    print(f"deny_detail={first['deny_detail']}")
    print(f"next_slice_token={first['next_slice_token']}")
    print("generated_hako_change=0")
    print("runtime_fallback=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except SelectionError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
