#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2083-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need("NoCarrierTypeRemainingAxisRootComponentRequirement" in task_order, "task-order missing stop reason")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("remaining_axis_component_requirement_inventory", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-inventory-v0.json"), "inventory input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("inventory_decision") == "SelectCarrierTypeRemainingAxisComponentRequirementRerun", "inventory decision drift")
need(previous.get("inventory_reason_token") == "CarrierTypeRemainingAxisComponentRequirementInventoryRecorded", "inventory reason drift")
need(previous.get("inventory_selected_next_card") == token, "inventory next card drift")
need(previous.get("accepted_component_evidence_source_count") == 0, "previous evidence count drift")
need(previous.get("ready_component_requirement_count") == 0, "previous ready count drift")
need(previous.get("root_component_requirement_count") == 0, "previous root count drift")
need(previous.get("basis_component_requirement_count") == 7, "basis requirement count drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisComponentRequirementSelectorV1", "selector drift")
need(rule.get("concrete_carrier_type_axis_selection") is False, "concrete axis rule drift")
need(rule.get("selection_requires_exactly_one_root_component_requirement") is True, "exactly-one rule drift")
need(rule.get("if_zero_root_requirements_keep_stopped") is True, "zero-root rule drift")
need(rule.get("if_multiple_root_requirements_keep_stopped") is True, "multiple-root rule drift")
need(rule.get("tie_breaking_forbidden") is True, "tie-break rule drift")

expected = {
    "TupleFieldDomainBoundaryPolicy": ("Unproven", "TupleFieldDomainBoundaryInventoryMissing"),
    "TupleElementTransportPolicy": ("Unproven", "TupleElementTransportPolicyMissing"),
    "CollectionPolicyOverlapResolution": ("Unproven", "CollectionPolicyOverlapResolutionMissing"),
    "CollectionElementCarrierPolicy": ("BlockedByComponentDependency", "CollectionElementCarrierPolicyBlockedByOverlapResolution"),
    "IteratorBorrowBoundaryRoutingPolicy": ("Unproven", "IteratorBorrowBoundaryRoutingPolicyMissing"),
    "OpaqueTypeBoundaryDeclaration": ("Unproven", "OpaqueTypeBoundaryDeclarationMissing"),
    "ScalarKnownCloseoutAuthority": ("Unproven", "ScalarKnownCloseoutAuthorityMissing"),
}
rows = fixture.get("component_requirement_rows") or []
need({row.get("requirement_id") for row in rows} == set(expected), "requirement set drift")
for row in rows:
    rid = row.get("requirement_id")
    status, reason = expected[rid]
    need(row.get("root_authority", {}).get("status") == status, f"root status drift: {rid}")
    need(row.get("root_authority", {}).get("reason_token") == reason, f"root reason drift: {rid}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple drift: {rid}")
    need(row.get("selection_eligible") is False, f"selection eligibility drift: {rid}")

summary = fixture.get("summary") or {}
need(summary.get("candidate_axis_count") == 5, "summary candidate count drift")
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("accepted_component_evidence_source_count") == 0, "summary evidence drift")
need(summary.get("ready_component_requirement_count") == 0, "summary ready drift")
need(summary.get("root_component_requirement_count") == 0, "summary root drift")
need(summary.get("selection_eligible_component_requirement_count") == 0, "summary component eligibility drift")
need(summary.get("component_specific_card_selection_eligible_count") == 0, "summary component card drift")
need(summary.get("concrete_carrier_type_axis_selection") == 0, "summary concrete selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeRemainingAxisRootComponentRequirement", "reason drift")
need(decision.get("selected_carrier_type_axis") is None, "axis must not be selected")
need(decision.get("selected_component_requirement") is None, "component must not be selected")
need(decision.get("selected_next_card") == design_stop, "next card drift")

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
need(manifest_row.get("card", "").endswith("2083-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_guard.sh"), "manifest guard drift")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun")
print("component_requirement_count=7")
print("root_component_requirement_count=0")
print("decision=KeepStopped")
print("reason=NoCarrierTypeRemainingAxisRootComponentRequirement")
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY
