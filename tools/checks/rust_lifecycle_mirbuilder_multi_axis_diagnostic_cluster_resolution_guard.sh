#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-multi-axis-diagnostic-cluster-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_multi_axis_diagnostic_cluster_resolution.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderMultiAxisDiagnosticClusterResolutionV1", "bad kind")
need(data.get("token") == "MIRBUILDER-MULTI-AXIS-DIAGNOSTIC-CLUSTER-RESOLUTION-001", "bad token")

counts = data.get("input_counts") or {}
need(counts.get("inventory_needs_multiple_diagnostic_axes_count") == 185, "inventory multi-axis count drift")
need(counts.get("source_report_owner_edge_missing_count") == 185, "source-report owner-edge missing count drift")
need(counts.get("other_repair_input_other_owner_cluster_count") == 185, "other repair input count drift")
need(counts.get("other_shape_input_other_owner_cluster_count") == 185, "other shape input count drift")
need(counts.get("other_shape_input_shape_signature_count") == 11, "other shape signature count drift")
need(counts.get("other_shape_selection_eligible_shape_count") == 0, "other shape should have no eligible descriptor")

axis = data.get("resolved_axis_summary") or {}
blocked = axis.get("blocked_axis_cluster_counts") or {}
need(blocked.get("CarrierPolicyGap") == 7, "carrier policy gap cluster count drift")
need(blocked.get("TypeTransportOrVerifierGap") == 4, "type/verifier gap cluster count drift")
need(blocked.get("BorrowOrReceiverPolicyGap") == 8, "borrow/receiver gap cluster count drift")
need(blocked.get("ProjectionPolicyDescriptorAlreadyLanded") == 1, "completed descriptor cluster count drift")
need(axis.get("carrier_type_transport_candidate_count") == 125, "carrier/type candidate count drift")
need(axis.get("borrow_or_receiver_candidate_count") == 139, "borrow/receiver candidate count drift")
need(axis.get("completed_shape_signature_count") == 1, "completed shape count drift")

rules = data.get("selection_rules") or {}
for key in [
    "consume_existing_other_decomposition",
    "shape_descriptor_candidate_wins_before_policy_inventory",
    "carrier_type_transport_inventory_before_borrow_gap_when_present",
]:
    need(rules.get(key) == 1, f"{key} must be 1")
for key in ["cluster_size_as_proof", "manual_axis_selection"]:
    need(rules.get(key) == 0, f"{key} must be 0")

decision = data.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportPolicyInventory", "bad decision kind")
need(
    decision.get("selected_next_card") == "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-001",
    "bad selected next card",
)
need(decision.get("reason_token") == "MultiAxisClustersBlockedByCarrierTypeTransportPolicy", "bad reason")

claims = data.get("claims") or {}
for key in [
    "converter_completion_inventory_consumed",
    "source_report_consumed",
    "other_owner_edge_repair_consumed",
    "other_shape_signature_resolution_consumed",
    "multi_axis_clusters_resolved_to_next_lane",
]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "manual_family_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

print("output_contract=rust-lifecycle-mirbuilder-multi-axis-diagnostic-cluster-resolution")
print("input_multi_axis_count=185")
print("other_shape_selection_eligible_shape_count=0")
print("decision=SelectCarrierTypeTransportPolicyInventory")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("manual_axis_selection=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
