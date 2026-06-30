#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-converter-completion-task-inventory-v0.json"

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderConverterCompletionTaskInventoryV1", "bad kind")
need(data.get("token") == "MIRBUILDER-CONVERTER-COMPLETION-TASK-INVENTORY-001", "bad token")

counts = data.get("diagnostic_counts", {})
need(counts.get("unconverted_surface_count") == 1584, "unexpected unconverted count")
need(counts.get("borrow_surface_needs_policy_count") == 112, "borrow count must be 112")
need(counts.get("needs_multiple_diagnostic_axes_count") == 185, "multi-axis count must be 185")
need(counts.get("unclosed_near_miss_projection_policy_cluster_count") == 0, "near-miss clusters must be closed")

order = data.get("recommended_order", [])
need(len(order) == 5, "recommended order must have 5 entries")
need(order[0].get("token") == "MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001", "bad first task")
need(all(not row.get("opens_hako_generation") for row in order), "inventory tasks must not open Hako generation")

decision = data.get("decision", {})
need(decision.get("kind") == "SelectBorrowSurfaceNeedsPolicyClusterResolution", "bad decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-BORROW-SURFACE-NEEDS-POLICY-CLUSTER-RESOLUTION-001", "bad next card")

claims = data.get("claims", {})
for key in [
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
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

print("output_contract=rust-lifecycle-mirbuilder-converter-completion-task-inventory")
print(f"borrow_surface_needs_policy_count={counts.get('borrow_surface_needs_policy_count')}")
print(f"needs_multiple_diagnostic_axes_count={counts.get('needs_multiple_diagnostic_axes_count')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
