#!/usr/bin/env python3
"""Define nominal ID domain boundaries for bounded ID scalar owner edges."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-id-domain-boundary-basis-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-ID-DOMAIN-BOUNDARY-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001"

PRIORITY = FIXTURES / "mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-002-v0.json"
FILE_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    file_boundary = read_json(FILE_BOUNDARY)
    directability = read_json(DIRECTABILITY)
    bounded_owners = {
        row["owner_edge_id"]
        for row in file_boundary.get("boundary_rows") or []
        if row.get("native_seed_file_boundary_derivable")
    }

    counts: Counter[str] = Counter()
    owner_counts: dict[str, Counter[str]] = {owner: Counter() for owner in bounded_owners}
    for row in directability.get("rerun_rows") or []:
        owner = row.get("known_owner_edge")
        if owner not in bounded_owners:
            continue
        canonical = row["canonical_id_type"]
        counts[canonical] += 1
        owner_counts[owner][canonical] += 1

    domain_rows = []
    for canonical in sorted(counts):
        domain_rows.append(
            {
                "canonical_id_type": canonical,
                "nominal_transport": f"{canonical}AsI64",
                "directable_row_count": counts[canonical],
                "owner_edge_counts": {
                    owner: owner_counts[owner][canonical]
                    for owner in sorted(owner_counts)
                    if owner_counts[owner][canonical]
                },
                "raw_i64_interchangeability": False,
                "cross_domain_assignment": False,
                "sentinel_policy": "NoSentinelUnlessSourceEvidence",
                "reserved_id_policy": "NotUsedUnlessSourceEvidence",
                "invalid_or_missing_id_behavior": "DenyInvalidOrMissingId",
                "map_key_domain": canonical,
                "equality_ordering_semantics": "VerifierVisibleWithinNominalDomainOnly",
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarIdDomainBoundaryBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "basis_component_priority_rerun_002": rel(PRIORITY),
            "native_seed_file_boundary_basis": rel(FILE_BOUNDARY),
            "directability_rerun": rel(DIRECTABILITY),
        },
        "provenance": {
            "basis_component_priority_rerun_002_hash": sha256_file(PRIORITY),
            "native_seed_file_boundary_basis_hash": sha256_file(FILE_BOUNDARY),
            "directability_rerun_hash": sha256_file(DIRECTABILITY),
        },
        "previous_state": {
            "selected_component_id": (priority.get("decision") or {}).get("selected_component_id"),
            "selected_next_card": (priority.get("decision") or {}).get("selected_next_card"),
            "native_seed_file_boundary_derivable_count": (
                file_boundary.get("candidate_pool") or {}
            ).get("native_seed_file_boundary_derivable_count"),
        },
        "boundary_policy": {
            "nominal_transport_required": True,
            "raw_i64_interchangeability": False,
            "cross_domain_assignment": False,
            "sentinel_semantics_inferred": False,
            "reserved_id_semantics_inferred": False,
            "invalid_id_behavior_declared": True,
        },
        "domain_boundaries": domain_rows,
        "candidate_pool": {
            "bounded_owner_count": len(bounded_owners),
            "id_domain_boundary_count": len(domain_rows),
            "directable_row_count": sum(counts.values()),
            "raw_i64_interchangeability_count": 0,
            "cross_domain_assignment_count": 0,
        },
        "decision": {
            "kind": "IdDomainBoundaryBasisDefined",
            "reason_token": "IdScalarIdDomainBoundaryBasisDefined",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "manual_owner_selection": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
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
        print("mirbuilder-id-scalar-id-domain-boundary-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
