#!/usr/bin/env python3
"""Resolve strict-deny near-miss clusters after descriptor closeout.

The resolver consumes the diagnostic near-miss probe and the Source Selfhost
family guard manifest. It excludes near-miss clusters whose shape signature is
already covered by a projection descriptor, then either selects exactly one
unclosed cluster or keeps the Source Selfhost design stop active.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PROBE = FIXTURES / "mirbuilder-strict-deny-near-miss-diagnostic-probe-v0.json"
MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-deny-near-miss-cluster-resolution-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

TOKEN = "MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def collect_shape_signatures(value: Any, output: set[str]) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "shape_signature" and isinstance(child, str):
                output.add(child)
            collect_shape_signatures(child, output)
    elif isinstance(value, list):
        for child in value:
            collect_shape_signatures(child, output)


def projection_descriptor_shape_index(manifest: dict[str, Any]) -> dict[str, list[str]]:
    index: dict[str, list[str]] = {}
    for row in manifest.get("rows", []):
        token = row.get("token", "")
        if "PROJECTION-POLICY" not in token:
            continue
        fixture = row.get("fixture")
        if not fixture:
            continue
        path = ROOT / fixture
        if not path.exists():
            continue
        try:
            data = read_json(path)
        except json.JSONDecodeError:
            continue
        shapes: set[str] = set()
        collect_shape_signatures(data, shapes)
        for shape in shapes:
            index.setdefault(shape, []).append(token)
    return {shape: sorted(tokens) for shape, tokens in sorted(index.items())}


def build_resolution() -> dict[str, Any]:
    probe = read_json(PROBE)
    manifest = read_json(MANIFEST)
    shape_index = projection_descriptor_shape_index(manifest)
    covered_shapes = set(shape_index)

    eligible = [row for row in probe.get("clusters", []) if row.get("selection_eligible")]
    unclosed = [row for row in eligible if row.get("shape_signature") not in covered_shapes]
    excluded = [
        {
            "cluster_id": row.get("cluster_id"),
            "shape_signature": row.get("shape_signature"),
            "candidate_count": row.get("candidate_count"),
            "covered_by_tokens": shape_index.get(row.get("shape_signature"), [])[:5],
        }
        for row in eligible
        if row.get("shape_signature") in covered_shapes
    ]

    if len(unclosed) == 1:
        selected = unclosed[0]
        decision = {
            "kind": "SelectNearMissProjectionPolicyCluster",
            "reason_token": "ExactlyOneUnclosedNearMissProjectionPolicyCluster",
            "selected_cluster_id": selected.get("cluster_id"),
            "selected_next_card": "MIRBUILDER-NEAR-MISS-PROJECTION-POLICY-001",
        }
    elif len(unclosed) > 1:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "AmbiguousUnclosedNearMissProjectionPolicyClusters",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUnclosedNearMissProjectionPolicyCluster",
            "selected_cluster_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictDenyNearMissClusterResolutionV1",
        "token": TOKEN,
        "input_authority": {
            "near_miss_probe": rel(PROBE),
            "source_selfhost_family_guard_manifest": rel(MANIFEST),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "near_miss_probe_hash": sha256_file(PROBE),
            "source_selfhost_family_guard_manifest_hash": sha256_file(MANIFEST),
        },
        "descriptor_coverage": {
            "covered_shape_signature_count": len(covered_shapes),
            "projection_descriptor_shape_index": shape_index,
        },
        "candidate_pool": {
            "eligible_near_miss_cluster_count": len(eligible),
            "excluded_existing_descriptor_cluster_count": len(excluded),
            "unclosed_near_miss_cluster_count": len(unclosed),
        },
        "excluded_existing_descriptor_clusters": excluded[:80],
        "unclosed_near_miss_clusters": unclosed[:80],
        "decision": decision,
        "claims": {
            "near_miss_probe_consumed": 1,
            "projection_descriptor_ledger_consumed": 1,
            "manual_cluster_selection": 0,
            "cluster_size_as_proof": 0,
            "strict_rules_changed": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify the checked-in resolution fixture.")
    args = parser.parse_args()

    output = stable_json(build_resolution())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-strict-deny-near-miss-cluster-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
