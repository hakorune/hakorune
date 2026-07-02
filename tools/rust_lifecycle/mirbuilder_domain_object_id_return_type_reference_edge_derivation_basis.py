#!/usr/bin/env python3
"""Define the basis for deriving typed dependency edges from return_type refs."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001"

EDGE_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-edge-evidence-inventory-v0.json"
DOMAIN_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def unresolved_rows(inventory: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        row
        for row in inventory.get("domain_object_id_source_id_ledger") or []
        if row.get("scope_state") == "UnresolvedNonIdDomainObject"
    ]


def build_fixture() -> dict[str, Any]:
    edge_inventory = read_json(EDGE_INVENTORY)
    domain_inventory = read_json(DOMAIN_INVENTORY)
    rows = unresolved_rows(domain_inventory)
    return_type_rows = [row for row in rows if row.get("return_type")]
    distinct_return_types = sorted({str(row["return_type"]) for row in return_type_rows})

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdReturnTypeReferenceEdgeDerivationBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "typed_dependency_edge_evidence_inventory": rel(EDGE_INVENTORY),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(DOMAIN_INVENTORY),
        },
        "provenance": {
            "typed_dependency_edge_evidence_inventory_hash": sha256_file(EDGE_INVENTORY),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(DOMAIN_INVENTORY),
        },
        "previous_state": {
            "selected_evidence_kind": edge_inventory.get("summary", {}).get("selected_evidence_kind"),
            "return_type_field_reference_candidate_count": edge_inventory.get("summary", {}).get(
                "return_type_field_reference_candidate_count"
            ),
            "accepted_edge_ready_count": edge_inventory.get("summary", {}).get(
                "accepted_edge_ready_count"
            ),
        },
        "derivation_basis": {
            "return_type_reference_is_dependency_edge_by_itself": False,
            "return_type_name_to_subaxis_map_allowed": False,
            "hardcoded_return_type_priority_allowed": False,
            "derivation_requires_resource_taxonomy": True,
            "derivation_requires_dependent_and_prerequisite_roles": True,
            "derivation_requires_concrete_source_row_reference": True,
            "derivation_requires_accepted_evidence_kind": "ReturnTypeFieldReference",
        },
        "accepted_derivation_rule": {
            "name": "ReturnTypeReferenceEdgeDerivationV1",
            "edge_direction": "dependent_subaxis_requires_prerequisite_subaxis",
            "may_emit_edge_only_if": [
                "return_type_resource_taxonomy_entry_exists",
                "return_type_resource_domain_subaxis_declared",
                "dependent_source_row_domain_subaxis_declared",
                "dependent_subaxis != prerequisite_subaxis",
                "dependency_role_declared",
                "typed_reference_token_stable",
                "proof_source_stable",
            ],
            "must_not_use": [
                "return_type_name_prefix",
                "return_type_name_contains",
                "observed_subaxis_count",
                "row_count",
                "owner_name",
                "source_path",
                "route_membership_alone",
                "lexical_order",
            ],
        },
        "return_type_inventory": {
            "return_type_reference_count": len(return_type_rows),
            "distinct_return_type_count": len(distinct_return_types),
            "resource_taxonomy_entry_count": 0,
            "edge_ready_return_type_count": 0,
            "sample_return_types": distinct_return_types[:20],
        },
        "decision": {
            "kind": "SelectReturnTypeResourceTaxonomyInventory",
            "reason_token": "ReturnTypeReferenceEdgeDerivationRequiresResourceTaxonomy",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "return_type_reference_edge_derivation_basis_defined": 1,
            "return_type_reference_is_dependency_edge_by_itself": 0,
            "return_type_name_to_subaxis_map_allowed": 0,
            "hardcoded_return_type_priority_allowed": 0,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "manual_family_selection": 0,
            "manual_shape_selection": 0,
            "manual_axis_selection": 0,
            "manual_carrier_selection": 0,
            "manual_subaxis_selection": 0,
            "hardcoded_subaxis_priority": 0,
            "row_count_as_proof": 0,
            "domain_object_count_as_proof": 0,
            "cluster_size_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "owner_name_as_proof": 0,
            "source_path_as_authority": 0,
            "route_membership_alone_as_proof": 0,
            "convenience_as_proof": 0,
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
        print("mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
