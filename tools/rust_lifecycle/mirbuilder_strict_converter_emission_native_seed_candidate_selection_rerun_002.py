#!/usr/bin/env python3
"""Rerun strict native-seed candidate selection with BridgePolicyV2."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
BRIDGE_V2 = FIXTURES / "mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"
SCOPE = FIXTURES / "mirbuilder-forbidden-nonclaim-boundary-scope-resolution-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json"

TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def owner_slug(owner_edge_id: str) -> str:
    return owner_edge_id.split("::")[-1].replace("_", "-").upper()


def seed_card_for(owner_edge_id: str) -> str:
    return f"MIRBUILDER-{owner_slug(owner_edge_id)}-HAKO-NATIVE-SOURCE-SEED-001"


def build_fixture() -> dict[str, Any]:
    bridge_v2 = read_json(BRIDGE_V2)
    scope = read_json(SCOPE)
    rows = []

    for row in scope.get("owner_edge_rows") or []:
        eligible = row.get("resolved_bridge_state") == "BridgePolicyV2Candidate"
        owner = row["owner_edge_id"]
        priority_tuple = [
            1,  # FixtureMapped owner confidence, matching previous strict bridge candidates.
            0,  # verifier/oracle/guard present by upstream verifier result.
            0,  # no required forbidden non-claim after scope resolution.
            owner,
        ]
        rows.append(
            {
                "owner_edge_id": owner,
                "verifier_result_fixture": row["verifier_result_fixture"],
                "bridge_state_after_bridge_policy_v2": "BridgeEligible" if eligible else "BridgeBlocked",
                "blocked_by_after_bridge_policy_v2": [] if eligible else row.get("input_blocked_by", []),
                "priority_tuple": priority_tuple,
                "next_card": seed_card_for(owner) if eligible else None,
            }
        )

    eligible_rows = sorted(
        [row for row in rows if row["bridge_state_after_bridge_policy_v2"] == "BridgeEligible"],
        key=lambda row: row["priority_tuple"],
    )
    selected = eligible_rows[0] if eligible_rows else None

    if selected:
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "reason_token": "BridgePolicyV2StrictEmissionCandidateSelected",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoBridgeEligibleCandidateAfterBridgePolicyV2",
            "selected_owner_edge_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerun002V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "bridge_policy_v2": rel(BRIDGE_V2),
            "forbidden_nonclaim_boundary_scope_resolution": rel(SCOPE),
        },
        "provenance": {
            "bridge_policy_v2_hash": sha256_file(BRIDGE_V2),
            "forbidden_nonclaim_boundary_scope_resolution_hash": sha256_file(SCOPE),
        },
        "candidate_pool": {
            "input_owner_edge_count": len(rows),
            "bridge_eligible_after_bridge_policy_v2_count": len(eligible_rows),
            "bridge_blocked_after_bridge_policy_v2_count": len(rows) - len(eligible_rows),
            "selected_candidate_count": 1 if selected else 0,
        },
        "candidate_rows": rows,
        "decision": decision,
        "claims": {
            "bridge_policy_v2_consumed": 1,
            "forbidden_nonclaim_boundary_scope_resolution_consumed": 1,
            "manual_family_selection": 0,
            "seed_eligibility_from_forbidden_nonclaim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
            "new_python_semantic_projector": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
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
        print("mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
