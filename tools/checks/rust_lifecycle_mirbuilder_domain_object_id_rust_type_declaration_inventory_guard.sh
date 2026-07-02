#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_rust_type_declaration_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2074-MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdRustTypeDeclarationInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("stable_type_resource_registry_authority", "").endswith("mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"), "stable registry input drift")

rule = fixture.get("inventory_rule") or {}
need(rule.get("name") == "RustTypeDeclarationInventoryV1", "rule drift")
for key in [
    "read_only_rust_source_inventory",
    "return_type_string_is_resolution_target_only",
    "source_path_is_locator_only",
    "source_path_is_not_policy_authority",
    "observed_domain_subaxis_set_is_diagnostic_only",
    "declared_resource_domain_subaxis_requires_explicit_semantic_declaration",
]:
    need(rule.get(key) is True, f"rule drift: {key}")
need(rule.get("return_type_string_to_subaxis_mapping") is False, "return type mapping drift")
need(rule.get("manual_type_to_subaxis_assignment") is False, "manual assignment drift")
need(rule.get("accepted_typed_dependency_edges_materialized_here") is False, "edge materialization drift")

summary = fixture.get("summary") or {}
need(summary.get("return_type_reference_count") == 85, "return type reference count drift")
need(summary.get("distinct_return_type_count") == 44, "distinct return type count drift")
need(summary.get("resolved_type_decl_ref_count", 0) > 0, "no resolved type declarations")
need(
    summary.get("resolved_type_decl_ref_count", 0)
    + summary.get("ambiguous_type_decl_ref_count", 0)
    + summary.get("unresolved_type_decl_ref_count", 0)
    == 44,
    "type declaration resolution partition drift",
)
need(summary.get("declared_resource_domain_subaxis_ready_count") == 0, "resource subaxis authority must remain missing")
need(summary.get("registry_ready_row_count") == 0, "registry rows must not be ready")
need(summary.get("type_identity_only_row_count") == summary.get("resolved_type_decl_ref_count"), "type identity count drift")
need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

rows = fixture.get("rust_type_declaration_inventory_rows") or []
need(len(rows) == 44, "inventory row count drift")
for row in rows:
    need(row.get("observed_domain_subaxis_set_is_diagnostic_only") is True, "observed subaxis diagnostic drift")
    need(row.get("observed_owner_edge_set_is_diagnostic_only") is True, "observed owner diagnostic drift")
    need(row.get("declared_resource_domain_subaxis") is None, "resource subaxis must not be assigned")
    need(row.get("declared_resource_domain_subaxis_authority") is None, "resource subaxis authority must be absent")
    claims = row.get("claims") or {}
    for key in [
        "return_type_string_as_policy_authority",
        "source_path_as_policy_authority",
        "observed_subaxis_set_as_policy_authority",
        "manual_type_to_subaxis_assignment",
    ]:
        need(claims.get(key) == 0, f"row forbidden claim drift: {key}")
    if row.get("registry_row_state") == "TypeIdentityOnly":
        need(row.get("type_decl_ref"), "type identity missing type_decl_ref")
        need(row.get("semantic_resource_id"), "type identity missing resource id")
        source = row.get("declaration_source") or {}
        need(source.get("kind") == "RustTypeDeclarationInventory", "declaration source kind drift")
        need(source.get("source_decl_hash"), "missing source decl hash")
        need(source.get("source_file_hash"), "missing source file hash")
        need("ExplicitResourceDomainDeclarationMissing" in (row.get("blocked_by") or []), "missing explicit domain blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectStableTypeResourceRegistryAuthorityRerun", "decision kind drift")
need(decision.get("reason_token") == "RustTypeDeclarationInventoryMaterializedTypeIdentityOnly", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("rust_type_declaration_inventory") == 1, "inventory claim drift")
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "manual_subaxis_selection",
    "manual_type_to_subaxis_assignment",
    "return_type_string_to_subaxis_mapping",
    "return_type_string_as_policy_authority",
    "source_path_as_policy_authority",
    "observed_domain_subaxis_set_as_policy_authority",
    "owner_name_as_proof",
    "shape_signature_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "hardcoded_subaxis_priority",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-rust-type-declaration-inventory")
print("resolved_type_decl_ref_count=" + str(summary.get("resolved_type_decl_ref_count")))
print("declared_resource_domain_subaxis_ready_count=0")
print("decision=SelectStableTypeResourceRegistryAuthorityRerun")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
