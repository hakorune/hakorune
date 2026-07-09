#!/usr/bin/env python3
"""Materialize the scoped MapLoad `.hako` route-decision authority pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-RERUN-001"

BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-mapload-hako-authority-pilot-basis-v0.json"
MAPLOAD_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMaploadHakoRouteDecisionAuthorityPilotV1",
        "token": TOKEN,
        "input_state": {
            "basis_fixture": rel(BASIS),
            "basis_fixture_hash": sha256_file(BASIS),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
            "basis_defined": (basis.get("summary") or {}).get("mapload_hako_authority_pilot_basis"),
        },
        "provenance": {
            "mapload_generated_typed_artifact": file_entry(MAPLOAD_ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "collection_read_routes": file_entry(COLLECTION_READ_ROUTES),
        },
        "implementation": {
            "surface": "MapLoadScalarI64Routes",
            "route_kind": "MapLoadScalarI64",
            "authority_function": "mapload_scalar_i64_hako_route_authority_pilot_decision",
            "legacy_shadow_wrapper_retained": "mapload_scalar_i64_shadow_consumed_decision",
            "live_route_calls_authority_function": True,
            "hako_decision_constructed_from": "MAPLOAD_SCALAR_I64_HAKO_POLICY",
            "rust_oracle_decision_constructed": True,
            "rust_oracle_compat_checker": True,
            "mismatch_policy": "FailFast",
            "runtime_source_text_parsing": False,
            "authority_scope": "MapLoadOnly",
        },
        "mapload_shape": {
            "core_op": "MapGet",
            "lowering_tier": "WarmDirectAbi",
            "return_shape": "ScalarI64OrMissingZero",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "read",
            "proof_family": "ScalarI64MapGetStoreFact",
            "allowed_proof_count": 3,
        },
        "decision": {
            "kind": "SelectMapLoadAuthorityPilotRerun",
            "reason_token": "MapLoadHakoRouteDecisionAuthorityPilotMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "mapload_hako_route_decision_authority_pilot": 1,
            "mapload_hako_authority_result_consumed": 1,
            "mapload_rust_oracle_compat_checker": 1,
            "mapload_mismatch_fail_fast": 1,
            "mapload_live_route_calls_authority_pilot": 1,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "mapload_hako_route_decision_authority_pilot": 1,
            "mapload_hako_authority_result_consumed": 1,
            "mapload_rust_oracle_compat_checker": 1,
            "mapload_mismatch_fail_fast": 1,
            "mapload_live_route_calls_authority_pilot": 1,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "caller_orientation_runtime_path": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "source_selfhost_claim": 0,
            "hako_generation": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "new_python_semantic_projector": 0,
            "manual_surface_selection": 0,
            "row_count_as_proof": 0,
            "route_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
            "route_membership_alone_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-mapload-hako-route-decision-authority-pilot unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
