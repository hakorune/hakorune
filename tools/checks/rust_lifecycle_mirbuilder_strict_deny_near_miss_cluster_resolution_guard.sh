#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-deny-near-miss-cluster-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_deny_near_miss_cluster_resolution.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderStrictDenyNearMissClusterResolutionV1", "bad kind")
need(data.get("token") == "MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001", "bad token")

pool = data.get("candidate_pool", {})
need(pool.get("eligible_near_miss_cluster_count") == 54, "unexpected eligible near-miss count")
need(pool.get("excluded_existing_descriptor_cluster_count") == 54, "unexpected excluded descriptor count")
need(pool.get("unclosed_near_miss_cluster_count") == 0, "unclosed near-miss clusters must be 0")

decision = data.get("decision", {})
need(decision.get("kind") == "KeepStopped", "decision must keep stopped")
need(decision.get("reason_token") == "NoUnclosedNearMissProjectionPolicyCluster", "bad reason")
need(decision.get("selected_next_card") == "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001", "bad next card")

claims = data.get("claims", {})
for key in [
    "manual_cluster_selection",
    "cluster_size_as_proof",
    "strict_rules_changed",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
]:
    need(claims.get(key) == 0, f"{key} must be 0")

need(claims.get("near_miss_probe_consumed") == 1, "near_miss_probe_consumed must be 1")
need(claims.get("projection_descriptor_ledger_consumed") == 1, "projection descriptor ledger not consumed")

print("output_contract=rust-lifecycle-mirbuilder-strict-deny-near-miss-cluster-resolution")
print(f"eligible_near_miss_cluster_count={pool.get('eligible_near_miss_cluster_count')}")
print(f"unclosed_near_miss_cluster_count={pool.get('unclosed_near_miss_cluster_count')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("strict_rules_changed=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
