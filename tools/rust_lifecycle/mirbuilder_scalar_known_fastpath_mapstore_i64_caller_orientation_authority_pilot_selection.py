#!/usr/bin/env python3
"""Record the Pro-selected MapStoreI64 caller-authority pilot boundary."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-mapstore-i64-caller-orientation-authority-pilot-selection-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001"

DESIGN_STOP = ROOT / "docs/development/current/main/phases/phase-296x/3452-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-COLLECTION-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001.md"
POLICY = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_hako_policy.rs"
CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_caller_orientation_contract.rs"
CALLER = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMapStoreI64CallerOrientationAuthorityPilotSelectionV1",
        "token": TOKEN,
        "provenance": {
            "design_stop": evidence(DESIGN_STOP),
            "generated_policy": evidence(POLICY),
            "generated_caller_contract": evidence(CONTRACT),
            "caller_module": evidence(CALLER),
            "shadow_route_module": evidence(SHADOW),
        },
        "selection": {
            "consultation_option": "A",
            "surface": "SetSurfacePolicy",
            "route_kind": "MapStoreI64",
            "policy_row_id": "map_store_i64_set_surface",
            "authority_scope": "policy_row_id_contract_only",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "key_domain": "I64",
            "stored_value_domain": "Any",
            "mutation_boundary": "DeclaredMetadataOnly",
            "mutation_authority": False,
            "any_key_boundary_opened": False,
            "stored_value_any_boundary_present": True,
            "boxshape_repair_required": True,
            "implementation_deferred": True,
            "selected_next_card": NEXT_CARD,
        },
        "authority_map": {
            "route_match_authority": "RustWriteRoutes",
            "policy_row_edit_authority": "HandAuthoredHako",
            "decision_payload_authority": "GeneratedRustArtifactFromHako",
            "compatibility_veto": "RustOracle",
            "mutation_backend_authority": "DownstreamRust",
            "caller_orientation_authority": "PolicyRowContractAcceptanceOnly",
        },
        "claims": {
            "mapstore_i64_caller_orientation_authority_pilot_selected": 1,
            "set_surface_policy_mapstore_i64_single_row_scope": 1,
            "mutation_boundary_declared_but_not_authorized": 1,
            "stored_value_any_boundary_declared": 1,
            "any_key_boundary_not_opened": 1,
            "rust_route_match_authority_retained": 1,
            "hako_decision_payload_edit_authority_retained": 1,
            "rust_compatibility_veto_retained": 1,
            "mapstore_i64_mismatch_fail_fast_required": 1,
            "mapstore_route_policy_key_value_boxshape_repair_required": 1,
            "mapstore_i64_caller_orientation_authority_pilot": 0,
            "runtime_mutation_authority": 0,
            "caller_orientation_runtime_path": 0,
            "backend_lowering_authority": 0,
            "publication_execution": 0,
            "array_append_any_caller_authority": 0,
            "mapstore_any_caller_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "scalar_known_wide_authority": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
            "row_count_as_proof": 0,
            "source_path_as_authority": 0,
            "owner_name_as_proof": 0,
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
        print("mapstore-i64 caller-orientation authority selection unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
