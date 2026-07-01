#!/usr/bin/env python3
"""Repair owner edges for ID-scalar directability rows."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_READINESS = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002"

READINESS = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-v0.json"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
OTHER_OWNER_REPAIR = FIXTURES / "mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def repair_required_rows(directability: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted(
        [
            row
            for row in directability.get("rerun_rows", [])
            if row.get("directability_state") == "OwnerEdgeRepairRequired"
        ],
        key=lambda row: row["source_id"],
    )


def build_fixture() -> dict[str, Any]:
    readiness = read_json(READINESS)
    directability = read_json(DIRECTABILITY)
    other_owner_repair = read_json(OTHER_OWNER_REPAIR)

    repair_index = {
        row["source_id"]: row
        for row in other_owner_repair.get("repaired_rows", [])
    }

    repaired_rows: list[dict[str, Any]] = []
    unrepaired_rows: list[dict[str, Any]] = []
    for row in repair_required_rows(directability):
        source_id = row["source_id"]
        repair = repair_index.get(source_id)
        output_row = {
            "source_id": source_id,
            "canonical_id_type": row.get("canonical_id_type"),
            "nominal_transport": row.get("nominal_transport"),
            "input_known_owner_edge": row.get("known_owner_edge"),
            "input_owner_edge_confidence": row.get("owner_edge_confidence"),
            "input_blocked_by": row.get("blocked_by") or [],
            "repair_authority": rel(OTHER_OWNER_REPAIR),
        }
        if repair:
            output_row.update(
                {
                    "repaired_owner_edge_id": repair.get("repaired_known_owner_edge"),
                    "repaired_owner_edge_confidence": repair.get("repaired_owner_edge_confidence"),
                    "source_path": repair.get("source_path"),
                    "symbol": repair.get("symbol"),
                    "repair_reason_token": repair.get("repair_reason_token"),
                    "repair_state": "Repaired",
                }
            )
            repaired_rows.append(output_row)
        else:
            output_row.update(
                {
                    "repaired_owner_edge_id": None,
                    "repaired_owner_edge_confidence": "None",
                    "repair_reason_token": "NoOtherOwnerEdgeRepairRow",
                    "repair_state": "Unrepaired",
                }
            )
            unrepaired_rows.append(output_row)

    owner_counts = Counter(row["repaired_owner_edge_id"] for row in repaired_rows)
    id_type_counts = Counter(row["canonical_id_type"] for row in repaired_rows)

    if unrepaired_rows:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "IdScalarOwnerEdgeRepairIncomplete",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }
    else:
        decision = {
            "kind": "SelectSeedReadinessResolutionRerun",
            "reason_token": "IdScalarOwnerEdgeRepairComplete",
            "selected_owner_edge_id": None,
            "selected_next_card": NEXT_READINESS,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainOwnerEdgeRepairV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_readiness_resolution": rel(READINESS),
            "directability_rerun": rel(DIRECTABILITY),
            "other_owner_edge_confidence_repair": rel(OTHER_OWNER_REPAIR),
        },
        "provenance": {
            "seed_readiness_resolution_hash": sha256_file(READINESS),
            "directability_rerun_hash": sha256_file(DIRECTABILITY),
            "other_owner_edge_confidence_repair_hash": sha256_file(OTHER_OWNER_REPAIR),
        },
        "repair_policy": {
            "policy_id": "IdScalarOwnerEdgeRepairFromExistingOtherOwnerEdgeRepairV1",
            "selection_authority": "exact_source_id_match_to_other_owner_edge_confidence_repair",
            "owner_edge_confidence_allowed": ["ExactSymbol", "FixtureMapped", "FileScoped"],
            "semantic_projection_inference": 0,
            "manual_owner_selection": 0,
        },
        "preconditions": {
            "seed_readiness_decision": (readiness.get("decision") or {}).get("kind"),
            "seed_readiness_reason_token": (readiness.get("decision") or {}).get("reason_token"),
            "input_owner_edge_repair_required_count": (readiness.get("candidate_pool") or {}).get("owner_edge_repair_required_count"),
        },
        "repaired_rows": repaired_rows,
        "unrepaired_rows": unrepaired_rows,
        "summary": {
            "input_repair_required_count": len(repaired_rows) + len(unrepaired_rows),
            "repaired_row_count": len(repaired_rows),
            "unrepaired_row_count": len(unrepaired_rows),
            "distinct_repaired_owner_edge_count": len(owner_counts),
            "canonical_id_type_counts": dict(sorted(id_type_counts.items())),
            "repaired_owner_edge_counts": [
                {"owner_edge_id": owner, "count": count}
                for owner, count in sorted(owner_counts.items())
            ],
        },
        "decision": decision,
        "claims": {
            "seed_readiness_resolution_consumed": 1,
            "directability_rerun_consumed": 1,
            "other_owner_edge_confidence_repair_consumed": 1,
            "all_repair_required_rows_have_repair_attempt": 1,
            "all_repair_required_rows_repaired": 1 if not unrepaired_rows else 0,
            "manual_owner_selection": 0,
            "family_name_based_policy": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "raw_i64_interchangeability": 0,
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
        print("mirbuilder-id-scalar-domain-owner-edge-repair unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
