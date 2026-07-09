#!/usr/bin/env python3
"""Record the Write Push generated typed Hako artifact shadow consume."""

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
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-"
    "WRITE-PUSH-001"
)
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003"

BASIS = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push-basis-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako"
ARTIFACT = ROOT / "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs"
GENERATOR = ROOT / "tools/rust_lifecycle/generate_write_push_hako_policy.py"
SHADOW_RS = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoGeneratedTypedArtifactShadowConsumeWritePushV1",
        "token": TOKEN,
        "surface": "WriteScalarI64Routes/PushSurfacePolicy",
        "source_hako": rel(HAKO_SOURCE),
        "generated_artifact": rel(ARTIFACT),
        "generator": rel(GENERATOR),
        "runtime_consumer": rel(SHADOW_RS),
        "runtime_path": rel(WRITE_ROUTES),
        "input_state": {
            "basis": rel(BASIS),
            "basis_hash": sha256_file(BASIS),
            "basis_decision": (basis.get("decision") or {}).get("kind"),
            "basis_selected_next_card": (basis.get("decision") or {}).get("selected_next_card"),
        },
        "decision": {
            "kind": "GeneratedTypedHakoArtifactShadowConsume",
            "route_kind": "ArrayAppendAny",
            "rust_authority": "retained",
            "runtime_build_parses_hako_source_text": False,
            "build_rs_hako_compiler_invocation": False,
            "selected_next_card": NEXT_CARD,
        },
        "artifact_fields": {
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
        },
        "source_hashes": {
            "source_hako": sha256_file(HAKO_SOURCE),
            "generated_artifact": sha256_file(ARTIFACT),
            "generator": sha256_file(GENERATOR),
            "runtime_consumer": sha256_file(SHADOW_RS),
            "runtime_path": sha256_file(WRITE_ROUTES),
        },
        "selected_next_card": NEXT_CARD,
        "claims": {
            "generated_typed_hako_artifact_shadow_consumed": 1,
            "checked_in_generated_typed_artifact": 1,
            "runtime_hako_source_text_parsing": 0,
            "write_push_fastpath_shadow_consumed": 1,
            "rust_hako_policy_match": 1,
            "generator_check_guard": 1,
            "rust_authority_retained": 1,
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
        print("mirbuilder-scalar-known-fastpath-hako-generated-typed-artifact-shadow-consume-write-push unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
