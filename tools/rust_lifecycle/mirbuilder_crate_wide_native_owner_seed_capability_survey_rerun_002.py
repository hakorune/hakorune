#!/usr/bin/env python3
"""Rerun native-owner seed capability after the source report is fresh."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-002-v0.json"

PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
OTHER_QUEUE = FIXTURES / "mirbuilder-missing-projection-policy-other-shape-signature-cluster-resolution-002-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
FAMILY_MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
PREVIOUS_SURVEY = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-v0.json"

TOKEN = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-002"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def projection_descriptor_rows(family_manifest: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        {
            "token": row.get("token"),
            "fixture": row.get("fixture"),
        }
        for row in family_manifest.get("rows", [])
        if "PROJECTION-POLICY" in (row.get("token") or "")
    ]


def projection_descriptor_ledger_hash(family_manifest: dict[str, Any]) -> str:
    payload = json.dumps(
        projection_descriptor_rows(family_manifest),
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def candidate_pool(previous_survey: dict[str, Any]) -> dict[str, int]:
    summary = previous_survey.get("summary") or {}
    return {
        "native_seed_ready_count": int(summary.get("native_seed_ready_count", 0)),
        "native_owner_seed_candidate_count": int(summary.get("convertible_leaf_count", 0))
        + int(summary.get("support_lane_seed_candidate_count", 0)),
        "generated_artifact_to_seed_candidate_count": 0,
        "route_repairable_inconsistency_count": int(summary.get("route_repair_needed_count", 0)),
        "other_blocker_axis_candidate_count": 0,
        "ambiguous_candidate_count": 0,
    }


def build_fixture() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    other = read_json(OTHER_QUEUE)
    report = read_json(UNCONVERTED_REPORT)
    family_manifest = read_json(FAMILY_MANIFEST)
    previous_survey = read_json(PREVIOUS_SURVEY)

    priority_summary = priority.get("summary") or {}
    other_summary = other.get("summary") or {}
    report_provenance = report.get("provenance") or {}
    ledger_hash = projection_descriptor_ledger_hash(family_manifest)
    report_ledger_hash = report_provenance.get("projection_descriptor_ledger_hash")
    previous_survey_hash = sha256_file(PREVIOUS_SURVEY)
    report_survey_hash = report_provenance.get("native_owner_seed_capability_survey_hash")
    needs_report_rerun = (
        report_ledger_hash != ledger_hash
        or report_survey_hash != previous_survey_hash
    )
    pool = candidate_pool(previous_survey)

    if priority_summary.get("selectable_cluster_count") != 0:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "GlobalProjectionPolicyQueueNotExhausted",
        }
    elif other_summary.get("selection_eligible_shape_count") != 0:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-002",
            "reason_token": "OtherShapeQueueNotExhausted",
        }
    elif needs_report_rerun:
        decision = {
            "kind": "SelectUnconvertedSurfaceReportRerun",
            "selected_owner_edge_id": None,
            "selected_next_card": "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001",
            "reason_token": "UnconvertedSurfaceReportStaleAfterProjectionDescriptorCloseout",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "reason_token": "NoMachineDerivedNativeOwnerSeedCandidateAfterProjectionQueueExhaustion",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV2",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "global_projection_policy_priority_resolution": rel(PRIORITY),
            "other_shape_signature_cluster_resolution": rel(OTHER_QUEUE),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "source_selfhost_family_guard_manifest": rel(FAMILY_MANIFEST),
            "previous_native_owner_seed_capability_survey": rel(PREVIOUS_SURVEY),
        },
        "queue_exhaustion": {
            "global_projection_policy": {
                "eligible_cluster_count": priority_summary.get("eligible_cluster_count"),
                "excluded_existing_decision_cluster_count": priority_summary.get("excluded_existing_decision_cluster_count"),
                "selectable_cluster_count": priority_summary.get("selectable_cluster_count"),
                "reason_token": (priority.get("decision") or {}).get("reason_token"),
            },
            "other_shape_signature_queue": {
                "input_shape_signature_count": other_summary.get("input_shape_signature_count"),
                "input_other_owner_cluster_count": other_summary.get("input_other_owner_cluster_count"),
                "completed_shape_signature_count": other_summary.get("completed_shape_signature_count"),
                "selection_eligible_shape_count": other_summary.get("selection_eligible_shape_count"),
                "reason_token": (other.get("decision") or {}).get("reason_token"),
            },
        },
        "freshness": {
            "projection_descriptor_ledger_hash": ledger_hash,
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "native_seed_survey_previous_hash": previous_survey_hash,
            "unconverted_surface_report_projection_descriptor_ledger_hash": report_ledger_hash,
            "unconverted_surface_report_survey_hash": report_survey_hash,
            "unconverted_surface_report_covers_landed_descriptors": report_ledger_hash == ledger_hash,
            "needs_unconverted_surface_report_rerun": needs_report_rerun,
        },
        "projection_descriptor_ledger": {
            "row_count": len(projection_descriptor_rows(family_manifest)),
            "rows": projection_descriptor_rows(family_manifest),
        },
        "candidate_pool": pool,
        "decision": decision,
        "claims": {
            "global_projection_policy_exhaustion_consumed": 1,
            "other_shape_queue_exhaustion_consumed": 1,
            "projection_descriptor_ledger_hash_recorded": 1,
            "unconverted_surface_report_hash_recorded": 1,
            "freshness_checked": 1,
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
            "family_name_based_policy": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in rerun fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-002 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
