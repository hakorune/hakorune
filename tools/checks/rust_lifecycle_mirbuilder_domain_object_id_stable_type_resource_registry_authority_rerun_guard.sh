#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_stable_type_resource_registry_authority_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2075-MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdStableTypeResourceRegistryAuthorityRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("rust_type_declaration_inventory", "").endswith("mirbuilder-domain-object-id-rust-type-declaration-inventory-v0.json"), "inventory input drift")

rule = fixture.get("authority_rule") or {}
need(rule.get("name") == "StableTypeResourceRegistryAuthorityV1", "rule drift")
for key in [
    "registry_is_independent_proof_source",
    "registry_must_not_be_self_signed",
    "type_decl_ref_required",
    "semantic_resource_id_required",
    "type_identity_only_is_not_resource_domain_authority",
    "declared_resource_domain_subaxis_requires_explicit_semantic_declaration",
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
need(summary.get("type_decl_ref_ready_count") == 42, "type decl ready drift")
need(summary.get("semantic_resource_id_ready_count") == 42, "resource id ready drift")
need(summary.get("declared_resource_domain_subaxis_ready_count") == 0, "resource domain authority drift")
need(summary.get("registry_ready_row_count") == 0, "registry ready drift")
need(summary.get("type_identity_only_row_count") == 42, "type identity drift")
need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge drift")

rows = fixture.get("type_resource_registry_rows") or []
need(len(rows) == 42, "registry row count drift")
for row in rows:
    need(row.get("registry_row_state") == "TypeIdentityOnly", "registry row state drift")
    need(row.get("type_decl_ref"), "missing type_decl_ref")
    need(row.get("semantic_resource_id"), "missing semantic_resource_id")
    need(row.get("declared_resource_domain_subaxis") is None, "resource subaxis must not be assigned")
    need(row.get("declared_resource_domain_subaxis_authority") is None, "resource subaxis authority must be absent")
    blocked = row.get("blocked_by") or []
    need("ExplicitResourceDomainDeclarationMissing" in blocked, "missing explicit declaration blocker")
    need("TypeIdentityOnlyIsNotResourceDomainAuthority" in blocked, "missing identity-only blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "StableTypeResourceRegistryHasTypeIdentityOnlyNoResourceDomainAuthority", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("stable_type_resource_registry_authority_rerun") == 1, "rerun claim drift")
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
    "self_signed_taxonomy",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-stable-type-resource-registry-authority-rerun")
print("type_identity_only_row_count=42")
print("registry_ready_row_count=0")
print("decision=KeepStopped")
print("reason=StableTypeResourceRegistryHasTypeIdentityOnlyNoResourceDomainAuthority")
print("source_selfhost_claim=0")
print("summary=ok")
PY
