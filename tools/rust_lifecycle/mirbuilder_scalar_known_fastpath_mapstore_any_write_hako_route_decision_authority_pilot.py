#!/usr/bin/env python3
"""Materialize the scoped MapStoreAny Write `.hako` route-decision authority pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-RERUN-001"

BASIS = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-authority-pilot-basis-v0.json"
ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


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
        "kind": "MirBuilderScalarKnownFastpathMapStoreAnyWriteHakoRouteDecisionAuthorityPilotV1",
        "token": TOKEN,
        "input_state": {
            "basis_fixture": rel(BASIS),
            "basis_fixture_hash": sha256_file(BASIS),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
        },
        "provenance": {
            "mapstore_any_generated_typed_artifact": file_entry(ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
        },
        "implementation": {
            "surface": "SetSurfacePolicy/MapStoreAny",
            "route_kind": "MapStoreAny",
            "authority_function": "mapstore_any_hako_route_authority_pilot_decision",
            "legacy_shadow_wrapper_retained": "mapstore_any_shadow_consumed_decision",
            "live_route_calls_authority_function": True,
            "hako_decision_constructed_from": "WRITE_SET_MAPSTORE_ANY_HAKO_POLICY",
            "rust_oracle_compat_checker": True,
            "mismatch_policy": "FailFast",
            "authority_scope": "SetSurfacePolicyMapStoreAnyOnly",
        },
        "decision": {
            "kind": "SelectMapStoreAnyWriteAuthorityPilotRerun",
            "reason_token": "MapStoreAnyWriteHakoRouteDecisionAuthorityPilotMaterialized",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "mapstore_any_hako_route_decision_authority_pilot": 1,
            "mapstore_any_hako_authority_result_consumed": 1,
            "mapstore_any_live_route_calls_authority_pilot": 1,
            "mapstore_any_rust_oracle_compat_checker": 1,
            "mapstore_any_mismatch_fail_fast": 1,
            "mapstore_any_any_boundary_metadata_only": 1,
            "any_write_boundary_runtime_authority": 0,
            "runtime_mutation_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "mapstore_any_hako_route_decision_authority_pilot": 1,
            "mapstore_any_hako_authority_result_consumed": 1,
            "mapstore_any_live_route_calls_authority_pilot": 1,
            "mapstore_any_rust_oracle_compat_checker": 1,
            "mapstore_any_mismatch_fail_fast": 1,
            "mapstore_any_any_boundary_metadata_only": 1,
            "any_write_boundary_runtime_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "write_wide_authority": 0,
            "write_surface_authority_closeout": 0,
            "mapdeleteany_authority": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
