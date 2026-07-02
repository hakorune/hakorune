#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-stable-type-resource-registry-authority-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_stable_type_resource_registry_authority.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2073-MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-001"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-RUST-TYPE-DECLARATION-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdStableTypeResourceRegistryAuthorityV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("return_type_resource_taxonomy_authority", "").endswith("mirbuilder-domain-object-id-return-type-resource-taxonomy-authority-v0.json"), "taxonomy authority input drift")

rule = fixture.get("authority_rule") or {}
need(rule.get("name") == "StableTypeResourceRegistryAuthorityV1", "rule drift")
for key in [
    "registry_is_independent_proof_source",
    "registry_must_not_be_self_signed",
    "type_decl_ref_required",
    "semantic_resource_id_required",
    "declared_resource_domain_subaxis_requires_explicit_semantic_declaration",
    "rust_source_declaration_inventory_allowed",
    "source_surface_inventory_not_registry_authority",
    "return_type_string_is_diagnostic_only",
    "source_path_is_not_policy_authority",
    "owner_name_is_not_policy_authority",
    "shape_signature_is_not_policy_authority",
    "observed_subaxis_set_is_not_policy_authority",
]:
    need(rule.get(key) is True, f"rule drift: {key}")
need(rule.get("return_type_string_to_subaxis_mapping") is False, "return type mapping drift")
need(rule.get("manual_type_to_subaxis_assignment") is False, "manual assignment drift")

summary = fixture.get("summary") or {}
need(summary.get("authority_source_candidate_count") == 4, "candidate count drift")
need(summary.get("accepted_registry_authority_source_count") == 0, "accepted source count drift")
need(summary.get("type_decl_ref_ready_count") == 0, "type decl ready drift")
need(summary.get("semantic_resource_id_ready_count") == 0, "resource id ready drift")
need(summary.get("declared_resource_domain_subaxis_ready_count") == 0, "subaxis ready drift")
need(summary.get("registry_ready_row_count") == 0, "registry ready drift")
need(summary.get("type_identity_only_row_count") == 0, "type identity drift")
need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

candidates = {row.get("source_kind"): row for row in fixture.get("authority_source_candidates") or []}
need(candidates.get("ExistingRustTypeDeclarationInventory", {}).get("candidate_state") == "Missing", "existing rust inventory drift")
need(candidates.get("ExistingProjectionDescriptorLedger", {}).get("candidate_state") == "Rejected", "projection ledger drift")
need(candidates.get("SourceSurfaceInventory", {}).get("candidate_state") == "Rejected", "source surface drift")
need(candidates.get("NewReadOnlyRustTypeDeclarationInventory", {}).get("candidate_state") == "SelectedIfNoExistingRegistry", "new inventory drift")
need(candidates.get("NewReadOnlyRustTypeDeclarationInventory", {}).get("selected_next_card") == next_card, "new inventory next drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectRustTypeDeclarationInventory", "decision kind drift")
need(decision.get("reason_token") == "StableTypeResourceRegistryAuthorityRequiresReadOnlyTypeDeclarationInventory", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("stable_type_resource_registry_authority_defined") == 1, "authority claim drift")
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "manual_subaxis_selection",
    "manual_type_to_subaxis_assignment",
    "return_type_string_to_subaxis_mapping",
    "self_signed_taxonomy",
    "source_path_as_policy_authority",
    "observed_subaxis_set_as_policy_authority",
    "owner_name_as_proof",
    "shape_signature_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "hardcoded_subaxis_priority",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-stable-type-resource-registry-authority")
print("accepted_registry_authority_source_count=0")
print("decision=SelectRustTypeDeclarationInventory")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
