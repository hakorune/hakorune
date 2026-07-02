#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_component_requirement_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2081-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("remaining_axis_priority_rerun_001", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-priority-rerun-v0.json"), "rerun input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("candidate_axis_count") == 5, "previous candidate count drift")
need(previous.get("scope_eligible_axis_count") == 5, "previous scope drift")
need(previous.get("guard_clean_axis_count") == 5, "previous guard drift")
need(previous.get("evidence_inventory_complete_axis_count") == 5, "previous evidence drift")
need(previous.get("proof_tuple_complete_axis_count") == 0, "previous proof tuple drift")
need(previous.get("selection_eligible_axis_count") == 0, "previous selection drift")
need(previous.get("previous_decision") == "KeepStopped", "previous decision drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeRemainingAxisMechanicalCandidate", "previous reason drift")
need(previous.get("selected_carrier_type_axis") is None, "previous axis selection drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisComponentRequirementSelectorV1", "selector rule drift")
need(rule.get("basis_selects_concrete_axis") is False, "basis must not select axis")
need(rule.get("axis_selection_deferred_to_remaining_axis_priority_rerun") is True, "axis deferral drift")
need(rule.get("component_specific_card_selection_allowed_if_exactly_one_root_requirement") is True, "component selection rule drift")
need(rule.get("tie_breaking_forbidden") is True, "tie breaking drift")
need(rule.get("if_multiple_root_requirements_keep_stopped") is True, "multiple roots rule drift")

requirements = fixture.get("component_requirements") or []
expected = {
    "TupleFieldDomainBoundaryPolicy": ("ProductTupleTransportAxis", "NotEvaluatedAtBasis"),
    "TupleElementTransportPolicy": ("ProductTupleTransportAxis", "NotEvaluatedAtBasis"),
    "CollectionPolicyOverlapResolution": ("CollectionCarrierTransportAxis", "NotEvaluatedAtBasis"),
    "CollectionElementCarrierPolicy": ("CollectionCarrierTransportAxis", "BlockedByComponentDependency"),
    "IteratorBorrowBoundaryRoutingPolicy": ("IteratorOrBorrowTypeTransportAxis", "NotEvaluatedAtBasis"),
    "OpaqueTypeBoundaryDeclaration": ("OpaqueTypeTransportAxis", "NotEvaluatedAtBasis"),
    "ScalarKnownCloseoutAuthority": ("ScalarKnownTransportAxis", "NotEvaluatedAtBasis"),
}
need({row.get("requirement_id") for row in requirements} == set(expected), "requirement set drift")
for row in requirements:
    rid = row.get("requirement_id")
    axis, status = expected[rid]
    need(row.get("candidate_axes") == [axis], f"requirement axis drift: {rid}")
    need(row.get("root_authority", {}).get("status") == status, f"root status drift: {rid}")
    need(row.get("selected_next_card_if_root"), f"missing next-card pointer: {rid}")

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
    need(row.get("component_requirement_complete") is False, f"component complete drift: {axis}")
    need(row.get("axis_selection_eligible") is False, f"axis selection drift: {axis}")

summary = fixture.get("summary") or {}
need(summary.get("candidate_axis_count") == 5, "summary candidate count drift")
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")
need(summary.get("root_component_requirement_count") == 0, "summary root count drift")
need(summary.get("component_specific_card_selection_eligible_count") == 0, "summary component selection drift")
need(summary.get("concrete_carrier_type_axis_selection") == 0, "summary concrete selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeRemainingAxisComponentRequirementInventory", "decision kind drift")
need(decision.get("reason_token") == "CarrierTypeRemainingAxisComponentRequirementsDefined", "reason drift")
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
need(manifest_row.get("card", "").endswith("2081-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_basis_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-component-requirement-basis")
print("component_requirement_count=7")
print("root_component_requirement_count=0")
print("concrete_carrier_type_axis_selection=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
