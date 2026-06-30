#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-policy-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_policy_inventory.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderCarrierTypeTransportPolicyInventoryV1", "bad kind")
need(data.get("token") == "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-001", "bad token")

rows = data.get("transport_rows") or []
need(len(rows) == 11, "transport row count must be 11")

summary = data.get("summary") or {}
need(summary.get("input_shape_cluster_count") == 11, "input shape cluster count drift")
need(summary.get("carrier_type_transport_candidate_count") == 125, "carrier/type count drift")
need(summary.get("policy_lane_selected_count") == 0, "policy lane must not be selected")

return_counts = summary.get("return_family_candidate_counts") or {}
for key, value in {
    "constructor_self": 7,
    "custom_carrier": 21,
    "iterator": 1,
    "option": 37,
    "result": 55,
    "vec": 4,
}.items():
    need(return_counts.get(key) == value, f"return family count drift: {key}")

type_counts = summary.get("type_transport_axis_candidate_counts") or {}
for key, value in {
    "ConstructorCarrier": 7,
    "KnownOptionCarrier": 37,
    "KnownVecCarrier": 4,
    "MissingTypeTransport": 21,
    "ResultCarrierNeedsVerifier": 55,
    "ReturnedIteratorNeedsPolicy": 1,
}.items():
    need(type_counts.get(key) == value, f"type axis count drift: {key}")

lane_counts = summary.get("policy_lane_candidate_counts") or {}
for key, value in {
    "ConstructorCarrierPolicyCandidate": 7,
    "MissingTypeTransportPolicyCandidate": 21,
    "OptionCarrierPolicyCandidate": 37,
    "ResultCarrierVerifierPolicyCandidate": 55,
    "ReturnedIteratorPolicyCandidate": 1,
    "VecCarrierPolicyCandidate": 4,
}.items():
    need(lane_counts.get(key) == value, f"policy lane count drift: {key}")

for lane in data.get("policy_lane_candidates") or []:
    need(lane.get("selection_eligible") is False, f"policy lane must be inventory-only: {lane.get('lane')}")
    need(lane.get("reason_token") == "InventoryOnlyPolicyLaneCandidate", "bad lane reason")

decision = data.get("decision") or {}
need(decision.get("kind") == "SelectStrictConverterEmissionProbe", "bad decision kind")
need(decision.get("reason_token") == "CarrierTypeTransportPolicyInventoryRecorded", "bad reason token")
need(decision.get("selected_next_card") == "MIRBUILDER-STRICT-CONVERTER-EMISSION-PROBE-001", "bad next card")

claims = data.get("claims") or {}
for key in [
    "multi_axis_resolution_consumed",
    "other_shape_resolution_consumed",
    "carrier_type_transport_inventory_ready",
]:
    need(claims.get(key) == 1, f"{key} must be 1")
for key in [
    "policy_lane_selected",
    "manual_carrier_selection",
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

print("output_contract=rust-lifecycle-mirbuilder-carrier-type-transport-policy-inventory")
print("carrier_type_transport_candidate_count=125")
print("policy_lane_selected=0")
print("decision=SelectStrictConverterEmissionProbe")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("hako_generation=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
