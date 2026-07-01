#!/usr/bin/env python3
"""Normalize denied-boundary vocabulary before strict seed candidate rerun."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
INPUT = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json"

TOKEN = "MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001"
NEXT = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001"

FORBIDDEN_NONCLAIM = {
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
}

SCOPE_EXCLUSION = {
    "module_metadata_publication",
    "semantic_refresh",
    "all_functions_phi_materialization",
    "full_finalize_module",
    "mainline_selected",
    "direct_state_plan_refresh",
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
    if boundary in SCOPE_EXCLUSION:
        return "ScopeExclusionBoundary"
    if boundary.endswith("_field_value_type_refresh") or boundary.endswith("_collection_field_element_refresh"):
        return "NarrowRefreshScopeExclusion"
    return "UnclassifiedDeniedBoundary"


def class_semantics(boundary_class: str) -> dict[str, Any]:
    if boundary_class == "ForbiddenNonClaimBoundary":
        return {
            "seed_eligibility_evidence": False,
            "transport_gap": False,
            "meaning": "forbidden non-claim remains denied",
        }
    if boundary_class == "ScopeExclusionBoundary":
        return {
            "seed_eligibility_evidence": False,
            "transport_gap": False,
            "meaning": "bounded scope exclusion remains outside this narrow owner",
        }
    if boundary_class == "NarrowRefreshScopeExclusion":
        return {
            "seed_eligibility_evidence": False,
            "transport_gap": False,
            "meaning": "bounded refresh-scope exclusion remains outside this narrow owner",
        }
    return {
        "seed_eligibility_evidence": False,
        "transport_gap": None,
        "meaning": "requires design consultation",
    }


def build_fixture() -> dict[str, Any]:
    source = read_json(INPUT)
    pool = source.get("candidate_pool") or {}
    rows = source.get("result_carrier_projection_rows") or []
    class_counts: Counter[str] = Counter()
    normalized_rows: list[dict[str, Any]] = []
    unclassified_boundaries: list[str] = []

    for row in rows:
        normalized_boundaries = []
        for boundary in row.get("remaining_denied_boundaries") or []:
            boundary_text = str(boundary)
            boundary_class = classify_boundary(boundary_text)
            class_counts[boundary_class] += 1
            if boundary_class == "UnclassifiedDeniedBoundary":
                unclassified_boundaries.append(boundary_text)
            normalized_boundaries.append(
                {
                    "boundary": boundary_text,
                    "class": boundary_class,
                    **class_semantics(boundary_class),
                }
            )

        normalized_rows.append(
            {
                "owner_edge_id": row["owner_edge_id"],
                "verifier_result_fixture": row["verifier_result_fixture"],
                "result_carrier_projection_policy_covered": row["result_carrier_projection_policy_covered"],
                "normalized_boundaries": normalized_boundaries,
                "normalized_classes": sorted({item["class"] for item in normalized_boundaries}),
                "bridge_state_after_normalization": "BridgeBlocked",
                "blocked_by_after_normalization": ["StrictCandidateSelectionNormalizedRerunRequired"],
            }
        )

    unclassified_count = len(set(unclassified_boundaries))
    if unclassified_count:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "StrictDeniedBoundaryVocabularyRequiresDesignConsultation",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }
    else:
        decision = {
            "kind": "SelectStrictConverterEmissionNativeSeedCandidateSelectionNormalizedRerun",
            "reason_token": "StrictDeniedBoundaryVocabularyNormalized",
            "selected_next_card": NEXT,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictDeniedBoundaryVocabularyNormalizationV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "strict_candidate_selection_rerun": rel(INPUT),
        },
        "provenance": {
            "strict_candidate_selection_rerun_hash": sha256_file(INPUT),
            "input_token": source.get("token"),
        },
        "normalization_policy": {
            "policy_id": "StrictDeniedBoundaryVocabularyNormalizationV1",
            "forbidden_nonclaim_never_proves_seed_eligibility": True,
            "scope_exclusion_not_transport_gap": True,
            "narrow_refresh_scope_exclusion_not_transport_gap": True,
            "unknown_boundary_requires_design_consultation": True,
        },
        "input_summary": {
            "result_carrier_policy_covered_count": pool.get("result_carrier_policy_covered_count"),
            "denied_boundary_vocabulary_blocked_count": pool.get("denied_boundary_vocabulary_blocked_count"),
            "input_unclassified_denied_boundary_count": pool.get("unclassified_denied_boundary_count"),
        },
        "normalized_boundary_rows": normalized_rows,
        "normalized_class_summary": dict(sorted(class_counts.items())),
        "summary": {
            "normalized_row_count": len(normalized_rows),
            "unclassified_denied_boundary_count": unclassified_count,
            "seed_eligibility_selected_count": 0,
        },
        "decision": decision,
        "claims": {
            "strict_candidate_selection_rerun_consumed": 1,
            "denied_boundary_vocabulary_normalized": 1 if not unclassified_count else 0,
            "manual_boundary_reclassification": 0,
            "seed_eligibility_selected": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
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
        print("mirbuilder-strict-denied-boundary-vocabulary-normalization unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
