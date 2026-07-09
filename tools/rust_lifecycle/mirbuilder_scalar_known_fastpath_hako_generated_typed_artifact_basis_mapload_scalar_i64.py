#!/usr/bin/env python3
"""Define the MapLoadScalarI64 generated typed Hako artifact basis."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-"
    "MAPLOAD-SCALAR-I64-001"
)
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-"
    "CONSUME-MAPLOAD-SCALAR-I64-001"
)

SELECTION = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-read-surface-generated-typed-artifact-selection-design-consultation-v0.json"
)
SCALAR_CONTRACT = ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    selection = read_json(SELECTION)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactBasisMaploadScalarI64V1",
        "token": TOKEN,
        "input_state": {
            "selection_consultation": rel(SELECTION),
            "selection_consultation_hash": sha256_file(SELECTION),
            "selection_decision": (selection.get("decision") or {}).get("kind"),
            "selection_reason": (selection.get("decision") or {}).get("reason_token"),
            "selection_selected_next_card": (selection.get("decision") or {}).get(
                "selected_next_card"
            ),
        },
        "basis": {
            "surface": "MapLoadScalarI64Routes",
            "route_kind": "MapLoadScalarI64",
            "core_op": "MapGet",
            "lowering_tier": "WarmDirectAbi",
            "return_shape": "ScalarI64OrMissingZero",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "read",
            "proof_family": "ScalarI64MapGetStoreFact",
            "allowed_existing_proofs": [
                "MapSetScalarI64SameKeyNoEscape",
                "MapSetScalarI64DominatesNoEscape",
                "MapSetScalarI64CoveredDynamicI64KeyNoEscape",
            ],
            "next_mechanism": "CheckedInGeneratedTypedHakoArtifactShadowConsume",
            "runtime_authority": "RustRetained",
            "generated_artifact_allowed_next": True,
            "fastpath_connection_allowed_next": True,
            "runtime_hako_source_text_parsing_allowed": False,
            "build_rs_hako_compiler_invocation_allowed": False,
        },
        "provenance": {
            "scalar_known_contract": rel(SCALAR_CONTRACT),
            "scalar_known_contract_hash": sha256_file(SCALAR_CONTRACT),
            "collection_read_routes": rel(COLLECTION_READ_ROUTES),
            "collection_read_routes_hash": sha256_file(COLLECTION_READ_ROUTES),
        },
        "decision": {
            "kind": "SelectMapLoadScalarI64GeneratedTypedArtifactShadowConsumeImplementation",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapload_scalar_i64_generated_typed_artifact_basis": 1,
            "checked_in_generated_typed_artifact_allowed_next": 1,
            "fastpath_shadow_consume_allowed_next": 1,
            "basis_only": 1,
            "generated_typed_hako_artifact_created": 0,
            "generated_typed_hako_artifact_shadow_consumed": 0,
            "mapload_fastpath_shadow_consumed": 0,
            "read_surface_connection_complete": 0,
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
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
