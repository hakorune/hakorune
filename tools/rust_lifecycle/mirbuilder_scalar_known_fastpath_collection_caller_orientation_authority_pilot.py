#!/usr/bin/env python3
"""Record the checked-in Collection caller-orientation authority pilot."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-collection-caller-orientation-authority-pilot-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001"

CALLER = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"
POLICY = ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/collection_scalar_i64_caller_orientation_contract.rs"
CARD = ROOT / "docs/development/current/main/phases/phase-296x/3450-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-001.md"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathCollectionCallerOrientationAuthorityPilotV1",
        "token": TOKEN,
        "scope": {
            "surface": "CollectionScalarI64Routes",
            "policy_row_ids": [
                "collection_map_entry_count_scalar_i64_routes",
                "collection_array_slot_len_scalar_i64_routes",
                "collection_string_len_scalar_i64_routes",
                "collection_any_length_scalar_i64_routes",
            ],
            "receiver_domains": ["MapBox", "ArrayBox", "StringBox", "Box"],
            "authority_scope": "policy_row_id_contract_only",
            "consumer_input": "PolicyRowIdOnly",
            "consumer_return": "Unit",
            "receiver_domain_input": "Forbidden",
            "anylength_box_semantics": "ExplicitRowMetadataNotWildcardSelector",
        },
        "provenance": {
            "implementation_card": evidence(CARD),
            "caller_orientation_module": evidence(CALLER),
            "shadow_route_module": evidence(SHADOW),
            "collection_policy_artifact": evidence(POLICY),
            "collection_caller_contract_artifact": evidence(CONTRACT),
        },
        "decision": {
            "kind": "CollectionCallerOrientationContractAuthorityPilot",
            "route_decision_authority_retained": True,
            "rust_oracle_compat_veto_retained": True,
            "mixed_receiver_domain_guarded": True,
            "anylength_box_explicit_row_guarded": True,
            "fallback": False,
            "runtime_backend_mutation_publication": False,
            "implementation_complete": True,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "collection_caller_orientation_authority_pilot": 1,
            "collection_caller_orientation_authority_scope_policy_row_id_contract_only": 1,
            "collection_caller_orientation_consumer_unit_only": 1,
            "collection_exact_four_row_scope": 1,
            "collection_mixed_receiver_domain_guarded": 1,
            "collection_anylength_box_explicit_row_guarded": 1,
            "collection_hako_route_decision_authority_retained": 1,
            "collection_rust_oracle_compat_checker_retained": 1,
            "collection_mismatch_fail_fast": 1,
            "no_new_route_authority": 1,
            "caller_orientation_runtime_path": 0,
            "hako_runtime_route_authority": 0,
            "scalar_known_hako_runtime_route_authority": 0,
            "route_selection_authority_switch": 0,
            "receiver_domain_authority_switch": 0,
            "receiver_domain_widening_authority": 0,
            "any_length_wildcard_selector": 0,
            "runtime_box_domain_fallback": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
            "non_delete_write_caller_orientation_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "scalar_known_wide_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
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
        print("collection caller-orientation authority pilot unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
