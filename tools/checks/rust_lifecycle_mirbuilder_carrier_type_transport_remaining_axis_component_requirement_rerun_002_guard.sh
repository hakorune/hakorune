#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2086-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$MANIFEST" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
manifest = json.load(open(sys.argv[4], encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportRemainingAxisComponentRequirementRerunV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("component_evidence_source_discovery_inventory", "").endswith("mirbuilder-carrier-type-transport-component-evidence-source-discovery-inventory-v0.json"), "inventory input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("inventory_decision") == "SelectCarrierTypeRemainingAxisComponentRequirementRerun", "inventory decision drift")
need(previous.get("inventory_reason_token") == "ComponentEvidenceSourceDiscoveryInventoryRecorded", "inventory reason drift")
need(previous.get("inventory_selected_next_card") == token, "inventory next-card drift")
need(previous.get("accepted_component_evidence_source_count") == 0, "accepted source drift")
need(previous.get("component_authority_source_count") == 0, "authority source drift")
need(previous.get("previous_rerun_reason_token") == "NoCarrierTypeRemainingAxisRootComponentRequirement", "previous rerun reason drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "CarrierTypeRemainingAxisComponentRequirementSelectorV2", "selector drift")
need(rule.get("selection_requires_exactly_one_root_component_requirement") is True, "exactly-one rule drift")
need(rule.get("if_zero_root_requirements_keep_stopped") is True, "zero-root rule drift")
need(rule.get("zero_root_does_not_open_parent_policy_lane") is True, "parent policy loop breaker drift")
need(rule.get("zero_root_returns_to_design_consultation") is True, "consultation return drift")
need(rule.get("concrete_carrier_type_axis_selection") is False, "axis selection rule drift")

rows = fixture.get("component_requirement_rows") or []
need(len(rows) == 7, "requirement count drift")
for row in rows:
    need(row.get("accepted_sources") == [], f"accepted source drift: {row.get('requirement_id')}")
    need(row.get("root_authority_status") == "Unproven", f"root status drift: {row.get('requirement_id')}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple drift: {row.get('requirement_id')}")
    need(row.get("selection_eligible") is False, f"selection drift: {row.get('requirement_id')}")

summary = fixture.get("summary") or {}
for key in [
    "accepted_component_evidence_source_count",
    "component_authority_source_count",
    "root_component_requirement_count",
    "selection_eligible_component_requirement_count",
    "component_specific_card_selection_eligible_count",
    "concrete_carrier_type_axis_selection",
]:
    need(summary.get(key) == 0, f"summary drift: {key}")
need(summary.get("component_requirement_count") == 7, "summary requirement count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "KeepStopped", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeComponentEvidenceSourceAuthority", "reason drift")
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
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2086-MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_002_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-002")
print("component_requirement_count=7")
print("root_component_requirement_count=0")
print("decision=KeepStopped")
print("reason=NoCarrierTypeComponentEvidenceSourceAuthority")
print("selected_next_card=" + design_stop)
print("source_selfhost_claim=0")
print("summary=ok")
PY
