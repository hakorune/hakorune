#!/usr/bin/env python3
"""Define the ResultBox carrier verifier policy for selected transport rows."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
LANE = FIXTURES / "mirbuilder-carrier-type-transport-policy-lane-priority-resolution-v0.json"
EVIDENCE = FIXTURES / "mirbuilder-carrier-type-transport-evidence-inventory-v0.json"
OUTPUT = FIXTURES / "mirbuilder-result-carrier-verifier-policy-v0.json"

TOKEN = "MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-001"
NEXT = "MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    lane = read_json(LANE)
    evidence = read_json(EVIDENCE)
    selected_lane = lane["decision"]["selected_policy_lane"]
    rows = [
        row for row in evidence.get("evidence_rows", [])
        if row["normalized_policy_lane_candidate"] == selected_lane
    ]

    policy_rows: list[dict[str, Any]] = []
    for row in rows:
        verifier_path = ROOT / row["verifier_result_fixture"]
        verifier = read_json(verifier_path)
        notes = verifier.get("transport_notes") or {}
        checks = verifier.get("checks") or {}
        policy_rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "family_id": row["family_id"],
                "verifier_result_fixture": row["verifier_result_fixture"],
                "result_transport": notes.get("result_transport"),
                "projection_contract": notes.get("projection_contract"),
                "plan_kind": checks.get("plan_kind"),
                "canonical_json_parity": checks.get("canonical_json_parity"),
                "runtime_fallback": checks.get("runtime_fallback"),
                "verified_operations": verifier.get("verified_operations") or [],
            }
        )

    all_ready = all(
        row["result_transport"]
        and str(row["result_transport"]).endswith("ResultBox")
        and row["projection_contract"]
        and row["canonical_json_parity"] == 1
        and row["runtime_fallback"] == 0
        for row in policy_rows
    )
    decision = {
        "kind": "SelectResultCarrierVerifierContract",
        "reason_token": "ResultCarrierVerifierPolicyDefined",
        "selected_next_card": NEXT,
    } if all_ready else {
        "kind": "KeepStopped",
        "reason_token": "ResultCarrierVerifierPolicyEvidenceIncomplete",
        "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderResultCarrierVerifierPolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "policy_lane_priority_resolution": rel(LANE),
            "carrier_type_transport_evidence_inventory": rel(EVIDENCE),
            "selected_policy_lane": selected_lane,
        },
        "provenance": {
            "policy_lane_priority_resolution_hash": sha256_file(LANE),
            "carrier_type_transport_evidence_inventory_hash": sha256_file(EVIDENCE),
        },
        "selected_policy": {
            "policy_id": "ResultCarrierVerifierPolicyV1",
            "result_transport_suffix": "ResultBox",
            "requires_projection_contract": True,
            "requires_canonical_json_parity": True,
            "requires_runtime_fallback_zero": True,
            "hako_generation": False,
        },
        "policy_rows": policy_rows,
        "summary": {
            "selected_policy_lane": selected_lane,
            "result_carrier_candidate_count": len(policy_rows),
            "result_carrier_policy_ready": 1 if all_ready else 0,
        },
        "decision": decision,
        "claims": {
            "policy_lane_priority_resolution_consumed": 1,
            "result_carrier_verifier_policy_defined": 1 if all_ready else 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "owner_name_as_transport_policy": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
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
        print("mirbuilder-result-carrier-verifier-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
