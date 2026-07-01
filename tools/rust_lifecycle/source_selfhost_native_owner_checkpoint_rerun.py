#!/usr/bin/env python3
"""Rerun the Source Selfhost native-owner checkpoint after seed rerun 008."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed
from source_selfhost_native_owner_checkpoint import native_owner_rows, blocker_quality


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "source-selfhost-native-owner-checkpoint-rerun-v0.json"

TOKEN = "SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_MISSING_PROJECTION = "MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
MANIFEST = FIXTURES / "source-selfhost-family-guard-manifest-v0.json"
SEED_RERUN_008 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    report = read_json(REPORT)
    manifest = read_json(MANIFEST)
    seed_rerun = read_json(SEED_RERUN_008)
    native_owners = native_owner_rows(manifest)
    blockers = blocker_quality(report)
    blockers["MissingProjectionPolicy"]["next_card"] = NEXT_MISSING_PROJECTION
    missing = blockers["MissingProjectionPolicy"]
    borrow = blockers["BorrowSurfaceNeedsPolicy"]

    if missing["selection_eligible"] and not borrow["selection_eligible"]:
        decision = {
            "kind": "SelectMissingProjectionPolicyClusterResolutionV3",
            "reason_token": "MissingProjectionPolicyEvidenceQualityWinsAfterCoverageReclassification",
            "selected_blocker_class": "MissingProjectionPolicy",
            "selected_next_card": NEXT_MISSING_PROJECTION,
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUniqueNativeOwnerCheckpointBlockerClassAfterCoverageReclassification",
            "selected_blocker_class": None,
            "selected_next_card": DESIGN_STOP,
        }

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostNativeOwnerCheckpointRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "seed_capability_rerun_008": rel(SEED_RERUN_008),
            "unconverted_surface_report": rel(REPORT),
            "source_selfhost_family_guard_manifest": rel(MANIFEST),
        },
        "provenance": {
            "seed_capability_rerun_008_hash": sha256_file(SEED_RERUN_008),
            "unconverted_surface_report_hash": sha256_file(REPORT),
            "source_selfhost_family_guard_manifest_hash": sha256_file(MANIFEST),
        },
        "seed_rerun_state": seed_rerun.get("decision"),
        "native_owner_map": {
            "native_owner_count": len(native_owners),
            "owners": native_owners,
        },
        "blocker_class_evidence": blockers,
        "selection_rule": {
            "route_repair_precedes_policy_lanes": True,
            "fresh_report_required": True,
            "seed_rerun_008_consumed": True,
            "evidence_quality_precedes_candidate_count": True,
            "missing_projection_requires_fixture_mapped_known_shape": True,
            "borrow_surface_requires_owner_confidence": True,
            "source_selfhost_claim_allowed": False,
            "manual_blocker_class_selection": False,
        },
        "decision": decision,
        "claims": {
            "native_owner_checkpoint_rerun": 1,
            "source_selfhost_claim": 0,
            "rust_deletion": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "candidate_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "generated_artifact_as_native_edit_authority": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
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
        print("source-selfhost-native-owner-checkpoint-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
