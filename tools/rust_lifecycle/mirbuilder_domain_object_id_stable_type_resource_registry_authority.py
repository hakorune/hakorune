#!/usr/bin/env python3
"""Select a non-self-signed stable type/resource registry authority path."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001"

TAXONOMY_AUTHORITY = FIXTURES / "mirbuilder-domain-object-id-return-type-resource-taxonomy-authority-v0.json"
DOMAIN_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    taxonomy_authority = read_json(TAXONOMY_AUTHORITY)

    authority_source_candidates = [
        {
            "source_kind": "ExistingRustTypeDeclarationInventory",
            "candidate_state": "Missing",
            "source_path": None,
            "source_hash": None,
            "accepted_for": [],
            "rejected_for": [
                "type_decl_ref",
                "declaration_hash",
                "declared_resource_domain_subaxis",
            ],
            "rejected_reason": "RustTypeDeclarationInventoryMissing",
        },
        {
            "source_kind": "ExistingProjectionDescriptorLedger",
            "candidate_state": "Rejected",
            "source_path": None,
            "source_hash": None,
            "accepted_for": [],
            "rejected_for": ["type_resource_registry_authority"],
            "rejected_reason": "ProjectionDescriptorLedgerNotSemanticResourceAuthority",
        },
        {
            "source_kind": "SourceSurfaceInventory",
            "candidate_state": "Rejected",
            "source_path": None,
            "source_hash": None,
            "accepted_for": ["return_type_reference_anchor"],
            "rejected_for": ["type_resource_registry_authority"],
            "rejected_reason": "SourceSurfaceInventoryNotRegistryAuthority",
        },
        {
            "source_kind": "NewReadOnlyRustTypeDeclarationInventory",
            "candidate_state": "SelectedIfNoExistingRegistry",
            "source_path": None,
            "source_hash": None,
            "accepted_for": ["type_decl_ref", "declaration_hash"],
            "rejected_for": ["declared_resource_domain_subaxis"],
            "rejected_reason": "ResourceDomainDeclarationStillRequiresExplicitSemanticDeclaration",
            "selected_next_card": NEXT_CARD,
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdStableTypeResourceRegistryAuthorityV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "return_type_resource_taxonomy_authority": rel(TAXONOMY_AUTHORITY),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(DOMAIN_INVENTORY),
        },
        "provenance": {
            "return_type_resource_taxonomy_authority_hash": sha256_file(TAXONOMY_AUTHORITY),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(DOMAIN_INVENTORY),
        },
        "previous_state": {
            "taxonomy_entry_count": taxonomy_authority.get("summary", {}).get("taxonomy_entry_count"),
            "resolved_type_decl_ref_count": taxonomy_authority.get("summary", {}).get(
                "resolved_type_decl_ref_count"
            ),
            "resource_taxonomy_join_ready_count": taxonomy_authority.get("summary", {}).get(
                "resource_taxonomy_join_ready_count"
            ),
            "previous_reason_token": taxonomy_authority.get("decision", {}).get("reason_token"),
        },
        "authority_rule": {
            "name": "StableTypeResourceRegistryAuthorityV1",
            "registry_is_independent_proof_source": True,
            "registry_must_not_be_self_signed": True,
            "type_decl_ref_required": True,
            "semantic_resource_id_required": True,
            "declared_resource_domain_subaxis_requires_explicit_semantic_declaration": True,
            "rust_source_declaration_inventory_allowed": True,
            "projection_descriptor_ledger_allowed_only_if_explicit_resource_declaration": True,
            "source_surface_inventory_not_registry_authority": True,
            "return_type_string_is_diagnostic_only": True,
            "return_type_string_to_subaxis_mapping": False,
            "source_path_is_not_policy_authority": True,
            "owner_name_is_not_policy_authority": True,
            "shape_signature_is_not_policy_authority": True,
            "observed_subaxis_set_is_not_policy_authority": True,
            "manual_type_to_subaxis_assignment": False,
        },
        "authority_source_candidates": authority_source_candidates,
        "type_resource_registry_rows": [],
        "summary": {
            "authority_source_candidate_count": len(authority_source_candidates),
            "accepted_registry_authority_source_count": 0,
            "type_decl_ref_ready_count": 0,
            "semantic_resource_id_ready_count": 0,
            "declared_resource_domain_subaxis_ready_count": 0,
            "registry_ready_row_count": 0,
            "type_identity_only_row_count": 0,
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": {
            "kind": "SelectRustTypeDeclarationInventory",
            "reason_token": "StableTypeResourceRegistryAuthorityRequiresReadOnlyTypeDeclarationInventory",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "stable_type_resource_registry_authority_defined": 1,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "manual_subaxis_selection": 0,
            "manual_type_to_subaxis_assignment": 0,
            "return_type_string_to_subaxis_mapping": 0,
            "self_signed_taxonomy": 0,
            "source_path_as_policy_authority": 0,
            "observed_subaxis_set_as_policy_authority": 0,
            "owner_name_as_proof": 0,
            "shape_signature_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "hardcoded_subaxis_priority": 0,
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
        print("mirbuilder-domain-object-id-stable-type-resource-registry-authority unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
