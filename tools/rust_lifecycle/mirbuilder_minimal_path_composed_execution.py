#!/usr/bin/env python3
"""Generate the composed minimal MirBuilder execution route.

This is a route-link artifact, not a new semantic projector. It composes the
already-verified prepared-state artifacts into one prepared-state route graph
and keeps the generated Hako surface unchanged.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import read_json, sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

REPORT_PATH = FIXTURES / "minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
ALLOC_ROUTE_PATH = (
    OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.route.json"
)
OUTPUT_PATH = OUT_DIR / "mirbuilder_minimal_path_composed_execution.route.json"

MODULE_SHELL_MANIFEST = OUT_DIR / "mir_module_minimal_shell.artifact.json"
CORE_CONTEXT_MANIFEST = OUT_DIR / "core_context.artifact.json"
FUNCTION_CONSTRUCTOR_MANIFEST = OUT_DIR / "mir_function_constructor_shell.artifact.json"
PREPARED_STATE_INSTALL_MANIFEST = OUT_DIR / "mirbuilder_prepared_state_install.artifact.json"
LITERAL_INTEGER_MANIFEST = OUT_DIR / "mirbuilder_literal_integer_lowering.artifact.json"

EXPECTED_SOURCE_PREFIX = (
    "entry.prepared_state_profile",
    "build_module.prepare_module",
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
)

EXPECTED_COMPOSITION_PREFIX = (
    "prepare_module.module_new",
    "prepare_module.next_block",
    "prepare_module.function_new",
    "prepare_module.state_install",
    "lower_root.literal_integer",
)


class CompositionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def validate_report(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    if report.get("kind") != "MinimalMirBuilderExecutionPathSemanticClosureReportV1":
        raise CompositionError("semantic closure report has wrong kind")
    closure = report.get("closure") or {}
    if closure.get("semantic_plan_closure") != "Closed":
        raise CompositionError("semantic plan closure must remain closed")
    if closure.get("rust_smoke_observation") != "Green":
        raise CompositionError("rust smoke observation must remain green")
    if closure.get("generated_hako_executable_closure") != "Open":
        raise CompositionError("generated Hako executable closure must remain open")
    if closure.get("full_path_mainline_eligible") is not False:
        raise CompositionError("full path mainline eligibility must remain false")
    if closure.get("source_selfhost_eligible") is not False:
        raise CompositionError("source selfhost eligibility must remain false")
    if closure.get("artifact_selfhost_checkpoint_complete") is not False:
        raise CompositionError("artifact selfhost checkpoint must remain open")

    edges = report.get("edges") or []
    by_id = {edge.get("edge_id"): edge for edge in edges}
    for edge_id in EXPECTED_SOURCE_PREFIX:
        if edge_id not in by_id:
            raise CompositionError(f"missing expected source edge: {edge_id}")

    expected_contract_edges = {
        "prepare_module.module_new": (
            "MirModuleMinimalShellTransport",
            "lang/generated/rust_derived/hakorune_mir_builder/mir_module_minimal_shell.artifact.json",
            "hakorune_mir::MirModuleMinimalShell",
        ),
        "prepare_module.next_block": (
            "CoreContext.scalar_counters_and_id_generators",
            "lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json",
            "hakorune_mir_builder::core_context",
        ),
        "prepare_module.function_new": (
            "MirFunctionConstructorTransport",
            "lang/generated/rust_derived/hakorune_mir_builder/mir_function_constructor_shell.artifact.json",
            "hakorune_mir::MirFunctionConstructorShell",
        ),
        "prepare_module.state_install": (
            "PreparedStateInstall",
            "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_prepared_state_install.artifact.json",
            "hakorune_mir_builder::prepared_state_install",
        ),
        "lower_root.literal_integer": (
            "LiteralIntegerLowering",
            "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json",
            "hakorune_mir_builder::literal_integer_lowering",
        ),
    }

    for edge_id, (capability, manifest_path, family_id) in expected_contract_edges.items():
        edge = by_id[edge_id]
        if edge.get("status") != "Available":
            raise CompositionError(f"edge is not available: {edge_id}")
        if edge.get("required_capability") != capability:
            raise CompositionError(f"edge capability drift: {edge_id}")
        if edge.get("evidence_tier") != "VerifiedArtifact":
            raise CompositionError(f"edge evidence tier drift: {edge_id}")
        if edge.get("artifact_materialization") != "ExecutableArtifactPresent":
            raise CompositionError(f"edge must remain executable artifact present: {edge_id}")
        if edge.get("route_state") != "DerivedShadow":
            raise CompositionError(f"edge route state drift: {edge_id}")
        provider = edge.get("provider_reference") or {}
        if provider.get("contract_kind") != "VerifiedFamilyArtifactContractV1":
            raise CompositionError(f"edge contract kind drift: {edge_id}")
        if provider.get("family_id") != family_id:
            raise CompositionError(f"edge family drift: {edge_id}")
        if provider.get("manifest_path") != manifest_path:
            raise CompositionError(f"edge manifest drift: {edge_id}")

    if by_id["entry.prepared_state_profile"].get("provider_reference", {}).get("kind") != "ExecutionProfile":
        raise CompositionError("prepared-state profile provider drift")
    if by_id["build_module.prepare_module"].get("provider_reference", {}).get("kind") != "LiveSourceOrder":
        raise CompositionError("build_module source-order provider drift")

    return by_id


def validate_artifact_manifest(path: Path, *, family_id: str, pilot_scope: str, expected_state: str = "DerivedShadow") -> dict[str, Any]:
    manifest = read_json(path)
    if manifest.get("kind") != "RustDerivedHakoArtifact":
        raise CompositionError(f"{path.name} has wrong manifest kind")
    if manifest.get("family_id") != family_id:
        raise CompositionError(f"{path.name} family drift")
    if manifest.get("state") != expected_state:
        raise CompositionError(f"{path.name} state drift")
    if manifest.get("pilot_scope") != pilot_scope:
        raise CompositionError(f"{path.name} pilot scope drift")
    output = manifest.get("output") or {}
    if output.get("hako_path") is None:
        raise CompositionError(f"{path.name} missing hako output path")
    hako_path = ROOT / output["hako_path"]
    if not hako_path.exists():
        raise CompositionError(f"{path.name} hako output missing: {hako_path}")
    if output.get("hako_sha256") != sha256_file(hako_path):
        raise CompositionError(f"{path.name} hako hash is stale")
    claims = manifest.get("claims") or {}
    for key in [
        "mainline_selected",
        "source_selfhost_claim",
        "runtime_fallback",
        "new_backend_route",
        "new_abi",
    ]:
        if claims.get(key, 0) != 0:
            raise CompositionError(f"{path.name} must keep {key}=0")
    if claims.get("generated_hako_manual_edit", 0) != 0:
        raise CompositionError(f"{path.name} must keep generated_hako_manual_edit=0")
    return manifest


def build_route() -> dict[str, Any]:
    report = read_json(REPORT_PATH)
    edges = validate_report(report)
    allocation_route = read_json(ALLOC_ROUTE_PATH)
    if allocation_route.get("kind") != "DerivedMainlineRouteSelectionV1":
        raise CompositionError("allocation policy route has wrong kind")
    if allocation_route.get("route_slot_id") != "hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1":
        raise CompositionError("allocation policy route slot drift")
    if allocation_route.get("fallback_policy") != "Forbidden":
        raise CompositionError("allocation policy route must stay forbidden")
    alloc_claims = allocation_route.get("claims") or {}
    if alloc_claims.get("mainline_selected") != 1:
        raise CompositionError("allocation policy route must stay mainline selected")
    if alloc_claims.get("runtime_try_hako_then_rust_fallback") != 0:
        raise CompositionError("allocation policy route must not allow fallback")
    if alloc_claims.get("new_backend_route") != 0 or alloc_claims.get("new_abi") != 0:
        raise CompositionError("allocation policy route must not add backend or ABI")

    manifests = {
        "prepare_module.module_new": validate_artifact_manifest(
            MODULE_SHELL_MANIFEST,
            family_id="hakorune_mir::MirModuleMinimalShell",
            pilot_scope="MirModuleMinimalShell_new_only",
        ),
        "prepare_module.next_block": validate_artifact_manifest(
            CORE_CONTEXT_MANIFEST,
            family_id="hakorune_mir_builder::core_context",
            pilot_scope="CoreContext_scalar_counters_and_id_generators",
        ),
        "prepare_module.function_new": validate_artifact_manifest(
            FUNCTION_CONSTRUCTOR_MANIFEST,
            family_id="hakorune_mir::MirFunctionConstructorShell",
            pilot_scope="MirFunctionConstructorShell_new_only",
        ),
        "prepare_module.state_install": validate_artifact_manifest(
            PREPARED_STATE_INSTALL_MANIFEST,
            family_id="hakorune_mir_builder::prepared_state_install",
            pilot_scope="PreparedStateInstall_only",
        ),
        "lower_root.literal_integer": validate_artifact_manifest(
            LITERAL_INTEGER_MANIFEST,
            family_id="hakorune_mir_builder::literal_integer_lowering",
            pilot_scope="LiteralIntegerLowering_prepared_state_only",
        ),
    }

    composition_prefix = []
    for edge_id in EXPECTED_COMPOSITION_PREFIX:
        edge = edges[edge_id]
        provider = edge.get("provider_reference") or {}
        manifest = manifests[edge_id]
        composition_prefix.append(
            {
                "edge_id": edge_id,
                "callsite": edge["callsite"],
                "required_capability": edge["required_capability"],
                "evidence_tier": edge["evidence_tier"],
                "artifact_materialization": edge["artifact_materialization"],
                "route_state": edge["route_state"],
                "provider_reference": {
                    "contract_kind": provider.get("contract_kind"),
                    "family_id": provider.get("family_id"),
                    "manifest_path": provider.get("manifest_path"),
                    "manifest_sha256": sha256_file(ROOT / provider["manifest_path"]),
                    "artifact_state": provider.get("artifact_state"),
                },
                "artifact_manifest": {
                    "path": provider["manifest_path"],
                    "sha256": sha256_file(ROOT / provider["manifest_path"]),
                    "state": manifest.get("state"),
                    "pilot_scope": manifest.get("pilot_scope"),
                },
            }
        )

    selected_existing_contracts = [
        {
            "edge_id": edge_id,
            "family_id": manifests[edge_id].get("family_id"),
            "manifest_path": rel(path),
            "manifest_sha256": sha256_file(path),
            "hako_path": manifests[edge_id].get("output", {}).get("hako_path"),
            "hako_sha256": manifests[edge_id].get("output", {}).get("hako_sha256"),
            "state": manifests[edge_id].get("state"),
            "pilot_scope": manifests[edge_id].get("pilot_scope"),
            "capability": edges[edge_id].get("required_capability"),
            "contract_kind": edges[edge_id].get("provider_reference", {}).get("contract_kind"),
        }
        for edge_id, path in [
            ("prepare_module.module_new", MODULE_SHELL_MANIFEST),
            ("prepare_module.next_block", CORE_CONTEXT_MANIFEST),
            ("prepare_module.function_new", FUNCTION_CONSTRUCTOR_MANIFEST),
            ("prepare_module.state_install", PREPARED_STATE_INSTALL_MANIFEST),
            ("lower_root.literal_integer", LITERAL_INTEGER_MANIFEST),
        ]
    ]

    route = {
        "schema_version": 0,
        "kind": "MinimalMirBuilderComposedExecutionRouteV1",
        "family_id": "hakorune_mir_builder::minimal_path_composed_execution",
        "route_slot_id": "hakorune_mir_builder.minimal_path.composed_execution.v1",
        "selected_scope": "PreparedMirBuilderStateV1",
        "input_profile": {"ast": "ASTNode::Literal(Integer(0))"},
        "source_authority": {
            "semantic_closure_report": {
                "path": rel(REPORT_PATH),
                "sha256": sha256_file(REPORT_PATH),
            },
            "allocation_policy_route": {
                "path": rel(ALLOC_ROUTE_PATH),
                "sha256": sha256_file(ALLOC_ROUTE_PATH),
            },
        },
        "source_order_prefix": [
            {
                "edge_id": "entry.prepared_state_profile",
                "callsite": edges["entry.prepared_state_profile"]["callsite"],
                "required_capability": edges["entry.prepared_state_profile"][
                    "required_capability"
                ],
                "provider_kind": edges["entry.prepared_state_profile"]["provider_reference"][
                    "kind"
                ],
            },
            {
                "edge_id": "build_module.prepare_module",
                "callsite": edges["build_module.prepare_module"]["callsite"],
                "required_capability": edges["build_module.prepare_module"][
                    "required_capability"
                ],
                "provider_kind": edges["build_module.prepare_module"]["provider_reference"][
                    "kind"
                ],
            },
            *composition_prefix,
        ],
        "composition_prefix": composition_prefix,
        "same_state_handoff": {
            "state_transport": "PreparedMirBuilderStateShell",
            "observed": 1,
            "selected_existing_contracts_consumed": 1,
            "fallback_to_standalone_harness": 0,
            "generated_hako_change": 0,
        },
        "dependency_routes": [
            {
                "kind": allocation_route["kind"],
                "route_slot_id": allocation_route["route_slot_id"],
                "route_path": rel(ALLOC_ROUTE_PATH),
                "route_sha256": sha256_file(ALLOC_ROUTE_PATH),
                "artifact_path": allocation_route["artifact"]["manifest_path"],
                "artifact_sha256": allocation_route["artifact"]["manifest_sha256"],
                "artifact_state": allocation_route["artifact"]["state"],
            }
        ],
        "selected_existing_contracts": selected_existing_contracts,
        "claims": {
            "generated_route_change": 1,
            "generated_hako_change": 0,
            "same_state_handoff_observed": 1,
            "selected_existing_contracts_consumed": 1,
            "semantic_recipe_recopy": 0,
            "fallback_to_standalone_harness": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "source_selfhost_claim": 0,
            "manual_next_edge_selection": 0,
        },
    }
    return route


def validate_route(route: dict[str, Any]) -> None:
    if route.get("kind") != "MinimalMirBuilderComposedExecutionRouteV1":
        raise CompositionError("route has wrong kind")
    if route.get("route_slot_id") != "hakorune_mir_builder.minimal_path.composed_execution.v1":
        raise CompositionError("route slot drift")
    if route.get("selected_scope") != "PreparedMirBuilderStateV1":
        raise CompositionError("selected scope drift")
    if route.get("input_profile", {}).get("ast") != "ASTNode::Literal(Integer(0))":
        raise CompositionError("input profile drift")
    source_prefix = route.get("source_order_prefix") or []
    if [row.get("edge_id") for row in source_prefix] != list(EXPECTED_SOURCE_PREFIX):
        raise CompositionError("source order prefix drift")
    composition_prefix = route.get("composition_prefix") or []
    if [row.get("edge_id") for row in composition_prefix] != list(EXPECTED_COMPOSITION_PREFIX):
        raise CompositionError("composition prefix drift")
    if len(route.get("selected_existing_contracts") or []) != 5:
        raise CompositionError("route must consume five existing contracts")
    same_state_handoff = route.get("same_state_handoff") or {}
    if same_state_handoff.get("state_transport") != "PreparedMirBuilderStateShell":
        raise CompositionError("prepared state transport drift")
    if same_state_handoff.get("observed") != 1:
        raise CompositionError("same-state handoff must be observed")
    if same_state_handoff.get("selected_existing_contracts_consumed") != 1:
        raise CompositionError("selected existing contracts must be consumed")
    if same_state_handoff.get("fallback_to_standalone_harness") != 0:
        raise CompositionError("standalone harness fallback must stay off")
    claims = route.get("claims") or {}
    for key in [
        "generated_route_change",
        "same_state_handoff_observed",
        "selected_existing_contracts_consumed",
    ]:
        if claims.get(key) != 1:
            raise CompositionError(f"route claim must remain 1: {key}")
    for key in [
        "generated_hako_change",
        "semantic_recipe_recopy",
        "fallback_to_standalone_harness",
        "runtime_fallback",
        "new_backend_route",
        "new_abi",
        "source_selfhost_claim",
        "manual_next_edge_selection",
    ]:
        if claims.get(key) != 0:
            raise CompositionError(f"route claim must remain 0: {key}")


def run(*, check: bool) -> None:
    route = build_route()
    validate_route(route)
    route_text = stable_json(route)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text() != route_text:
            raise CompositionError(f"{rel(OUTPUT_PATH)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, route_text)

    print("output_contract=rust-lifecycle-minimal-path-composed-execution-route-v0")
    print("route_guard=green")
    print(f"route_slot_id={route['route_slot_id']}")
    print("same_state_handoff_observed=1")
    print("selected_existing_contracts_consumed=1")
    print("generated_route_change=1")
    print("generated_hako_change=0")
    print("runtime_fallback=0")
    print("new_backend_route=0")
    print("new_abi=0")
    print("source_selfhost_claim=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except CompositionError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
