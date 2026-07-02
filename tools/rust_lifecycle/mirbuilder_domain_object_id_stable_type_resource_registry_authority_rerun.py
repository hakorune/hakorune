#!/usr/bin/env python3
"""Rerun stable type/resource registry authority after Rust declaration inventory."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

INITIAL_AUTHORITY = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"
TYPE_DECL_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_fixture() -> dict[str, Any]:
    initial = read_json(INITIAL_AUTHORITY)
    inventory = read_json(TYPE_DECL_INVENTORY)
    type_rows = inventory.get("rust_type_declaration_inventory_rows") or []
    identity_rows = [
        row for row in type_rows if row.get("registry_row_state") == "TypeIdentityOnly"
    ]

    registry_rows: list[dict[str, Any]] = []
    for row in identity_rows:
        registry_rows.append(
            {
                "registry_row_state": "TypeIdentityOnly",
                "diagnostic_return_type": row.get("diagnostic_return_type"),
                "normalized_type_name": row.get("normalized_type_name"),
                "type_decl_ref": row.get("type_decl_ref"),
                "type_decl_kind": "RustTypeDeclaration",
                "semantic_resource_id": row.get("semantic_resource_id"),
                "resource_kind": row.get("resource_kind"),
                "declared_resource_domain_subaxis": None,
                "declared_resource_domain_subaxis_authority": None,
                "dependency_role": None,
                "declaration_source": row.get("declaration_source"),
                "diagnostic_return_type_tokens": [row.get("diagnostic_return_type")],
                "blocked_by": [
                    "ExplicitResourceDomainDeclarationMissing",
                    "TypeIdentityOnlyIsNotResourceDomainAuthority",
                ],
                "claims": {
                    "return_type_string_as_policy_authority": 0,
                    "source_path_as_policy_authority": 0,
                    "observed_subaxis_set_as_policy_authority": 0,
                    "manual_type_to_subaxis_assignment": 0,
                },
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdStableTypeResourceRegistryAuthorityRerunV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "stable_type_resource_registry_authority": rel(INITIAL_AUTHORITY),
            "rust_type_declaration_inventory": rel(TYPE_DECL_INVENTORY),
        },
        "provenance": {
            "stable_type_resource_registry_authority_hash": sha256_file(INITIAL_AUTHORITY),
            "rust_type_declaration_inventory_hash": sha256_file(TYPE_DECL_INVENTORY),
        },
        "previous_state": {
            "previous_reason_token": initial.get("decision", {}).get("reason_token"),
            "inventory_decision": inventory.get("decision", {}).get("kind"),
            "inventory_reason_token": inventory.get("decision", {}).get("reason_token"),
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
            "name": "StableTypeResourceRegistryAuthorityV1",
            "registry_is_independent_proof_source": True,
            "registry_must_not_be_self_signed": True,
            "type_decl_ref_required": True,
            "semantic_resource_id_required": True,
            "type_identity_only_is_not_resource_domain_authority": True,
            "declared_resource_domain_subaxis_requires_explicit_semantic_declaration": True,
            "return_type_string_is_diagnostic_only": True,
            "return_type_string_to_subaxis_mapping": False,
            "source_path_is_not_policy_authority": True,
            "owner_name_is_not_policy_authority": True,
            "shape_signature_is_not_policy_authority": True,
            "observed_subaxis_set_is_not_policy_authority": True,
            "manual_type_to_subaxis_assignment": False,
        },
        "type_resource_registry_rows": registry_rows,
        "summary": {
            "type_decl_ref_ready_count": len(identity_rows),
            "semantic_resource_id_ready_count": len(identity_rows),
            "declared_resource_domain_subaxis_ready_count": 0,
            "registry_ready_row_count": 0,
            "type_identity_only_row_count": len(identity_rows),
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "StableTypeResourceRegistryHasTypeIdentityOnlyNoResourceDomainAuthority",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        },
        "recovery": {
            "next_consultation_topic": "Decide whether to define an explicit semantic resource-domain declaration basis, try prior closed-lane continuation with stable resource IDs, or return to wider route selector.",
            "do_not": [
                "map type_decl_ref or return_type names to domain subaxes",
                "use source path as policy authority",
                "select a non-ID DomainObject/Id subaxis",
            ],
        },
        "claims": {
            "stable_type_resource_registry_authority_rerun": 1,
            "source_selfhost_claim": 0,
            "native_seed_materialization": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "accepted_typed_dependency_edge_materialized": 0,
            "manual_subaxis_selection": 0,
            "manual_type_to_subaxis_assignment": 0,
            "return_type_string_to_subaxis_mapping": 0,
            "return_type_string_as_policy_authority": 0,
            "source_path_as_policy_authority": 0,
            "observed_domain_subaxis_set_as_policy_authority": 0,
            "owner_name_as_proof": 0,
            "shape_signature_as_proof": 0,
            "row_count_as_proof": 0,
            "coverage_percentage_as_proof": 0,
            "route_membership_alone_as_proof": 0,
            "hardcoded_subaxis_priority": 0,
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
        print("mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
