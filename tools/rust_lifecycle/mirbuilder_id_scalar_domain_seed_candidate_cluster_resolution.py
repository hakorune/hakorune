#!/usr/bin/env python3
"""Resolve ID scalar seed-candidate owner clusters after rerun 009."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
RERUN = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN)
    owner_edges = rerun.get("directable_owner_edges") or []

    clusters: list[dict[str, Any]] = []
    for row in owner_edges:
        owner_edge_id = row["owner_edge_id"]
        # Current evidence quality is intentionally coarse: all rows are
        # FixtureMapped, nominal-ID transport only, and no seed materialization
        # evidence exists yet.
        priority_tuple = [
            0,  # owner_edge_confidence = FixtureMapped
            0,  # nominal ID scalar transport present
            0,  # no raw i64 / object layout claim
            owner_edge_id,
        ]
        clusters.append(
            {
                "owner_edge_id": owner_edge_id,
                "directable_row_count": row.get("directable_row_count"),
                "canonical_id_type_counts": row.get("canonical_id_type_counts") or {},
                "selection_eligible": True,
                "priority_tuple": priority_tuple,
                "blocked_by": ["NoUniqueEvidenceQualityDiscriminator"],
            }
        )

    eligible = [row for row in clusters if row["selection_eligible"]]
    unique_evidence_tuple_count = len({tuple(row["priority_tuple"][:3]) for row in eligible})
    if len(eligible) == 1:
        owner = eligible[0]["owner_edge_id"]
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "reason_token": "ExactlyOneIdScalarOwnerEdgeCluster",
            "selected_owner_edge_id": owner,
            "selected_next_card": f"MIRBUILDER-{owner.split('::')[-1].upper().replace('_', '-')}-HAKO-NATIVE-SOURCE-SEED-001",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "MultipleEqualEvidenceIdScalarOwnerEdgeClusters",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainSeedCandidateClusterResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "native_owner_seed_capability_survey_rerun_009": rel(RERUN),
        },
        "provenance": {
            "native_owner_seed_capability_survey_rerun_009_hash": sha256_file(RERUN),
        },
        "input_decision": rerun.get("decision"),
        "clusters": clusters,
        "summary": {
            "input_directable_owner_edge_count": len(owner_edges),
            "selection_eligible_cluster_count": len(eligible),
            "unique_evidence_quality_tuple_count": unique_evidence_tuple_count,
            "selected_cluster_count": 0 if decision["kind"] == "KeepStopped" else 1,
        },
        "selection_rule": {
            "cluster_size_as_proof": False,
            "lexical_tiebreaker_allowed_for_seed_selection": False,
            "manual_owner_selection": False,
            "equal_evidence_quality_keeps_stopped": True,
        },
        "decision": decision,
        "claims": {
            "native_owner_seed_capability_survey_rerun_009_consumed": 1,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "manual_family_selection": 0,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
        print("mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
