#!/usr/bin/env python3
"""Materialize the MapLoad-only caller-orientation basis fixture."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-NARROW-CALLER-ORIENTATION-BASIS-001"
NEXT_CARD = TOKEN
MAPLOAD_PILOT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot-v0.json"
MAPLOAD_BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis-v0.json"
MAPLOAD_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
MAPLOAD_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    pilot = read_json(MAPLOAD_PILOT)
    basis = read_json(MAPLOAD_BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMaploadNarrowCallerOrientationBasisV1",
        "token": TOKEN,
        "input_state": {
            "mapload_route_decision_authority_pilot": rel(MAPLOAD_PILOT),
            "mapload_route_decision_authority_pilot_hash": sha256_file(MAPLOAD_PILOT),
            "mapload_basis": rel(MAPLOAD_BASIS),
            "mapload_basis_hash": sha256_file(MAPLOAD_BASIS),
            "pilot_claim": (pilot.get("claims") or {}).get("mapload_hako_route_decision_authority_pilot"),
            "pilot_rust_oracle_compat_checker": (pilot.get("claims") or {}).get("mapload_rust_oracle_compat_checker"),
            "pilot_mismatch_fail_fast": (pilot.get("claims") or {}).get("mapload_mismatch_fail_fast"),
            "basis_surface": (basis.get("basis") or {}).get("surface"),
        },
        "basis": {
            "surface": "MapLoadScalarI64Routes",
            "route_kind": "MapLoadScalarI64",
            "scope": "single_surface",
            "orientation_kind": "CallerOrientationContractMetadataOnly",
            "authority_source": "ExistingMapLoadHakoRouteDecisionAuthority",
            "rust_role": "oracle / compat checker",
            "mismatch_policy": "FailFast",
            "effect_class": "read",
            "publication_policy": "NoPublication",
            "runtime_consumer": False,
            "backend_lowering_consumer": False,
            "mutation_consumer": False,
            "publication_consumer": False,
        },
        "proof_axis": [
            "PriorScopedMapLoadHakoRouteDecisionAuthority",
            "SingleSurfaceMapLoadCallerOrientationScope",
            "RustOracleCompatFailFastRetained",
            "CallerOrientationMetadataOnly",
            "NoRuntimePathNoBackendLoweringNoMutationNoPublication",
        ],
        "decision": {
            "kind": "AdoptMapLoadNarrowCallerOrientationBasisOnly",
            "selected_next_card": NEXT_CARD,
            "implementation_deferred": True,
        },
        "claims": {
            "mapload_caller_orientation_basis": 1,
            "mapload_hako_route_decision_authority_retained": 1,
            "mapload_rust_oracle_compat_checker_retained": 1,
            "mapload_mismatch_fail_fast": 1,
            "basis_only": 1,
            "mapload_single_surface_scope": 1,
            "caller_orientation_implementation_deferred": 1,
            "caller_orientation_contract_metadata_only": 1,
            "no_new_route_authority": 1,
            "prior_scoped_mapload_hako_route_decision_authority": 1,
            "single_surface_mapload_caller_orientation_scope": 1,
            "rust_oracle_compat_fail_fast_retained": 1,
            "no_runtime_path_no_backend_lowering_no_mutation_no_publication": 1,
            "caller_orientation_runtime_path": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "source_selfhost_claim": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "caller_selected_route_authority": 0,
            "caller_runtime_dispatch_authority": 0,
            "caller_orientation_result_consumed_by_runtime": 0,
            "caller_orientation_result_consumed_by_backend": 0,
            "route_selection_authority_switch": 0,
            "mapload_to_scalar_known_wide_authority": 0,
            "read_surface_to_runtime_authority": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "manual_surface_selection": 0,
        },
        "provenance": {
            "mapload_generated_typed_policy": rel(MAPLOAD_ARTIFACT),
            "mapload_generated_typed_policy_hash": sha256_file(MAPLOAD_ARTIFACT),
            "mapload_route_consumer": rel(MAPLOAD_ROUTES),
            "mapload_route_consumer_hash": sha256_file(MAPLOAD_ROUTES),
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
        print("mirbuilder-scalar-known-fastpath-mapload-narrow-caller-orientation-basis unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
