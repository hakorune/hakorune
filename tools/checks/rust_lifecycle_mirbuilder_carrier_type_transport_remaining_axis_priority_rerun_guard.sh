#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_priority_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2080-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
manifest = json.load(open(sys.argv[5], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001"
basis_token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisPriorityRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("remaining_axis_priority_basis", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-priority-basis-v0.json"), "basis input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_decision") == "SelectCarrierTypeRemainingAxisPriorityRerun", "basis decision drift")
need(previous.get("basis_selected_next_card") == token, "basis next drift")
need(previous.get("domain_object_id_lane_parked") == 1, "DomainObject/Id park drift")
need(previous.get("basis_candidate_axis_count") == 5, "basis candidate count drift")

summary = fixture.get("summary") or {}
need(summary.get("candidate_axis_count") == 5, "candidate count drift")
need(summary.get("scope_eligible_axis_count") == 5, "scope eligible drift")
need(summary.get("guard_clean_axis_count") == 5, "guard clean drift")
need(summary.get("evidence_inventory_complete_axis_count") == 5, "evidence completeness drift")
need(summary.get("dependency_root_authority_proven_count") == 0, "dependency root drift")
need(summary.get("prior_closed_lane_continuation_authority_proven_count") == 0, "prior closed drift")
need(summary.get("policy_contract_readiness_proven_count") == 0, "policy contract drift")
need(summary.get("proof_tuple_complete_axis_count") == 0, "proof tuple drift")
need(summary.get("selection_eligible_axis_count") == 0, "selection drift")
need(summary.get("component_requirement_basis_required_count") == 5, "component basis required drift")

expected_axes = {
    "ProductTupleTransportAxis",
    "CollectionCarrierTransportAxis",
    "IteratorOrBorrowTypeTransportAxis",
    "OpaqueTypeTransportAxis",
    "ScalarKnownTransportAxis",
}
rows = fixture.get("candidate_axes") or []
need({row.get("axis") for row in rows} == expected_axes, "candidate set drift")
for row in rows:
    axis = row.get("axis")
    need(row.get("scope_eligible") is True, f"scope drift: {axis}")
    need(row.get("guard_clean_authority", {}).get("status") == "Proven", f"guard clean drift: {axis}")
    need(row.get("evidence_inventory_completeness", {}).get("status") == "Proven", f"evidence completeness drift: {axis}")
    for key in [
        "dependency_root_authority",
        "prior_closed_lane_continuation_authority",
        "policy_contract_readiness",
    ]:
        need(row.get(key, {}).get("status") == "Unproven", f"{key} drift: {axis}")
    need(row.get("component_requirement_basis_required") is True, f"component basis drift: {axis}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple must be false: {axis}")
    need(row.get("selection_eligible") is False, f"axis must not be selected: {axis}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeRemainingAxisMechanicalCandidate", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "carrier axis must not be selected")
need(decision.get("selected_domain_subaxis") is None, "domain subaxis must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
need(claims.get("remaining_axis_priority_basis_consumed") == 1, "basis consumed claim drift")
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "manual_axis_selection",
    "manual_carrier_selection",
    "hardcoded_carrier_axis_priority",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "observed_subaxis_set_as_proof",
    "return_type_string_mapping_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2080-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_priority_rerun_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-priority-rerun")
print("candidate_axis_count=5")
print("selection_eligible_axis_count=0")
print("decision=KeepStopped")
print("reason=NoCarrierTypeRemainingAxisMechanicalCandidate")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
