#!/usr/bin/env python3
"""Define the MapStoreAny Write `.hako` authority pilot basis."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-authority-pilot-basis-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-AUTHORITY-PILOT-BASIS-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-ANY-WRITE-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

DESIGN_STOP = ROOT / "docs/development/current/main/phases/phase-296x/3410-MIRBUILDER-SCALAR-KNOWN-FASTPATH-NEXT-WRITE-HAKO-AUTHORITY-SURFACE-DESIGN-STOP-002.md"
SHADOW_FIXTURE = FIXTURES / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-v0.json"
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
    shadow = read_json(SHADOW_FIXTURE)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMapStoreAnyWriteHakoAuthorityPilotBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop_card": rel(DESIGN_STOP),
            "design_stop_card_hash": sha256_file(DESIGN_STOP),
            "prior_shadow_consume_fixture": rel(SHADOW_FIXTURE),
            "prior_shadow_consume_fixture_hash": sha256_file(SHADOW_FIXTURE),
            "prior_shadow_consumed": (shadow.get("claims") or {}).get(
                "generated_typed_hako_artifact_shadow_consumed"
            ),
        },
        "provenance": {
            "mapstore_any_generated_typed_artifact": file_entry(ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
        },
        "basis": {
            "basis_only": True,
            "selected_write_surface": "SetSurfacePolicy/MapStoreAny",
            "selected_route_family": "MapStoreAny",
            "proof_axis": [
                "PriorScopedWriteAuthorityPilotsMapStoreI64AndPush",
                "ExistingGeneratedTypedArtifactShadowConsumed",
                "AnyWriteBoundaryDeclaredButRuntimeAuthorityNotOpened",
                "SetSurfacePolicyContinuationAfterMapStoreI64",
                "RustOracleCompatFailFastRetained",
            ],
            "authority_source": "WRITE_SET_MAPSTORE_ANY_HAKO_POLICY",
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "write_shape": {
            "surface": "SetSurfacePolicy/MapStoreAny",
            "route_kind": "MapStoreAny",
            "core_op": "MapSet",
            "lowering_tier": "ColdFallback",
            "return_shape": "None",
            "value_boundary": "Any",
            "any_boundary_policy": "DeclaredMetadataOnly",
            "any_write_boundary_runtime_authority": False,
        },
        "decision": {
            "kind": "SelectMapStoreAnyWriteRouteDecisionAuthorityPilotImplementation",
            "reason_token": "MapStoreAnyHasGeneratedArtifactAndAnyBoundaryIsMetadataOnly",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "mapstore_any_write_hako_authority_pilot_basis": 1,
            "existing_generated_typed_artifact_shadow_consumed": 1,
            "any_write_boundary_declared_but_runtime_authority_not_opened": 1,
            "set_surface_policy_continuation_after_mapstore_i64": 1,
            "rust_oracle_compat_checker_retained": 1,
            "basis_only": 1,
            "mapstore_any_hako_route_decision_authority_pilot": 0,
            "any_write_boundary_runtime_authority": 0,
            "runtime_mutation_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "mapstore_any_write_hako_authority_pilot_basis": 1,
            "existing_generated_typed_artifact_shadow_consumed": 1,
            "any_write_boundary_declared_but_runtime_authority_not_opened": 1,
            "set_surface_policy_continuation_after_mapstore_i64": 1,
            "rust_oracle_compat_checker_retained": 1,
            "basis_only": 1,
            "mapstore_any_hako_route_decision_authority_pilot": 0,
            "mapstore_any_hako_authority_result_consumed": 0,
            "mapstore_any_live_route_calls_authority_pilot": 0,
            "any_write_boundary_runtime_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "write_wide_authority": 0,
            "write_surface_authority_closeout": 0,
            "mapdeleteany_authority": 0,
            "source_selfhost_claim": 0,
            "route_count_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-authority-pilot-basis unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
