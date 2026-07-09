#!/usr/bin/env python3
"""Define the Write Push generated typed Hako artifact shadow-consume basis."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-basis-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "WRITE-PUSH-BASIS-001"
)
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "WRITE-PUSH-001"
)

INVENTORY_RERUN = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-inventory-rerun-002-v0.json"
)
PUSH_HAKO = ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako"
MAPSTORE_ANY_ARTIFACT = (
    ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs"
)
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_push_row() -> list[str]:
    prefix = '"array_append_any_push_surface|'
    for line in PUSH_HAKO.read_text(encoding="utf-8").splitlines():
        start = line.find(prefix)
        if start < 0:
            continue
        rest = line[start + 1 :]
        end = rest.find('"')
        if end < 0:
            raise SystemExit("Push policy row is unterminated")
        fields = rest[:end].split("|")
        if len(fields) != 12:
            raise SystemExit(f"Push policy row has {len(fields)} fields, expected 12")
        return fields
    raise SystemExit("Push policy row not found")


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY_RERUN)
    row = read_push_row()
    (
        row_id,
        surface,
        route_kind,
        core_op,
        lowering_tier,
        result_class,
        return_shape,
        value_demand,
        publication_policy,
        effect_class,
        mutation_class,
        role,
    ) = row

    expected = {
        "row_id": "array_append_any_push_surface",
        "surface": "PushSurfacePolicy",
        "route_kind": "ArrayAppendAny",
        "core_op": "ArrayPush",
        "lowering_tier": "ColdFallback",
        "result_class": "ScalarI64Result",
        "return_shape": "ScalarI64",
        "value_demand": "WriteAny",
        "publication_policy": "NoPublication",
        "effect_class": "mutate",
        "mutation_class": "MutatesReceiverOrContainer",
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
        "publication_policy": publication_policy,
        "effect_class": effect_class,
        "mutation_class": mutation_class,
        "role": role,
    }
    if actual != expected:
        raise SystemExit(f"unexpected Push policy row: {actual!r}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeWritePushBasisV1",
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
            "surface": "WriteScalarI64Routes/PushSurfacePolicy",
            "route_kind": "ArrayAppendAny",
            "prior_connected_surfaces": [
                "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64",
                "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny",
            ],
            "proof_axis": "PriorWriteRouteGeneratedTypedArtifactContinuationV1",
            "next_mechanism": "CheckedInGeneratedTypedHakoArtifactShadowConsume",
            "runtime_authority": "RustRetained",
            "generated_artifact_allowed_next": True,
            "fastpath_connection_allowed_next": True,
            "runtime_hako_source_text_parsing_allowed": False,
            "build_rs_hako_compiler_invocation_allowed": False,
        },
        "provenance": {
            "push_hako_source": rel(PUSH_HAKO),
            "push_hako_source_hash": sha256_file(PUSH_HAKO),
            "mapstore_any_generated_artifact": rel(MAPSTORE_ANY_ARTIFACT),
            "mapstore_any_generated_artifact_hash": sha256_file(MAPSTORE_ANY_ARTIFACT),
            "write_routes_hash": sha256_file(WRITE_ROUTES),
        },
        "artifact_shape": actual,
        "decision": {
            "kind": "SelectWritePushGeneratedTypedArtifactShadowConsumeImplementation",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "write_push_generated_typed_artifact_shadow_consume_basis": 1,
            "checked_in_generated_typed_artifact_allowed_next": 1,
            "fastpath_shadow_consume_allowed_next": 1,
            "prior_write_route_generated_typed_artifact_continuation": 1,
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
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
