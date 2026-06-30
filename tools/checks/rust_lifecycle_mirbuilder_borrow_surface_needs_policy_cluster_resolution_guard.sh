#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-borrow-surface-needs-policy-cluster-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_borrow_surface_needs_policy_cluster_resolution.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderBorrowSurfaceNeedsPolicyClusterResolutionV1", "bad kind")
need(data.get("token") == "MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001", "bad token")

summary = data.get("summary", {})
need(summary.get("borrow_surface_needs_policy_count") == 112, "borrow count must be 112")
need(summary.get("returned_read_borrow_count") == 109, "read borrow count must be 109")
need(summary.get("returned_mutable_borrow_count") == 3, "mutable borrow count must be 3")
need(summary.get("owner_edge_confidence_none_count") == 112, "owner confidence none count must be 112")
need(summary.get("selection_eligible_cluster_count") == 0, "no cluster may be eligible before owner repair")

decision = data.get("decision", {})
need(decision.get("kind") == "SelectBorrowSurfaceOwnerEdgeConfidenceRepair", "bad decision kind")
need(decision.get("reason_token") == "BorrowSurfaceOwnerEdgeConfidenceMissingForAllCandidates", "bad reason")
need(decision.get("selected_next_card") == "MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001", "bad next card")

claims = data.get("claims", {})
for key in [
    "borrow_policy_selected",
    "mut_lease_selected",
    "owned_read_snapshot_selected_for_new_surface",
    "explicit_mutation_api_selected_for_new_surface",
    "manual_borrow_policy_selection",
    "manual_owner_edge_selection",
    "strict_rules_changed",
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

need(claims.get("report_consumed") == 1, "report_consumed must be 1")

print("output_contract=rust-lifecycle-mirbuilder-borrow-surface-needs-policy-cluster-resolution")
print(f"borrow_surface_needs_policy_count={summary.get('borrow_surface_needs_policy_count')}")
print(f"owner_edge_confidence_none_count={summary.get('owner_edge_confidence_none_count')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("borrow_policy_selected=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
