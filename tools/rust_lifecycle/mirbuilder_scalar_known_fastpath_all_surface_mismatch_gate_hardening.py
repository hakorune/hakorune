#!/usr/bin/env python3
"""Harden the all-surface ScalarKnown fast-path shadow mismatch gate."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-ALL-SURFACE-MISMATCH-GATE-HARDENING-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-HAKO-AUTHORITY-PILOT-DESIGN-CONSULTATION-001"

CLOSEOUT = FIXTURES / "mirbuilder-scalar-known-fastpath-connected-closeout-rerun-v0.json"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
STRING_ROUTES = ROOT / "src/mir/generic_method_route_plan/string_routes.rs"
COLLECTION_READ_ROUTES = ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs"

SURFACES: list[dict[str, str]] = [
    {
        "surface_id": "WriteScalarI64Routes/PushSurfacePolicy",
        "route_kinds": "ArrayAppendAny",
        "live_route_file": "src/mir/generic_method_route_plan/write_routes.rs",
        "live_call": "write_push_shadow_consumed_decision",
        "helper": "assert_hako_write_push_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/write_push_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_write_push_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/write_push_surface_policy_classifier.hako",
    },
    {
        "surface_id": "WriteScalarI64Routes/SetSurfacePolicy/MapStoreI64",
        "route_kinds": "MapStoreI64",
        "live_route_file": "src/mir/generic_method_route_plan/write_routes.rs",
        "live_call": "mapstore_i64_shadow_consumed_decision",
        "helper": "assert_hako_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_write_set_mapstore_i64_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako",
    },
    {
        "surface_id": "WriteScalarI64Routes/SetSurfacePolicy/MapStoreAny",
        "route_kinds": "MapStoreAny",
        "live_route_file": "src/mir/generic_method_route_plan/write_routes.rs",
        "live_call": "mapstore_any_shadow_consumed_decision",
        "helper": "assert_hako_mapstore_any_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/write_set_mapstore_any_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_write_set_mapstore_any_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako",
    },
    {
        "surface_id": "MapLoadScalarI64Routes",
        "route_kinds": "MapLoadScalarI64",
        "live_route_file": "src/mir/generic_method_route_plan/collection_read_routes.rs",
        "live_call": "mapload_scalar_i64_shadow_consumed_decision",
        "helper": "assert_hako_mapload_scalar_i64_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/mapload_scalar_i64_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_mapload_scalar_i64_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako",
    },
    {
        "surface_id": "StringScalarI64Routes",
        "route_kinds": "StringIndexOf,StringLastIndexOf,StringContains",
        "live_route_file": "src/mir/generic_method_route_plan/string_routes.rs",
        "live_call": "string_scalar_i64_shadow_consumed_decision",
        "helper": "assert_hako_string_scalar_i64_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako",
    },
    {
        "surface_id": "CollectionScalarI64Routes",
        "route_kinds": "MapEntryCount,ArraySlotLen,StringLen,AnyLength",
        "live_route_file": "src/mir/generic_method_route_plan/collection_read_routes.rs",
        "live_call": "collection_scalar_i64_shadow_consumed_decision",
        "helper": "assert_hako_collection_scalar_i64_policy_matches_rust",
        "generated_artifact": "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs",
        "generator": "tools/rust_lifecycle/generate_collection_len_scalar_i64_hako_policy.py",
        "hako_source": "lang/src/compiler/lib/collection_len_scalar_i64_policy_classifier.hako",
    },
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def file_entry(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def surface_rows() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for row in SURFACES:
        artifact = ROOT / row["generated_artifact"]
        generator = ROOT / row["generator"]
        hako_source = ROOT / row["hako_source"]
        live_file = ROOT / row["live_route_file"]
        rows.append(
            {
                **row,
                "generated_artifact_sha256": sha256_file(artifact),
                "generator_sha256": sha256_file(generator),
                "hako_source_sha256": sha256_file(hako_source),
                "live_route_file_sha256": sha256_file(live_file),
            }
        )
    return rows


def build_fixture() -> dict[str, Any]:
    closeout = read_json(CLOSEOUT)
    closeout_decision = closeout.get("decision") or {}
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathAllSurfaceMismatchGateHardeningV1",
        "token": TOKEN,
        "input_state": {
            "closeout_rerun": rel(CLOSEOUT),
            "closeout_rerun_hash": sha256_file(CLOSEOUT),
            "closeout_selected_next_card": closeout_decision.get("selected_next_card"),
            "closeout_fastpath_connected": (closeout.get("summary") or {}).get(
                "fastpath_connected_closeout"
            ),
        },
        "provenance": {
            "shadow_consumer": file_entry(SHADOW_SOURCE),
            "write_routes": file_entry(WRITE_ROUTES),
            "string_routes": file_entry(STRING_ROUTES),
            "collection_read_routes": file_entry(COLLECTION_READ_ROUTES),
        },
        "surfaces": surface_rows(),
        "hardening": {
            "decision": "SelectAllSurfaceMismatchGateHardeningBeforeAuthorityPilot",
            "all_scalar_known_shadow_mismatch_gate_current": True,
            "generated_typed_artifact_drift_check_current": True,
            "shadow_consumer_mismatch_tests_current": True,
            "runtime_hako_source_text_parsing": False,
            "runtime_source_text_parser_forbidden_tokens": ["include_str!", "split('|')"],
            "rust_authority_retained": True,
            "hako_runtime_route_authority": False,
            "authority_switch_deferred": True,
            "cargo_test_filter": "scalar_known_hako_shadow",
            "minimum_should_panic_mismatch_tests": 13,
        },
        "decision": {
            "kind": "SelectMapLoadAuthorityPilotDesignConsultation",
            "reason_token": "AllSurfaceMismatchGateCurrentRustAuthorityRetained",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "all_surface_mismatch_gate_hardening": 1,
            "all_scalar_known_shadow_mismatch_gate_current": 1,
            "generated_typed_artifact_drift_check_current": 1,
            "shadow_consumer_mismatch_tests_current": 1,
            "connected_surface_row_count": len(SURFACES),
            "runtime_hako_source_text_parsing": 0,
            "rust_authority_retained": 1,
            "hako_runtime_route_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "all_surface_mismatch_gate_hardening": 1,
            "all_scalar_known_shadow_mismatch_gate_current": 1,
            "generated_typed_artifact_drift_check_current": 1,
            "shadow_consumer_mismatch_tests_current": 1,
            "rust_authority_retained": 1,
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
        print("mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
