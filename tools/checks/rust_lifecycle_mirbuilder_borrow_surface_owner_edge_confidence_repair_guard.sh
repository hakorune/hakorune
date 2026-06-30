#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-borrow-surface-owner-edge-confidence-repair-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_borrow_surface_owner_edge_confidence_repair.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderBorrowSurfaceOwnerEdgeConfidenceRepairV1", "bad kind")
need(data.get("token") == "MIRBUILDER-BORROW-SURFACE-OWNER-EDGE-CONFIDENCE-REPAIR-001", "bad token")

summary = data.get("summary", {})
need(summary.get("input_borrow_surface_candidate_count") == 112, "input borrow count must be 112")
need(summary.get("repaired_candidate_count") == 112, "repaired candidate count must be 112")
need(summary.get("repaired_cluster_count") == 61, "repaired cluster count must be 61")
need(summary.get("file_scoped_owner_edge_count") == 46, "file scoped owner count must be 46")
need(summary.get("selection_eligible_for_borrow_policy_count") == 61, "all clusters must become policy-eligible")

for row in data.get("repaired_clusters", []):
    need(row.get("old_owner_edge_confidence") == "None", "old confidence must be None")
    need(row.get("repaired_owner_edge_confidence") == "FileScoped", "new confidence must be FileScoped")
    need(row.get("selection_eligible_for_borrow_policy") is True, "cluster must be policy eligible after repair")
    need(str(row.get("repaired_owner_edge_id", "")).startswith("mirbuilder::borrow_surface::"), "bad owner edge")

decision = data.get("decision", {})
need(decision.get("kind") == "SelectBorrowSurfacePolicyClusterRerun", "bad decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001", "bad next card")

claims = data.get("claims", {})
for key in [
    "manual_owner_edge_selection",
    "borrow_policy_selected",
    "mut_lease_selected",
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

need(claims.get("owner_edge_confidence_repaired") == 1, "owner repair must be 1")

print("output_contract=rust-lifecycle-mirbuilder-borrow-surface-owner-edge-confidence-repair")
print(f"repaired_candidate_count={summary.get('repaired_candidate_count')}")
print(f"file_scoped_owner_edge_count={summary.get('file_scoped_owner_edge_count')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("borrow_policy_selected=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
