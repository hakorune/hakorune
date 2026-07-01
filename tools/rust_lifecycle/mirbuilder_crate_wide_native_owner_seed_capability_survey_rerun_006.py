#!/usr/bin/env python3
"""Rerun native-owner seed capability after type_context adoption."""

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
OUTPUT = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006-v0.json"

TOKEN = "MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-006"
TYPE_ADOPTION = FIXTURES / "mirbuilder-type-context-hako-adoption-decision-v0.json"
BRIDGE = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"
STRICT_PROBE = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
RERUN_005 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-005-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    type_adoption = read_json(TYPE_ADOPTION)
    selection = build_selection(cutoff_token=TOKEN)
    selected = selection["decision"]

    return {
        "schema_version": 0,
        "kind": "MirBuilderCrateWideNativeOwnerSeedCapabilitySurveyRerunV6",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "type_context_adoption_decision": rel(TYPE_ADOPTION),
            "bridge_policy": rel(BRIDGE),
            "strict_converter_emission_probe": rel(STRICT_PROBE),
            "previous_rerun": rel(RERUN_005),
        },
        "provenance": {
            "type_context_adoption_decision_hash": sha256_file(TYPE_ADOPTION),
            "bridge_policy_hash": sha256_file(BRIDGE),
            "strict_converter_emission_probe_hash": sha256_file(STRICT_PROBE),
            "previous_rerun_hash": sha256_file(RERUN_005),
        },
        "adoption_delta": {
            "family_id": type_adoption["family_id"],
            "decision": type_adoption["decision"]["value"],
            "hako_adopted": type_adoption["post_decision_state"]["hako_adopted"],
            "source_selfhost_claim": type_adoption["claims"]["source_selfhost_claim"],
        },
        "candidate_pool": selection["candidate_pool"],
        "selected_candidate": {
            "owner_edge_id": selected["selected_owner_edge_id"],
            "selected_next_card": selected["selected_next_card"],
            "reason_token": selected["reason_token"],
        },
        "decision": {
            "kind": selected["kind"],
            "selected_owner_edge_id": selected["selected_owner_edge_id"],
            "selected_next_card": selected["selected_next_card"],
            "reason_token": "NoBridgeEligibleStrictEmissionNativeSeedCandidateAfterTypeContextAdoption",
        },
        "claims": {
            "type_context_adoption_consumed": 1,
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
        print("mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-006 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
