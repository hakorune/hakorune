#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-borrow-surface-policy-cluster-rerun-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_borrow_surface_policy_cluster_rerun.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderBorrowSurfacePolicyClusterRerunV1", "bad kind")
need(data.get("token") == "MIRBUILDER-BORROW-SURFACE-POLICY-CLUSTER-RERUN-001", "bad token")

pool = data.get("candidate_pool", {})
need(pool.get("selection_eligible_cluster_count") == 61, "eligible cluster count must be 61")
need(pool.get("returned_mutable_borrow_cluster_count") == 3, "mutable cluster count must be 3")
need(pool.get("returned_read_borrow_cluster_count") == 58, "read cluster count must be 58")

selected = data.get("selected_cluster") or {}
need(selected.get("borrow_kind") == "ReturnedMutableBorrow", "selected cluster must be mutable borrow")
need(selected.get("return_shape") == "mutable_ref", "selected return shape must be mutable_ref")
need(selected.get("receiver_axis") == "mutable_receiver", "selected receiver must be mutable")

decision = data.get("decision", {})
need(decision.get("kind") == "SelectBorrowProjectionPolicyCluster", "bad decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001", "bad next card")

claims = data.get("claims", {})
need(claims.get("borrow_policy_cluster_selected") == 1, "borrow policy cluster must be selected")
for key in [
    "borrow_policy_selected",
    "mut_lease_selected",
    "owned_read_snapshot_selected_for_new_surface",
    "explicit_mutation_api_selected_for_new_surface",
    "manual_borrow_policy_selection",
    "cluster_size_as_proof",
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

print("output_contract=rust-lifecycle-mirbuilder-borrow-surface-policy-cluster-rerun")
print(f"selection_eligible_cluster_count={pool.get('selection_eligible_cluster_count')}")
print(f"selected_cluster_id={decision.get('selected_cluster_id')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("borrow_policy_selected=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
