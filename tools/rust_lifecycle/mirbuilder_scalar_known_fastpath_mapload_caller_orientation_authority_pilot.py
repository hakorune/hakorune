#!/usr/bin/env python3
"""Record the checked-in MapLoad caller-orientation authority pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-caller-orientation-authority-pilot-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001"
MODULE = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_caller_orientation_contract.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMapLoadCallerOrientationAuthorityPilotV1",
        "token": TOKEN,
        "scope": {
            "surface": "MapLoadScalarI64Routes",
            "route": "MapLoadScalarI64",
            "policy_row_id": "map_load_scalar_i64_routes",
            "authority_scope": "policy_row_id_contract_only",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "exhaustive_row_ids": ["map_load_scalar_i64_routes"],
        },
        "provenance": {
            "caller_orientation_module": rel(MODULE),
            "caller_orientation_module_hash": sha256_file(MODULE),
            "shadow_route_module": rel(SHADOW),
            "shadow_route_module_hash": sha256_file(SHADOW),
            "mapload_policy_artifact": rel(POLICY),
            "mapload_policy_artifact_hash": sha256_file(POLICY),
            "mapload_caller_contract_artifact": rel(CONTRACT),
            "mapload_caller_contract_artifact_hash": sha256_file(CONTRACT),
        },
        "decision": {
            "kind": "MapLoadCallerOrientationContractAuthorityPilot",
            "route_decision_authority_retained": True,
            "rust_oracle_compat_veto_retained": True,
            "fallback": False,
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapload_caller_orientation_authority_pilot": 1,
            "mapload_caller_orientation_authority_scope_policy_row_id_contract_only": 1,
            "mapload_caller_orientation_consumer_unit_only": 1,
            "mapload_hako_route_decision_authority_retained": 1,
            "mapload_rust_oracle_compat_checker_retained": 1,
            "mapload_mismatch_fail_fast": 1,
            "read_caller_orientation_assertion_closeout_retained": 1,
            "non_delete_write_caller_orientation_assertion_closeout_retained": 1,
            "single_surface_mapload_scope": 1,
            "no_new_route_authority": 1,
            "caller_orientation_runtime_path": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "caller_selected_route_authority": 0,
            "caller_runtime_dispatch_authority": 0,
            "caller_orientation_result_consumed_by_runtime": 0,
            "caller_orientation_result_consumed_by_backend": 0,
            "mapload_backend_lowering_authority": 0,
            "mapload_runtime_path_authority": 0,
            "mapload_runtime_fallback": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "write_wide_authority": 0,
            "write_surface_authority_closeout": 0,
            "delete_surface_authority": 0,
            "scalar_known_wide_authority": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "manual_surface_selection": 0,
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
        print("mapload caller-orientation authority pilot unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
