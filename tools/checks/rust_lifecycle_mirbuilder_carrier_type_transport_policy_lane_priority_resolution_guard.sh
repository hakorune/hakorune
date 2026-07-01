#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-policy-lane-priority-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_policy_lane_priority_resolution.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[carrier-type-transport-policy-lane-priority-resolution-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderCarrierTypeTransportPolicyLanePriorityResolutionV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001":
    die("fixture token mismatch")

priority = data.get("lane_priority") or []
if priority != [
    "ResultCarrierVerifierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "VecOrArrayCarrierPolicyCandidate",
    "GenericCarrierPolicyCandidate",
]:
    die("lane priority drift")

summary = data.get("summary") or {}
if summary.get("input_candidate_count") != 23:
    die("input candidate count must be 23")
if summary.get("policy_lane_count") != 5:
    die("policy lane count must be 5")
if summary.get("eligible_policy_lane_count") != 4:
    die("eligible policy lane count must be 4")
if summary.get("known_type_transport_no_policy_count") != 2:
    die("known type transport no-policy count must be 2")
if summary.get("selected_policy_lane_candidate_count") != 3:
    die("selected policy lane candidate count must be 3")

lanes = {lane["lane"]: lane for lane in data.get("policy_lanes") or []}
if lanes["KnownTypeTransportNoCarrierPolicy"].get("selection_eligible") is not False:
    die("KnownTypeTransportNoCarrierPolicy must be excluded")
if lanes["ResultCarrierVerifierPolicyCandidate"].get("priority_index") != 0:
    die("result carrier lane must have priority index 0")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectCarrierTypeTransportPolicyLane":
    die("decision kind mismatch")
if decision.get("selected_policy_lane") != "ResultCarrierVerifierPolicyCandidate":
    die("selected policy lane mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-RESULT-CARRIER-VERIFIER-POLICY-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_carrier_selection",
    "owner_name_as_transport_policy",
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
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")

if claims.get("carrier_type_transport_evidence_inventory_consumed") != 1:
    die("1981 evidence inventory must be consumed")
if claims.get("policy_lane_priority_resolution_ready") != 1:
    die("priority resolution ready claim missing")

print("[carrier-type-transport-policy-lane-priority-resolution-guard] OK")
PY
