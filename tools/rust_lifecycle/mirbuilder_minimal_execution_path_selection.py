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
            "provider": None,
            "unsupported": {
                "deny_reason": "UnsupportedDirectShape",
                "deny_detail": "MetadataValueTypePublicationRequired",
                "semantic_owner": "MirBuilder::finalize_module metadata value type publication",
                "next_slice_token": "MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-001",
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
