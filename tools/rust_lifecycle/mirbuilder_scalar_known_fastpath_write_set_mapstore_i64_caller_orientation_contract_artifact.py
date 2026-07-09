#!/usr/bin/env python3
"""Record the checked-in MapStoreI64 caller-orientation contract artifact."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-caller-orientation-contract-artifact-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-PUSH-ARRAYAPPENDANY-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001"
BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis-v0.json"
CONTRACT = ROOT / "lang/src/compiler/lib/write_set_mapstore_i64_caller_orientation_contract.hako"
POLICY = ROOT / "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"
ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_caller_orientation_contract.rs"
POLICY_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
GENERATOR = ROOT / "tools/rust_lifecycle/generate_write_set_mapstore_i64_caller_orientation_contract.py"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathWriteSetMapStoreI64CallerOrientationContractArtifactV1",
        "token": TOKEN,
        "input_state": {
            "mapstore_i64_authority_basis": rel(BASIS),
            "mapstore_i64_authority_basis_hash": sha256_file(BASIS),
            "basis_surface": (basis.get("basis") or {}).get("surface"),
        },
        "contract": {
            "policy_row_ids": ["map_store_i64_set_surface"],
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
            "mapstore_i64_policy": rel(POLICY),
            "mapstore_i64_policy_hash": sha256_file(POLICY),
            "generated_artifact": rel(ARTIFACT),
            "generated_artifact_hash": sha256_file(ARTIFACT),
            "mapstore_i64_policy_artifact": rel(POLICY_ARTIFACT),
            "mapstore_i64_policy_artifact_hash": sha256_file(POLICY_ARTIFACT),
            "generator": rel(GENERATOR),
            "generator_hash": sha256_file(GENERATOR),
            "existing_mapstore_i64_oracle": rel(SHADOW),
            "existing_mapstore_i64_oracle_hash": sha256_file(SHADOW),
        },
        "decision": {
            "kind": "MaterializeMapStoreI64CallerOrientationContractArtifact",
            "runtime_consumer_registered": False,
            "backend_lowering_consumer_registered": False,
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapstore_i64_caller_orientation_hako_contract_materialized": 1,
            "mapstore_i64_caller_orientation_generated_typed_artifact": 1,
            "mapstore_i64_caller_orientation_policy_row_reference_verified": 1,
            "mapstore_i64_caller_orientation_artifact_current": 1,
            "mapstore_i64_caller_orientation_no_live_consumer_guard": 1,
            "mapstore_i64_hako_route_decision_authority_retained": 1,
            "mapstore_i64_rust_oracle_compat_checker_retained": 1,
            "mapstore_i64_mismatch_fail_fast": 1,
            "mutation_metadata_copied_to_caller_contract": 0,
            "effect_metadata_copied_to_caller_contract": 0,
            "value_boundary_copied_to_caller_contract": 0,
            "caller_orientation_live_consumer": 0,
            "caller_orientation_runtime_path": 0,
            "route_selection_authority_switch": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "write_mutation_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "write_wide_authority": 0,
            "scalar_known_wide_authority": 0,
            "backend_lowering_authority": 0,
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
        print("mapstore-i64 caller-orientation contract artifact unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
