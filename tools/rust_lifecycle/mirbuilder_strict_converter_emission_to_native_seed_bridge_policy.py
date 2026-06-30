#!/usr/bin/env python3
"""Define the strict-emission to native-seed bridge policy."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json"

RERUN_003 = FIXTURES / "mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-003-v0.json"
STRICT_PROBE = FIXTURES / "mirbuilder-strict-converter-emission-probe-v0.json"
UNCONVERTED_REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
DERIVED_TO_NATIVE = ROOT / "docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md"

TOKEN = "MIRBUILDER-STRICT-CONVERTER-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-001"
NEXT = "MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build_fixture() -> dict[str, Any]:
    rerun = read_json(RERUN_003)
    strict_probe = read_json(STRICT_PROBE)

    return {
        "schema_version": 0,
        "kind": "MirBuilderStrictConverterEmissionToNativeSeedBridgePolicyV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
            "native_seed_survey_rerun_003": rel(RERUN_003),
            "strict_converter_emission_probe": rel(STRICT_PROBE),
            "unconverted_surface_report": rel(UNCONVERTED_REPORT),
            "derived_to_native_model": rel(DERIVED_TO_NATIVE),
        },
        "provenance": {
            "native_seed_survey_rerun_003_hash": sha256_file(RERUN_003),
            "strict_converter_emission_probe_hash": sha256_file(STRICT_PROBE),
            "unconverted_surface_report_hash": sha256_file(UNCONVERTED_REPORT),
        },
        "previous_state": {
            "rerun_003_decision": rerun["decision"]["kind"],
            "rerun_003_reason_token": rerun["decision"]["reason_token"],
            "rerun_003_selected_next_card": rerun["decision"]["selected_next_card"],
            "strict_verified_hako_family_ir_count": strict_probe["summary"]["verified_hako_family_ir_count"],
            "strict_probe_hako_generation": strict_probe["claims"]["hako_generation"],
            "strict_probe_rules_changed": strict_probe["claims"]["strict_rules_changed"],
        },
        "policy": {
            "generated_artifact_as_native_edit_authority": False,
            "generated_artifact_as_seed_draft_input": True,
            "seed_draft_input_state_name": "DerivedArtifactSeedDraftInput",
            "native_seed_state_name": "NativeSourceSeed",
            "hako_adopted_state_name": "HakoAdopted",
            "source_selfhost_claim_allowed": False,
        },
        "candidate_requirements": {
            "verified_hako_family_ir_present": True,
            "deterministic_regeneration_present": True,
            "owner_edge_confidence_allowed": [
                "ExactSymbol",
                "FixtureMapped",
            ],
            "generated_artifact_provenance_manifest_present": True,
            "verifier_or_oracle_or_guard_present": True,
            "borrow_policy_gap": False,
            "carrier_policy_gap": False,
            "type_transport_gap": False,
            "route_repairable_inconsistency": False,
            "runtime_fallback": False,
            "new_backend_route": False,
            "new_abi": False,
            "new_python_semantic_projector": False,
        },
        "candidate_states": [
            {
                "state": "BridgeEligible",
                "meaning": "May be selected by stable priority for a native seed materialization card.",
            },
            {
                "state": "BridgeBlocked",
                "required_reason_token": True,
            },
        ],
        "decision": {
            "kind": "PolicyDefined",
            "reason_token": "StrictEmissionToNativeSeedBridgePolicyDefined",
            "selected_next_card": NEXT,
        },
        "claims": {
            "native_seed_survey_rerun_003_consumed": 1,
            "strict_converter_emission_probe_consumed": 1,
            "strict_verified_hako_family_ir_count": strict_probe["summary"]["verified_hako_family_ir_count"],
            "previous_keep_stopped_consumed": 1,
            "generated_artifact_as_native_edit_authority": 0,
            "generated_artifact_as_seed_draft_input": 1,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_family_selection": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in bridge policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-strict-converter-emission-to-native-seed-bridge-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
