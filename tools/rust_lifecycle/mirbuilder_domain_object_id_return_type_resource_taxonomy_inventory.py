#!/usr/bin/env python3
"""Inventory return_type resource taxonomy availability for DomainObject/Id."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

BASIS = FIXTURES / "mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json"
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
    basis = read_json(BASIS)
    domain_inventory = read_json(DOMAIN_INVENTORY)
    rows = unresolved_rows(domain_inventory)

    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row.get("return_type"):
            grouped[str(row["return_type"])].append(row)

    taxonomy_rows: list[dict[str, Any]] = []
    for return_type, type_rows in sorted(grouped.items()):
        taxonomy_rows.append(
            {
                "return_type": return_type,
                "reference_count": len(type_rows),
                "observed_domain_subaxis_set": sorted(
                    {row["domain_subaxis"] for row in type_rows}
                ),
                "observed_owner_edge_set": sorted(
                    {str(row.get("known_owner_edge")) for row in type_rows}
                ),
                "taxonomy_entry_state": "MissingTaxonomyEntry",
                "resource_domain_subaxis_declared": False,
                "dependency_role_declared": False,
                "edge_ready": False,
                "blocked_by": [
                    "ReturnTypeResourceTaxonomyEntryMissing",
                    "ObservedSubaxisSetIsDiagnosticOnly",
                    "ReturnTypeNameIsNotPolicyAuthority",
                ],
            }
        )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdReturnTypeResourceTaxonomyInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "return_type_reference_edge_derivation_basis": rel(BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(DOMAIN_INVENTORY),
        },
        "provenance": {
            "return_type_reference_edge_derivation_basis_hash": sha256_file(BASIS),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(DOMAIN_INVENTORY),
        },
        "previous_state": {
            "return_type_reference_count": basis.get("return_type_inventory", {}).get(
                "return_type_reference_count"
            ),
            "distinct_return_type_count": basis.get("return_type_inventory", {}).get(
                "distinct_return_type_count"
            ),
            "edge_ready_return_type_count": basis.get("return_type_inventory", {}).get(
                "edge_ready_return_type_count"
            ),
            "previous_next_card": basis.get("decision", {}).get("selected_next_card"),
        },
        "taxonomy_policy": {
            "observed_subaxis_set_is_diagnostic_only": True,
            "return_type_name_is_not_policy_authority": True,
            "owner_edge_is_not_policy_authority": True,
            "row_count_is_not_policy_authority": True,
            "taxonomy_entries_must_be_typed_fixture_rows": True,
        },
        "return_type_taxonomy_rows": taxonomy_rows,
        "summary": {
            "return_type_reference_count": len(rows),
            "distinct_return_type_count": len(taxonomy_rows),
            "taxonomy_entry_count": 0,
            "missing_taxonomy_entry_count": len(taxonomy_rows),
            "edge_ready_return_type_count": 0,
            "accepted_edge_candidate_count": 0,
        },
        "decision": {
            "kind": "KeepStopped",
            "reason_token": "ReturnTypeResourceTaxonomyEntriesMissing",
            "selected_domain_subaxis": None,
            "selected_next_card": DESIGN_STOP,
        },
        "recovery": {
            "next_consultation_topic": "Define typed fixture authority for return_type resource taxonomy rows.",
            "do_not": [
                "map return_type names to subaxes by string",
                "use observed subaxis set as policy proof",
                "select a non-ID DomainObject/Id subaxis",
            ],
        },
        "claims": {
            "return_type_resource_taxonomy_inventory": 1,
            "return_type_name_as_policy_authority": 0,
            "observed_subaxis_set_as_policy_proof": 0,
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
        print("mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
