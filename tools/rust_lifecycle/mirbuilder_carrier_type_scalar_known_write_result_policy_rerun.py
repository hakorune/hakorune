#!/usr/bin/env python3
"""Rerun the ScalarKnown WriteResultPolicy basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"
)

TOKEN = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001"
NEXT_CARD = "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001"

WRITE_POLICY_BASIS = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-basis-v0.json"
)
RUST_BOUNDARY = (
    ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
)
WRITE_SOURCE = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
DESCRIPTORS = ROOT / "src/mir/generated/generic_method_route_descriptors.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def route_count(row: dict[str, Any]) -> int:
    return len(row.get("routes") or [])


def normalized_signature(row: dict[str, Any]) -> str:
    return "::".join(
        [
            str(row.get("normalized_result_class")),
            str(row.get("publication_class")),
            str(row.get("mutation_class")),
        ]
    )


def build_fixture() -> dict[str, Any]:
    basis = read_json(WRITE_POLICY_BASIS)
    policy = basis.get("policy") or {}
    sub_surfaces = policy.get("sub_surfaces") or []
    signatures = sorted({normalized_signature(row) for row in sub_surfaces})
    whole_direct_contract_allowed = (
        len(signatures) == 1
        and not policy.get("mixed_return_publication_decomposition", {}).get(
            "mixed_state_is_not_direct_closeout_contract"
        )
    )
    priority_candidates = [
        {
            "subsurface_id": row.get("subsurface_id"),
            "route_count": route_count(row),
            "routes": row.get("routes") or [],
            "normalized_result_class": row.get("normalized_result_class"),
            "publication_class": row.get("publication_class"),
            "mutation_class": row.get("mutation_class"),
            "future_direct_contract_split_allowed": row.get(
                "future_direct_contract_split_allowed"
            )
            is True,
            "selection_eligible_without_priority_basis": False,
            "blocked_by": ["NoWriteSubsurfacePriorityBasis"],
        }
        for row in sub_surfaces
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteResultPolicyRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_result_policy_basis": rel(WRITE_POLICY_BASIS),
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_selected_next_card": basis.get("decision", {}).get(
                "selected_next_card"
            ),
            "basis_write_result_policy_ready": basis.get("summary", {}).get(
                "write_result_policy_ready"
            ),
            "basis_write_direct_closeout_materialized": basis.get("summary", {}).get(
                "write_direct_closeout_materialized"
            ),
        },
        "provenance": {
            "write_result_policy_basis_hash": sha256_file(WRITE_POLICY_BASIS),
            "rust_boundary_hash": sha256_file(RUST_BOUNDARY),
            "write_source_hash": sha256_file(WRITE_SOURCE),
            "generated_descriptor_hash": sha256_file(DESCRIPTORS),
        },
        "selector_rule": {
            "name": "WriteResultPolicyRerunSelectorV1",
            "whole_direct_contract_requires_single_normalized_signature": True,
            "mixed_return_publication_forbids_whole_direct_contract": True,
            "subsurface_selection_requires_priority_basis": True,
            "if_multiple_subsurfaces_require_priority_basis": True,
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "source_path_as_authority": False,
            "owner_name_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "evaluated_policy": {
            "policy_id": policy.get("policy_id"),
            "target_surface_id": policy.get("target_surface_id"),
            "route_kind_set": policy.get("route_kind_set") or [],
            "subsurface_count": len(sub_surfaces),
            "normalized_signature_count": len(signatures),
            "normalized_signatures": signatures,
            "whole_direct_contract_allowed": whole_direct_contract_allowed,
            "whole_direct_contract_blocked_by": [
                "MixedReturnPublicationNotStableDirectContract",
                "MultipleWriteSubsurfaceResultPublicationSignatures",
            ],
        },
        "priority_candidates": priority_candidates,
        "summary": {
            "write_result_policy_rerun": 1,
            "write_result_policy_basis_consumed": 1,
            "write_surface_whole_direct_contract_rejected": 1,
            "write_subsurface_split_required": 1,
            "write_subsurface_priority_basis_selected": 1,
            "write_subsurface_candidate_count": len(priority_candidates),
            "whole_direct_contract_candidate_count": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteSubsurfacePriorityBasis",
            "reason_token": "MultipleWriteSubsurfacesRequirePriorityBasis",
            "selected_subsurface": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_result_policy_rerun": 1,
            "write_result_policy_basis_consumed": 1,
            "write_surface_whole_direct_contract_rejected": 1,
            "write_subsurface_split_required": 1,
            "write_subsurface_priority_basis_selected": 1,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "direct_whole_write_contract_basis": 0,
            "component_specific_direct_contract_materialized": 0,
            "hako_adoption": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "new_python_semantic_projector": 0,
            "manual_subsurface_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "route_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-carrier-type-scalar-known-write-result-policy-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
