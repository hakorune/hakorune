#!/usr/bin/env python3
"""Rerun strict native-seed candidate selection after ResultBox carrier policy."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed
from mirbuilder_strict_converter_emission_native_seed_candidate_selection import build_fixture as build_selection


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
POLICY = FIXTURES / "mirbuilder-result-carrier-verifier-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-v0.json"

TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001"
NEXT = "MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001"

FORBIDDEN_NONCLAIM = {
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def classify_boundary(boundary: str) -> str:
    if boundary in FORBIDDEN_NONCLAIM:
        return "ForbiddenNonClaimBoundary"
    if boundary in {
        "module_metadata_publication",
        "semantic_refresh",
        "all_functions_phi_materialization",
        "full_finalize_module",
        "mainline_selected",
        "direct_state_plan_refresh",
    }:
        return "ScopeExclusionBoundary"
    if boundary.endswith("_field_value_type_refresh") or boundary.endswith("_collection_field_element_refresh"):
        return "NarrowRefreshScopeExclusion"
    return "UnclassifiedDeniedBoundary"


def build_fixture() -> dict[str, Any]:
    policy = read_json(POLICY)
    base = build_selection(
        cutoff_token="MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001"
    )
    covered_owner_edges = {row["owner_edge_id"] for row in policy.get("policy_rows", [])}
    projection_rows: list[dict[str, Any]] = []
    class_counts: Counter[str] = Counter()
    unclassified_count = 0

    for item in base.get("candidates", []):
        if item["owner_edge_id"] not in covered_owner_edges:
            continue
        verifier = read_json(ROOT / item["verifier_result_fixture"])
        denied = verifier.get("denied_boundaries") or []
        classes = sorted(set(classify_boundary(str(boundary)) for boundary in denied))
        for cls in classes:
            class_counts[cls] += 1
        if "UnclassifiedDeniedBoundary" in classes:
            unclassified_count += 1
        projection_rows.append(
            {
                "owner_edge_id": item["owner_edge_id"],
                "verifier_result_fixture": item["verifier_result_fixture"],
                "result_carrier_projection_policy_covered": True,
                "remaining_denied_boundaries": denied,
                "remaining_denied_boundary_classes": classes,
                "bridge_state_after_policy": "BridgeBlocked",
                "blocked_by_after_policy": ["DeniedBoundaryVocabularyRequiresNormalization"],
            }
        )

    decision = {
        "kind": "SelectDeniedBoundaryVocabularyNormalization",
        "reason_token": "ResultCarrierPolicyCoveredButDeniedBoundaryVocabularyStillBlocksSeed",
        "selected_next_card": NEXT,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionNativeSeedCandidateSelectionRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "result_carrier_verifier_projection_policy": rel(POLICY),
        },
        "provenance": {
            "result_carrier_verifier_projection_policy_hash": sha256_file(POLICY),
            "base_selection_cutoff_token": "MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001",
        },
        "candidate_pool": {
            "base_verified_hako_family_ir_count": base["candidate_pool"]["verified_hako_family_ir_count"],
            "base_bridge_eligible_count": base["candidate_pool"]["bridge_eligible_count"],
            "result_carrier_policy_covered_count": len(projection_rows),
            "bridge_eligible_after_policy_count": 0,
            "denied_boundary_vocabulary_blocked_count": len(projection_rows),
            "unclassified_denied_boundary_count": unclassified_count,
        },
        "result_carrier_projection_rows": projection_rows,
        "denied_boundary_class_counts": dict(sorted(class_counts.items())),
        "decision": decision,
        "claims": {
            "result_carrier_verifier_projection_policy_consumed": 1,
            "strict_candidate_selection_rerun_ready": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "owner_name_as_transport_policy": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
