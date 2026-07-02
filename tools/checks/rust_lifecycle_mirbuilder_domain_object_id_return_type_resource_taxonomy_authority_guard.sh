#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-return-type-resource-taxonomy-authority-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_return_type_resource_taxonomy_authority.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2072-MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdReturnTypeResourceTaxonomyAuthorityV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("return_type_resource_taxonomy_inventory", "").endswith("mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory-v0.json"), "taxonomy inventory input drift")

rule = fixture.get("authority_rule") or {}
need(rule.get("name") == "ReturnTypeResourceTaxonomyAuthorityV1", "authority rule drift")
for key in [
    "return_type_string_is_not_policy_authority",
    "return_type_string_to_subaxis_mapping_forbidden",
    "resource_id_required",
    "type_decl_ref_required",
    "resource_domain_subaxis_must_come_from_taxonomy",
    "stable_proof_source_required",
    "self_signed_taxonomy_forbidden",
]:
    need(rule.get(key) is True, f"authority rule drift: {key}")
need(rule.get("accepted_typed_dependency_edges_materialized_here") is False, "edges must not materialize here")

summary = fixture.get("summary") or {}
need(summary.get("return_type_reference_count") == 85, "return type reference count drift")
need(summary.get("distinct_return_type_count") == 44, "distinct return type count drift")
need(summary.get("taxonomy_entry_count") == 0, "taxonomy entry count drift")
need(summary.get("resolved_type_decl_ref_count") == 0, "resolved type decl count drift")
need(summary.get("resource_taxonomy_join_ready_count") == 0, "join ready count drift")
need(summary.get("edge_ready_return_type_count") == 0, "edge ready count drift")
need(summary.get("accepted_typed_dependency_edge_count") == 0, "accepted edge count drift")

need(fixture.get("resource_taxonomy_rows") == [], "taxonomy rows should be empty without independent registry")
need(fixture.get("return_type_reference_rows") == [], "reference rows should be empty without independent registry")
readiness = fixture.get("edge_derivation_readiness_rows") or []
need(len(readiness) == 44, "readiness row count drift")
for row in readiness:
    need(row.get("edge_ready") is False, "edge readiness drift")
    blocked = row.get("blocked_by") or []
    need("StableTypeDeclarationResourceRegistryMissing" in blocked, "missing registry blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "ReturnTypeResourceTaxonomyAuthorityEntriesMissing", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("return_type_resource_taxonomy_authority_defined") == 1, "authority claim drift")
for key in [
    "return_type_string_to_subaxis_mapping",
    "return_type_string_as_policy_authority",
    "observed_domain_subaxis_set_as_proof",
    "self_signed_taxonomy",
    "accepted_typed_dependency_edge_materialized",
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subaxis_selection",
    "hardcoded_subaxis_priority",
    "row_count_as_proof",
    "domain_object_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "owner_name_as_proof",
    "shape_signature_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "convenience_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-return-type-resource-taxonomy-authority")
print("taxonomy_entry_count=0")
print("edge_ready_return_type_count=0")
print("decision=KeepStopped")
print("reason=ReturnTypeResourceTaxonomyAuthorityEntriesMissing")
print("source_selfhost_claim=0")
print("summary=ok")
PY
