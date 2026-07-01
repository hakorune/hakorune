#!/usr/bin/env python3
"""Rerun native-owner seed capability after descriptor coverage reclassification."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed
from mirbuilder_strict_converter_emission_native_seed_candidate_selection import build_fixture as build_selection


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008-v0.json"

TOKEN = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CHECKPOINT = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
REPORT_RECLASS_CARD = ROOT / "docs/development/current/main/phases/phase-296x/2005-MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001.md"
BRIDGE = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
STRICT_PROBE = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
RERUN_007 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    report_summary = report.get("summary") or {}
    selection = build_selection(cutoff_token=TOKEN)
    selected = selection["decision"]
    pool = selection["candidate_pool"]
    reclassified_count = report_summary.get("projection_descriptor_coverage_reclassified_count", 0)

    if selected["kind"] == "SelectNativeSeedCandidate":
        decision = {
            "kind": "SelectNativeSeedCandidate",
            "selected_owner_edge_id": selected["selected_owner_edge_id"],
            "selected_next_card": selected["selected_next_card"],
            "reason_token": "StrictEmissionBridgeEligibleCandidateSelectedAfterCoverageReclassification",
        }
    elif reclassified_count:
        decision = {
            "kind": "SelectNativeOwnerCheckpointRerun",
            "selected_owner_edge_id": None,
            "selected_next_card": NEXT_CHECKPOINT,
            "reason_token": "NoSeedCandidateAfterCoverageReclassificationNeedsCheckpointRerun",
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "selected_owner_edge_id": None,
            "selected_next_card": DESIGN_STOP,
            "reason_token": "NoBridgeEligibleStrictEmissionNativeSeedCandidateAfterCoverageReclassification",
        }

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV8",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "reclassified_unconverted_surface_report": rel(REPORT),
            "projection_descriptor_coverage_reclassification_card": rel(REPORT_RECLASS_CARD),
            "bridge_policy": rel(BRIDGE),
            "strict_converter_emission_probe": rel(STRICT_PROBE),
            "previous_rerun": rel(RERUN_007),
        },
        "provenance": {
            "reclassified_unconverted_surface_report_hash": sha256_file(REPORT),
            "projection_descriptor_coverage_reclassification_card_hash": sha256_file(REPORT_RECLASS_CARD),
            "bridge_policy_hash": sha256_file(BRIDGE),
            "strict_converter_emission_probe_hash": sha256_file(STRICT_PROBE),
            "previous_rerun_hash": sha256_file(RERUN_007),
        },
        "reclassified_report_state": {
            "decision": report["decision"]["kind"],
            "reason_token": report["decision"]["reason_token"],
            "scanned_surface_count": report_summary.get("scanned_surface_count"),
            "projection_descriptor_coverage_reclassified_count": reclassified_count,
            "missing_projection_policy_count": report_summary.get("missing_projection_policy_count"),
            "mapped_to_known_owner_count": report_summary.get("mapped_to_known_owner_count"),
            "borrow_policy_needed_count": report_summary.get("borrow_policy_needed_count"),
        },
        "candidate_pool": pool,
        "selected_candidate": {
            "owner_edge_id": selected["selected_owner_edge_id"],
            "selected_next_card": selected["selected_next_card"],
            "reason_token": selected["reason_token"],
        },
        "decision": decision,
        "claims": {
            "reclassified_unconverted_surface_report_consumed": 1,
            "projection_descriptor_coverage_reclassification_consumed": 1,
            "bridge_policy_consumed": 1,
            "strict_converter_emission_probe_consumed": 1,
            "manual_family_selection": 0,
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
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
