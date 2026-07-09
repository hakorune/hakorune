#!/usr/bin/env python3
"""Record StringScalarI64 generated typed Hako artifact shadow consumption."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-"
    "CONSUME-STRING-SCALAR-I64-001"
)
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-005"

BASIS = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-basis-string-scalar-i64-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako"
GENERATED_ARTIFACT = (
    ROOT / "src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs"
)
GENERATOR = ROOT / "tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
STRING_ROUTES = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeStringScalarI64V1",
        "token": TOKEN,
        "input_state": {
            "basis": rel(BASIS),
            "basis_hash": sha256_file(BASIS),
            "basis_decision": (basis.get("decision") or {}).get("kind"),
            "basis_selected_next_card": (basis.get("decision") or {}).get(
                "selected_next_card"
            ),
        },
        "provenance": {
            "hako_source": rel(HAKO_SOURCE),
            "hako_source_hash": sha256_file(HAKO_SOURCE),
            "generated_artifact": rel(GENERATED_ARTIFACT),
            "generated_artifact_hash": sha256_file(GENERATED_ARTIFACT),
            "generator": rel(GENERATOR),
            "generator_hash": sha256_file(GENERATOR),
            "shadow_consumer": rel(SHADOW),
            "shadow_consumer_hash": sha256_file(SHADOW),
            "string_routes": rel(STRING_ROUTES),
            "string_routes_hash": sha256_file(STRING_ROUTES),
        },
        "shadow_consumed_decision": {
            "surface": "StringScalarI64Routes",
            "route_kind_family": [
                "StringIndexOf",
                "StringLastIndexOf",
                "StringContains",
            ],
            "core_ops": [
                "StringIndexOf",
                "StringLastIndexOf",
                "StringContains",
            ],
            "lowering_tier": "WarmDirectAbi",
            "return_shape": "ScalarI64",
            "value_demand": "ScalarI64",
            "publication_policy": "NoPublication",
            "effect_class": "read",
            "proof_or_policy_sources": [
                "IndexOfSurfacePolicy",
                "LastIndexOfSurfacePolicy",
                "ContainsSurfacePolicy",
            ],
            "selected_next_card": NEXT_CARD,
        },
        "implementation": {
            "checked_in_generated_typed_artifact": True,
            "runtime_hako_source_text_parsing": False,
            "string_fastpath_shadow_consumed": True,
            "rust_hako_policy_match": True,
            "mismatch_policy": "panic_guard_fail",
            "rust_authority_retained": True,
        },
        "decision": {
            "kind": "SelectConnectedCloseoutInventoryRerun005",
            "reason_token": "StringScalarI64GeneratedTypedArtifactShadowConsumed",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "generated_typed_hako_artifact_shadow_consumed": 1,
            "checked_in_generated_typed_artifact": 1,
            "runtime_hako_source_text_parsing": 0,
            "string_fastpath_shadow_consumed": 1,
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
        "selected_next_card": NEXT_CARD,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-string-scalar-i64 unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
