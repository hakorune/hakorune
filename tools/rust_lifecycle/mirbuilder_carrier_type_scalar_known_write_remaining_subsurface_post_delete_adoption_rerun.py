#!/usr/bin/env python3
"""Rerun Write surface selection after the Delete .hako adoption decision."""

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
    / "mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-"
    "POST-DELETE-ADOPTION-RERUN-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-"
    "TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001"
)

DELETE_ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-delete-surface-hako-adoption-decision-v0.json"
)
PUSH_ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-hako-adoption-decision-v0.json"
)
PUSH_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-direct-closeout-rerun-v0.json"
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
    delete_adoption = read_json(DELETE_ADOPTION)
    push_adoption = read_json(PUSH_ADOPTION)
    push_closeout = read_json(PUSH_CLOSEOUT)
    priority = read_json(PRIORITY_RERUN)
    rows = priority_rows(priority)
    adopted = delete_adoption.get("adoption_decision") or {}
    push_adopted = push_adoption.get("adoption_decision", {}).get("hako_adopted") is True

    candidate_rows = []
    for subsurface_id in ["PushSurfacePolicy", "DeleteSurfacePolicy", "SetSurfacePolicy"]:
        row = rows[subsurface_id]
        is_push = subsurface_id == "PushSurfacePolicy"
        is_delete = subsurface_id == "DeleteSurfacePolicy"
        push_materialized = is_push and (
            push_closeout.get("claims", {}).get("write_push_surface_direct_closeout_materialized") == 1
        )
        delete_adopted = is_delete and adopted.get("hako_adopted") is True
        hako_adopted = (is_push and push_adopted) or delete_adopted
        eligible = delete_adopted and not push_materialized
        blocked_by: list[str] = []
        if push_materialized:
            blocked_by.append("AlreadyScopedDirectCloseoutMaterialized")
        elif not delete_adopted:
            blocked_by.append("NoHakoAdoptedWriteSubsurfacePilot")

        candidate_rows.append(
            {
                "subsurface_id": subsurface_id,
                "routes": row.get("routes"),
                "hako_adopted": hako_adopted,
                "parity_gate_green": hako_adopted,
                "implementation_pilot_ready": hako_adopted,
                "direct_closeout_materialized": push_materialized,
                "basis_selection_eligible": eligible,
                "blocked_by": blocked_by,
            }
        )

    eligible_rows = [row for row in candidate_rows if row["basis_selection_eligible"]]
    hako_adopted_rows = [row for row in candidate_rows if row["hako_adopted"]]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostDeleteAdoptionRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_delete_surface_adoption_decision": rel(DELETE_ADOPTION),
            "write_delete_surface_adoption_hash": sha256_file(DELETE_ADOPTION),
            "write_push_surface_adoption_decision": rel(PUSH_ADOPTION),
            "write_push_surface_adoption_hash": sha256_file(PUSH_ADOPTION),
            "write_push_surface_direct_closeout_rerun": rel(PUSH_CLOSEOUT),
            "write_push_surface_direct_closeout_hash": sha256_file(PUSH_CLOSEOUT),
            "write_subsurface_priority_rerun": rel(PRIORITY_RERUN),
            "write_subsurface_priority_rerun_hash": sha256_file(PRIORITY_RERUN),
            "adoption_decision": adopted.get("decision"),
            "adopted_surface": adopted.get("adopted_surface"),
            "adopted_owner": adopted.get("adopted_owner"),
        },
        "candidate_subsurfaces": candidate_rows,
        "selector_rule": {
            "name": "WriteRemainingSubsurfacePostDeleteAdoptionRerunSelectorV1",
            "basis_selection_allowed_after_exactly_one_hako_adopted_write_pilot": True,
            "already_materialized_scoped_closeouts_not_eligible": True,
            "direct_closeout_materialization_allowed": False,
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
        },
        "summary": {
            "write_remaining_subsurface_post_delete_adoption_rerun": 1,
            "write_delete_surface_hako_adopted": 1,
            "hako_adopted_write_subsurface_count": len(hako_adopted_rows),
            "basis_selection_eligible_subsurface_count": len(eligible_rows),
            "selected_write_subsurface_count": 1 if len(eligible_rows) == 1 else 0,
            "selected_write_subsurface": eligible_rows[0]["subsurface_id"] if len(eligible_rows) == 1 else None,
            "write_delete_surface_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectWriteDeleteSurfaceTypedDirectCloseoutContractBasis"
            if len(eligible_rows) == 1
            else "KeepStopped",
            "reason_token": "ExactlyOneHakoAdoptedWriteSubsurfacePilotNeedsScopedCloseout"
            if len(eligible_rows) == 1
            else "NoExactlyOneHakoAdoptedWriteSubsurfacePilotNeedsScopedCloseout",
            "selected_subsurface": eligible_rows[0]["subsurface_id"] if len(eligible_rows) == 1 else None,
            "selected_next_card": NEXT_CARD
            if len(eligible_rows) == 1
            else "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "claims": {
            "write_remaining_subsurface_post_delete_adoption_rerun": 1,
            "write_delete_surface_hako_adopted": 1,
            "hako_adopted_write_subsurface_count": len(hako_adopted_rows),
            "basis_selection_eligible_subsurface_count": len(eligible_rows),
            "write_subsurface_selected": 1 if len(eligible_rows) == 1 else 0,
            "write_delete_surface_direct_closeout_materialized": 0,
            "write_direct_closeout_materialized": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-delete-adoption-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
