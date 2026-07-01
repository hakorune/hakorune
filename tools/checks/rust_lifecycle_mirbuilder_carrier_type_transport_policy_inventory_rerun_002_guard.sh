#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-policy-inventory-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_policy_inventory_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2009-MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportPolicyInventoryRerunV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("missing_projection_policy_cluster_resolution_v3", "").endswith("mirbuilder-missing-projection-policy-cluster-resolution-v3-v0.json"), "V3 input drift")

summary = fixture.get("summary") or {}
need(summary.get("type_transport_missing_cluster_count") == 76, "type cluster count drift")
need(summary.get("type_transport_missing_item_count") == 944, "type item count drift")
need(summary.get("eligible_policy_lane_count") == 4, "eligible lane count drift")
lane_counts = summary.get("policy_lane_candidate_counts") or {}
expected_lanes = {
    "ResultCarrierPolicyCandidate": 557,
    "OptionCarrierPolicyCandidate": 166,
    "SelfConstructorTransportPolicyCandidate": 56,
    "CollectionCarrierPolicyCandidate": 35,
    "CarrierTypeTransportEvidenceInventoryRequired": 130,
}
for lane, count in expected_lanes.items():
    need(lane_counts.get(lane) == count, f"lane count drift: {lane}")

labels = summary.get("evidence_label_counts") or {}
for label in [
    "ResultCarrierEvidence",
    "OptionCarrierEvidence",
    "CollectionCarrierEvidence",
    "DomainObjectOrIdTransportEvidence",
]:
    need(label in labels, f"missing evidence label: {label}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportEvidenceInventoryRerun002", "decision kind drift")
need(decision.get("reason_token") == "MultipleCarrierTypeTransportLanesRequireEvidenceInventory", "reason drift")
need(decision.get("selected_next_card") == next_card, "next card drift")
need(decision.get("selected_policy_lane") is None, "policy lane must stay null")

claims = fixture.get("claims") or {}
need(claims.get("missing_projection_policy_v3_consumed") == 1, "V3 consumed claim drift")
need(claims.get("carrier_type_transport_inventory_rerun_ready") == 1, "ready claim drift")
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "return_type_count_as_proof",
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

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
for needle in [
    token,
    next_card,
    "type_transport_missing_item_count = 944",
    "eligible_policy_lane_count = 4",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-policy-inventory-rerun-002")
print("type_transport_missing_cluster_count=76")
print("type_transport_missing_item_count=944")
print("eligible_policy_lane_count=4")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
