#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2090-MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
next_card = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeParentPolicyLaneEvidenceSourceDiscoveryInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("parent_policy_lane_evidence_source_discovery_basis", "").endswith("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json"), "basis input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("basis_decision") == "SelectParentPolicyLaneEvidenceSourceDiscoveryInventory", "basis decision drift")
need(previous.get("basis_reason_token") == "ParentPolicyLaneEvidenceSourceDiscoveryBasisDefined", "basis reason drift")
need(previous.get("basis_selected_next_card") == token, "basis selected next drift")
need(previous.get("candidate_parent_policy_lane_count") == 4, "basis candidate count drift")
need(previous.get("allowed_source_kind_count") == 5, "basis source count drift")

rule = fixture.get("inventory_rule") or {}
need(rule.get("name") == "ParentPolicyLaneEvidenceSourceDiscoveryInventoryV1", "inventory rule drift")
need(rule.get("reads_existing_authority_sources_only") is True, "read-only rule drift")
need(rule.get("accepted_source_must_join_current_policy_lane") is True, "join rule drift")
need(rule.get("accepted_source_must_have_stable_id") is True, "stable id rule drift")
need(rule.get("accepted_source_must_have_proof_source_hash") is True, "proof hash rule drift")
need(rule.get("self_signed_parent_policy_authority_forbidden") is True, "self-signed rule drift")
need(rule.get("historical_result_contract_alone_is_not_authority") is True, "historical result rule drift")
need(rule.get("parent_policy_candidate_selection") is False, "candidate selection rule drift")

source_rows = fixture.get("source_kind_rows") or []
expected_sources = {
    "CurrentReusablePolicyContract",
    "CurrentVerifierContractCompatibility",
    "StableParentPolicyDependencyRoot",
    "PriorClosedPolicyContinuationContract",
    "CrossLanePolicyHandoffContract",
}
need({row.get("source_kind") for row in source_rows} == expected_sources, "source set drift")
for row in source_rows:
    need(row.get("accepted_source_count") == 0, f"accepted source drift: {row.get('source_kind')}")
    need(row.get("discovery_state") == "NoAcceptedSource", f"state drift: {row.get('source_kind')}")
    need("proof_source_hash" in (row.get("required_fields") or []), f"proof hash missing: {row.get('source_kind')}")

lane_rows = fixture.get("parent_policy_lane_source_rows") or []
expected_lanes = {
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
}
need({row.get("policy_lane") for row in lane_rows} == expected_lanes, "lane set drift")
for row in lane_rows:
    need(row.get("accepted_sources") == [], f"accepted sources drift: {row.get('policy_lane')}")
    need(row.get("authority_source_count") == 0, f"authority count drift: {row.get('policy_lane')}")
    need(row.get("proof_tuple_complete") is False, f"proof tuple drift: {row.get('policy_lane')}")
    need(row.get("selection_eligible") is False, f"selection drift: {row.get('policy_lane')}")

summary = fixture.get("summary") or {}
for key in [
    "accepted_parent_policy_evidence_source_count",
    "parent_policy_authority_source_count",
    "parent_policy_lane_with_accepted_source_count",
    "current_reusable_policy_contract_count",
    "current_verifier_contract_compatibility_count",
    "stable_parent_policy_dependency_root_count",
    "prior_closed_policy_continuation_contract_count",
    "cross_lane_policy_handoff_contract_count",
    "parent_policy_candidate_selection",
]:
    need(summary.get(key) == 0, f"summary zero drift: {key}")
need(summary.get("candidate_parent_policy_lane_count") == 4, "summary candidate count drift")
need(summary.get("allowed_source_kind_count") == 5, "summary source count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectWiderRouteSelectionBasis", "decision kind drift")
need(decision.get("reason_token") == "NoCarrierTypeParentPolicyLaneEvidenceSourceAuthority", "reason drift")
need(decision.get("selected_parent_policy_candidate") is None, "parent policy must not be selected")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "accepted_typed_dependency_edge_materialized",
    "parent_policy_candidate_selection",
    "direct_parent_policy_candidate_selection",
    "manual_parent_policy_selection",
    "hardcoded_parent_policy_priority",
    "row_count_as_proof",
    "return_type_count_as_proof",
    "source_path_as_authority",
    "owner_name_as_proof",
    "route_membership_alone_as_proof",
    "historical_preference_as_proof",
    "result_history_as_direct_selection_proof",
    "self_signed_parent_policy_authority",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
manifest_row = rows_by_token.get(token) or {}
need(manifest_row.get("card", "").endswith("2090-MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_inventory_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-inventory")
print("candidate_parent_policy_lane_count=4")
print("accepted_parent_policy_evidence_source_count=0")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
