#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-carrier-type-transport-evidence-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_carrier_type_transport_evidence_inventory.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[carrier-type-transport-evidence-inventory-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderCarrierTypeTransportEvidenceInventoryV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-001":
    die("fixture token mismatch")

summary = data.get("summary") or {}
if summary.get("input_candidate_count") != 23:
    die("input candidate count must be 23")
if summary.get("input_transport_notes_missing_count") != 4:
    die("input transport_notes missing count must be 4")
if summary.get("evidence_inventory_complete_count") != 23:
    die("all evidence rows must be complete")
if summary.get("unclassified_evidence_count") != 0:
    die("unclassified evidence must be 0")

lane_counts = summary.get("policy_lane_candidate_counts") or {}
expected = {
    "GenericCarrierPolicyCandidate": 12,
    "KnownTypeTransportNoCarrierPolicy": 2,
    "OptionCarrierPolicyCandidate": 3,
    "ResultCarrierVerifierPolicyCandidate": 3,
    "VecOrArrayCarrierPolicyCandidate": 3,
}
if lane_counts != expected:
    die(f"policy lane counts drift: {lane_counts}")

source_counts = summary.get("evidence_source_counts") or {}
if source_counts.get("verified_operations") != 23:
    die("verified_operations evidence must cover all rows")
if source_counts.get("checks") != 23:
    die("checks evidence must cover all rows")
if source_counts.get("transport_notes") != 19:
    die("transport_notes evidence must cover 19 rows")

rows = data.get("evidence_rows") or []
if len(rows) != 23:
    die("evidence_rows must have 23 rows")
for row in rows:
    if not row.get("owner_edge_id"):
        die("row missing owner_edge_id")
    if not row.get("normalized_evidence_labels"):
        die(f"row missing normalized labels: {row.get('owner_edge_id')}")
    if not row.get("evidence_sources"):
        die(f"row missing evidence sources: {row.get('owner_edge_id')}")
    if row.get("evidence_inventory_complete") is not True:
        die(f"row evidence must be complete: {row.get('owner_edge_id')}")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectCarrierTypeTransportPolicyLanePriorityResolution":
    die("decision kind mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-LANE-PRIORITY-RESOLUTION-001":
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

if claims.get("carrier_type_transport_policy_inventory_rerun_consumed") != 1:
    die("1980 rerun must be consumed")
if claims.get("transport_evidence_inventory_ready") != 1:
    die("inventory ready claim missing")

print("[carrier-type-transport-evidence-inventory-guard] OK")
PY
