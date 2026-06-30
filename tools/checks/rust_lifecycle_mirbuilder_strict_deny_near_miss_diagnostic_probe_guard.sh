#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-deny-near-miss-diagnostic-probe-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_strict_deny_near_miss_diagnostic_probe.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys

path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

need(data.get("kind") == "MirBuilderStrictDenyNearMissDiagnosticProbeV1", "bad kind")
need(data.get("token") == "MIRBUILDER-STRICT-DENY-NEAR-MISS-DIAGNOSTIC-PROBE-001", "bad token")

rules = data.get("rules", {})
need(rules.get("strict_classification_remains_authority") == 1, "strict authority not preserved")
need(rules.get("diagnostic_relaxed_mode_only") == 1, "not diagnostic-only")
need(rules.get("hako_emission") == 0, "hako emission must be forbidden")
need(rules.get("hako_adoption_decision") == 0, "hako adoption must be forbidden")
need(rules.get("source_selfhost_claim") == 0, "source selfhost claim must be forbidden")
need(rules.get("runtime_fallback") == 0, "runtime fallback must be forbidden")

summary = data.get("cluster_summary", {})
need(summary.get("needs_projection_policy_only_count", 0) > 0, "near-miss projection policy count missing")
need(summary.get("selection_eligible_cluster_count", 0) > 0, "selection eligible clusters missing")

decision = data.get("decision", {})
need(decision.get("kind") == "SelectNearMissClusterResolution", "unexpected decision kind")
need(decision.get("selected_next_card") == "MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001", "bad next card")

claims = data.get("claims", {})
for key in [
    "strict_rules_changed",
    "manual_family_selection",
    "manual_shape_selection",
    "manual_axis_selection",
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
    "generated_artifact_as_native_edit_authority",
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

need(claims.get("diagnostic_probe_only") == 1, "diagnostic_probe_only must be 1")

print("output_contract=rust-lifecycle-mirbuilder-strict-deny-near-miss-diagnostic-probe")
print(f"needs_projection_policy_only_count={summary.get('needs_projection_policy_only_count')}")
print(f"selection_eligible_cluster_count={summary.get('selection_eligible_cluster_count')}")
print(f"selected_next_card={decision.get('selected_next_card')}")
print("strict_rules_changed=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("summary=ok")
PY
