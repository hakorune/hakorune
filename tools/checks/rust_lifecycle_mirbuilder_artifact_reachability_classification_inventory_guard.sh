#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-artifact-reachability-classification-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_artifact_reachability_classification_inventory.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3344-MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

source "$ROOT/tools/checks/lib/guard_common.sh"

TAG="rust-lifecycle-mirbuilder-artifact-reachability-classification-inventory"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001"
next_card = "MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001"

need(fixture.get("kind") == "MirBuilderArtifactReachabilityClassificationInventoryV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

classes = fixture.get("artifact_classes") or {}
live = classes.get("live_fastpath") or []
compare = classes.get("proof_only_rust_bridge") or []
hako = classes.get("shadow_mirror_library") or {}
guards = classes.get("unreached_guard_ecosystem") or {}

need(len(live) == 6, "live fastpath example count drift")
need(all(row.get("exists") is True for row in live), "live fastpath example missing")
need(len(compare) == 6, "compare bridge count drift")
need(all(row.get("builder_mod_declared") is True for row in compare), "compare bridge mod drift")
need(all(row.get("production_fastpath_reference_count") == 0 for row in compare), "compare bridge fastpath drift")
need(sum(row.get("line_count", 0) for row in compare) == 974, "compare bridge line total drift")

need(hako.get("compiler_reachable_lib_count") == 0, "compiler lib reachability drift")
need(hako.get("lib_hako_count", 0) >= 198, "hako lib count unexpectedly low")
need(hako.get("guard_referenced_lib_hako_count", 0) >= 198, "hako guard mirror count unexpectedly low")

need(guards.get("script_count", 0) >= 700, "rust_lifecycle script count unexpectedly low")
for key in [
    "dev_gate_rust_lifecycle_refs",
    "dev_gate_quick_rust_lifecycle_refs",
    "guard_rows_rust_lifecycle_refs",
    "proof_apps_rust_lifecycle_refs",
    "workflow_rust_lifecycle_refs",
]:
    need(guards.get(key) == 0, f"guard auto-entry drift: {key}")

summary = fixture.get("summary") or {}
for key in [
    "artifact_reachability_classification_inventory",
    "active_guard_resolver_required",
]:
    need(summary.get(key) == 1, f"missing summary claim: {key}")
for key in [
    "compare_proof_bridge_production_connected",
    "hako_lib_compiler_reachable_count",
    "hako_mirror_library_fastpath_connected",
    "run_all_rust_lifecycle_guards_by_default",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"forbidden summary drift: {key}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectActiveGuardResolverBeforeShadowConsume", "decision kind drift")
need(decision.get("reason_token") == "ReachabilityMixedClosedWorldArtifacts", "reason drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
need(claims.get("inventory_only") == 1, "inventory claim drift")
for key in [
    "compare_bridge_deleted",
    "compare_bridge_production_connected",
    "hako_runtime_route_authority",
    "hako_backend_lowering_authority",
    "all_rust_lifecycle_guards_in_ci",
    "all_rust_lifecycle_guards_in_dev_gate",
    "rust_fastpath_rewired",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(token in task_order, "task order missing token")
need(f"selected_next_card={next_card}" in task_order, "task order next drift")

print("output_contract=rust-lifecycle-mirbuilder-artifact-reachability-classification-inventory")
print("compare_proof_bridge_file_count=6")
print("compare_proof_bridge_total_lines=974")
print("compare_proof_bridge_production_connected=0")
print("hako_lib_compiler_reachable_count=0")
print("run_all_rust_lifecycle_guards_by_default=0")
print("active_guard_resolver_required=1")
print("selected_next_card=" + next_card)
print("summary=ok")
PY
