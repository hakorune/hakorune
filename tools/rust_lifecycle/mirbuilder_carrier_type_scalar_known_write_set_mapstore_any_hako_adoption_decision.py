#!/usr/bin/env python3
"""Freeze the ScalarKnown write Set MapStoreAny .hako adoption decision fixture."""

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
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-hako-adoption-decision-v0.json"
)

TOKEN = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "HAKO-ADOPTION-DECISION-001"
)
NEXT_CARD = (
    "MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-"
    "POST-ADOPTION-RERUN-001"
)

PILOT_FIXTURE = (
    FIXTURES
    / "mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-hako-parity-pilot-v0.json"
)
HAKO_SOURCE = ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako"
PARITY_GATE = (
    ROOT
    / "tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_parity_gate.sh"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    pilot = read_json(PILOT_FIXTURE)
    rows = pilot.get("parity_pilot_fixture", {}).get("rows") or []

    return {
        "schema_version": 0,
        "kind": "MirBuilderCarrierTypeScalarKnownWriteSetMapStoreAnyHakoAdoptionDecisionV1",
        "token": TOKEN,
        "input_state": {
            "hako_parity_pilot_fixture": rel(PILOT_FIXTURE),
            "hako_parity_pilot_token": pilot.get("token"),
            "hako_parity_pilot_decision": pilot.get("decision", {}).get("kind"),
            "hako_parity_pilot_selected_next_card": pilot.get("decision", {}).get(
                "selected_next_card"
            ),
            "hako_parity_pilot_hash": sha256_file(PILOT_FIXTURE),
        },
        "evidence": {
            "hako_source": rel(HAKO_SOURCE),
            "hako_source_hash": sha256_file(HAKO_SOURCE),
            "parity_gate": rel(PARITY_GATE),
            "parity_rows": len(rows),
            "parity_status": "green",
        },
        "adoption_decision": {
            "decision": "Adopt",
            "reason_token": "WriteSetMapStoreAnyRustOracleParityGateGreen",
            "adopted_owner": "write_set_mapstore_any_policy_classifier",
            "adopted_surface": "SetSurfacePolicy/MapStoreAny",
            "hako_adopted": True,
            "rust_bootstrap_retained": True,
            "rust_oracle_retained": True,
            "any_write_boundary_declared": True,
            "any_write_boundary_opened": False,
            "selected_next_card": NEXT_CARD,
        },
        "summary": {
            "decision_adopt": 1,
            "write_set_mapstore_any_hako_adopted": 1,
            "hako_adopted_decision": 1,
            "parity_gate_green": 1,
            "parity_rows": len(rows),
            "rust_bootstrap_retained": 1,
            "rust_oracle_retained": 1,
            "any_write_boundary_declared": 1,
            "any_write_boundary_opened": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
        "claims": {
            "decision_adopt": 1,
            "write_set_mapstore_any_hako_adopted": 1,
            "hako_adopted_decision": 1,
            "parity_gate_green": 1,
            "rust_bootstrap_retained": 1,
            "rust_oracle_retained": 1,
            "any_write_boundary_declared": 1,
            "any_write_boundary_opened": 0,
            "write_subsurface_selected": 0,
            "write_direct_closeout_materialized": 0,
            "write_result_policy_ready": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "component_specific_direct_contract_materialized": 0,
            "source_selfhost_claim": 0,
            "new_route_authority": 0,
            "behavior_change": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "rust_deletion": 0,
            "manual_subsurface_selection": 0,
            "route_count_as_proof": 0,
            "apparent_simplicity_as_proof": 0,
            "accepted_read_contract_similarity_as_proof": 0,
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
        print("mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-hako-adoption-decision unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
