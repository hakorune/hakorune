#!/usr/bin/env python3
"""Consume ScalarKnown fast-path `.hako` connection design consultation."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-hako-connection-design-consultation-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001"

INVENTORY = FIXTURES / "mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory-v0.json"
MAPSTORE_I64_ADOPTION = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-adoption-decision-v0.json"
)
MAPSTORE_I64_CLOSEOUT = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-direct-closeout-rerun-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    inventory = read_json(INVENTORY)
    adoption = read_json(MAPSTORE_I64_ADOPTION)
    closeout = read_json(MAPSTORE_I64_CLOSEOUT)

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoConnectionDesignConsultationV1",
        "token": TOKEN,
        "input_state": {
            "connection_inventory": rel(INVENTORY),
            "connection_inventory_hash": sha256_file(INVENTORY),
            "inventory_decision": inventory.get("decision", {}).get("kind"),
            "inventory_reason": inventory.get("decision", {}).get("reason_token"),
            "mapstore_i64_hako_adoption": rel(MAPSTORE_I64_ADOPTION),
            "mapstore_i64_hako_adoption_hash": sha256_file(MAPSTORE_I64_ADOPTION),
            "mapstore_i64_direct_closeout": rel(MAPSTORE_I64_CLOSEOUT),
            "mapstore_i64_direct_closeout_hash": sha256_file(MAPSTORE_I64_CLOSEOUT),
            "hako_source": rel(HAKO_SOURCE),
            "hako_source_hash": sha256_file(HAKO_SOURCE),
            "rust_fastpath_owner": rel(WRITE_ROUTES),
            "rust_fastpath_owner_hash": sha256_file(WRITE_ROUTES),
            "mapstore_i64_hako_adopted": adoption.get("summary", {}).get(
                "write_set_mapstore_i64_hako_adopted"
            ),
            "mapstore_i64_scoped_closeout_materialized": closeout.get("summary", {}).get(
                "write_set_mapstore_i64_direct_closeout_materialized"
            ),
        },
        "consultation_result": {
            "choice": "B-shadow-consumption-first",
            "connection_mechanism": "CompiledOrGeneratedHakoPolicyArtifactShadowConsumedAtRustFastpath",
            "first_surface": "SetSurfacePolicy/MapStoreI64",
            "authority_policy": {
                "rust_fastpath_authority_retained": True,
                "hako_fastpath_shadow_consumed": True,
                "hako_runtime_route_authority": False,
                "hako_backend_lowering_authority": False,
                "route_selection_authority_switch": False,
            },
            "mismatch_policy": "FailGuardDiagnostic",
            "hako_adopted_redefinition": {
                "hako_adopted_as_executable_mirror": True,
                "fastpath_connected_closeout_until_shadow_consumed": False,
                "runtime_authority_until_explicit_switch": False,
            },
        },
        "selection_rule": {
            "name": "ScalarKnownFastpathHakoShadowConsumeFirstConnectionV1",
            "immediate_authority_switch_allowed": False,
            "shadow_consumption_required_before_authority_switch": True,
            "first_surface_must_be_hako_adopted": True,
            "first_surface_must_have_scoped_closeout": True,
            "first_surface_must_avoid_any_write_boundary": True,
            "route_count_as_proof": False,
            "manual_surface_selection": False,
        },
        "summary": {
            "fastpath_hako_connection_design_consultation": 1,
            "selected_connection_mechanism_shadow_consumption": 1,
            "selected_surface_set_mapstore_i64": 1,
            "hako_adopted_as_executable_mirror": 1,
            "fastpath_connected_closeout": 0,
            "hako_fastpath_runtime_authority": 0,
            "rust_fastpath_authority_retained": 1,
            "route_selection_authority_switch": 0,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectFastpathShadowConsumeHandoff",
            "reason_token": "MapStoreI64HakoAdoptedScopedCloseoutAvoidsAnyBoundary",
            "selected_surface": "SetSurfacePolicy/MapStoreI64",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "design_consultation_consumed": 1,
            "shadow_consumption_first_connection_selected": 1,
            "hako_fastpath_shadow_consumed": 0,
            "rust_fastpath_rewired": 0,
            "hako_runtime_route_authority": 0,
            "hako_backend_lowering_authority": 0,
            "route_selection_authority": 0,
            "new_route_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("mirbuilder-scalar-known-fastpath-hako-connection-design-consultation unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
