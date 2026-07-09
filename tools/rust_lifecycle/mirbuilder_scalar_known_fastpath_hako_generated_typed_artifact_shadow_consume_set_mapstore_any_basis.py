#!/usr/bin/env python3
"""Define the MapStoreAny generated typed Hako artifact shadow-consume basis."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "SET-MAPSTORE-ANY-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "SET-MAPSTORE-ANY-001"
)

INVENTORY_RERUN = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-v0.json"
)
MAPSTORE_ANY_HAKO = ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"
MAPSTORE_I64_ARTIFACT = (
    ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
)
SHADOW_RS = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_mapstore_any_row() -> list[str]:
    prefix = '"map_store_any_set_surface|'
    for line in MAPSTORE_ANY_HAKO.read_text(encoding="utf-8").splitlines():
        start = line.find(prefix)
        if start < 0:
            continue
        rest = line[start + 1 :]
        end = rest.find('"')
        if end < 0:
            raise SystemExit("MapStoreAny policy row is unterminated")
        fields = rest[:end].split("|")
        if len(fields) != 14:
            raise SystemExit(f"MapStoreAny policy row has {len(fields)} fields, expected 14")
        return fields
    raise SystemExit("MapStoreAny policy row not found")


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY_RERUN)
    row = read_mapstore_any_row()
    (
        row_id,
        surface,
        route_kind,
        core_op,
        lowering_tier,
        result_class,
        return_shape,
        value_demand,
        value_boundary,
        publication_policy,
        effect_class,
        mutation_class,
        any_boundary_policy,
        role,
    ) = row

    expected = {
        "row_id": "map_store_any_set_surface",
        "surface": "SetSurfacePolicy/MapStoreAny",
        "route_kind": "MapStoreAny",
        "core_op": "MapSet",
        "lowering_tier": "ColdFallback",
        "result_class": "NoneResult",
        "return_shape": "None",
        "value_demand": "WriteAny",
        "value_boundary": "Any",
        "publication_policy": "NonePublication",
        "effect_class": "mutate",
        "mutation_class": "MutatesReceiverOrContainer",
        "any_boundary_policy": "DeclaredMetadataOnly",
        "role": "classifier_policy_mirror_only",
    }
    actual = {
        "row_id": row_id,
        "surface": surface,
        "route_kind": route_kind,
        "core_op": core_op,
        "lowering_tier": lowering_tier,
        "result_class": result_class,
        "return_shape": return_shape,
        "value_demand": value_demand,
        "value_boundary": value_boundary,
        "publication_policy": publication_policy,
        "effect_class": effect_class,
        "mutation_class": mutation_class,
        "any_boundary_policy": any_boundary_policy,
        "role": role,
    }
    if actual != expected:
        raise SystemExit(f"unexpected MapStoreAny policy row: {actual!r}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeSetMapstoreAnyBasisV1",
        "token": TOKEN,
        "input_state": {
            "inventory_rerun": rel(INVENTORY_RERUN),
            "inventory_rerun_hash": sha256_file(INVENTORY_RERUN),
            "inventory_decision": (inventory.get("decision") or {}).get("kind"),
            "inventory_selected_next_card": (inventory.get("decision") or {}).get(
                "selected_next_card"
            ),
        },
        "basis": {
            "surface": "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny",
            "route_kind": "MapStoreAny",
            "prior_connected_surface": "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64",
            "proof_axis": "PriorGeneratedTypedArtifactSameSetSurfacePolicyMinimalDeltaV1",
            "next_mechanism": "CheckedInGeneratedTypedHakoArtifactShadowConsume",
            "runtime_authority": "RustRetained",
            "generated_artifact_allowed_next": True,
            "fastpath_connection_allowed_next": True,
            "runtime_hako_source_text_parsing_allowed": False,
            "build_rs_hako_compiler_invocation_allowed": False,
        },
        "provenance": {
            "mapstore_any_hako_source": rel(MAPSTORE_ANY_HAKO),
            "mapstore_any_hako_source_hash": sha256_file(MAPSTORE_ANY_HAKO),
            "mapstore_i64_generated_artifact": rel(MAPSTORE_I64_ARTIFACT),
            "mapstore_i64_generated_artifact_hash": sha256_file(MAPSTORE_I64_ARTIFACT),
            "shadow_consumer_hash": sha256_file(SHADOW_RS),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
        },
        "artifact_shape": actual,
        "decision": {
            "kind": "SelectMapStoreAnyGeneratedTypedArtifactShadowConsumeImplementation",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapstore_any_generated_typed_artifact_shadow_consume_basis": 1,
            "checked_in_generated_typed_artifact_allowed_next": 1,
            "fastpath_shadow_consume_allowed_next": 1,
            "same_set_surface_policy_minimal_delta": 1,
            "basis_only": 1,
            "generated_typed_hako_artifact_shadow_consumed": 0,
            "checked_in_generated_typed_artifact": 0,
            "fastpath_connected_closeout": 0,
            "runtime_hako_source_text_parsing": 0,
            "hako_runtime_route_authority": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "build_rs_hako_compiler_invocation": 0,
            "live_hako_authority": 0,
            "caller_orientation_runtime_path": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
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
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-set-mapstore-any-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
