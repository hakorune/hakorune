#!/usr/bin/env python3
"""Rerun Write surface selection after the Push .hako adoption decision."""

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
    / "mirbuilder-carrier-type-scalar-known-write-surface-post-push-adoption-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SURFACE-"
    "POST-PUSH-ADOPTION-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)

ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-hako-adoption-decision-v0.json"
)
PRIORITY_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-subsurface-priority-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def priority_rows(priority: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        str(row["subsurface_id"]): row
        for row in priority.get("candidate_subsurfaces") or []
    }


def build_fixture() -> dict[str, Any]:
    adoption = read_json(ADOPTION)
    priority = read_json(PRIORITY_RERUN)
    rows = priority_rows(priority)
    adopted = adoption.get("adoption_decision") or {}

    candidate_rows = []
    for subsurface_id in ["PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"]:
        row = rows[subsurface_id]
        is_push = subsurface_id == "PushSurfacePolicy"
        adopted_by_hako = is_push and adopted.get("hako_adopted") is True
        candidate_rows.append(
            {
                "subsurface_id": subsurface_id,
                "routes": row.get("routes"),
                "hako_adopted": adopted_by_hako,
                "parity_gate_green": adopted_by_hako,
                "implementation_pilot_ready": adopted_by_hako,
                "direct_closeout_materialized": False,
                "basis_selection_eligible": adopted_by_hako,
                "blocked_by": [] if adopted_by_hako else ["NoHakoAdoptedWriteSubsurfacePilot"],
            }
        )

    eligible = [row for row in candidate_rows if row["basis_selection_eligible"]]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSurfacePostPushAdoptionRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_push_surface_adoption_decision": rel(ADOPTION),
            "write_push_surface_adoption_hash": sha256_file(ADOPTION),
            "write_subsurface_priority_rerun": rel(PRIORITY_RERUN),
            "write_subsurface_priority_rerun_hash": sha256_file(PRIORITY_RERUN),
            "adoption_decision": adopted.get("decision"),
            "adopted_surface": adopted.get("adopted_surface"),
            "adopted_owner": adopted.get("adopted_owner"),
        },
        "candidate_subsurfaces": candidate_rows,
        "selector_rule": {
            "name": "WriteSurfacePostPushAdoptionRerunSelectorV1",
            "basis_selection_allowed_after_exactly_one_hako_adopted_write_pilot": True,
            "direct_closeout_materialization_allowed": False,
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
        },
        "summary": {
            "write_surface_post_push_adoption_rerun": 1,
            "hako_adopted_write_subsurface_count": len(eligible),
            "basis_selection_eligible_subsurface_count": len(eligible),
            "selected_write_subsurface_count": 1 if len(eligible) == 1 else 0,
            "selected_write_subsurface": eligible[0]["subsurface_id"] if len(eligible) == 1 else None,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWritePushSurfaceTypedDirectCloseoutContractBasis"
            if len(eligible) == 1
            else "KeepStopped",
            "reason_token": "ExactlyOneHakoAdoptedWriteSubsurfacePilot"
            if len(eligible) == 1
            else "NoExactlyOneHakoAdoptedWriteSubsurfacePilot",
            "selected_subsurface": eligible[0]["subsurface_id"] if len(eligible) == 1 else None,
            "selected_next_card": NEXT_CARD
            if len(eligible) == 1
            else "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "claims": {
            "write_surface_post_push_adoption_rerun": 1,
            "hako_adopted_write_subsurface_count": len(eligible),
            "basis_selection_eligible_subsurface_count": len(eligible),
            "write_subsurface_selected": 1 if len(eligible) == 1 else 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "manual_subsurface_selection": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "accepted_read_contract_similarity_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-surface-post-push-adoption-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
