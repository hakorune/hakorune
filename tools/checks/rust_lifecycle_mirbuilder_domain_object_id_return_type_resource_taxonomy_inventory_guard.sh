#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_return_type_resource_taxonomy_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2071-MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdReturnTypeResourceTaxonomyInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("return_type_reference_edge_derivation_basis", "").endswith("mirbuilder-domain-object-id-return-type-reference-edge-derivation-basis-v0.json"), "basis input drift")

policy = fixture.get("taxonomy_policy") or {}
for key in [
    "observed_subaxis_set_is_diagnostic_only",
    "return_type_name_is_not_policy_authority",
    "owner_edge_is_not_policy_authority",
    "row_count_is_not_policy_authority",
    "taxonomy_entries_must_be_typed_fixture_rows",
]:
    need(policy.get(key) is True, f"policy drift: {key}")

summary = fixture.get("summary") or {}
need(summary.get("return_type_reference_count") == 85, "return type reference count drift")
need(summary.get("distinct_return_type_count") == 44, "distinct return type count drift")
need(summary.get("taxonomy_entry_count") == 0, "taxonomy entry count drift")
need(summary.get("missing_taxonomy_entry_count") == 44, "missing taxonomy count drift")
need(summary.get("edge_ready_return_type_count") == 0, "edge-ready return type drift")
need(summary.get("accepted_edge_candidate_count") == 0, "accepted edge candidate drift")

rows = fixture.get("return_type_taxonomy_rows") or []
need(len(rows) == 44, "taxonomy row count drift")
for row in rows:
    need(row.get("taxonomy_entry_state") == "MissingTaxonomyEntry", "taxonomy state drift")
    need(row.get("resource_domain_subaxis_declared") is False, "resource subaxis drift")
    need(row.get("dependency_role_declared") is False, "dependency role drift")
    need(row.get("edge_ready") is False, "edge ready drift")
    blocked = row.get("blocked_by") or []
    need("ReturnTypeNameIsNotPolicyAuthority" in blocked, "missing name-policy blocker")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "ReturnTypeResourceTaxonomyEntriesMissing", "reason drift")
need(decision.get("selected_domain_subaxis") is None, "subaxis must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("return_type_resource_taxonomy_inventory") == 1, "inventory claim drift")
for key in [
    "return_type_name_as_policy_authority",
    "observed_subaxis_set_as_policy_proof",
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
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "convenience_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-return-type-resource-taxonomy-inventory")
print("distinct_return_type_count=44")
print("taxonomy_entry_count=0")
print("decision=KeepStopped")
print("reason=ReturnTypeResourceTaxonomyEntriesMissing")
print("source_selfhost_claim=0")
print("summary=ok")
PY
