#!/usr/bin/env python3
"""Select the post-ID-scalar Source Selfhost lane.

This resolver consumes the parent-owned context_registry closeout and selects
the next evidence-refresh lane. It is a local mechanical selector, not a
semantic owner selection.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-wider-route-selection-basis-007-v0.json"

TOKEN = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007"
CONTRACT = "rust-lifecycle-source-selfhost-wider-route-selection-basis-007-v0"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

PARENT_BOUNDARY = FIXTURES / "mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json"
RERUN_010 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
EMISSION_ADOPTION = FIXTURES / "mirbuilder-emission-ssa-phi-hako-adoption-decision-v0.json"
POLICY = ROOT / "docs/development/current/main/design/current-docs-update-policy-ssot.md"

REPORT_RERUN_004 = "MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004"
CHECKPOINT_RERUN_002 = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    parent_boundary = read_json(PARENT_BOUNDARY)
    rerun = read_json(RERUN_010)
    report = read_json(UNCONVERTED_REPORT)
    adoption = read_json(EMISSION_ADOPTION)

    classification = parent_boundary.get("classification") or {}
    parent_decision = parent_boundary.get("decision") or {}
    rerun_pool = rerun.get("candidate_pool") or {}
    rerun_decision = rerun.get("decision") or {}
    report_provenance = report.get("provenance") or {}
    adoption_claims = adoption.get("claims") or {}
    adoption_delta = []
    if adoption_claims.get("hako_adopted") == 1:
        adoption_delta.append(adoption.get("token"))

    report_ledger_hash = report_provenance.get("native_owner_adoption_ledger_hash")
    current_manifest_hash = sha256_file(MANIFEST)
    report_fresh = report_ledger_hash == current_manifest_hash and not adoption_delta

    if not report_fresh:
        selected_kind = "SelectUnconvertedSurfaceReportRerun"
        reason = "SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption"
        selected_next = REPORT_RERUN_004
    else:
        selected_kind = "SelectNativeOwnerCheckpointRerun"
        reason = "NativeOwnerCheckpointStaleAfterEmissionSsaPhiAdoption"
        selected_next = CHECKPOINT_RERUN_002

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostWiderRouteSelectionBasisV7",
        "output_contract": CONTRACT,
        "token": TOKEN,
        "local_authority": {
            "local_selection_authority": "LocalMechanicalSelectorAuthorityV1",
            "worker_inventory": "consumed",
            "worker_inventory_scope": "read_only_current_fixtures_cards_ledgers",
        },
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "parent_owned_subject_boundary": rel(PARENT_BOUNDARY),
            "native_owner_seed_capability_rerun_010": rel(RERUN_010),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "source_selfhost_family_guard_manifest": rel(MANIFEST),
            "emission_ssa_phi_hako_adoption_decision": rel(EMISSION_ADOPTION),
            "local_mechanical_selector_policy": rel(POLICY),
        },
        "provenance": {
            "parent_owned_subject_boundary_hash": sha256_file(PARENT_BOUNDARY),
            "native_owner_seed_capability_rerun_010_hash": sha256_file(RERUN_010),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
            "source_selfhost_family_guard_manifest_hash": current_manifest_hash,
            "emission_ssa_phi_hako_adoption_decision_hash": sha256_file(EMISSION_ADOPTION),
            "local_mechanical_selector_policy_hash": sha256_file(POLICY),
        },
        "parent_owned_closeout": {
            "input_decision": parent_decision.get("kind"),
            "reason_token": parent_decision.get("reason_token"),
            "selected_next_card": parent_decision.get("selected_next_card"),
            "classification": classification.get("kind"),
            "standalone_projection_subject_established": classification.get(
                "standalone_projection_subject_established"
            ),
            "source_plan_materialization_allowed": classification.get(
                "source_plan_materialization_allowed"
            ),
        },
        "seed_capability_after_adoption": {
            "input_decision": rerun_decision.get("kind"),
            "reason_token": rerun_decision.get("reason_token"),
            "selected_next_card": rerun_decision.get("selected_next_card"),
            "remaining_owner_count": rerun_pool.get("remaining_owner_count"),
            "selection_eligible_count": rerun_pool.get("selection_eligible_count"),
            "native_seed_candidate_count": rerun_pool.get("native_seed_candidate_count"),
        },
        "freshness": {
            "unconverted_surface_report_fresh_after_emission_ssa_phi_adoption": report_fresh,
            "report_native_owner_adoption_ledger_hash": report_ledger_hash,
            "current_native_owner_manifest_hash": current_manifest_hash,
            "emission_ssa_phi_hako_adopted": adoption_claims.get("hako_adopted"),
            "emission_ssa_phi_source_selfhost_claim": adoption_claims.get("source_selfhost_claim"),
            "native_owner_adoption_delta_count": len(adoption_delta),
            "latest_native_owner_delta_tokens": adoption_delta,
            "freshness_reason_token": "ReportFresh"
            if report_fresh
            else "SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption",
        },
        "candidate_lanes": [
            {
                "lane": "UnconvertedSurfaceReportRerun004",
                "selection_eligible": not report_fresh,
                "reason_token": "SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption"
                if not report_fresh
                else "SourceSurfaceReportFresh",
                "next_card": REPORT_RERUN_004,
            },
            {
                "lane": "NativeOwnerCheckpointRerun002",
                "selection_eligible": report_fresh,
                "reason_token": "NativeOwnerCheckpointStaleAfterEmissionSsaPhiAdoption"
                if report_fresh
                else "NeedsFreshSurfaceReportFirst",
                "next_card": CHECKPOINT_RERUN_002,
            },
            {
                "lane": "MissingProjectionPolicyClusterResolutionV4",
                "selection_eligible": False,
                "reason_token": "RequiresFreshReportAndCheckpoint",
                "next_card": "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4",
            },
            {
                "lane": "BorrowSurfacePolicyClusterRerun003",
                "selection_eligible": False,
                "reason_token": "RequiresFreshReportAndCheckpoint",
                "next_card": "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-003",
            },
            {
                "lane": "CarrierTypeTransportPriorityRerun",
                "selection_eligible": False,
                "reason_token": "RequiresFreshReportAndCheckpoint",
                "next_card": "MIRBUILDER-CARRIER-TYPE-TRANSPORT-PRIORITY-RERUN-003",
            },
        ],
        "selection_rule": {
            "consume_parent_owned_boundary": True,
            "context_registry_remain_parent_owned_required": True,
            "report_freshness_precedes_checkpoint": True,
            "native_owner_checkpoint_precedes_blocker_class_selection": True,
            "exactly_one_lane_or_keep_stopped": True,
            "local_mechanical_selector_authority": True,
            "worker_inventory_required_or_waived": True,
            "manual_lane_selection": False,
            "remaining_owner_count_as_proof": False,
            "owner_name_as_proof": False,
            "cluster_size_as_proof": False,
            "coverage_percentage_as_proof": False,
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
            "manual_lane_selection": 0,
            "remaining_owner_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_symbol_as_proof": 0,
            "source_path_as_authority": 0,
            "keep_parent_owner_as_standalone_proof": 0,
            "projection_descriptor_coverage_as_standalone_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "source_plan_materialization": 0,
            "behavior_recipe_materialization": 0,
            "verifier_result_materialization": 0,
            "derived_artifact_seed_draft_materialization": 0,
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
        print("source-selfhost-wider-route-selection-basis-007 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
