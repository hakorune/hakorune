#!/usr/bin/env python3
"""Close out the non-Delete Write `.hako` route-decision authority island."""

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
    / "mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout-v0.json"
)

TOKEN = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-DELETE-RETIRED-PARK-"
    "NON-DELETE-WRITE-AUTHORITY-ISLAND-CLOSEOUT-001"
)
NEXT_CARD = (
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-"
    "AUTHORITY-ISLAND-CLOSEOUT-DESIGN-STOP-001"
)

DESIGN_STOP = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-write-delete-surface-authority-design-stop-v0.json"
)
MAPSTORE_I64 = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-write-set-mapstore-i64-hako-route-decision-authority-pilot-v0.json"
)
PUSH = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-push-write-hako-route-decision-authority-pilot-v0.json"
)
MAPSTORE_ANY = (
    FIXTURES
    / "mirbuilder-scalar-known-fastpath-mapstore-any-write-hako-route-decision-authority-pilot-v0.json"
)
MISMATCH_GATE = (
    FIXTURES / "mirbuilder-scalar-known-fastpath-all-surface-mismatch-gate-hardening-v0.json"
)
DELETE_RETIRE_CARD = ROOT / "docs/development/current/main/phases/phase-296x/3353-MIRBUILDER-SCALAR-KNOWN-WRITE-DELETE-SURFACE-MIRROR-RETIRE-001.md"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
SHADOW_SOURCE = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def input_fixture(path: Path) -> dict[str, Any]:
    data = read_json(path)
    return {
        "path": rel(path),
        "sha256": sha256_file(path),
        "token": data.get("token"),
    }


def build_fixture() -> dict[str, Any]:
    design_stop = read_json(DESIGN_STOP)
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathDeleteRetiredParkNonDeleteWriteAuthorityIslandCloseoutV1",
        "token": TOKEN,
        "input_state": {
            "delete_authority_design_stop": input_fixture(DESIGN_STOP),
            "delete_design_stop_decision": (design_stop.get("decision") or {}).get("reason_token"),
            "delete_retire_card": rel(DELETE_RETIRE_CARD),
            "delete_retire_card_hash": sha256_file(DELETE_RETIRE_CARD),
            "all_surface_mismatch_gate": input_fixture(MISMATCH_GATE),
        },
        "closed_set": {
            "surface_set_id": (
                "SetSurfacePolicy_MapStoreI64__PushSurfacePolicy_ArrayAppendAny__"
                "SetSurfacePolicy_MapStoreAny"
            ),
            "surfaces": [
                "SetSurfacePolicy/MapStoreI64",
                "PushSurfacePolicy/ArrayAppendAny",
                "SetSurfacePolicy/MapStoreAny",
            ],
            "closeout_scope_non_delete_write_only": True,
            "delete_surface_excluded": True,
        },
        "input_fixtures": {
            "set_mapstore_i64_authority_pilot": input_fixture(MAPSTORE_I64),
            "push_arrayappendany_authority_pilot": input_fixture(PUSH),
            "set_mapstore_any_authority_pilot": input_fixture(MAPSTORE_ANY),
        },
        "provenance": {
            "write_routes": {"path": rel(WRITE_ROUTES), "sha256": sha256_file(WRITE_ROUTES)},
            "shadow_consumer": {"path": rel(SHADOW_SOURCE), "sha256": sha256_file(SHADOW_SOURCE)},
        },
        "proof_axis": {
            "closed_enumerated_non_delete_write_authority_surface_set": True,
            "prior_scoped_non_delete_write_hako_route_decision_authority_pilots": True,
            "delete_surface_retired_special_case_parked": True,
            "rust_delete_route_preservation_guard_retained": True,
            "generated_typed_artifact_mismatch_gate_current_for_non_delete_write": True,
            "rust_oracle_compat_fail_fast_retained": True,
            "no_write_wide_authority_claim": True,
        },
        "decision": {
            "kind": "SelectPostNonDeleteWriteAuthorityIslandCloseoutDesignStop",
            "reason_token": "NonDeleteWriteAuthorityIslandClosedDeleteParkedAsRetiredRustPreservedRoute",
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "non_delete_write_hako_route_decision_authority_island_closeout": 1,
            "delete_surface_retired_special_case_parked": 1,
            "delete_surface_hako_mirror_retired": 1,
            "delete_surface_live_rust_route_preserved": 1,
            "delete_surface_direct_closeout_materialized": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "source_selfhost_claim": 0,
        },
        "claims": {
            "non_delete_write_hako_route_decision_authority_island_closeout": 1,
            "closed_enumerated_non_delete_write_authority_surface_set": 1,
            "prior_scoped_non_delete_write_hako_route_decision_authority_pilots": 1,
            "closed_non_delete_write_surface_set": (
                "SetSurfacePolicy_MapStoreI64__PushSurfacePolicy_ArrayAppendAny__"
                "SetSurfacePolicy_MapStoreAny"
            ),
            "set_mapstore_i64_hako_route_decision_authority_pilot": 1,
            "push_arrayappendany_hako_route_decision_authority_pilot": 1,
            "set_mapstore_any_hako_route_decision_authority_pilot": 1,
            "delete_surface_retired_special_case_parked": 1,
            "delete_surface_hako_mirror_retired": 1,
            "delete_surface_live_rust_route_preserved": 1,
            "delete_surface_direct_closeout_materialized": 0,
            "rust_oracle_compat_fail_fast_retained": 1,
            "generated_typed_artifact_mismatch_gate_current_for_non_delete_write": 1,
            "closeout_scope_non_delete_write_only": 1,
            "delete_hako_route_decision_authority_pilot": 0,
            "delete_hako_authority_result_consumed": 0,
            "delete_live_route_calls_authority_pilot": 0,
            "mapdeleteany_authority": 0,
            "delete_generated_typed_artifact_authority": 0,
            "delete_classifier_hako_authority": 0,
            "delete_shadow_consumer_authority": 0,
            "delete_mirror_reactivated": 0,
            "retired_delete_mirror_as_authority": 0,
            "write_surface_authority_closeout": 0,
            "write_wide_authority": 0,
            "write_mutation_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "scalar_known_transport_axis_authority_switch": 0,
            "rust_fastpath_rewired": 0,
            "route_selection_authority_switch": 0,
            "backend_lowering_authority": 0,
            "caller_orientation_runtime_path": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "route_count_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
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
        print("mirbuilder-scalar-known-fastpath-delete-retired-park-non-delete-write-authority-island-closeout unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
