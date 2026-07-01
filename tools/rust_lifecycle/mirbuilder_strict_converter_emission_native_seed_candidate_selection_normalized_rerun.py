#!/usr/bin/env python3
"""Rerun strict seed candidate selection after denied-boundary normalization."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
NORMALIZED = FIXTURES / "mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-normalized-rerun-v0.json"

TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def bridge_state_after_normalization(row: dict[str, Any]) -> tuple[str, list[str]]:
    classes = set(row.get("normalized_classes") or [])
    blocked_by: list[str] = []
    if "ForbiddenNonClaimBoundary" in classes:
        blocked_by.append("ForbiddenNonClaimBoundaryStillDenied")
    if "UnclassifiedDeniedBoundary" in classes:
        blocked_by.append("UnclassifiedDeniedBoundary")
    if not blocked_by:
        return "BridgeEligibleAfterDeniedBoundaryNormalization", []
    return "BridgeBlocked", blocked_by


def build_fixture() -> dict[str, Any]:
    normalized = read_json(NORMALIZED)
    rows = []
    forbidden_blocked = 0
    eligible = 0

    for row in normalized.get("normalized_boundary_rows") or []:
        bridge_state, blocked_by = bridge_state_after_normalization(row)
        if bridge_state != "BridgeBlocked":
            eligible += 1
        if "ForbiddenNonClaimBoundaryStillDenied" in blocked_by:
            forbidden_blocked += 1
        rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "verifier_result_fixture": row["verifier_result_fixture"],
                "normalized_classes": row["normalized_classes"],
                "bridge_state_after_normalized_rerun": bridge_state,
                "blocked_by_after_normalized_rerun": blocked_by,
                "next_card": None,
            }
        )

    if eligible == 1:
        selected = next(row for row in rows if row["bridge_state_after_normalized_rerun"] != "BridgeBlocked")
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": f"MIRBUILDER-{selected['owner_edge_id'].split('::')[-1].upper().replace('_', '-')}-HAKO-NATIVE-SOURCE-SEED-001",
            "reason_token": "ExactlyOneBridgeEligibleCandidateAfterDeniedBoundaryNormalization",
        }
    elif eligible > 1:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "AmbiguousBridgeEligibleCandidatesAfterDeniedBoundaryNormalization",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoBridgeEligibleCandidateAfterDeniedBoundaryNormalization",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionNormalizedRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "denied_boundary_vocabulary_normalization": rel(NORMALIZED),
        },
        "provenance": {
            "denied_boundary_vocabulary_normalization_hash": sha256_file(NORMALIZED),
            "input_token": normalized.get("token"),
        },
        "candidate_pool": {
            "normalized_row_count": len(rows),
            "bridge_eligible_after_normalization_count": eligible,
            "forbidden_nonclaim_blocked_count": forbidden_blocked,
            "unclassified_denied_boundary_count": normalized.get("summary", {}).get("unclassified_denied_boundary_count"),
        },
        "normalized_candidate_rows": rows,
        "decision": decision,
        "claims": {
            "denied_boundary_vocabulary_normalization_consumed": 1,
            "manual_family_selection": 0,
            "manual_boundary_reclassification": 0,
            "seed_eligibility_from_forbidden_nonclaim": 0,
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
        print("mirbuilder-strict-converter-emission-native-seed-candidate-selection-normalized-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
