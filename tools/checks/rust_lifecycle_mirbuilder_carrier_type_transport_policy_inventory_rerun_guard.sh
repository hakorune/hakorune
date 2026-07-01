#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-policy-inventory-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_policy_inventory_rerun.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture_path = Path(sys.argv[1])
data = json.loads(fixture_path.read_text())

def die(message: str) -> None:
    print(f"[carrier-type-transport-policy-inventory-rerun-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderCarrierTypeTransportPolicyInventoryRerunV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-001":
    die("fixture token mismatch")

summary = data.get("summary") or {}
if summary.get("carrier_type_transport_only_count") != 23:
    die("carrier/type-only candidate count must be 23")
if summary.get("mixed_borrow_carrier_type_transport_count") != 1:
    die("mixed borrow+carrier gap count must remain 1")
if summary.get("transport_notes_missing_count") != 4:
    die("transport notes missing count must be 4")
if summary.get("eligible_policy_lane_count") != 4:
    die("eligible policy lane count must be 4")

lane_counts = summary.get("policy_lane_candidate_counts") or {}
for lane in [
    "GenericCarrierPolicyCandidate",
    "OptionCarrierPolicyCandidate",
    "ResultCarrierVerifierPolicyCandidate",
    "TransportEvidenceInventoryRequired",
    "VecOrArrayCarrierPolicyCandidate",
]:
    if lane not in lane_counts:
        die(f"missing policy lane count: {lane}")

rows = data.get("transport_rows") or []
if len(rows) != 23:
    die("transport_rows must classify exactly 23 candidates")
seen = set()
for row in rows:
    owner = row.get("owner_edge_id")
    if not owner:
        die("transport row missing owner_edge_id")
    if owner in seen:
        die(f"duplicate owner_edge_id: {owner}")
    seen.add(owner)
    if not row.get("evidence_labels"):
        die(f"transport row missing evidence labels: {owner}")
    if not row.get("policy_lane_candidate"):
        die(f"transport row missing policy lane candidate: {owner}")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectCarrierTypeTransportEvidenceInventory":
    die("decision must select evidence inventory")
if decision.get("selected_next_card") != "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-001":
    die("selected next card mismatch")
if decision.get("selected_policy_lane") is not None:
    die("selected policy lane must remain null")

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

if claims.get("bridge_gap_cluster_resolution_consumed") != 1:
    die("bridge gap cluster resolution must be consumed")
if claims.get("carrier_type_transport_inventory_rerun_ready") != 1:
    die("inventory rerun ready claim missing")
if claims.get("mixed_gap_deferred") != 1:
    die("mixed gap must be deferred")

print("[carrier-type-transport-policy-inventory-rerun-guard] OK")
PY
