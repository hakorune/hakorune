#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-evidence-inventory-rerun-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_evidence_inventory_rerun_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2061-MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-003.md"
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
_task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-003"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-002"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderCarrierTypeTransportEvidenceInventoryRerunV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need("LocalMechanicalSelectorAuthorityV1" in card, "card missing local authority")
need("worker_inventory = consumed" in card, "card missing worker inventory")

inputs = fixture.get("input_state") or {}
need(inputs.get("current_blocker") == design_stop, "blocker drift")
need(
    inputs.get("carrier_type_transport_policy_inventory_rerun_003", "").endswith(
        "mirbuilder-carrier-type-transport-policy-inventory-rerun-003-v0.json"
    ),
    "input drift",
)

local = fixture.get("local_authority") or {}
need(local.get("local_selection_authority") == "LocalMechanicalSelectorAuthorityV1", "local authority drift")
need(local.get("worker_inventory") == "consumed", "worker inventory drift")
need(local.get("worker_inventory_scope") == "read_only_current_fixtures_cards_ledgers", "worker scope drift")

input_decision = fixture.get("input_decision") or {}
need(input_decision.get("kind") == "SelectCarrierTypeTransportEvidenceInventoryRerun003", "input decision drift")
need(input_decision.get("selected_next_card") == token, "input next drift")

summary = fixture.get("summary") or {}
need(summary.get("input_candidate_count") == 944, "input count drift")
need(summary.get("evidence_inventory_complete_count") == 814, "complete count drift")
need(summary.get("unclassified_evidence_count") == 130, "unclassified count drift")
lane_counts = summary.get("policy_lane_candidate_counts") or {}
need(lane_counts.get("ResultCarrierPolicyCandidate") == 557, "result lane drift")
need(lane_counts.get("OptionCarrierPolicyCandidate") == 166, "option lane drift")
need(lane_counts.get("CarrierTypeTransportEvidenceInventoryRequired") == 130, "unclassified lane drift")

selection_rule = fixture.get("selection_rule") or {}
for key in [
    "source_return_type_is_evidence_not_policy",
    "unclassified_evidence_blocks_policy_priority",
    "worker_inventory_required_or_waived",
]:
    need(selection_rule.get(key) is True, f"selection rule drift: {key}")
for key in [
    "return_type_count_as_proof",
    "manual_carrier_selection",
    "cluster_size_as_proof",
]:
    need(selection_rule.get(key) is False, f"forbidden selection rule drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportUnclassifiedEvidenceResolution", "decision kind drift")
need(decision.get("reason_token") == "UnclassifiedCarrierTypeTransportEvidenceRemainsAfterRerun003", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("carrier_type_transport_policy_inventory_rerun_003_consumed") == 1, "inventory consumed claim drift")
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

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-evidence-inventory-rerun-003")
print("input_candidate_count=944")
print("evidence_inventory_complete_count=814")
print("unclassified_evidence_count=130")
print(f"selected_next_card={next_card}")
print("source_selfhost_claim=0")
print("summary=ok")
PY
