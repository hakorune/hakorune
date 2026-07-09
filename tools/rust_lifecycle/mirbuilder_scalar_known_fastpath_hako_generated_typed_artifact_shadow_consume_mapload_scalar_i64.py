#!/usr/bin/env python3
"""Record the MapLoadScalarI64 generated typed Hako artifact shadow consume."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-"
    "CONSUME-MAPLOAD-SCALAR-I64-001"
)
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-004"

BASIS = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-mapload-scalar-i64-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako"
ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs"
GENERATOR = ROOT / "tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py"
SHADOW_RS = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeMaploadScalarI64V1",
        "token": TOKEN,
        "surface": "MapLoadScalarI64Routes",
        "source_hako": rel(HAKO_SOURCE),
        "generated_artifact": rel(ARTIFACT),
        "generator": rel(GENERATOR),
        "runtime_consumer": rel(SHADOW_RS),
        "runtime_path": rel(COLLECTION_READ_ROUTES),
        "input_state": {
            "basis": rel(BASIS),
            "basis_hash": sha256_file(BASIS),
            "basis_decision": (basis.get("decision") or {}).get("kind"),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
        },
        "decision": {
            "kind": "GeneratedTypedHakoArtifactShadowConsume",
            "route_kind": "MapLoadScalarI64",
            "rust_authority": "retained",
            "runtime_build_parses_hako_source_text": False,
            "build_rs_hako_compiler_invocation": False,
            "selected_next_card": NEXT_CARD,
        },
        "artifact_fields": {
            "surface": "MapLoadScalarI64Routes",
            "route_kind": "MapLoadScalarI64",
            "core_op": "MapGet",
            "lowering_tier": "WarmDirectAbi",
            "result_class": "ScalarI64OrMissingZeroResult",
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
            "role": "classifier_policy_mirror_only",
        },
        "source_hashes": {
            "source_hako": sha256_file(HAKO_SOURCE),
            "generated_artifact": sha256_file(ARTIFACT),
            "generator": sha256_file(GENERATOR),
            "runtime_consumer": sha256_file(SHADOW_RS),
            "runtime_path": sha256_file(COLLECTION_READ_ROUTES),
        },
        "selected_next_card": NEXT_CARD,
        "claims": {
            "generated_typed_hako_artifact_shadow_consumed": 1,
            "checked_in_generated_typed_artifact": 1,
            "runtime_hako_source_text_parsing": 0,
            "mapload_fastpath_shadow_consumed": 1,
            "rust_hako_policy_match": 1,
            "generator_check_guard": 1,
            "rust_authority_retained": 1,
            "read_surface_connection_complete": 0,
            "fastpath_connected_closeout": 0,
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
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-mapload-scalar-i64 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
