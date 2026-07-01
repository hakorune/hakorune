#!/usr/bin/env python3
"""Resolve ID scalar native-seed materialization readiness."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_REPAIR = "MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001"

CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"
SURVEY_RERUN_009 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    cluster = read_json(CLUSTER_RESOLUTION)
    survey = read_json(SURVEY_RERUN_009)
    directability = read_json(DIRECTABILITY)

    directability_summary = directability.get("summary") or {}
    repair_required_count = directability_summary.get("owner_edge_repair_required_count", 0)

    owner_edge_readiness: list[dict[str, Any]] = []
    for owner in survey.get("directable_owner_edges") or []:
        blocked_by = []
        if repair_required_count:
            blocked_by.append("OwnerEdgeRepairRequired")
        owner_edge_readiness.append(
            {
                "owner_edge_id": owner["owner_edge_id"],
                "directable_row_count": owner.get("directable_row_count"),
                "canonical_id_type_counts": owner.get("canonical_id_type_counts") or {},
                "owner_edge_complete": repair_required_count == 0,
                "native_seed_file_boundary": "BlockedByOwnerEdgeRepairRequired" if repair_required_count else "Unknown",
                "module_export_readiness": "BlockedByOwnerEdgeRepairRequired" if repair_required_count else "Unknown",
                "generator_overwrite_guard_readiness": "BlockedByOwnerEdgeRepairRequired" if repair_required_count else "Unknown",
                "derived_artifact_seed_draft_input_available": False,
                "verifier_result_fixture_present": False,
                "source_plan_and_recipe_present": False,
                "nominal_id_domain_isolation": "Preserved",
                "policy_gap": False,
                "selection_eligible_for_seed_materialization": False,
                "blocked_by": blocked_by,
                "next_card": None,
            }
        )

    if repair_required_count:
        decision = {
            "kind": "SelectOwnerEdgeRepair",
            "reason_token": "IdScalarOwnerEdgeRepairRequiredBeforeSeedReadiness",
            "selected_owner_edge_id": None,
            "selected_next_card": NEXT_REPAIR,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoIdScalarSeedMaterializationReadyOwnerEdge",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainSeedReadinessResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_scalar_cluster_resolution": rel(CLUSTER_RESOLUTION),
            "native_owner_seed_survey_rerun_009": rel(SURVEY_RERUN_009),
            "directability_rerun": rel(DIRECTABILITY),
        },
        "provenance": {
            "id_scalar_cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "native_owner_seed_survey_rerun_009_hash": sha256_file(SURVEY_RERUN_009),
            "directability_rerun_hash": sha256_file(DIRECTABILITY),
        },
        "preconditions": {
            "input_directable_owner_edge_count": (cluster.get("summary") or {}).get("input_directable_owner_edge_count"),
            "selection_eligible_cluster_count": (cluster.get("summary") or {}).get("selection_eligible_cluster_count"),
            "unique_evidence_quality_tuple_count": (cluster.get("summary") or {}).get("unique_evidence_quality_tuple_count"),
            "owner_edge_repair_required_count": repair_required_count,
            "owner_edge_completeness_required_before_seed_selection": True,
        },
        "readiness_axes": [
            "owner_edge_completeness",
            "native_seed_file_boundary",
            "module_export_readiness",
            "generator_overwrite_guard_readiness",
            "derived_artifact_seed_draft_input_available",
            "verifier_result_fixture_present",
            "source_plan_and_recipe_present",
            "nominal_id_domain_isolation",
            "no_policy_gap",
            "no_runtime_or_backend_or_abi_requirement",
        ],
        "owner_edge_readiness": owner_edge_readiness,
        "candidate_pool": {
            "readiness_input_owner_edge_count": len(owner_edge_readiness),
            "owner_edge_repair_required_count": repair_required_count,
            "seed_materialization_ready_count": 0,
            "ambiguous_ready_count": 0,
        },
        "decision": decision,
        "claims": {
            "id_scalar_cluster_resolution_consumed": 1,
            "native_owner_seed_survey_rerun_009_consumed": 1,
            "directability_rerun_consumed": 1,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "lexical_tiebreaker_as_seed_selection_proof": 0,
            "coverage_percentage_as_proof": 0,
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
        print("mirbuilder-id-scalar-domain-seed-readiness-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
