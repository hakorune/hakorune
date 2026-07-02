#!/usr/bin/env python3
"""Inventory Rust type declarations referenced by non-ID DomainObject/Id return types."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from collections import defaultdict
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json"

TOKEN = "MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001"
DESIGN_STOP = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT_CARD = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001"

DOMAIN_INVENTORY = FIXTURES / "mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
STABLE_REGISTRY_AUTHORITY = FIXTURES / "mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"

DECL_RE = re.compile(
    r"^\s*(?:(?:pub|pub\s*\([^)]*\))\s+)?"
    r"(?P<kind>struct|enum|type)\s+"
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\b"
)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def rust_source_files() -> list[Path]:
    roots = [ROOT / "src", ROOT / "crates"]
    files: list[Path] = []
    for root in roots:
        if not root.exists():
            continue
        files.extend(path for path in root.rglob("*.rs") if path.is_file())
    return sorted(files)


def normalize_return_type(return_type: str) -> str:
    value = return_type.strip()
    if "<" in value:
        value = value.split("<", 1)[0]
    if "::" in value:
        value = value.rsplit("::", 1)[-1]
    return value.strip()


def collect_declarations() -> dict[str, list[dict[str, Any]]]:
    declarations: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for path in rust_source_files():
        lines = path.read_text(encoding="utf-8").splitlines()
        file_hash = sha256_file(path)
        for line_number, line in enumerate(lines, start=1):
            match = DECL_RE.match(line)
            if not match:
                continue
            name = match.group("name")
            declaration_line = line.strip()
            type_decl_ref = f"type-decl:{rel(path)}::{name}:L{line_number}"
            declarations[name].append(
                {
                    "type_decl_ref": type_decl_ref,
                    "type_decl_kind": "RustTypeDeclaration",
                    "rust_decl_kind": match.group("kind"),
                    "declared_type_name": name,
                    "source_file": rel(path),
                    "source_line": line_number,
                    "source_decl_span": "locator-only",
                    "source_decl_hash": sha256_text(declaration_line),
                    "source_file_hash": file_hash,
                    "declaration_line_preview": declaration_line,
                }
            )
    return declarations


def unresolved_rows(inventory: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        row
        for row in inventory.get("domain_object_id_source_id_ledger") or []
        if row.get("scope_state") == "UnresolvedNonIdDomainObject"
        and row.get("return_type")
    ]


def row_state(candidates: list[dict[str, Any]]) -> str:
    if len(candidates) == 1:
        return "TypeIdentityOnly"
    if len(candidates) > 1:
        return "AmbiguousRustTypeDeclaration"
    return "RustTypeDeclarationMissing"


def build_fixture() -> dict[str, Any]:
    domain_inventory = read_json(DOMAIN_INVENTORY)
    stable_registry_authority = read_json(STABLE_REGISTRY_AUTHORITY)
    rows = unresolved_rows(domain_inventory)
    declarations = collect_declarations()

    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        grouped[str(row["return_type"])].append(row)

    inventory_rows: list[dict[str, Any]] = []
    for return_type, type_rows in sorted(grouped.items()):
        normalized = normalize_return_type(return_type)
        candidates = declarations.get(normalized, [])
        state = row_state(candidates)
        resolved = candidates[0] if len(candidates) == 1 else None
        type_decl_ref = resolved.get("type_decl_ref") if resolved else None
        semantic_resource_id = (
            f"semantic-resource:{type_decl_ref}" if type_decl_ref is not None else None
        )
        blocked_by = []
        if state == "TypeIdentityOnly":
            blocked_by.append("ExplicitResourceDomainDeclarationMissing")
        elif state == "AmbiguousRustTypeDeclaration":
            blocked_by.append("AmbiguousRustTypeDeclaration")
        else:
            blocked_by.append("RustTypeDeclarationMissing")

        inventory_rows.append(
            {
                "diagnostic_return_type": return_type,
                "normalized_type_name": normalized,
                "reference_count": len(type_rows),
                "observed_domain_subaxis_set": sorted(
                    {row["domain_subaxis"] for row in type_rows}
                ),
                "observed_domain_subaxis_set_is_diagnostic_only": True,
                "observed_owner_edge_set": sorted(
                    {str(row.get("known_owner_edge")) for row in type_rows}
                ),
                "observed_owner_edge_set_is_diagnostic_only": True,
                "registry_row_state": state,
                "type_decl_ref": type_decl_ref,
                "semantic_resource_id": semantic_resource_id,
                "resource_kind": "DomainObjectResource" if resolved else None,
                "declared_resource_domain_subaxis": None,
                "declared_resource_domain_subaxis_authority": None,
                "declaration_source": (
                    {
                        "kind": "RustTypeDeclarationInventory",
                        "inventory": rel(OUTPUT),
                        "inventory_hash": None,
                        "source_decl_hash": resolved["source_decl_hash"],
                        "source_file_hash": resolved["source_file_hash"],
                        "source_decl_span": resolved["source_decl_span"],
                    }
                    if resolved
                    else None
                ),
                "declaration_candidates": candidates,
                "blocked_by": blocked_by,
                "claims": {
                    "return_type_string_as_policy_authority": 0,
                    "source_path_as_policy_authority": 0,
                    "observed_subaxis_set_as_policy_authority": 0,
                    "manual_type_to_subaxis_assignment": 0,
                },
            }
        )

    resolved_count = sum(
        1 for row in inventory_rows if row["registry_row_state"] == "TypeIdentityOnly"
    )
    ambiguous_count = sum(
        1
        for row in inventory_rows
        if row["registry_row_state"] == "AmbiguousRustTypeDeclaration"
    )
    missing_count = sum(
        1 for row in inventory_rows if row["registry_row_state"] == "RustTypeDeclarationMissing"
    )

    return {
        "schema_version": 0,
        "kind": "MirBuilderDomainObjectIdRustTypeDeclarationInventoryV1",
        "token": TOKEN,
        "input_state": {
            "current_blocker": DESIGN_STOP,
            "stable_type_resource_registry_authority": rel(STABLE_REGISTRY_AUTHORITY),
            "domain_object_id_transport_policy_inventory_rerun_002": rel(DOMAIN_INVENTORY),
        },
        "provenance": {
            "stable_type_resource_registry_authority_hash": sha256_file(
                STABLE_REGISTRY_AUTHORITY
            ),
            "domain_object_id_transport_policy_inventory_rerun_002_hash": sha256_file(
                DOMAIN_INVENTORY
            ),
            "rust_source_file_count": len(rust_source_files()),
        },
        "previous_state": {
            "previous_reason_token": stable_registry_authority.get("decision", {}).get(
                "reason_token"
            ),
            "selected_next_card": stable_registry_authority.get("decision", {}).get(
                "selected_next_card"
            ),
        },
        "inventory_rule": {
            "name": "RustTypeDeclarationInventoryV1",
            "read_only_rust_source_inventory": True,
            "return_type_string_is_resolution_target_only": True,
            "return_type_string_to_subaxis_mapping": False,
            "source_path_is_locator_only": True,
            "source_path_is_not_policy_authority": True,
            "observed_domain_subaxis_set_is_diagnostic_only": True,
            "manual_type_to_subaxis_assignment": False,
            "declared_resource_domain_subaxis_requires_explicit_semantic_declaration": True,
            "accepted_typed_dependency_edges_materialized_here": False,
        },
        "rust_type_declaration_inventory_rows": inventory_rows,
        "summary": {
            "return_type_reference_count": len(rows),
            "distinct_return_type_count": len(inventory_rows),
            "resolved_type_decl_ref_count": resolved_count,
            "ambiguous_type_decl_ref_count": ambiguous_count,
            "unresolved_type_decl_ref_count": missing_count,
            "type_decl_ref_ready_count": resolved_count,
            "semantic_resource_id_ready_count": resolved_count,
            "declared_resource_domain_subaxis_ready_count": 0,
            "registry_ready_row_count": 0,
            "type_identity_only_row_count": resolved_count,
            "accepted_typed_dependency_edge_count": 0,
        },
        "decision": {
            "kind": "SelectStableTypeResourceRegistryAuthorityRerun"
            if resolved_count > 0
            else "KeepStopped",
            "reason_token": "RustTypeDeclarationInventoryMaterializedTypeIdentityOnly"
            if resolved_count > 0
            else "RustTypeDeclarationInventoryMissing",
            "selected_domain_subaxis": None,
            "selected_next_card": NEXT_CARD if resolved_count > 0 else DESIGN_STOP,
        },
        "claims": {
            "rust_type_declaration_inventory": 1,
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
        print("mirbuilder-domain-object-id-rust-type-declaration-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
