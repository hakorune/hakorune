#!/usr/bin/env python3
"""Record the completed MapStoreI64 caller-orientation pilot."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-mapstore-i64-caller-orientation-authority-pilot-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-MAPSTORE-I64-CALLER-ORIENTATION-PILOT-DESIGN-STOP-001"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3454-MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001.md"
CALLER = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_route_policy.rs"
CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/write_set_mapstore_i64_caller_orientation_contract.rs"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathMapStoreI64CallerOrientationAuthorityPilotV1",
        "token": TOKEN,
        "scope": {
            "surface": "SetSurfacePolicy",
            "route_kind": "MapStoreI64",
            "policy_row_id": "map_store_i64_set_surface",
            "authority_scope": "policy_row_id_contract_only",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "key_domain": "I64",
            "stored_value_domain": "Any",
            "mutation_boundary": "DeclaredMetadataOnly",
        },
        "provenance": {
            "card": evidence(CARD),
            "caller_orientation": evidence(CALLER),
            "shadow": evidence(SHADOW),
            "typed_policy": evidence(POLICY),
            "caller_contract": evidence(CONTRACT),
        },
        "decision": {
            "kind": "MapStoreI64CallerOrientationContractAuthorityPilot",
            "route_decision_authority_retained": True,
            "rust_oracle_compat_veto_retained": True,
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "mapstore_i64_caller_orientation_authority_pilot": 1,
            "mapstore_i64_caller_orientation_authority_scope_policy_row_id_contract_only": 1,
            "mapstore_i64_caller_orientation_consumer_unit_only": 1,
            "mapstore_i64_key_domain_i64": 1,
            "mapstore_i64_stored_value_domain_any": 1,
            "mapstore_i64_mismatch_fail_fast": 1,
            "rust_route_match_authority_retained": 1,
            "rust_compatibility_veto_retained": 1,
            "mutation_boundary_declared_but_not_authorized": 1,
            "caller_selected_route_authority": 0,
            "caller_runtime_dispatch_authority": 0,
            "caller_orientation_runtime_path": 0,
            "runtime_mutation_authority": 0,
            "backend_lowering_authority": 0,
            "publication_execution": 0,
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
        print("mapstore-i64 caller-orientation authority pilot unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
