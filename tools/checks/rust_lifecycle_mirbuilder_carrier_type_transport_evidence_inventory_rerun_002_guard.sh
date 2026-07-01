#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-evidence-inventory-rerun-002-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_evidence_inventory_rerun_002.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2010-MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002.md"
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


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportEvidenceInventoryRerunV2", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(inputs.get("carrier_type_transport_policy_inventory_rerun_002", "").endswith("mirbuilder-carrier-type-transport-policy-inventory-rerun-002-v0.json"), "input drift")

summary = fixture.get("summary") or {}
need(summary.get("input_candidate_count") == 944, "input count drift")
need(summary.get("evidence_inventory_complete_count") == 814, "complete count drift")
need(summary.get("unclassified_evidence_count") == 130, "unclassified count drift")
lane_counts = summary.get("policy_lane_candidate_counts") or {}
need(lane_counts.get("ResultCarrierPolicyCandidate") == 557, "result lane drift")
need(lane_counts.get("OptionCarrierPolicyCandidate") == 166, "option lane drift")
need(lane_counts.get("CarrierTypeTransportEvidenceInventoryRequired") == 130, "unclassified lane drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportUnclassifiedEvidenceResolution", "decision kind drift")
need(decision.get("reason_token") == "UnclassifiedCarrierTypeTransportEvidenceRemainsAfterRerun002", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("carrier_type_transport_policy_inventory_rerun_002_consumed") == 1, "inventory consumed claim drift")
need(claims.get("transport_evidence_inventory_ready") == 1, "ready claim drift")
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
    "input_candidate_count = 944",
    "unclassified_evidence_count = 130",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-evidence-inventory-rerun-002")
print("input_candidate_count=944")
print("evidence_inventory_complete_count=814")
print("unclassified_evidence_count=130")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
