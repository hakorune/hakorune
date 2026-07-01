#!/usr/bin/env python3
"""Select the post-BridgePolicyV2 Source Selfhost lane.

This resolver consumes the strict native-seed rerun that exhausted the
BridgePolicyV2 candidate set. It does not pick a family, shape, or axis by hand;
it only selects the next evidence-refresh lane.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-003-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-003"
CONTRACT = "rust-lifecycle-source-selfhost-wider-route-selection-basis-003-v0"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

RERUN_005 = FIXTURES / "mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-005-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
BRIDGE_POLICY = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
BRIDGE_POLICY_V2 = FIXTURES / "mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"

REPORT_RERUN_002 = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-002"
REPORT_RERUN_003 = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rows_after_token(rows: list[dict[str, Any]], token: str) -> list[dict[str, Any]]:
    for index, row in enumerate(rows):
        if row.get("token") == token:
            return rows[index + 1 :]
    return rows


def adoption_delta_tokens(manifest: dict[str, Any]) -> list[str]:
    rows = rows_after_token(manifest.get("rows") or [], REPORT_RERUN_002)
    return [
        str(row["token"])
        for row in rows
        if str(row.get("token", "")).endswith("-HAKO-ADOPTION-DECISION-001")
    ]


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_005)
    report = read_json(UNCONVERTED_REPORT)
    manifest = read_json(MANIFEST)

    pool = rerun.get("candidate_pool") or {}
    decision = rerun.get("decision") or {}
    adoption_delta = adoption_delta_tokens(manifest)
    report_fresh = len(adoption_delta) == 0

    if not report_fresh:
        selected_kind = "SelectUnconvertedSurfaceReportRerun"
        reason = "SourceSurfaceReportStaleAfterNativeOwnerAdoption"
        selected_next = REPORT_RERUN_003
    else:
        selected_kind = "SelectNativeOwnerCheckpoint"
        reason = "NativeOwnerCheckpointRequiredAfterBridgePolicyV2Exhaustion"
        selected_next = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001"

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasisV3",
        "output_contract": CONTRACT,
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "strict_candidate_rerun_005": rel(RERUN_005),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "source_selfhost_family_guard_manifest": rel(MANIFEST),
            "bridge_policy": rel(BRIDGE_POLICY),
            "bridge_policy_v2": rel(BRIDGE_POLICY_V2),
        },
        "provenance": {
            "strict_candidate_rerun_005_hash": sha256_file(RERUN_005),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "source_selfhost_family_guard_manifest_hash": sha256_file(MANIFEST),
            "bridge_policy_hash": sha256_file(BRIDGE_POLICY),
            "bridge_policy_v2_hash": sha256_file(BRIDGE_POLICY_V2),
        },
        "bridge_policy_v2_exhaustion": {
            "input_owner_edge_count": pool.get("input_owner_edge_count"),
            "already_hako_adopted_count": pool.get("already_hako_adopted_count"),
            "bridge_eligible_remaining_count": pool.get("bridge_eligible_remaining_count"),
            "selected_candidate_count": pool.get("selected_candidate_count"),
            "input_decision": decision.get("kind"),
            "reason_token": decision.get("reason_token"),
        },
        "freshness": {
            "unconverted_surface_report_fresh": report_fresh,
            "freshness_reason_token": "ReportFresh" if report_fresh else "SourceSurfaceReportStaleAfterNativeOwnerAdoption",
            "unconverted_surface_report_token": report.get("token"),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "native_owner_adoption_delta_count": len(adoption_delta),
            "latest_native_owner_delta_tokens": adoption_delta,
        },
        "candidate_lanes": [
            {
                "lane": "UnconvertedSurfaceReportRerun",
                "selection_eligible": not report_fresh,
                "reason_token": "SourceSurfaceReportStaleAfterNativeOwnerAdoption"
                if not report_fresh
                else "SourceSurfaceReportFresh",
                "next_card": REPORT_RERUN_003,
            },
            {
                "lane": "NativeOwnerCheckpoint",
                "selection_eligible": report_fresh,
                "reason_token": "NativeOwnerCheckpointRequiredAfterBridgePolicyV2Exhaustion"
                if report_fresh
                else "NeedsFreshSurfaceReportFirst",
                "next_card": "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001",
            },
            {
                "lane": "MissingProjectionPolicyClusterResolutionV2",
                "selection_eligible": False,
                "reason_token": "RequiresFreshReportOrCheckpoint",
                "next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2",
            },
            {
                "lane": "BorrowSurfacePolicyClusterRerun002",
                "selection_eligible": False,
                "reason_token": "RequiresFreshReportOrCheckpoint",
                "next_card": "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-002",
            },
        ],
        "selection_rule": {
            "consume_rerun_005": True,
            "bridge_policy_v2_remaining_candidates_must_be_zero": True,
            "report_freshness_precedes_checkpoint": True,
            "native_owner_checkpoint_precedes_blocker_class_selection": True,
            "exactly_one_lane_or_keep_stopped": True,
            "cluster_size_as_proof": False,
            "coverage_percentage_as_proof": False,
            "manual_lane_selection": False,
        },
        "decision": {
            "kind": selected_kind,
            "reason_token": reason,
            "selected_next_card": selected_next,
            "selected_owner_edge_id": None,
        },
        "claims": {
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
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
            "new_canonical_mir_instruction": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "family_name_based_policy": 0,
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
        print("source-selfhost-wider-route-selection-basis-003 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
