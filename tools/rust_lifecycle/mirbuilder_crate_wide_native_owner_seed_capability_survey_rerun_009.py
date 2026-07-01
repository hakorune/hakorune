#!/usr/bin/env python3
"""Rerun native-owner seed capability after ID scalar directability unlock."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json"

TOKEN = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CLUSTER = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    directability = read_json(DIRECTABILITY)
    rows = directability.get("rerun_rows") or []
    directable = [
        row for row in rows
        if row.get("directability_state") == "DirectableWithNominalIdScalarTransport"
    ]
    repair = [
        row for row in rows
        if row.get("directability_state") == "OwnerEdgeRepairRequired"
    ]

    owner_counts = Counter(row.get("known_owner_edge") or "<none>" for row in directable)
    type_counts = Counter(row.get("canonical_id_type") or "<unsupported>" for row in directable)
    owner_edges = sorted(owner_counts)
    candidate_owner_edges = [
        {
            "owner_edge_id": owner,
            "directable_row_count": owner_counts[owner],
            "canonical_id_type_counts": dict(sorted(Counter(
                row.get("canonical_id_type") or "<unsupported>"
                for row in directable
                if (row.get("known_owner_edge") or "<none>") == owner
            ).items())),
            "selection_eligible": False,
            "blocked_by": ["MultipleDirectableOwnerEdgesRequireClusterResolution"],
        }
        for owner in owner_edges
    ]

    if len(owner_edges) == 1:
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "selected_owner_edge_id": owner_edges[0],
            "selected_next_card": f"MIRBUILDER-{owner_edges[0].split('::')[-1].upper().replace('_', '-')}-HAKO-NATIVE-SOURCE-SEED-001",
            "reason_token": "ExactlyOneIdScalarDirectableOwnerEdge",
        }
    elif owner_edges:
        decision = {
            "kind": "SelectIdScalarDomainSeedCandidateClusterResolution",
            "selected_owner_edge_id": None,
            "selected_next_card": NEXT_CLUSTER,
            "reason_token": "MultipleIdScalarDirectableOwnerEdgesRequireClusterResolution",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
            "reason_token": "NoIdScalarDirectableOwnerEdge",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV9",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_scalar_domain_transport_directability_rerun": rel(DIRECTABILITY),
        },
        "provenance": {
            "id_scalar_domain_transport_directability_rerun_hash": sha256_file(DIRECTABILITY),
        },
        "input_decision": directability.get("decision"),
        "candidate_pool": {
            "input_id_scalar_row_count": len(rows),
            "directable_row_count": len(directable),
            "owner_edge_repair_required_count": len(repair),
            "directable_owner_edge_count": len(owner_edges),
            "native_seed_candidate_count": 0,
        },
        "directable_owner_edges": candidate_owner_edges,
        "summary": {
            "directable_owner_edge_counts": dict(sorted(owner_counts.items())),
            "directable_canonical_id_type_counts": dict(sorted(type_counts.items())),
        },
        "selection_rule": {
            "exactly_one_owner_edge_required_for_seed_selection": True,
            "multiple_owner_edges_require_cluster_resolution": True,
            "cluster_size_as_proof": False,
            "manual_owner_selection": False,
            "source_selfhost_claim_allowed": False,
        },
        "decision": decision,
        "claims": {
            "id_scalar_domain_transport_directability_rerun_consumed": 1,
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
        print("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
