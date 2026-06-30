#!/usr/bin/env python3
"""Resolve eligible projection-policy clusters by deterministic priority."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
OUTPUT = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"

PROXIMITY_PRIORITY = {
    "AdoptedNeighbor": 0,
    "SeedNeighbor": 1,
    "MinimalPath": 2,
    "None": 3,
}
CONTROL_FLOW_PRIORITY = {
    "StraightLine": 0,
    "StructuredLoop": 1,
    "PhiRequired": 2,
    "Unstructured": 3,
}
BORROW_PRIORITY = {
    "NoBorrow": 0,
    "NoReturnedBorrow": 1,
    "BorrowPolicyKnown": 2,
    "BorrowPolicyNeeded": 3,
    "ReturnedMutableAliasUnknown": 4,
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def card_slug(cluster: dict[str, Any]) -> str:
    shape = cluster["shape_signature"].removeprefix("shape.")
    shape = re.sub(r"[^a-zA-Z0-9]+", "_", shape).strip("_").upper()
    return f"MIRBUILDER-{shape.replace('_', '-')}-PROJECTION-POLICY-001"


def manifest_fixture_paths(family_manifest: dict[str, Any]) -> list[Path]:
    paths: list[Path] = []
    for row in family_manifest.get("rows", []):
        fixture = row.get("fixture") or ""
        if fixture:
            paths.append(ROOT / fixture)
    return paths


def decomposed_cluster_ids(family_manifest: dict[str, Any]) -> set[str]:
    cluster_ids: set[str] = set()
    for path in manifest_fixture_paths(family_manifest):
        if not path.exists() or path == OUTPUT:
            continue
        try:
            fixture = read_json(path)
        except json.JSONDecodeError:
            continue
        source_cluster_id = (fixture.get("input_state") or {}).get("source_cluster_id")
        if source_cluster_id:
            cluster_ids.add(source_cluster_id)
    return cluster_ids


def priority_tuple(cluster: dict[str, Any]) -> tuple[Any, ...]:
    return (
        PROXIMITY_PRIORITY.get(cluster["native_seed_or_adoption_proximity"], 99),
        CONTROL_FLOW_PRIORITY.get(cluster["control_flow_axis"], 99),
        BORROW_PRIORITY.get(cluster["borrow_axis"], 99),
        0 if cluster["verifier_or_oracle_state"] == "Present" else 1,
        0 if cluster["type_transport_axis"] == "Known" else 1,
        cluster["candidate_count"],
        cluster["cluster_id"],
    )


def build_resolution() -> dict[str, Any]:
    cluster_resolution = read_json(CLUSTER_RESOLUTION)
    family_manifest = read_json(FAMILY_MANIFEST)
    existing_tokens = {row["token"] for row in family_manifest.get("rows", [])}
    existing_decomposed_cluster_ids = decomposed_cluster_ids(family_manifest)
    eligible = [
        cluster for cluster in cluster_resolution["clusters"]
        if cluster.get("selection_eligible") is True
    ]
    eligible_with_cards = [
        (cluster, card_slug(cluster))
        for cluster in eligible
    ]
    excluded_existing = [
        (cluster, next_card)
        for cluster, next_card in eligible_with_cards
        if next_card in existing_tokens
        or cluster["cluster_id"] in existing_decomposed_cluster_ids
    ]
    selectable = [
        cluster
        for cluster, next_card in eligible_with_cards
        if next_card not in existing_tokens
        and cluster["cluster_id"] not in existing_decomposed_cluster_ids
    ]
    ranked = sorted(selectable, key=priority_tuple)
    selected = ranked[0] if ranked else None

    ranked_items = [
        {
            "rank": index + 1,
            "cluster_id": cluster["cluster_id"],
            "candidate_count": cluster["candidate_count"],
            "priority_tuple": list(priority_tuple(cluster)),
            "shape_signature": cluster["shape_signature"],
            "native_seed_or_adoption_proximity": cluster["native_seed_or_adoption_proximity"],
            "control_flow_axis": cluster["control_flow_axis"],
            "borrow_axis": cluster["borrow_axis"],
            "type_transport_axis": cluster["type_transport_axis"],
            "verifier_or_oracle_state": cluster["verifier_or_oracle_state"],
            "next_card": card_slug(cluster),
        }
        for index, cluster in enumerate(ranked)
    ]

    if selected:
        decision = {
            "kind": "SelectProjectionPolicyCluster",
            "selected_cluster_id": selected["cluster_id"],
            "selected_next_card": card_slug(selected),
            "reason_token": "DeterministicProjectionPolicyClusterPrioritySelected",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoEligibleProjectionPolicyCluster",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderProjectionPolicyClusterPriorityResolutionV1",
        "token": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
        "input_state": {
            "cluster_resolution": rel(CLUSTER_RESOLUTION),
            "eligible_cluster_count": len(eligible),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "provenance": {
            "cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "family_manifest_hash": sha256_file(FAMILY_MANIFEST),
        },
        "existing_decision_filter": {
            "enabled": True,
            "excluded_cluster_count": len(excluded_existing),
            "reason_token": "ProjectionPolicyDecisionAlreadyLanded",
            "source_cluster_decomposition_filter_enabled": True,
        },
        "excluded_existing_decision_clusters": [
            {
                "cluster_id": cluster["cluster_id"],
                "next_card": next_card,
                "candidate_count": cluster["candidate_count"],
                "reason_token": (
                    "ProjectionPolicySourceClusterDecompositionAlreadyLanded"
                    if cluster["cluster_id"] in existing_decomposed_cluster_ids
                    else "ProjectionPolicyDecisionAlreadyLanded"
                ),
            }
            for cluster, next_card in sorted(excluded_existing, key=lambda pair: priority_tuple(pair[0]))
        ],
        "priority_rules": [
            "native_seed_or_adoption_proximity",
            "control_flow_axis",
            "borrow_axis",
            "verifier_or_oracle_state",
            "type_transport_axis",
            "cluster_size_tiebreaker_only",
            "cluster_id_lexical_tiebreaker",
        ],
        "ranked_clusters": ranked_items,
        "summary": {
            "eligible_cluster_count": len(eligible),
            "excluded_existing_decision_cluster_count": len(excluded_existing),
            "selectable_cluster_count": len(selectable),
            "selected_cluster_id": selected["cluster_id"] if selected else None,
            "selected_candidate_count": selected["candidate_count"] if selected else 0,
            "cluster_size_as_proof": 0,
        },
        "decision": decision,
        "claims": {
            "eligible_cluster_count": len(eligible),
            "existing_decision_filter_enabled": 1,
            "source_cluster_decomposition_filter_enabled": 1,
            "excluded_existing_decision_cluster_count": len(excluded_existing),
            "selectable_cluster_count": len(selectable),
            "deterministic_priority_resolution": 1,
            "cluster_size_as_proof": 0,
            "cluster_size_tiebreaker_only": 1,
            "manual_family_selection": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_edit_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "family_name_based_policy": 0,
            "hako_emission": 0,
            "hako_adopted_decision": 0,
            "native_source_seed_materialization": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in priority resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-projection-policy-cluster-priority-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
