#!/usr/bin/env python3
"""Rerun strict native-seed selection after typed object adoption."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"

RERUN_004 = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-004-v0.json"
TYPED_ADOPTION = FIXTURES / "mirbuilder-typed-object-plan-refresh-hako-adoption-decision-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-005-v0.json"

TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-005"
ADOPTED_OWNERS = {
    "hakorune_mir_builder::direct_state_plan_refresh",
    "hakorune_mir_builder::record_packed_layout_refresh",
    "hakorune_mir_builder::typed_object_plan_refresh",
}


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    previous = read_json(RERUN_004)
    adoption = read_json(TYPED_ADOPTION)

    if adoption["decision"]["value"] != "Adopt":
        raise SystemExit("typed_object_plan_refresh must be adopted before rerun 005")
    if adoption["family_id"] != "hakorune_mir_builder::typed_object_plan_refresh":
        raise SystemExit("typed adoption family mismatch")
    if adoption["claims"]["source_selfhost_claim"] != 0:
        raise SystemExit("typed adoption must not claim Source Selfhost")

    rows = []
    for row in previous.get("candidate_rows") or []:
        owner = row["owner_edge_id"]
        already_adopted = owner in ADOPTED_OWNERS
        input_state = row.get("input_bridge_state", row.get("bridge_state_after_bridge_policy_v2"))
        selection_eligible = input_state == "BridgeEligible" and not already_adopted
        rows.append(
            {
                "owner_edge_id": owner,
                "verifier_result_fixture": row["verifier_result_fixture"],
                "input_bridge_state": input_state,
                "already_hako_adopted": already_adopted,
                "selection_eligible_after_adoption": selection_eligible,
                "blocked_by_after_adoption": ["AlreadyHakoAdopted"] if already_adopted else row.get("blocked_by_after_adoption", []),
                "priority_tuple": row["priority_tuple"],
                "next_card": row["next_card"] if selection_eligible else None,
            }
        )

    eligible_rows = [row for row in rows if row["selection_eligible_after_adoption"]]
    decision = {
        "kind": "KeepStopped",
        "reason_token": "NoBridgeEligibleCandidateAfterTypedObjectPlanAdoption",
        "selected_owner_edge_id": None,
        "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerun005V1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "previous_rerun": rel(RERUN_004),
            "typed_object_plan_refresh_adoption": rel(TYPED_ADOPTION),
        },
        "provenance": {
            "previous_rerun_hash": sha256_file(RERUN_004),
            "typed_object_plan_refresh_adoption_hash": sha256_file(TYPED_ADOPTION),
        },
        "candidate_pool": {
            "input_owner_edge_count": len(rows),
            "already_hako_adopted_count": sum(1 for row in rows if row["already_hako_adopted"]),
            "bridge_eligible_remaining_count": len(eligible_rows),
            "bridge_blocked_remaining_count": len(rows) - sum(1 for row in rows if row["already_hako_adopted"]) - len(eligible_rows),
            "selected_candidate_count": 0,
        },
        "candidate_rows": rows,
        "decision": decision,
        "claims": {
            "previous_rerun_consumed": 1,
            "typed_object_plan_refresh_adoption_consumed": 1,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-005 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
