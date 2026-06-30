#!/usr/bin/env python3
"""Classify strict-deny MirBuilder Rust surfaces into diagnostic near-miss buckets.

This probe is intentionally not a converter. It consumes the crate-wide
unconverted-surface report, keeps strict classifications authoritative, and
adds a relaxed diagnostic view that answers "what evidence would make this
surface actionable?". It does not emit Hako, select Source Selfhost, or create a
projection policy.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
OUTPUT = FIXTURES / "mirbuilder-strict-deny-near-miss-diagnostic-probe-v0.json"
CURRENT_STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

TOKEN = "MIRBUILDER-STRICT-DENY-NEAR-MISS-DIAGNOSTIC-PROBE-001"

SELECTABLE_OWNER_CONFIDENCE = {"ExactSymbol", "FixtureMapped", "FileScoped"}
WEAK_OWNER_CONFIDENCE = {"Heuristic", "None", None}
KNOWN_SHAPE_SENTINELS = {"", None, "unknown_shape"}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def shape_known(item: dict[str, Any]) -> bool:
    return item.get("shape_signature") not in KNOWN_SHAPE_SENTINELS


def deny_reason_known(item: dict[str, Any]) -> bool:
    return item.get("stable_deny_reason") not in {"", None, "Unknown"}


def near_miss_bucket(item: dict[str, Any]) -> tuple[str, str, list[str]]:
    classification = item.get("classification")
    confidence = item.get("owner_edge_confidence")

    if classification == "BorrowSurfacePolicyKnown":
        return (
            "AlreadyCoveredByKnownBorrowPolicy",
            "StrictBorrowDenyAlreadyHasReplacementPolicy",
            [],
        )

    if classification == "BorrowSurfaceNeedsPolicy":
        return (
            "NeedsBorrowPolicy",
            item.get("reason_token") or "BorrowSurfaceNeedsPolicy",
            ["borrow_policy"],
        )

    if classification == "CompositeSuspected":
        return (
            "NeedsCompositeEvidenceInventory",
            "CompositeSuspectedRequiresEvidenceBeforeDecomposition",
            ["composite_evidence"],
        )

    if classification in {"DebugOnlySurface", "TestOnlySurface", "IgnoredNonSemanticHelper"}:
        return ("IgnoredNonSemanticSurface", item.get("reason_token") or classification, [])

    if classification != "MissingProjectionPolicy":
        return (
            "StillUnsupported",
            item.get("reason_token") or "UnsupportedStrictClassification",
            ["classification_policy"],
        )

    missing: list[str] = []
    if confidence in WEAK_OWNER_CONFIDENCE:
        missing.append("owner_edge_confidence")
    if not shape_known(item):
        missing.append("shape_signature")
    if not deny_reason_known(item):
        missing.append("stable_deny_reason")

    if missing == ["owner_edge_confidence"]:
        return ("NeedsOwnerEdgeMappingOnly", "OwnerEdgeConfidenceRequired", missing)
    if missing == ["shape_signature"]:
        return ("NeedsShapeSignatureOnly", "ShapeSignatureRequired", missing)
    if missing == ["stable_deny_reason"]:
        return ("NeedsStableDenyReasonOnly", "StableDenyReasonRequired", missing)
    if missing:
        return ("NeedsMultipleDiagnosticAxes", "MultipleDiagnosticAxesRequired", missing)

    if confidence in SELECTABLE_OWNER_CONFIDENCE:
        return (
            "NeedsProjectionPolicyOnly",
            "ProjectionPolicyOnlyNearMiss",
            ["projection_policy"],
        )

    return ("StillUnsupported", "ProjectionPolicyNearMissNotSelectable", ["selection_evidence"])


def cluster_id(item: dict[str, Any], bucket: str) -> str:
    return "::".join(
        [
            "near_miss",
            bucket,
            str(item.get("stable_deny_reason") or "NoStableDenyReason"),
            str(item.get("shape_signature") or "NoShapeSignature"),
            str(item.get("owner_edge_confidence") or "NoOwnerEdgeConfidence"),
            str(item.get("likely_owner_cluster") or item.get("known_owner_edge") or "NoOwner"),
        ]
    )


def build_probe() -> dict[str, Any]:
    report = read_json(REPORT)
    buckets: Counter[str] = Counter()
    reason_tokens: Counter[str] = Counter()
    missing_axes: Counter[str] = Counter()
    clusters: dict[str, dict[str, Any]] = {}
    examples: dict[str, list[dict[str, Any]]] = defaultdict(list)

    for item in report.get("items", []):
        bucket, reason, missing = near_miss_bucket(item)
        buckets[bucket] += 1
        reason_tokens[reason] += 1
        for axis in missing:
            missing_axes[axis] += 1

        cid = cluster_id(item, bucket)
        if cid not in clusters:
            clusters[cid] = {
                "cluster_id": cid,
                "bucket": bucket,
                "reason_token": reason,
                "candidate_count": 0,
                "stable_deny_reason": item.get("stable_deny_reason"),
                "shape_signature": item.get("shape_signature"),
                "owner_edge_confidence": item.get("owner_edge_confidence"),
                "likely_owner_cluster": item.get("likely_owner_cluster"),
                "selection_eligible": bucket == "NeedsProjectionPolicyOnly",
                "missing_axes": sorted(set(missing)),
                "next_owner_kind": (
                    "ProjectionPolicyClusterResolution"
                    if bucket == "NeedsProjectionPolicyOnly"
                    else "DiagnosticRepair"
                    if bucket.startswith("Needs")
                    else "None"
                ),
                "evidence_refs": sorted(set(item.get("evidence_refs", []))),
            }
        clusters[cid]["candidate_count"] += 1
        if len(examples[cid]) < 3:
            examples[cid].append(
                {
                    "source_id": item.get("source_id"),
                    "symbol": item.get("symbol"),
                    "source_path": item.get("source_path"),
                    "line": item.get("line"),
                    "strict_classification": item.get("classification"),
                }
            )

    cluster_rows = []
    for cid, row in clusters.items():
        row["examples"] = examples[cid]
        cluster_rows.append(row)
    cluster_rows.sort(
        key=lambda row: (
            0 if row["bucket"] == "NeedsProjectionPolicyOnly" else 1,
            -row["candidate_count"],
            row["cluster_id"],
        )
    )

    eligible_clusters = [row for row in cluster_rows if row["selection_eligible"]]
    if eligible_clusters:
        decision = {
            "kind": "SelectNearMissClusterResolution",
            "reason_token": "ProjectionPolicyNearMissClustersAvailable",
            "selected_next_card": "MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001",
            "selected_cluster_id": None,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoProjectionPolicyNearMissClusterAvailable",
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "selected_cluster_id": None,
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictDenyNearMissDiagnosticProbeV1",
        "token": TOKEN,
        "input_authority": {
            "unconverted_surface_report": rel(REPORT),
            "current_state": rel(CURRENT_STATE),
        },
        "provenance": {
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "probe_mode": "diagnostic_relaxed_classification_only",
        },
        "rules": {
            "strict_classification_remains_authority": 1,
            "diagnostic_relaxed_mode_only": 1,
            "hako_emission": 0,
            "hako_adoption_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
        },
        "bucket_summary": dict(sorted(buckets.items())),
        "reason_token_summary": dict(sorted(reason_tokens.items())),
        "missing_axis_summary": dict(sorted(missing_axes.items())),
        "cluster_summary": {
            "cluster_count": len(cluster_rows),
            "selection_eligible_cluster_count": len(eligible_clusters),
            "needs_projection_policy_only_count": buckets.get("NeedsProjectionPolicyOnly", 0),
            "needs_borrow_policy_count": buckets.get("NeedsBorrowPolicy", 0),
            "needs_owner_edge_mapping_only_count": buckets.get("NeedsOwnerEdgeMappingOnly", 0),
            "ignored_nonsemantic_surface_count": buckets.get("IgnoredNonSemanticSurface", 0),
        },
        "clusters": cluster_rows[:80],
        "decision": decision,
        "claims": {
            "report_consumed": 1,
            "strict_rules_changed": 0,
            "diagnostic_probe_only": 1,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
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
    parser.add_argument("--check", action="store_true", help="Verify the checked-in probe fixture.")
    args = parser.parse_args()

    output = stable_json(build_probe())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-strict-deny-near-miss-diagnostic-probe unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
