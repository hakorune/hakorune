#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2089-MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001.md"
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


token = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001"
next_card = "MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-INVENTORY-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeParentPolicyLaneEvidenceSourceDiscoveryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(token in task_order, "task-order missing token")
need(next_card in task_order, "task-order missing next card")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("parent_policy_lane_priority_rerun", "").endswith("mirbuilder-carrier-type-transport-parent-policy-lane-priority-rerun-v0.json"), "rerun input drift")

previous = fixture.get("previous_state") or {}
need(previous.get("previous_decision") == "KeepStopped", "previous decision drift")
need(previous.get("previous_reason_token") == "NoCarrierTypeParentPolicyLaneMechanicalCandidate", "previous reason drift")
need(previous.get("candidate_parent_policy_lane_count") == 4, "candidate count drift")
need(previous.get("selection_eligible_parent_policy_lane_count") == 0, "selection count drift")
need(previous.get("historical_result_contract_present") == 1, "historical result contract drift")
need(previous.get("historical_result_contract_as_direct_selection_proof") == 0, "historical proof drift")

rule = fixture.get("selector_rule") or {}
need(rule.get("name") == "ParentPolicyLaneEvidenceSourceDiscoveryAuthorityV1", "rule drift")
need(rule.get("basis_selects_parent_policy_candidate") is False, "basis selection drift")
need(rule.get("discovery_source_must_be_independent") is True, "independent source drift")
need(rule.get("self_signed_parent_policy_authority_forbidden") is True, "self-signed rule drift")
need(rule.get("hardcoded_parent_policy_priority_forbidden") is True, "hardcoded rule drift")
need(rule.get("historical_result_contract_is_diagnostic_until_current_compatibility_proven") is True, "historical diagnostic rule drift")

expected_sources = {
    "CurrentReusablePolicyContract",
    "CurrentVerifierContractCompatibility",
    "StableParentPolicyDependencyRoot",
    "PriorClosedPolicyContinuationContract",
    "CrossLanePolicyHandoffContract",
}
source_rows = fixture.get("allowed_evidence_source_kinds") or []
need({row.get("source_kind") for row in source_rows} == expected_sources, "source kind drift")
for row in source_rows:
    need("proof_source_hash" in (row.get("required_fields") or []), f"proof hash missing: {row.get('source_kind')}")

forbidden = set(fixture.get("forbidden_evidence_source_kinds") or [])
for item in [
    "RowCount",
    "ReturnTypeCount",
    "HistoricalPreference",
    "ResultHistoryAlone",
    "OwnerNameInference",
    "SourcePathOrModuleInference",
    "RouteMembershipAlone",
    "LexicalOrder",
    "HardcodedParentPolicyPriority",
    "SelfSignedFixture",
]:
    need(item in forbidden, f"forbidden kind missing: {item}")

expectations = fixture.get("parent_policy_lane_source_expectations") or []
expected_lanes = {
    "ResultCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "SelfConstructorTransportPolicyCandidate",
    "CollectionCarrierPolicyCandidate",
}
need({row.get("policy_lane") for row in expectations} == expected_lanes, "expectation lane drift")
for row in expectations:
    need(row.get("accepted_source_kinds"), f"accepted source kinds missing: {row.get('policy_lane')}")
    need(row.get("if_no_source_reason"), f"missing no-source reason: {row.get('policy_lane')}")

summary = fixture.get("summary") or {}
need(summary.get("candidate_parent_policy_lane_count") == 4, "summary candidate count drift")
need(summary.get("allowed_source_kind_count") == 5, "summary source count drift")
need(summary.get("accepted_parent_policy_evidence_source_count") == 0, "summary accepted drift")
need(summary.get("parent_policy_candidate_selection") == 0, "summary selection drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectParentPolicyLaneEvidenceSourceDiscoveryInventory", "decision kind drift")
need(decision.get("reason_token") == "ParentPolicyLaneEvidenceSourceDiscoveryBasisDefined", "reason drift")
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
need(manifest_row.get("card", "").endswith("2089-MIRBUILDER-CARRIER-TYPE-PARENT-POLICY-LANE-EVIDENCE-SOURCE-DISCOVERY-BASIS-001.md"), "manifest card drift")
need(manifest_row.get("fixture", "").endswith("mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis-v0.json"), "manifest fixture drift")
need(manifest_row.get("legacy_guard", "").endswith("rust_lifecycle_mirbuilder_carrier_type_parent_policy_lane_evidence_source_discovery_basis_guard.sh"), "manifest guard drift")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-parent-policy-lane-evidence-source-discovery-basis")
print("candidate_parent_policy_lane_count=4")
print("allowed_source_kind_count=5")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
