#!/usr/bin/env python3
"""Record the checked-in MapLoad caller-orientation contract artifact."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-caller-orientation-contract-artifact-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis-v0.json"
CONTRACT = ROOT / "lang/src/compiler/lib/map_load_scalar_i64_caller_orientation_contract.hako"
POLICY = ROOT / "lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako"
ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_caller_orientation_contract.rs"
POLICY_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
GENERATOR = ROOT / "tools/rust_lifecycle/generate_mapload_scalar_i64_caller_orientation_contract.py"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMaploadCallerOrientationContractArtifactV1",
        "token": TOKEN,
        "input_state": {
            "basis": rel(BASIS),
            "basis_hash": sha256_file(BASIS),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
        },
        "contract": {
            "policy_row_id": "map_load_scalar_i64_routes",
            "orientation_kind": "CallerOrientationContractMetadataOnly",
            "scope": "SingleSurface",
            "runtime_consumer": "Forbidden",
            "backend_lowering_consumer": "Forbidden",
            "mutation_consumer": "Forbidden",
            "publication_consumer": "Forbidden",
            "mismatch_policy": "FailFast",
        },
        "provenance": {
            "hako_contract": rel(CONTRACT),
            "hako_contract_hash": sha256_file(CONTRACT),
            "mapload_policy": rel(POLICY),
            "mapload_policy_hash": sha256_file(POLICY),
            "generated_artifact": rel(ARTIFACT),
            "generated_artifact_hash": sha256_file(ARTIFACT),
            "mapload_policy_artifact": rel(POLICY_ARTIFACT),
            "mapload_policy_artifact_hash": sha256_file(POLICY_ARTIFACT),
            "generator": rel(GENERATOR),
            "generator_hash": sha256_file(GENERATOR),
            "existing_mapload_oracle": rel(SHADOW),
            "existing_mapload_oracle_hash": sha256_file(SHADOW),
        },
        "decision": {
            "kind": "MaterializeMapLoadCallerOrientationContractArtifact",
            "runtime_consumer_registered": False,
            "backend_lowering_consumer_registered": False,
            "implementation_complete": True,
        },
        "claims": {
            "mapload_caller_orientation_hako_contract_materialized": 1,
            "mapload_caller_orientation_generated_typed_artifact": 1,
            "mapload_caller_orientation_policy_row_reference_verified": 1,
            "mapload_caller_orientation_artifact_current": 1,
            "mapload_caller_orientation_no_live_consumer_guard": 1,
            "mapload_hako_route_decision_authority_retained": 1,
            "mapload_rust_oracle_compat_checker_retained": 1,
            "mapload_mismatch_fail_fast": 1,
            "caller_orientation_runtime_path": 0,
            "caller_runtime_dispatch_authority": 0,
            "caller_selected_route_authority": 0,
            "caller_orientation_result_consumed_by_runtime": 0,
            "caller_orientation_result_consumed_by_backend": 0,
            "route_selection_authority_switch": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "mapload_to_scalar_known_wide_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "source_selfhost_claim": 0,
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
        print("mirbuilder-scalar-known-fastpath-mapload-caller-orientation-contract-artifact unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
