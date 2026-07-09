#!/usr/bin/env python3
"""Define the MapStoreI64 Write `.hako` authority pilot basis."""

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
    / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis-v0.json"
)

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-BASIS-001"
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-"
    "HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"
)

DESIGN_STOP = (
    ROOT
    / "docs/development/current/main/phases/phase-296x/3402-MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SURFACE-AUTHORITY-PILOT-DESIGN-STOP-001.md"
)
SHADOW_FIXTURE = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-i64-v0.json"
)
WRITE_ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
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
        "kind": "MirBuilderScalarKnownFastpathWriteSetMapStoreI64HakoAuthorityPilotBasisV1",
        "token": TOKEN,
        "input_state": {
            "design_stop_card": rel(DESIGN_STOP),
            "design_stop_card_hash": sha256_file(DESIGN_STOP),
            "read_surface_authority_closeout_precedes_write_basis": True,
            "prior_shadow_consume_fixture": rel(SHADOW_FIXTURE),
            "prior_shadow_consume_fixture_hash": sha256_file(SHADOW_FIXTURE),
            "prior_shadow_consumed": (shadow.get("claims") or {}).get(
                "generated_typed_hako_artifact_shadow_consumed"
            ),
        },
        "provenance": {
            "mapstore_i64_generated_typed_artifact": file_entry(WRITE_ARTIFACT),
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
        },
        "basis": {
            "basis_only": True,
            "surface": "SetSurfacePolicy/MapStoreI64",
            "route_kind": "MapStoreI64",
            "proof_axis": [
                "ReadSurfaceAuthorityCloseoutPrecedesWriteAuthority",
                "TypedScalarWriteBeforeAnyWrite",
                "PriorGeneratedTypedArtifactShadowConsumed",
                "RustOracleCompatFailFastRetained",
            ],
            "authority_source": "WRITE_SET_MAPSTORE_I64_HAKO_POLICY",
            "rust_oracle_compat_checker_retained": True,
            "mismatch_policy": "FailFast",
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "write_shape": {
            "surface": "SetSurfacePolicy",
            "route_kind": "MapStoreI64",
            "core_op": "MapSet",
            "lowering_tier": "ColdFallback",
            "result_class": "NoneResult",
            "return_shape": "None",
            "value_demand": "WriteAny",
            "value_boundary": "ScalarI64",
            "publication_policy": "NonePublication",
            "effect_class": "mutate",
            "mutation_class": "MutatesReceiverOrContainer",
        },
        "decision": {
            "kind": "SelectWriteSetMapStoreI64RouteDecisionAuthorityPilotImplementation",
            "reason_token": "TypedScalarWriteBoundaryBeforeAnyWrite",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "write_set_mapstore_i64_hako_authority_pilot_basis": 1,
            "selected_surface": "SetSurfacePolicy/MapStoreI64",
            "typed_scalar_write_before_any_write": 1,
            "prior_generated_typed_artifact_shadow_consumed": 1,
            "rust_oracle_compat_fail_fast_retained": 1,
            "basis_only": 1,
            "write_surface_authority_pilot": 0,
            "mapstore_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "write_set_mapstore_i64_hako_authority_pilot_basis": 1,
            "read_surface_authority_closeout_precedes_write_authority": 1,
            "typed_scalar_write_before_any_write": 1,
            "prior_generated_typed_artifact_shadow_consumed": 1,
            "rust_oracle_compat_fail_fast_retained": 1,
            "basis_only": 1,
            "write_surface_authority_pilot": 0,
            "mapstore_authority": 0,
            "mapdelete_authority": 0,
            "arrayappend_authority": 0,
            "write_mutation_authority": 0,
            "write_publication_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
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
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-authority-pilot-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
