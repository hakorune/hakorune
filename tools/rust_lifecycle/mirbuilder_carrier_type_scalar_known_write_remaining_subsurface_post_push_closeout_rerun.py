#!/usr/bin/env python3
"""Rerun remaining Write sub-surfaces after Push scoped closeout."""

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
    / "mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-REMAINING-SUBSURFACE-"
    "POST-PUSH-CLOSEOUT-RERUN-001"
)
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PUSH_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-push-surface-direct-closeout-rerun-v0.json"
)
WRITE_POLICY_RERUN = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def policy_candidates(policy: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {
        str(row["subsurface_id"]): row
        for row in policy.get("priority_candidates") or []
    }


def build_fixture() -> dict[str, Any]:
    push = read_json(PUSH_CLOSEOUT)
    policy = read_json(WRITE_POLICY_RERUN)
    policy_rows = policy_candidates(policy)

    remaining_rows = []
    for subsurface_id in ["DeleteSurfacePolicy", "SetSurfacePolicy"]:
        row = policy_rows[subsurface_id]
        remaining_rows.append(
            {
                "subsurface_id": subsurface_id,
                "routes": row.get("routes"),
                "normalized_result_class": row.get("normalized_result_class"),
                "publication_class": row.get("publication_class"),
                "mutation_class": row.get("mutation_class"),
                "hako_adopted": False,
                "basis_selection_eligible": False,
                "blocked_by": [
                    "NoHakoAdoptedWriteSubsurfacePilot",
                    "NoConsultationApprovedNextWritePilotProofAxis",
                ],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteRemainingSubsurfacePostPushCloseoutRerunV1",
        "token": TOKEN,
        "input_state": {
            "write_push_surface_direct_closeout_rerun": rel(PUSH_CLOSEOUT),
            "write_push_surface_direct_closeout_hash": sha256_file(PUSH_CLOSEOUT),
            "write_result_policy_rerun": rel(WRITE_POLICY_RERUN),
            "write_result_policy_rerun_hash": sha256_file(WRITE_POLICY_RERUN),
            "accepted_scoped_closeout_count": push.get("summary", {}).get(
                "accepted_scoped_closeout_count"
            ),
            "remaining_write_subsurface_count": push.get("summary", {}).get(
                "remaining_write_subsurface_count"
            ),
        },
        "remaining_subsurfaces": remaining_rows,
        "selector_rule": {
            "name": "WriteRemainingSubsurfacePostPushCloseoutRerunSelectorV1",
            "manual_subsurface_selection": False,
            "route_count_as_proof": False,
            "apparent_simplicity_as_proof": False,
            "accepted_read_contract_similarity_as_proof": False,
            "if_zero_hako_adopted_remaining_subsurfaces_keep_stopped": True,
            "next_pilot_requires_design_consultation": True,
        },
        "summary": {
            "write_remaining_subsurface_post_push_closeout_rerun": 1,
            "remaining_write_subsurface_count": len(remaining_rows),
            "hako_adopted_remaining_write_subsurface_count": 0,
            "basis_selection_eligible_subsurface_count": 0,
            "selected_write_subsurface_count": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "NoHakoAdoptedRemainingWriteSubsurfacePilot",
            "recommended_consultation_topic": "WriteRemainingSubsurfaceHakoPilotSelection",
            "selected_next_card": DESIGN_STOP,
            "selected_subsurface": None,
        },
        "claims": {
            "write_remaining_subsurface_post_push_closeout_rerun": 1,
            "remaining_write_subsurface_count": len(remaining_rows),
            "hako_adopted_remaining_write_subsurface_count": 0,
            "basis_selection_eligible_subsurface_count": 0,
            "write_subsurface_selected": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subsurface_selection": 0,
            "row_count_as_proof": 0,
            "route_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-remaining-subsurface-post-push-closeout-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
