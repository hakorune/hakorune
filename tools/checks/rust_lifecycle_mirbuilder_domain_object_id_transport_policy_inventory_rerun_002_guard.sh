#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-domain-object-id-transport-policy-inventory-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_domain_object_id_transport_policy_inventory_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2063-MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002.md"
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


token = "MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002"
next_card = "MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderDomainObjectIdTransportPolicyInventoryRerunV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need("worker_inventory = consumed" in card, "card missing worker inventory")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(
    inputs.get("carrier_type_transport_unclassified_evidence_resolution_002", "").endswith(
        "mirbuilder-carrier-type-transport-unclassified-evidence-resolution-002-v0.json"
    ),
    "unclassified input drift",
)
need(
    inputs.get("previous_id_scalar_directability_rerun", "").endswith(
        "mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json"
    ),
    "previous id scalar input drift",
)

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")
need(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

input_decision = fixture.get("input_decision") or {}
need(input_decision.get("kind") == "SelectDomainObjectIdTransportPolicyInventoryRerun002", "input decision drift")
need(input_decision.get("selected_next_card") == token, "input next drift")

ledger = fixture.get("domain_object_id_source_id_ledger") or []
need(len(ledger) == 116, "full source-id ledger missing")
need(all(row.get("source_id") for row in ledger), "ledger row missing source_id")

summary = fixture.get("summary") or {}
need(summary.get("domain_object_id_input_count") == 116, "domain input count drift")
need(summary.get("closed_id_scalar_row_count") == 31, "closed id scalar count drift")
need(summary.get("new_id_scalar_source_id_count") == 0, "new id scalar count drift")
need(summary.get("unresolved_non_id_domain_row_count") == 85, "unresolved non-id count drift")
subaxis = summary.get("domain_subaxis_counts") or {}
expected_subaxis = {
    "IdScalarDomainTransportAxis": 31,
    "PlanRecipeDomainTransportAxis": 48,
    "MirDomainTransportAxis": 19,
    "AstNodeDomainTransportAxis": 14,
    "ContextOrSpanDomainTransportAxis": 3,
    "OtherDomainObjectTransportAxis": 1,
}
for key, value in expected_subaxis.items():
    need(subaxis.get(key) == value, f"subaxis count drift: {key}")

unresolved = summary.get("unresolved_non_id_domain_subaxis_counts") or {}
for key in [
    "PlanRecipeDomainTransportAxis",
    "MirDomainTransportAxis",
    "AstNodeDomainTransportAxis",
    "ContextOrSpanDomainTransportAxis",
    "OtherDomainObjectTransportAxis",
]:
    need(unresolved.get(key) == expected_subaxis[key], f"unresolved subaxis drift: {key}")

closed = fixture.get("closed_id_scalar_lane") or {}
need(closed.get("previous_id_scalar_directability_row_count") == 31, "previous id scalar count drift")
need(closed.get("current_id_scalar_row_count") == 31, "current id scalar count drift")
need(closed.get("id_scalar_source_id_overlap_with_previous_directability_rerun") == 31, "id scalar overlap drift")
need(closed.get("new_id_scalar_source_ids") == [], "new id scalar rows should be empty")
need(closed.get("previous_id_scalar_source_ids_missing_from_current") == [], "previous id scalar missing rows should be empty")
need(closed.get("closed_id_scalar_lane_consumed") is True, "closed id scalar lane not consumed")

scope_counts = summary.get("scope_state_counts") or {}
need(scope_counts.get("ClosedIdScalarLane") == 31, "closed scope count drift")
need(scope_counts.get("UnresolvedNonIdDomainObject") == 85, "unresolved scope count drift")
need(not any(row.get("scope_state") == "NewlyUncoveredIdScalar" for row in ledger), "new id scalar scope leaked")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "full_source_id_ledger_required",
    "closed_id_scalar_lane_must_be_partitioned_before_subaxis_priority",
    "id_scalar_reselection_forbidden_when_closed_lane_matches_previous_source_ids",
]:
    need(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in ["manual_subaxis_selection", "return_type_count_as_proof", "domain_object_count_as_proof"]:
    need(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectUnresolvedSubaxisPriorityResolution", "decision kind drift")
need(decision.get("reason_token") == "ClosedIdScalarLaneConsumedAndNonIdDomainRowsRemain", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("carrier_type_transport_unclassified_evidence_resolution_002_consumed") == 1, "input consumed claim drift")
need(claims.get("domain_object_id_transport_inventory_rerun_ready") == 1, "ready claim drift")
need(claims.get("full_source_id_ledger_present") == 1, "ledger claim drift")
need(claims.get("closed_id_scalar_lane_consumed") == 1, "closed lane claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "manual_subaxis_selection",
    "return_type_count_as_proof",
    "domain_object_count_as_proof",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-domain-object-id-transport-policy-inventory-rerun-002")
print("domain_object_id_input_count=116")
print("closed_id_scalar_row_count=31")
print("new_id_scalar_source_id_count=0")
print("unresolved_non_id_domain_row_count=85")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
