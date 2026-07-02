#!/usr/bin/env python3
"""Inventory explicit semantic resource-domain declarations for DomainObject/Id."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008"

BASIS = FIXTURES / "mirbuilder-domain-object-id-explicit-semantic-resource-domain-declaration-basis-v0.json"
REGISTRY_RERUN = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def walk_values(value: Any):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from walk_values(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_values(child)


def fixture_files() -> list[Path]:
    return sorted(FIXTURES.glob("*.json"))


def explicit_declaration_sources() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    required = {
        "semantic_resource_id",
        "type_decl_ref",
        "declared_resource_domain_subaxis",
        "resource_domain_declaration_ref",
        "proof_source_hash",
    }
    excluded = {OUTPUT.name, BASIS.name}
    for path in fixture_files():
        if path.name in excluded:
            continue
        data = read_json(path)
        for item in walk_values(data):
            if required.issubset(item.keys()) and all(item.get(key) for key in required):
                rows.append(
                    {
                        "source_path": rel(path),
                        "source_hash": sha256_file(path),
                        "semantic_resource_id": item["semantic_resource_id"],
                        "type_decl_ref": item["type_decl_ref"],
                        "declared_resource_domain_subaxis": item[
                            "declared_resource_domain_subaxis"
                        ],
                        "resource_domain_declaration_ref": item[
                            "resource_domain_declaration_ref"
                        ],
                        "proof_source_hash": item["proof_source_hash"],
                    }
                )
    return rows


def closed_resource_manifest_rows() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in fixture_files():
        if path == OUTPUT:
            continue
        data = read_json(path)
        for item in walk_values(data):
            if item.get("closed_resource_id") and item.get("closed_lane_status") == "Closed":
                rows.append(
                    {
                        "source_path": rel(path),
                        "source_hash": sha256_file(path),
                        "closed_resource_id": item.get("closed_resource_id"),
                        "closed_lane_status": item.get("closed_lane_status"),
                    }
                )
    return rows


def build_fixture() -> dict[str, Any]:
    basis = read_json(BASIS)
    registry_rerun = read_json(REGISTRY_RERUN)
    candidate_rows = basis.get("candidate_registry_rows") or []
    declaration_sources = explicit_declaration_sources()
    closed_rows = closed_resource_manifest_rows()

    inventory_rows = []
    for row in candidate_rows:
        inventory_rows.append(
            {
                "type_decl_ref": row.get("type_decl_ref"),
                "semantic_resource_id": row.get("semantic_resource_id"),
                "resource_domain_declaration_state": "Missing",
                "declared_resource_domain_subaxis": None,
                "declared_resource_domain_subaxis_authority": None,
                "blocked_by": [
                    "ExplicitSemanticResourceDomainDeclarationSourceMissing",
                    "TypeIdentityOnlyIsNotResourceDomainAuthority",
                ],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdSemanticResourceDomainDeclarationInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "explicit_semantic_resource_domain_declaration_basis": rel(BASIS),
            "stable_type_resource_registry_authority_rerun": rel(REGISTRY_RERUN),
        },
        "provenance": {
            "explicit_semantic_resource_domain_declaration_basis_hash": sha256_file(BASIS),
            "stable_type_resource_registry_authority_rerun_hash": sha256_file(REGISTRY_RERUN),
        },
        "previous_state": {
            "basis_decision": basis.get("decision", {}).get("kind"),
            "basis_reason_token": basis.get("decision", {}).get("reason_token"),
            "type_identity_only_row_count": registry_rerun.get("summary", {}).get(
                "type_identity_only_row_count"
            ),
            "registry_ready_row_count": registry_rerun.get("summary", {}).get(
                "registry_ready_row_count"
            ),
        },
        "inventory_rule": {
            "name": "SemanticResourceDomainDeclarationInventoryV1",
            "reads_existing_explicit_semantic_declarations_only": True,
            "self_signed_declaration_forbidden": True,
            "manual_type_to_subaxis_assignment": False,
            "return_type_string_to_subaxis_mapping": False,
            "type_name_or_source_path_inference": False,
            "observed_subaxis_set_inference": False,
            "source_path_as_policy_authority": False,
        },
        "authority_source_inventory": {
            "explicit_declaration_sources": declaration_sources,
            "stable_closed_resource_manifest_rows": closed_rows,
        },
        "resource_domain_declaration_inventory_rows": inventory_rows,
        "summary": {
            "candidate_registry_row_count": len(candidate_rows),
            "explicit_semantic_resource_domain_declaration_source_count": len(
                declaration_sources
            ),
            "resource_domain_declaration_ready_count": 0,
            "stable_closed_resource_manifest_count": len(closed_rows),
            "registry_ready_row_count": 0,
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": {
            "kind": "SelectWiderRouteSelectionBasis",
            "reason_token": "ExplicitSemanticResourceDomainDeclarationSourceMissing",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "semantic_resource_domain_declaration_inventory": 1,
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
        print("mirbuilder-domain-object-id-semantic-resource-domain-declaration-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
