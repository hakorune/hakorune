#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_component_requirement_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2082-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("remaining_axis_component_requirement_basis", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"), "basis input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_decision") == "SelectCarrierTypeRemainingAxisComponentRequirementInventory", "basis decision drift")
need(previous.get("basis_reason_token") == "CarrierTypeRemainingAxisComponentRequirementsDefined", "basis reason drift")
need(previous.get("basis_selected_next_card") == token, "basis next-card drift")
need(previous.get("basis_component_requirement_count") == 7, "basis requirement count drift")
need(previous.get("basis_root_component_requirement_count") == 0, "basis root count drift")
need(previous.get("priority_rerun_reason_token") == "NoCarrierTypeRemainingAxisMechanicalCandidate", "priority reason drift")
need(previous.get("priority_rerun_selection_eligible_axis_count") == 0, "priority selection drift")

rule = fixture.get("inventory_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisComponentRequirementInventoryV1", "inventory rule drift")
need(rule.get("reads_existing_typed_component_evidence_only") is True, "typed evidence rule drift")
need(rule.get("component_evidence_must_be_non_self_signed") is True, "self-signed rule drift")
need(rule.get("component_evidence_must_have_stable_proof_source_hash") is True, "proof hash rule drift")
need(rule.get("basis_selects_concrete_axis") is False, "basis must not select axis")
need(rule.get("concrete_carrier_type_axis_selection") is False, "concrete axis selection drift")

expected = {
    "TupleFieldDomainBoundaryPolicy": ("Missing", "Unproven", "TupleFieldDomainBoundaryInventoryMissing"),
    "TupleElementTransportPolicy": ("Missing", "Unproven", "TupleElementTransportPolicyMissing"),
    "CollectionPolicyOverlapResolution": ("Missing", "Unproven", "CollectionPolicyOverlapResolutionMissing"),
    "CollectionElementCarrierPolicy": ("BlockedByComponentDependency", "BlockedByComponentDependency", "CollectionElementCarrierPolicyBlockedByOverlapResolution"),
    "IteratorBorrowBoundaryRoutingPolicy": ("Missing", "Unproven", "IteratorBorrowBoundaryRoutingPolicyMissing"),
    "OpaqueTypeBoundaryDeclaration": ("Missing", "Unproven", "OpaqueTypeBoundaryDeclarationMissing"),
    "ScalarKnownCloseoutAuthority": ("Missing", "Unproven", "ScalarKnownCloseoutAuthorityMissing"),
}
rows = fixture.get("component_evidence_inventory_rows") or []
need({row.get("requirement_id") for row in rows} == set(expected), "requirement inventory set drift")
for row in rows:
    rid = row.get("requirement_id")
    state_expected, status_expected, reason_expected = expected[rid]
    need(row.get("inventory_state") == state_expected, f"inventory state drift: {rid}")
    need(row.get("accepted_evidence_sources") == [], f"accepted evidence drift: {rid}")
    need(row.get("root_authority", {}).get("status") == status_expected, f"root status drift: {rid}")
    need(row.get("root_authority", {}).get("reason_token") == reason_expected, f"root reason drift: {rid}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple drift: {rid}")
    need(row.get("selection_eligible") is False, f"selection eligibility drift: {rid}")

candidate_axes = fixture.get("candidate_axes") or []
expected_axis_requirements = {
    "ProductTupleTransportAxis": {
        "TupleFieldDomainBoundaryPolicy",
        "TupleElementTransportPolicy",
    },
    "CollectionCarrierTransportAxis": {
        "CollectionPolicyOverlapResolution",
        "CollectionElementCarrierPolicy",
    },
    "IteratorOrBorrowTypeTransportAxis": {"IteratorBorrowBoundaryRoutingPolicy"},
    "OpaqueTypeTransportAxis": {"OpaqueTypeBoundaryDeclaration"},
    "ScalarKnownTransportAxis": {"ScalarKnownCloseoutAuthority"},
}
need({row.get("axis") for row in candidate_axes} == set(expected_axis_requirements), "candidate axis set drift")
for row in candidate_axes:
    axis = row.get("axis")
    need(set(row.get("component_requirement_ids") or []) == expected_axis_requirements[axis], f"axis requirement drift: {axis}")
    need(row.get("ready_component_requirement_count") == 0, f"ready component drift: {axis}")
    need(row.get("root_component_requirement_count") == 0, f"root component drift: {axis}")
    need(row.get("component_requirement_complete") is False, f"component complete drift: {axis}")
    need(row.get("axis_selection_eligible") is False, f"axis selection drift: {axis}")

summary = fixture.get("summary") or {}
need(summary.get("candidate_axis_count") == 5, "summary candidate count drift")
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("accepted_component_evidence_source_count") == 0, "summary evidence count drift")
need(summary.get("ready_component_requirement_count") == 0, "summary ready count drift")
need(summary.get("root_component_requirement_count") == 0, "summary root count drift")
need(summary.get("component_specific_card_selection_eligible_count") == 0, "summary component selection drift")
need(summary.get("concrete_carrier_type_axis_selection") == 0, "summary concrete selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeRemainingAxisComponentRequirementRerun", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeRemainingAxisComponentRequirementInventoryRecorded", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") is None, "component must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "component_specific_card_selection",
    "concrete_carrier_type_axis_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "hardcoded_carrier_axis_priority",
    "row_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "return_type_string_mapping_as_proof",
    "observed_subaxis_set_as_proof",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2082-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_inventory_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory")
print("component_requirement_count=7")
print("accepted_component_evidence_source_count=0")
print("root_component_requirement_count=0")
print("concrete_carrier_type_axis_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
