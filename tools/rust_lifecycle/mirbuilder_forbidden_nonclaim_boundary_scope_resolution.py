#!/usr/bin/env python3
"""Resolve forbidden non-claim boundary occurrence scope."""

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
NORMALIZED_RERUN = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-normalized-rerun-v0.json"
NORMALIZATION = FIXTURES / "mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json"
OUTPUT = FIXTURES / "mirbuilder-forbidden-nonclaim-boundary-scope-resolution-v0.json"

TOKEN = "MIRBUILDER-FORBIDDEN-NONCLAIM-BOUNDARY-SCOPE-RESOLUTION-001"
NEXT_BRIDGE_V2 = "MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001"
NEXT_PERMANENT = "MIRBUILDER-RESULT-CARRIER-REFRESH-OWNERS-PERMANENT-DERIVED-CLASSIFICATION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def evidence_mentions_boundary(verifier: dict[str, Any], boundary: str) -> bool:
    checks = verifier.get("checks") or {}
    if checks.get(boundary) not in (None, 0, False):
        return True
    for collection_key in ("verified_operations",):
        if boundary in [str(item) for item in verifier.get(collection_key, [])]:
            return True
    transport = verifier.get("transport_notes") or {}
    return any(str(value) == boundary for value in transport.values())


def classify_occurrence(verifier: dict[str, Any], boundary: str) -> tuple[str, str, bool]:
    checks = verifier.get("checks") or {}
    if checks.get(boundary) in (0, False):
        return (
            "WiderDeniedBoundaryMentionOnly",
            "VerifierCheckExplicitlyDeniesSelectedNarrowSurfaceRequirement",
            False,
        )
    if evidence_mentions_boundary(verifier, boundary):
        return (
            "RequiredBySelectedNarrowSeedSurface",
            "BoundaryAppearsInSelectedNarrowSurfaceEvidence",
            True,
        )
    return (
        "WiderDeniedBoundaryMentionOnly",
        "BoundaryAppearsOnlyInDeniedBoundaryList",
        False,
    )


def build_fixture() -> dict[str, Any]:
    rerun = read_json(NORMALIZED_RERUN)
    normalization = read_json(NORMALIZATION)
    normalized_by_owner = {
        row["owner_edge_id"]: row
        for row in normalization.get("normalized_boundary_rows", [])
    }
    rows = []
    scope_counts: Counter[str] = Counter()

    for candidate in rerun.get("normalized_candidate_rows") or []:
        owner = candidate["owner_edge_id"]
        normalized = normalized_by_owner[owner]
        verifier_path = ROOT / candidate["verifier_result_fixture"]
        verifier = read_json(verifier_path)
        occurrences = []
        for boundary in normalized.get("normalized_boundaries") or []:
            if boundary.get("class") != "ForbiddenNonClaimBoundary":
                continue
            scope_class, reason, blocks_seed = classify_occurrence(verifier, boundary["boundary"])
            scope_counts[scope_class] += 1
            occurrences.append(
                {
                    "boundary": boundary["boundary"],
                    "input_class": "ForbiddenNonClaimBoundary",
                    "scope_class": scope_class,
                    "seed_eligibility_evidence": False,
                    "seed_eligibility_blocker": blocks_seed,
                    "reason_token": reason,
                }
            )

        owner_scope_counts = Counter(item["scope_class"] for item in occurrences)
        if owner_scope_counts.get("UnclassifiedForbiddenNonClaim", 0):
            resolved_state = "BridgeBlocked"
        elif owner_scope_counts.get("RequiredBySelectedNarrowSeedSurface", 0) or owner_scope_counts.get("PermanentForbiddenNonClaim", 0):
            resolved_state = "PermanentDerivedCandidate"
        elif occurrences and owner_scope_counts.get("WiderDeniedBoundaryMentionOnly", 0) == len(occurrences):
            resolved_state = "BridgePolicyV2Candidate"
        else:
            resolved_state = "DiagnosticLaneCandidate"

        rows.append(
            {
                "owner_edge_id": owner,
                "verifier_result_fixture": candidate["verifier_result_fixture"],
                "input_bridge_state": candidate["bridge_state_after_normalized_rerun"],
                "input_blocked_by": candidate["blocked_by_after_normalized_rerun"],
                "boundary_occurrences": occurrences,
                "summary": {
                    "required_by_selected_narrow_seed_surface_count": owner_scope_counts.get("RequiredBySelectedNarrowSeedSurface", 0),
                    "wider_denied_boundary_mention_only_count": owner_scope_counts.get("WiderDeniedBoundaryMentionOnly", 0),
                    "scoped_forbidden_nonclaim_exclusion_count": owner_scope_counts.get("ScopedForbiddenNonClaimExclusion", 0),
                    "permanent_forbidden_nonclaim_count": owner_scope_counts.get("PermanentForbiddenNonClaim", 0),
                    "unclassified_forbidden_nonclaim_count": owner_scope_counts.get("UnclassifiedForbiddenNonClaim", 0),
                },
                "resolved_bridge_state": resolved_state,
                "selected_next_card": NEXT_BRIDGE_V2 if resolved_state == "BridgePolicyV2Candidate" else None,
            }
        )

    unclassified = scope_counts.get("UnclassifiedForbiddenNonClaim", 0)
    required = scope_counts.get("RequiredBySelectedNarrowSeedSurface", 0)
    permanent = scope_counts.get("PermanentForbiddenNonClaim", 0)
    mention_only = scope_counts.get("WiderDeniedBoundaryMentionOnly", 0)
    total_occurrences = sum(scope_counts.values())

    if unclassified:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "UnclassifiedForbiddenNonclaimRequiresDesignStop",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "selected_owner_edge_id": None,
        }
    elif required or permanent:
        decision = {
            "kind": "SelectPermanentDerivedClassification",
            "reason_token": "RequiredForbiddenNonclaimBlocksNativeSeed",
            "selected_next_card": NEXT_PERMANENT,
            "selected_owner_edge_id": None,
        }
    elif total_occurrences and mention_only == total_occurrences:
        decision = {
            "kind": "SelectBridgePolicyV2",
            "reason_token": "ForbiddenNonclaimMentionOnlyCanBeScopedOut",
            "selected_next_card": NEXT_BRIDGE_V2,
            "selected_owner_edge_id": None,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "ForbiddenNonclaimBoundaryScopeUnresolved",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "selected_owner_edge_id": None,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderForbiddenNonclaimBoundaryScopeResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "normalized_rerun": rel(NORMALIZED_RERUN),
            "denied_boundary_vocabulary_normalization": rel(NORMALIZATION),
        },
        "provenance": {
            "normalized_rerun_hash": sha256_file(NORMALIZED_RERUN),
            "denied_boundary_vocabulary_normalization_hash": sha256_file(NORMALIZATION),
        },
        "scope_resolution_policy": {
            "forbidden_nonclaim_never_proves_seed_eligibility": True,
            "required_by_selected_narrow_seed_surface_blocks_seed": True,
            "wider_denied_boundary_mention_only_is_not_seed_evidence": True,
            "wider_denied_boundary_mention_only_may_be_excluded_from_seed_blockers": True,
            "unclassified_forbidden_nonclaim_blocks_seed": True,
            "manual_boundary_reclassification": False,
        },
        "owner_edge_rows": rows,
        "candidate_pool": {
            "input_owner_edge_count": len(rows),
            "required_by_selected_narrow_seed_surface_count": required,
            "wider_denied_boundary_mention_only_count": mention_only,
            "scoped_forbidden_nonclaim_exclusion_count": scope_counts.get("ScopedForbiddenNonClaimExclusion", 0),
            "permanent_forbidden_nonclaim_count": permanent,
            "unclassified_forbidden_nonclaim_count": unclassified,
            "bridge_policy_v2_candidate_count": sum(1 for row in rows if row["resolved_bridge_state"] == "BridgePolicyV2Candidate"),
            "permanent_derived_candidate_count": sum(1 for row in rows if row["resolved_bridge_state"] == "PermanentDerivedCandidate"),
            "diagnostic_lane_candidate_count": sum(1 for row in rows if row["resolved_bridge_state"] == "DiagnosticLaneCandidate"),
        },
        "decision": decision,
        "claims": {
            "normalized_rerun_consumed": 1,
            "denied_boundary_vocabulary_normalization_consumed": 1,
            "manual_boundary_reclassification": 0,
            "seed_eligibility_from_forbidden_nonclaim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
            "new_python_semantic_projector": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
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
        print("mirbuilder-forbidden-nonclaim-boundary-scope-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
