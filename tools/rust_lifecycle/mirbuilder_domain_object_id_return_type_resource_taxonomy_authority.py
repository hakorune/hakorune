#!/usr/bin/env python3
"""Define return_type resource taxonomy authority without self-signed proof."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-return-type-resource-taxonomy-authority-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_RERUN = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-EDGE-DERIVATION-RERUN-001"

TAXONOMY_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory-v0.json"
DERIVATION_BASIS = FIXTURES / "mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json"
ROOT_BASIS = FIXTURES / "mirbuilder-domain-object-id-typed-dependency-root-authority-basis-v0.json"
DOMAIN_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    taxonomy_inventory = read_json(TAXONOMY_INVENTORY)
    derivation_basis = read_json(DERIVATION_BASIS)
    return_type_rows = taxonomy_inventory.get("return_type_taxonomy_rows") or []

    readiness_rows = []
    for row in return_type_rows:
        readiness_rows.append(
            {
                "return_type": row["return_type"],
                "diagnostic_reference_count": row["reference_count"],
                "resource_taxonomy_join_state": "Blocked",
                "resolved_type_decl_ref_count": 0,
                "referenced_resource_id_count": 0,
                "edge_ready": False,
                "blocked_by": [
                    "StableTypeDeclarationResourceRegistryMissing",
                    "ReturnTypeResourceTaxonomyAuthorityEntriesMissing",
                ],
            }
        )

    decision = {
        "kind": "KeepStopped",
        "reason_token": "ReturnTypeResourceTaxonomyAuthorityEntriesMissing",
        "selected_domain_subaxis": None,
        "selected_next_card": DESIGN_STOP,
    }

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdReturnTypeResourceTaxonomyAuthorityV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "return_type_resource_taxonomy_inventory": rel(TAXONOMY_INVENTORY),
            "return_type_reference_edge_derivation_basis": rel(DERIVATION_BASIS),
            "typed_dependency_root_authority_basis": rel(ROOT_BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(DOMAIN_INVENTORY),
        },
        "provenance": {
            "return_type_resource_taxonomy_inventory_hash": sha256_file(TAXONOMY_INVENTORY),
            "return_type_reference_edge_derivation_basis_hash": sha256_file(DERIVATION_BASIS),
            "typed_dependency_root_authority_basis_hash": sha256_file(ROOT_BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(DOMAIN_INVENTORY),
        },
        "previous_state": {
            "return_type_reference_count": taxonomy_inventory.get("summary", {}).get(
                "return_type_reference_count"
            ),
            "distinct_return_type_count": taxonomy_inventory.get("summary", {}).get(
                "distinct_return_type_count"
            ),
            "taxonomy_entry_count": taxonomy_inventory.get("summary", {}).get(
                "taxonomy_entry_count"
            ),
            "edge_ready_return_type_count": taxonomy_inventory.get("summary", {}).get(
                "edge_ready_return_type_count"
            ),
            "previous_reason_token": taxonomy_inventory.get("decision", {}).get("reason_token"),
        },
        "authority_rule": {
            "name": "ReturnTypeResourceTaxonomyAuthorityV1",
            "return_type_string_is_not_policy_authority": True,
            "return_type_string_to_subaxis_mapping_forbidden": True,
            "resource_id_required": True,
            "type_decl_ref_required": True,
            "resource_domain_subaxis_must_come_from_taxonomy": True,
            "dependent_domain_subaxis_may_come_from_source_row_classification": True,
            "stable_proof_source_required": True,
            "self_signed_taxonomy_forbidden": True,
            "accepted_typed_dependency_edges_materialized_here": False,
        },
        "required_independent_authority_sources": [
            "StableTypeDeclarationResourceRegistry",
            "StableSemanticResourceRegistry",
            "DurableDeclarationFixtureHash",
        ],
        "resource_taxonomy_rows": [],
        "return_type_reference_rows": [],
        "edge_derivation_readiness_rows": readiness_rows,
        "summary": {
            "return_type_reference_count": taxonomy_inventory.get("summary", {}).get(
                "return_type_reference_count"
            ),
            "distinct_return_type_count": taxonomy_inventory.get("summary", {}).get(
                "distinct_return_type_count"
            ),
            "taxonomy_entry_count": 0,
            "resolved_type_decl_ref_count": 0,
            "resource_taxonomy_join_ready_count": 0,
            "edge_ready_return_type_count": 0,
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": decision,
        "recovery": {
            "reason_tokens": [
                "ReturnTypeResourceTaxonomyAuthorityEntriesMissing",
                "ReturnTypeResourceTaxonomyProofSourceCircular",
                "ReturnTypeStringToSubaxisMappingForbidden",
                "ReturnTypeReferenceResolutionMissingTypeDeclarationRef",
            ],
            "next_if_authority_rows_exist": NEXT_RERUN,
            "selected_next_card": DESIGN_STOP,
        },
        "claims": {
            "return_type_resource_taxonomy_authority_defined": 1,
            "return_type_string_to_subaxis_mapping": 0,
            "return_type_string_as_policy_authority": 0,
            "observed_domain_subaxis_set_as_proof": 0,
            "self_signed_taxonomy": 0,
            "accepted_typed_dependency_edge_materialized": 0,
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
            "shape_signature_as_proof": 0,
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
        print("mirbuilder-domain-object-id-return-type-resource-taxonomy-authority unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
