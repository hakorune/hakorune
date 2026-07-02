#!/usr/bin/env python3
"""Define explicit semantic resource-domain declaration authority for DomainObject/Id."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001"

REGISTRY_RERUN = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"
TYPE_DECL_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json"
INITIAL_AUTHORITY = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    registry_rerun = read_json(REGISTRY_RERUN)
    inventory = read_json(TYPE_DECL_INVENTORY)
    registry_rows = registry_rerun.get("type_resource_registry_rows") or []

    candidate_rows: list[dict[str, Any]] = []
    for row in registry_rows:
        candidate_rows.append(
            {
                "type_decl_ref": row.get("type_decl_ref"),
                "semantic_resource_id": row.get("semantic_resource_id"),
                "current_registry_row_state": row.get("registry_row_state"),
                "resource_domain_declaration_state": "Missing",
                "declared_resource_domain_subaxis": None,
                "declared_resource_domain_subaxis_authority": None,
                "blocked_by": [
                    "ExplicitResourceDomainDeclarationMissing",
                    "TypeIdentityOnlyIsNotResourceDomainAuthority",
                ],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdExplicitSemanticResourceDomainDeclarationBasisV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "stable_type_resource_registry_authority_rerun": rel(REGISTRY_RERUN),
            "rust_type_declaration_inventory": rel(TYPE_DECL_INVENTORY),
            "stable_type_resource_registry_authority": rel(INITIAL_AUTHORITY),
        },
        "provenance": {
            "stable_type_resource_registry_authority_rerun_hash": sha256_file(REGISTRY_RERUN),
            "rust_type_declaration_inventory_hash": sha256_file(TYPE_DECL_INVENTORY),
            "stable_type_resource_registry_authority_hash": sha256_file(INITIAL_AUTHORITY),
        },
        "previous_state": {
            "previous_reason_token": registry_rerun.get("decision", {}).get("reason_token"),
            "type_identity_only_row_count": registry_rerun.get("summary", {}).get(
                "type_identity_only_row_count"
            ),
            "declared_resource_domain_subaxis_ready_count": registry_rerun.get(
                "summary", {}
            ).get("declared_resource_domain_subaxis_ready_count"),
            "registry_ready_row_count": registry_rerun.get("summary", {}).get(
                "registry_ready_row_count"
            ),
            "resolved_type_decl_ref_count": inventory.get("summary", {}).get(
                "resolved_type_decl_ref_count"
            ),
            "ambiguous_type_decl_ref_count": inventory.get("summary", {}).get(
                "ambiguous_type_decl_ref_count"
            ),
            "unresolved_type_decl_ref_count": inventory.get("summary", {}).get(
                "unresolved_type_decl_ref_count"
            ),
        },
        "authority_rule": {
            "name": "ExplicitSemanticResourceDomainDeclarationBasisV1",
            "type_identity_only_is_not_resource_domain_authority": True,
            "declared_resource_domain_subaxis_requires_explicit_semantic_declaration": True,
            "semantic_resource_id_required": True,
            "type_decl_ref_required": True,
            "proof_source_hash_required": True,
            "self_signed_declaration_forbidden": True,
            "manual_type_to_subaxis_assignment": False,
            "return_type_string_to_subaxis_mapping": False,
            "source_path_as_policy_authority": False,
            "observed_subaxis_set_as_policy_authority": False,
            "owner_name_as_proof": False,
            "shape_signature_as_proof": False,
            "row_count_as_proof": False,
            "route_membership_alone_as_proof": False,
        },
        "allowed_authority_sources": [
            {
                "source_kind": "ExistingSemanticResourceDeclarationFixture",
                "allowed": True,
                "required_fields": [
                    "semantic_resource_id",
                    "type_decl_ref",
                    "declared_resource_domain_subaxis",
                    "resource_domain_declaration_ref",
                    "proof_source_hash",
                ],
            },
            {
                "source_kind": "ProjectionDescriptorLedgerExplicitResourceDeclaration",
                "allowed": True,
                "allowed_only_if_explicitly_declares": [
                    "semantic_resource_id",
                    "declared_resource_domain_subaxis",
                    "proof_source_hash",
                ],
                "forbidden_if_only": [
                    "descriptor_coverage",
                    "route_membership",
                    "owner_name",
                    "source_path",
                ],
            },
            {
                "source_kind": "NewReadOnlySemanticResourceDeclarationInventory",
                "allowed": True,
                "allowed_only_if": [
                    "reads_existing_explicit_semantic_declarations",
                    "does_not_infer_from_type_name",
                    "does_not_infer_from_source_path",
                    "does_not_infer_from_observed_subaxis_set",
                ],
            },
        ],
        "resource_domain_declaration_requirements": {
            "semantic_resource_id_must_join_registry_row": True,
            "type_decl_ref_must_join_registry_row": True,
            "declared_resource_domain_subaxis_must_be_present": True,
            "declared_resource_domain_subaxis_must_be_from_allowed_set": True,
            "dependency_role_must_be_declared": True,
            "proof_source_must_be_stable": True,
            "proof_source_hash_must_be_recorded": True,
        },
        "candidate_registry_rows": candidate_rows,
        "summary": {
            "candidate_registry_row_count": len(candidate_rows),
            "type_identity_only_row_count": registry_rerun.get("summary", {}).get(
                "type_identity_only_row_count"
            ),
            "resource_domain_declaration_ready_count": 0,
            "registry_ready_row_count": 0,
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": {
            "kind": "SelectSemanticResourceDomainDeclarationInventory",
            "reason_token": "ExplicitSemanticResourceDomainDeclarationBasisDefined",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "guard": {
            "reusable_lane_guard": "tools/checks/rust_lifecycle_mirbuilder_domain_object_id_lane_guard.sh",
            "profile": "explicit_semantic_resource_domain_declaration_basis",
            "row_specific_guard_added": False,
        },
        "claims": {
            "explicit_semantic_resource_domain_declaration_basis_defined": 1,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "manual_subaxis_selection": 0,
            "manual_type_to_subaxis_assignment": 0,
            "return_type_string_to_subaxis_mapping": 0,
            "source_path_as_policy_authority": 0,
            "observed_subaxis_set_as_policy_authority": 0,
            "row_count_as_proof": 0,
            "owner_name_as_proof": 0,
            "shape_signature_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "self_signed_taxonomy": 0,
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
        print("mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
