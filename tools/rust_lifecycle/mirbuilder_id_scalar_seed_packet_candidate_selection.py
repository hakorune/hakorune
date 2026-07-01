#!/usr/bin/env python3
"""Select an ID scalar seed packet materialization candidate."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-seed-packet-candidate-selection-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
CONTRACT = FIXTURES / "mirbuilder-id-scalar-seed-evidence-contract-v0.json"

CONFIDENCE_RANK = {
    "ExactSymbol": 0,
    "FixtureMapped": 1,
    "FileScoped": 2,
    "Heuristic": 3,
    "None": 4,
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def owner_slug(owner_edge_id: str) -> str:
    return owner_edge_id.upper().replace("::", "-").replace("_", "-")


def best_confidence(confidences: list[str]) -> str:
    if not confidences:
        return "None"
    return sorted(confidences, key=lambda item: CONFIDENCE_RANK.get(item, 99))[0]


def build_fixture() -> dict[str, Any]:
    contract = read_json(CONTRACT)
    rows: list[dict[str, Any]] = []
    for row in contract.get("owner_edge_contract_rows", []):
        confidence = best_confidence(row.get("owner_edge_confidence_set") or [])
        eligible = (
            row.get("owner_edge_complete") is True
            and row.get("nominal_id_domain_isolation") == "Preserved"
            and row.get("directability_evidence_present") is True
            and row.get("seed_packet_state") == "MissingPacket"
            and confidence in {"ExactSymbol", "FixtureMapped", "FileScoped"}
        )
        priority_tuple = [
            CONFIDENCE_RANK.get(confidence, 99),
            0 if row.get("owner_edge_complete") is True else 1,
            0 if row.get("nominal_id_domain_isolation") == "Preserved" else 1,
            0 if row.get("seed_packet_state") == "MissingPacket" else 1,
        ]
        rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "owner_edge_confidence": confidence,
                "owner_edge_complete": row.get("owner_edge_complete"),
                "nominal_id_domain_isolation": row.get("nominal_id_domain_isolation"),
                "directability_evidence_present": row.get("directability_evidence_present"),
                "seed_packet_state": row.get("seed_packet_state"),
                "packet_generation_candidate": eligible,
                "priority_tuple": priority_tuple,
                "blocked_by": [] if eligible else ["SeedPacketCandidatePreconditionMissing"],
                "next_card": (
                    "MIRBUILDER-" + owner_slug(row["owner_edge_id"]) + "-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001"
                    if eligible else None
                ),
            }
        )

    eligible_rows = [row for row in rows if row["packet_generation_candidate"]]
    selected_rows: list[dict[str, Any]] = []
    if eligible_rows:
        best_tuple = min(row["priority_tuple"] for row in eligible_rows)
        selected_rows = [row for row in eligible_rows if row["priority_tuple"] == best_tuple]

    if len(selected_rows) == 1:
        decision = {
            "kind": "SelectSourcePlanAndRecipe",
            "reason_token": "ExactlyOneIdScalarSeedPacketCandidate",
            "selected_owner_edge_id": selected_rows[0]["owner_edge_id"],
            "selected_next_card": selected_rows[0]["next_card"],
        }
    else:
        reason = "NoIdScalarSeedPacketCandidate"
        if selected_rows:
            reason = "MultipleEqualIdScalarSeedPacketCandidates"
        decision = {
            "kind": "KeepStopped",
            "reason_token": reason,
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarSeedPacketCandidateSelectionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_evidence_contract": rel(CONTRACT),
        },
        "provenance": {
            "seed_evidence_contract_hash": sha256_file(CONTRACT),
        },
        "selection_rule": {
            "primary": "SeedPacketMaterializationReadiness",
            "allowed_signals": [
                "owner_edge_confidence",
                "owner_edge_completeness",
                "nominal_id_domain_isolation",
                "seed_packet_state",
            ],
            "forbidden_proofs": [
                "cluster_size",
                "directable_row_count",
                "lexical_order",
                "coverage_percentage",
                "route_membership_alone",
                "manual_owner_preference",
            ],
        },
        "candidate_rows": rows,
        "candidate_pool": {
            "input_owner_edge_count": len(rows),
            "packet_generation_candidate_count": len(eligible_rows),
            "selected_candidate_count": 1 if len(selected_rows) == 1 else 0,
            "ambiguous_candidate_count": len(selected_rows) if len(selected_rows) > 1 else 0,
        },
        "decision": decision,
        "claims": {
            "seed_evidence_contract_consumed": 1,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
            "lexical_tiebreaker_as_seed_selection_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "source_plan_and_recipe_materialization": 0,
            "verifier_result_fixture_materialization": 0,
            "derived_artifact_seed_draft_input_materialization": 0,
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
        print("mirbuilder-id-scalar-seed-packet-candidate-selection unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
