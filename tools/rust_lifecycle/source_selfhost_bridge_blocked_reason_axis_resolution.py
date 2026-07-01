#!/usr/bin/env python3
"""Resolve BridgeBlocked reason axes after native seed rerun 007."""

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
OUTPUT = FIXTURES / "source-selfhost-bridge-blocked-reason-axis-resolution-v0.json"

TOKEN = "SOURCE-SELFHOST-BRIDGE-BLOCKED-REASON-AXIS-RESOLUTION-001"
RERUN_007 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-007-v0.json"
BRIDGE_POLICY = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
STRICT_PROBE = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def reason_axes(candidates: list[dict[str, Any]]) -> dict[str, Any]:
    blocked = [item for item in candidates if item.get("bridge_state") == "BridgeBlocked"]
    groups: Counter[tuple[str, ...]] = Counter()
    for item in blocked:
        reasons = tuple(item.get("blocked_by") or ["<none>"])
        groups[reasons] += 1

    pure_gap = [
        item for item in blocked
        if item.get("blocked_by") == ["PolicyGapInDeniedBoundaries"]
    ]
    composite = [
        item for item in blocked
        if "CompositeOrIntegrationOwner" in (item.get("blocked_by") or [])
    ]
    unscoped = [
        item for item in blocked
        if "AlreadyCoveredByUnscopedAdoptionDecision" in (item.get("blocked_by") or [])
    ]
    already_adopted = [
        item for item in blocked
        if "AlreadyHakoAdopted" in (item.get("blocked_by") or [])
    ]

    return {
        "bridge_blocked_count": len(blocked),
        "reason_groups": [
            {
                "blocked_by": list(reasons),
                "candidate_count": count,
            }
            for reasons, count in sorted(groups.items(), key=lambda row: (row[0], row[1]))
        ],
        "axes": [
            {
                "axis": "PolicyGapInDeniedBoundaries",
                "candidate_count": len(pure_gap),
                "selection_eligible": len(pure_gap) > 0,
                "reason_token": "PurePolicyGapAxisHasMachineDerivedCandidates",
                "next_owner_kind": "BridgeBlockedGapClusterResolution",
                "next_card": "MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001",
            },
            {
                "axis": "CompositeOrIntegrationOwner",
                "candidate_count": len(composite),
                "selection_eligible": False,
                "reason_token": "CompositeRequiresDecompositionBeforePolicyGapRepair",
                "next_owner_kind": "CompositeDecomposition",
                "next_card": "MIRBUILDER-BRIDGE-BLOCKED-COMPOSITE-DECOMPOSITION-001",
            },
            {
                "axis": "AlreadyCoveredByUnscopedAdoptionDecision",
                "candidate_count": len(unscoped),
                "selection_eligible": False,
                "reason_token": "UnscopedAdoptionRequiresScopeNormalizationBeforeSeedSelection",
                "next_owner_kind": "AdoptionScopeNormalization",
                "next_card": "MIRBUILDER-BRIDGE-BLOCKED-ADOPTION-SCOPE-NORMALIZATION-001",
            },
            {
                "axis": "AlreadyHakoAdopted",
                "candidate_count": len(already_adopted),
                "selection_eligible": False,
                "reason_token": "AlreadyAdoptedRowsAreNotRepairCandidates",
                "next_owner_kind": "None",
                "next_card": None,
            },
        ],
    }


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_007)
    selection = build_selection(cutoff_token="MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-007")
    axes = reason_axes(selection.get("candidates", []))
    eligible = [axis for axis in axes["axes"] if axis["selection_eligible"]]

    if len(eligible) == 1:
        selected = eligible[0]
        decision = {
            "kind": "SelectBridgeBlockedGapClusterResolution",
            "reason_token": "ExactlyOneBridgeBlockedReasonAxisEligible",
            "selected_axis": selected["axis"],
            "selected_next_card": selected["next_card"],
        }
    else:
        decision = {
            "kind": "KeepStopped",
            "reason_token": "NoUniqueBridgeBlockedReasonAxis",
            "selected_axis": None,
            "selected_next_card": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        }

    return {
        "schema_version": 0,
        "kind": "SourceSelfhostBridgeBlockedReasonAxisResolutionV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "native_owner_seed_rerun_007": rel(RERUN_007),
            "bridge_policy": rel(BRIDGE_POLICY),
            "strict_converter_emission_probe": rel(STRICT_PROBE),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
        },
        "provenance": {
            "native_owner_seed_rerun_007_hash": sha256_file(RERUN_007),
            "bridge_policy_hash": sha256_file(BRIDGE_POLICY),
            "strict_converter_emission_probe_hash": sha256_file(STRICT_PROBE),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
        },
        "input_decision": rerun["decision"],
        "input_candidate_pool": rerun["candidate_pool"],
        "reason_axis_resolution": axes,
        "selection_rule": {
            "exclude_already_hako_adopted": True,
            "exclude_unscoped_adoption_before_scope_normalization": True,
            "exclude_composite_before_decomposition": True,
            "select_pure_policy_gap_axis_if_unique": True,
            "cluster_size_as_proof": False,
            "manual_axis_selection": False,
        },
        "decision": decision,
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
        print("source-selfhost-bridge-blocked-reason-axis-resolution unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
