#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bridge-blocked-gap-cluster-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_bridge_blocked_gap_cluster_resolution.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1979-MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001.md"
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

token = "MIRBUILDER-BRIDGE-BLOCKED-GAP-CLUSTER-RESOLUTION-001"
next_card = "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001"
design_stop = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"

need(fixture.get("kind") == "MirBuilderBridgeBlockedGapClusterResolutionV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(fixture.get("pure_policy_gap_candidate_count") == 24, "pure gap count drift")

clusters = {cluster.get("cluster_id"): cluster for cluster in fixture.get("gap_clusters", [])}
carrier = clusters["bridge_gap::carrier_type_transport_only"]
mixed = clusters["bridge_gap::borrow_and_carrier_type_transport"]
borrow = clusters["bridge_gap::borrow_policy_only"]
need(carrier["candidate_count"] == 23, "carrier/type gap count drift")
need(carrier["selection_eligible"] is True, "carrier/type gap must be eligible")
need(mixed["candidate_count"] == 1, "mixed gap count drift")
need(mixed["selection_eligible"] is False, "mixed gap must be deferred")
need(borrow["candidate_count"] == 0, "borrow-only gap count drift")
need(borrow["selection_eligible"] is False, "borrow-only gap must not be eligible")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectCarrierTypeTransportPolicyInventoryRerun", "bad decision kind")
need(decision.get("reason_token") == "ExactlyOneBridgeBlockedGapClusterEligible", "bad reason")
need(decision.get("selected_cluster_id") == "bridge_gap::carrier_type_transport_only", "bad selected cluster")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "manual_cluster_selection",
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
    need(claims.get(key) == 0, f"{key} must be 0")

need(state.get("current_blocker_token") == design_stop, "CURRENT_STATE blocker drift")
need(next_card in task_order, "task-order missing next card")

print("output_contract=rust-lifecycle-mirbuilder-bridge-blocked-gap-cluster-resolution")
print("pure_policy_gap_candidate_count=24")
print("selected_cluster=bridge_gap::carrier_type_transport_only")
print(f"selected_next_card={next_card}")
print("manual_cluster_selection=0")
print("cluster_size_as_proof=0")
print("source_selfhost_claim=0")
print("summary=ok")
PY
