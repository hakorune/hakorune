#!/usr/bin/env python3
"""Select the full Collection caller-orientation authority pilot packet."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-collection-caller-orientation-authority-pilot-basis-v0.json"
TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-BASIS-001"

DESIGN_STOP = ROOT / "docs/development/current/main/phases/phase-296x/3448-MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-STRING-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001.md"
ROUTE_BASIS = ROOT / "docs/development/current/main/phases/phase-296x/3396-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-BASIS-001.md"
ROUTE_PILOT = ROOT / "docs/development/current/main/phases/phase-296x/3397-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001.md"
ASSERTION_DECISION = ROOT / "docs/development/current/main/phases/phase-296x/3424-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-CONSUMER-DESIGN-STOP-001.md"
ASSERTION_CONSUMER = ROOT / "docs/development/current/main/phases/phase-296x/3429-MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001.md"
POLICY = ROOT / "src/mir/generic_method_route_plan/generated/collection_len_scalar_i64_hako_policy.rs"
CONTRACT = ROOT / "src/mir/generic_method_route_plan/generated/collection_scalar_i64_caller_orientation_contract.rs"
CALLER = ROOT / "src/mir/generic_method_route_plan/caller_orientation.rs"
SHADOW = ROOT / "src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs"

PACKET = [
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-BASIS-001",
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-001",
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001",
    "MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-COLLECTION-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def build_fixture() -> dict[str, Any]:
    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathCollectionCallerOrientationAuthorityPilotBasisV1",
        "token": TOKEN,
        "provenance": {
            "design_stop": evidence(DESIGN_STOP),
            "route_authority_basis": evidence(ROUTE_BASIS),
            "route_authority_pilot": evidence(ROUTE_PILOT),
            "caller_assertion_decision": evidence(ASSERTION_DECISION),
            "caller_assertion_consumer": evidence(ASSERTION_CONSUMER),
            "generated_policy": evidence(POLICY),
            "generated_caller_contract": evidence(CONTRACT),
            "caller_module": evidence(CALLER),
            "shadow_route_module": evidence(SHADOW),
        },
        "selection": {
            "consultation_option": "B",
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
            "implementation_deferred": True,
            "task_packet": PACKET,
        },
        "proof_axis": [
            "PriorMapLoadAndStringCallerOrientationAuthorityContinuation",
            "PriorCollectionFourRowRouteDecisionAuthority",
            "PriorCollectionPolicyRowIdOnlyAssertionConsumer",
            "ExplicitEnumeratedMixedReceiverDomainBoundary",
            "AnyLengthBoxDomainExplicitRowNotWildcardSelector",
            "RustOracleCompatFailFastRetained",
        ],
        "consultation_boundary": {
            "next_required_at": PACKET[-1],
            "reason": "NonDeleteWriteMutationAndAnyWriteBoundary",
            "collection_requires_external_consultation": False,
            "write_requires_design_selection": True,
        },
        "claims": {
            "collection_caller_orientation_authority_pilot_basis": 1,
            "collection_exact_four_row_scope": 1,
            "collection_mixed_receiver_domain_boundary_retained": 1,
            "collection_anylength_box_explicit_row_retained": 1,
            "collection_hako_route_decision_authority_retained": 1,
            "collection_rust_oracle_compat_checker_retained": 1,
            "collection_mismatch_fail_fast_required": 1,
            "basis_only": 1,
            "no_new_route_authority": 1,
            "collection_caller_orientation_authority_pilot": 0,
            "receiver_domain_authority_switch": 0,
            "receiver_domain_widening_authority": 0,
            "any_length_wildcard_selector": 0,
            "runtime_box_domain_fallback": 0,
            "non_delete_write_caller_orientation_authority": 0,
            "delete_hako_route_decision_authority_pilot": 0,
            "scalar_known_wide_authority": 0,
            "caller_orientation_runtime_path": 0,
            "hako_runtime_route_authority": 0,
            "backend_lowering_authority": 0,
            "runtime_mutation_authority": 0,
            "publication_execution": 0,
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
        print("collection caller-orientation authority pilot basis unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
