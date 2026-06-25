#!/usr/bin/env python3
"""Project the prepared-state allocation policy kernel to DerivedMainline.

This file owns the family-scoped mainline-selection plan for the prepared-state
MirBuilder allocation policy kernel. It does not select the full MirBuilder
path and does not introduce runtime fallback.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

PLAN_PATH = FIXTURES / "mirbuilder-allocation-policy-mainline-selection-plan-v0.json"
ROUTE_PATH = OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.route.json"
ARTIFACT_MANIFEST = OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.artifact.json"
ARTIFACT_HAKO = OUT_DIR / "mirbuilder_next_value_id_prepared_state_kernel.hako"
ID_ALLOC = ROOT / "src/mir/builder/utils/id_alloc.rs"

FAMILY_ID = "hakorune_mir_builder::next_value_id_prepared_state_kernel"
ROUTE_SLOT_ID = "hakorune_mir_builder.allocation_policy.next_value_id.prepared_state.v1"
SELECTED_SCOPE = "PreparedStateMirBuilderNextValueIdKernel"
SELECTED_CAPABILITY = "MirBuilderAllocationPolicy.prepared_state_next_value_id"


class SelectionError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def build_plan() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderAllocationPolicyMainlineSelectionPlanV1",
        "family_id": FAMILY_ID,
        "route_slot_id": ROUTE_SLOT_ID,
        "selected_scope": SELECTED_SCOPE,
        "selected_capability": SELECTED_CAPABILITY,
        "profiles": {
            "selfhost_mainline": {"route": "derived_hako"},
            "rust_bootstrap": {"route": "rust_bootstrap"},
            "platform_bringup": {"route": "rust_bootstrap"},
        },
        "derived_provider": {
            "contract_kind": "VerifiedFamilyArtifactContractV1",
            "artifact_manifest": rel(ARTIFACT_MANIFEST),
        },
        "rust_provider": {
            "route": "rust_bootstrap",
            "source": rel(ID_ALLOC),
            "roles": ["bootstrap", "platform_bringup", "oracle"],
        },
        "state_transition": {"from": "DerivedShadow", "to": "DerivedMainline"},
        "selection_timing": "PreExecutionArtifactGraphComposition",
        "fallback_policy": "Forbidden",
        "claims": {
            "prepared_state_policy_kernel": 1,
            "mainline_selected": 1,
            "rust_bootstrap_retained": 1,
            "full_mirbuilder_object_method": 0,
            "hako_adopted": 0,
            "native_hako_edit_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
        },
        "non_authority": {
            "callee_name": 1,
            "bundle_manifest": 1,
            "artifact_existence_alone": 1,
            "coverage_percentage": 1,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    if plan.get("kind") != "MirBuilderAllocationPolicyMainlineSelectionPlanV1":
        raise SelectionError("selection plan has wrong kind")
    if plan.get("family_id") != FAMILY_ID:
        raise SelectionError("selection plan has wrong family_id")
    if plan.get("route_slot_id") != ROUTE_SLOT_ID:
        raise SelectionError("selection plan has wrong route_slot_id")
    if plan.get("selected_scope") != SELECTED_SCOPE:
        raise SelectionError("selection plan has wrong selected_scope")
    if plan.get("selected_capability") != SELECTED_CAPABILITY:
        raise SelectionError("selection plan has wrong selected_capability")
    if plan.get("selection_timing") != "PreExecutionArtifactGraphComposition":
        raise SelectionError("selection must be pre-execution")
    if plan.get("fallback_policy") != "Forbidden":
        raise SelectionError("fallback must be forbidden")
    profiles = plan.get("profiles") or {}
    if profiles.get("selfhost_mainline", {}).get("route") != "derived_hako":
        raise SelectionError("selfhost_mainline must select derived_hako")
    for profile in ["rust_bootstrap", "platform_bringup"]:
        if profiles.get(profile, {}).get("route") != "rust_bootstrap":
            raise SelectionError(f"{profile} must select rust_bootstrap")
    if "MirBuilderAllocationPolicyApi.next_value_id/4" in plan.get("route_slot_id", ""):
        raise SelectionError("route slot must not be a callee-name key")
    claims = plan.get("claims") or {}
    expected_ones = {
        "prepared_state_policy_kernel",
        "mainline_selected",
        "rust_bootstrap_retained",
    }
    for key, value in claims.items():
        expected = 1 if key in expected_ones else 0
        if value != expected:
            raise SelectionError(f"unexpected claim {key}={value}")


def validate_artifact_manifest(plan: dict[str, Any], manifest: dict[str, Any]) -> None:
    if manifest.get("kind") != "RustDerivedHakoArtifact":
        raise SelectionError("artifact manifest has wrong kind")
    if manifest.get("family_id") != FAMILY_ID:
        raise SelectionError("artifact manifest has wrong family_id")
    if manifest.get("state") != "DerivedMainline":
        raise SelectionError("artifact must be DerivedMainline")
    if manifest.get("pilot_scope") != SELECTED_SCOPE:
        raise SelectionError("artifact has wrong pilot_scope")
    claims = manifest.get("claims") or {}
    required_claims = {
        "prepared_state_policy_kernel": 1,
        "mainline_selected": 1,
        "rust_bootstrap_retained": 1,
        "full_mirbuilder_object_method": 0,
        "hako_adopted": 0,
        "native_hako_edit_authority": 0,
        "source_selfhost_claim": 0,
        "runtime_fallback": 0,
        "new_backend_route": 0,
        "new_abi": 0,
        "new_canonical_mir_instruction": 0,
    }
    for key, value in required_claims.items():
        if claims.get(key) != value:
            raise SelectionError(f"artifact claim expected {key}={value}, got {claims.get(key)}")
    selection_input = (manifest.get("inputs") or {}).get("mainline_selection_plan") or {}
    if selection_input.get("path") != rel(PLAN_PATH):
        raise SelectionError("artifact manifest does not reference selection plan input")
    if selection_input.get("sha256") != sha256_file(PLAN_PATH):
        raise SelectionError("artifact manifest selection plan hash is stale")
    selection = manifest.get("mainline_selection") or {}
    if selection.get("route_slot_id") != plan["route_slot_id"]:
        raise SelectionError("artifact mainline_selection route_slot_id mismatch")
    if selection.get("fallback_policy") != "Forbidden":
        raise SelectionError("artifact mainline_selection fallback policy mismatch")


def build_route(plan: dict[str, Any], manifest: dict[str, Any]) -> dict[str, Any]:
    validate_plan(plan)
    validate_artifact_manifest(plan, manifest)
    hako_path = Path(manifest["output"]["hako_path"])
    hako_hash = manifest["output"]["hako_sha256"]
    if sha256_file(ROOT / hako_path) != hako_hash:
        raise SelectionError("artifact hako hash is stale")
    return {
        "schema_version": 0,
        "kind": "DerivedMainlineRouteSelectionV1",
        "family_id": FAMILY_ID,
        "route_slot_id": ROUTE_SLOT_ID,
        "selected_scope": SELECTED_SCOPE,
        "selected_capability": SELECTED_CAPABILITY,
        "selection_plan": {
            "path": rel(PLAN_PATH),
            "sha256": sha256_file(PLAN_PATH),
        },
        "artifact": {
            "manifest_path": rel(ARTIFACT_MANIFEST),
            "manifest_sha256": sha256_file(ARTIFACT_MANIFEST),
            "hako_path": manifest["output"]["hako_path"],
            "hako_sha256": hako_hash,
            "state": "DerivedMainline",
        },
        "profiles": {
            "selfhost_mainline": {
                "route": "derived_hako",
                "provider": "checked_in_generated_hako",
            },
            "rust_bootstrap": {
                "route": "rust_bootstrap",
                "provider": "src/mir/builder/utils/id_alloc.rs",
            },
            "platform_bringup": {
                "route": "rust_bootstrap",
                "provider": "src/mir/builder/utils/id_alloc.rs",
            },
        },
        "selected_route_closure": [
            {
                "symbol": "MirBuilderAllocationPolicyApi.next_value_id/4",
                "classification": "SameArtifactHako",
            },
            {
                "symbol": "FunctionValueIdCounterStateApi.next/1",
                "classification": "SameArtifactHako",
            },
            {
                "symbol": "ReservedValueIdMembershipViewApi.has/2",
                "classification": "SameArtifactHako",
            },
            {
                "symbol": "CoreContextApi.next_value/1",
                "classification": "SameArtifactHako",
            },
            {
                "symbol": "ValueIdOrderedMapBox",
                "classification": "AllowedHostSubstrate",
            },
        ],
        "fallback_policy": "Forbidden",
        "claims": {
            "mainline_selected": 1,
            "runtime_try_hako_then_rust_fallback": 0,
            "source_selfhost_claim": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
    }


def resolve_family_route(route: dict[str, Any], *, profile: str, route_slot_id: str) -> dict[str, Any]:
    if route.get("route_slot_id") != route_slot_id:
        raise SelectionError("route slot mismatch")
    profiles = route.get("profiles") or {}
    selected = profiles.get(profile)
    if selected is None:
        raise SelectionError(f"unknown profile: {profile}")
    if route.get("fallback_policy") != "Forbidden":
        raise SelectionError("fallback must be forbidden")
    return selected


def verify_route(route: dict[str, Any]) -> None:
    if route.get("kind") != "DerivedMainlineRouteSelectionV1":
        raise SelectionError("route projection has wrong kind")
    if route.get("route_slot_id") != ROUTE_SLOT_ID:
        raise SelectionError("route projection has wrong slot")
    if route.get("fallback_policy") != "Forbidden":
        raise SelectionError("route fallback must be forbidden")
    closure = route.get("selected_route_closure") or []
    forbidden = [
        row for row in closure if row.get("classification") == "ForbiddenRustSemanticDependency"
    ]
    if forbidden:
        raise SelectionError(f"route closure has forbidden Rust dependency: {forbidden}")
    selfhost = resolve_family_route(route, profile="selfhost_mainline", route_slot_id=ROUTE_SLOT_ID)
    bootstrap = resolve_family_route(route, profile="rust_bootstrap", route_slot_id=ROUTE_SLOT_ID)
    if selfhost.get("route") != "derived_hako":
        raise SelectionError("selfhost_mainline did not resolve to derived_hako")
    if bootstrap.get("route") != "rust_bootstrap":
        raise SelectionError("rust_bootstrap did not resolve to rust_bootstrap")
    if selfhost.get("route") == bootstrap.get("route"):
        raise SelectionError("selfhost and bootstrap routes must stay distinct")
    claims = route.get("claims") or {}
    if claims.get("runtime_try_hako_then_rust_fallback") != 0:
        raise SelectionError("runtime fallback claim must be 0")


def run(*, check: bool) -> None:
    plan = build_plan()
    validate_plan(plan)
    plan_text = stable_json(plan)

    if check:
        if not PLAN_PATH.exists() or PLAN_PATH.read_text() != plan_text:
            raise SelectionError(f"{rel(PLAN_PATH)} is stale")
    else:
        write_if_changed(PLAN_PATH, plan_text)

    manifest = read_json(ARTIFACT_MANIFEST)
    route = build_route(plan, manifest)
    verify_route(route)
    route_text = stable_json(route)
    if check:
        if not ROUTE_PATH.exists() or ROUTE_PATH.read_text() != route_text:
            raise SelectionError(f"{rel(ROUTE_PATH)} is stale")
    else:
        write_if_changed(ROUTE_PATH, route_text)

    print("output_contract=rust-lifecycle-mirbuilder-allocation-policy-mainline-selection-v0")
    print(f"family_id={FAMILY_ID}")
    print(f"route_slot_id={ROUTE_SLOT_ID}")
    print("selfhost_mainline=derived_hako")
    print("rust_bootstrap=rust_bootstrap")
    print("artifact_state=DerivedMainline")
    print("mainline_selected=1")
    print("fallback_policy=forbidden")
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
