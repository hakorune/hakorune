#!/usr/bin/env python3
"""Rerun ID-scalar seed readiness after owner-edge repair."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-id-scalar-domain-seed-readiness-resolution-002-v0.json"

TOKEN = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

CLUSTER_RESOLUTION = FIXTURES / "mirbuilder-id-scalar-domain-seed-candidate-cluster-resolution-v0.json"
DIRECTABILITY = FIXTURES / "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
OWNER_EDGE_REPAIR = FIXTURES / "mirbuilder-id-scalar-domain-owner-edge-repair-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def directability_rows(directability: dict[str, Any]) -> list[dict[str, Any]]:
    return sorted(directability.get("rerun_rows", []), key=lambda row: row["source_id"])


def evidence_texts(patterns: list[str]) -> dict[str, str]:
    out: dict[str, str] = {}
    for pattern in patterns:
        for path in sorted(FIXTURES.glob(pattern)):
            if path == OUTPUT:
                continue
            try:
                out[rel(path)] = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
    return out


def matching_refs(owner_edge_id: str, texts: dict[str, str]) -> list[str]:
    return [path for path, text in texts.items() if owner_edge_id in text]


def repaired_row_index(repair: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {row["source_id"]: row for row in repair.get("repaired_rows", [])}


def build_fixture() -> dict[str, Any]:
    cluster = read_json(CLUSTER_RESOLUTION)
    directability = read_json(DIRECTABILITY)
    repair = read_json(OWNER_EDGE_REPAIR)
    repair_index = repaired_row_index(repair)

    native_seed_texts = evidence_texts(["*hako-native-source-seed*.json"])
    adoption_texts = evidence_texts(["*adoption*.json"])
    verifier_texts = evidence_texts(["*verifier-result*.json"])
    plan_recipe_texts = evidence_texts(["*-plan-v0.json", "*-behavior-recipe-v0.json"])

    owner_rows: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in directability_rows(directability):
        source_id = row["source_id"]
        if row.get("directability_state") == "OwnerEdgeRepairRequired":
            repaired = repair_index.get(source_id)
            if not repaired:
                continue
            owner_edge_id = repaired["repaired_owner_edge_id"]
            owner_confidence = repaired["repaired_owner_edge_confidence"]
        else:
            owner_edge_id = row.get("known_owner_edge")
            owner_confidence = row.get("owner_edge_confidence")
        owner_rows[owner_edge_id].append(
            {
                "source_id": source_id,
                "canonical_id_type": row.get("canonical_id_type"),
                "nominal_transport": row.get("nominal_transport"),
                "owner_edge_confidence": owner_confidence,
                "directability_state": "DirectableWithRepairedOwnerEdge",
            }
        )

    readiness_rows: list[dict[str, Any]] = []
    ready_count = 0
    for owner_edge_id in sorted(owner_rows):
        rows = owner_rows[owner_edge_id]
        id_counts = Counter(row["canonical_id_type"] for row in rows)
        native_seed_refs = matching_refs(owner_edge_id, native_seed_texts)
        adoption_refs = matching_refs(owner_edge_id, adoption_texts)
        verifier_refs = matching_refs(owner_edge_id, verifier_texts)
        plan_recipe_refs = matching_refs(owner_edge_id, plan_recipe_texts)

        has_seed_draft_input = bool(native_seed_refs)
        has_verifier = bool(verifier_refs)
        has_plan_recipe = bool(plan_recipe_refs)
        blocked_by = []
        if not has_seed_draft_input:
            blocked_by.append("MissingDerivedArtifactSeedDraftInput")
        if not has_verifier:
            blocked_by.append("MissingVerifierResultFixture")
        if not has_plan_recipe:
            blocked_by.append("MissingSourcePlanAndRecipe")
        if blocked_by:
            blocked_by.append("DirectabilityOnlyIsNotSeedEvidence")

        eligible = not blocked_by
        if eligible:
            ready_count += 1
        readiness_rows.append(
            {
                "owner_edge_id": owner_edge_id,
                "directable_row_count": len(rows),
                "canonical_id_type_counts": dict(sorted(id_counts.items())),
                "owner_edge_complete": True,
                "owner_edge_confidence_set": sorted({row["owner_edge_confidence"] for row in rows}),
                "native_seed_file_boundary": "DerivableIfSeedCardSelected",
                "module_export_readiness": "DerivableIfSeedCardSelected",
                "generator_overwrite_guard_readiness": "NeedsNativeSeedMaterialization",
                "derived_artifact_seed_draft_input_available": has_seed_draft_input,
                "verifier_result_fixture_present": has_verifier,
                "source_plan_and_recipe_present": has_plan_recipe,
                "nominal_id_domain_isolation": "Preserved",
                "policy_gap": False,
                "selection_eligible_for_seed_materialization": eligible,
                "blocked_by": blocked_by,
                "evidence_refs": {
                    "native_seed": native_seed_refs,
                    "adoption": adoption_refs,
                    "verifier": verifier_refs,
                    "plan_or_recipe": plan_recipe_refs,
                },
                "next_card": (
                    "MIRBUILDER-" + owner_edge_id.upper().replace("::", "-").replace("_", "-") + "-HAKO-NATIVE-SOURCE-SEED-001"
                    if eligible else None
                ),
            }
        )

    if ready_count == 1:
        selected = next(row for row in readiness_rows if row["selection_eligible_for_seed_materialization"])
        decision = {
            "kind": "SelectNativeSeedMaterialization",
            "reason_token": "ExactlyOneIdScalarSeedMaterializationReadyOwnerEdge",
            "selected_owner_edge_id": selected["owner_edge_id"],
            "selected_next_card": selected["next_card"],
        }
    else:
        reason = "NoIdScalarSeedMaterializationReadyOwnerEdgeAfterOwnerEdgeRepair"
        if ready_count > 1:
            reason = "MultipleIdScalarSeedMaterializationReadyOwnerEdges"
        decision = {
            "kind": "KeepStopped",
            "reason_token": reason,
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderIdScalarDomainSeedReadinessResolutionV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "id_scalar_cluster_resolution": rel(CLUSTER_RESOLUTION),
            "directability_rerun": rel(DIRECTABILITY),
            "owner_edge_repair": rel(OWNER_EDGE_REPAIR),
        },
        "provenance": {
            "id_scalar_cluster_resolution_hash": sha256_file(CLUSTER_RESOLUTION),
            "directability_rerun_hash": sha256_file(DIRECTABILITY),
            "owner_edge_repair_hash": sha256_file(OWNER_EDGE_REPAIR),
        },
        "preconditions": {
            "input_directable_owner_edge_count": (cluster.get("summary") or {}).get("input_directable_owner_edge_count"),
            "previous_unique_evidence_quality_tuple_count": (cluster.get("summary") or {}).get("unique_evidence_quality_tuple_count"),
            "owner_edge_repair_unrepaired_row_count": (repair.get("summary") or {}).get("unrepaired_row_count"),
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
        "owner_edge_readiness": readiness_rows,
        "candidate_pool": {
            "readiness_input_owner_edge_count": len(readiness_rows),
            "owner_edge_repair_required_count": 0,
            "seed_materialization_ready_count": ready_count,
            "ambiguous_ready_count": ready_count if ready_count > 1 else 0,
            "missing_seed_evidence_owner_edge_count": len([row for row in readiness_rows if not row["selection_eligible_for_seed_materialization"]]),
        },
        "decision": decision,
        "claims": {
            "id_scalar_cluster_resolution_consumed": 1,
            "directability_rerun_consumed": 1,
            "owner_edge_repair_consumed": 1,
            "manual_owner_selection": 0,
            "cluster_size_as_proof": 0,
            "directable_row_count_as_proof": 0,
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
        print("mirbuilder-id-scalar-domain-seed-readiness-resolution-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
