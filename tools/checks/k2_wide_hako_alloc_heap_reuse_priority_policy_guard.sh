#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-heap-reuse-priority-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-heap-reuse-priority-policy-proof/main.hako"
APP_README="apps/hako-alloc-heap-reuse-priority-policy-proof/README.md"
APP_TEST="apps/hako-alloc-heap-reuse-priority-policy-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-253-M208-HEAP-REUSE-PRIORITY-POLICY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
POLICY="lang/src/hako_alloc/memory/heap_reuse_priority_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
LIFECYCLE="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_heap_reuse_priority_policy_guard.sh"

echo "[$TAG] checking M208 heap reuse priority policy"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$POLICY" \
  "$MODULE" \
  "$MEMORY_README" \
  "$LIFECYCLE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M208 card must be complete"
guard_expect_in_file "$TAG" 'M208 status:' "$PLAN" "mimalloc plan must record M208 status"
guard_expect_in_file "$TAG" '`293x-253`' "$PHASE_README" "phase README must list M208 row"
guard_expect_in_file "$TAG" '\[x\] `293x-253`' "$TASKBOARD" "taskboard must mark M208 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M208 guard"
guard_expect_in_file "$TAG" 'id = "M208"' "$PROOF_MANIFEST" "proof app manifest must list M208"
guard_expect_in_file "$TAG" 'memory.heap_reuse_priority_box = "memory/heap_reuse_priority_box.hako"' "$MODULE" "hako_alloc module must export heap reuse priority box"
guard_expect_in_file "$TAG" 'box HakoAllocHeapReusePriorityPolicy' "$POLICY" "reuse priority policy box must exist"
guard_expect_in_file "$TAG" 'selectHeapPage' "$POLICY" "reuse priority policy must expose selectHeapPage"
guard_expect_in_file "$TAG" 'route: i64 = 0' "$POLICY" "decision route field must exist"
guard_expect_in_file "$TAG" 'active reuse' "$POLICY" "policy must document active route"
guard_expect_in_file "$TAG" 'observer: HakoAllocPageLifecycleInvariantObserver' "$POLICY" "policy must consume lifecycle observer facts"
guard_expect_in_file "$TAG" 'owns M208 heap reuse priority policy' "$MEMORY_README" "memory README must define M208 owner"

if rg -n 'acquire[[:space:]]*\(|releaseLocal[[:space:]]*\(|reactivate[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$POLICY" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: M208 policy must stay read-only and must not mutate pages or source memory" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'hako-alloc-heap-reuse-priority-policy-proof|HakoAllocHeapReusePriority' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M208 app/policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_heap_reuse_priority_policy_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M208 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m208_hako_alloc_reuse_priority.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m208.mir.json"
exe_out="$tmp_dir/m208.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocHeapReusePriorityPolicy.selectHeapPage/2",
    "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
    "HakoAllocRecommitHeapIntegration.attemptHeapPage/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocHeapReusePriorityDecision", "HakoAllocHeapReusePriorityPolicy"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

decision_fields = {
    field.get("name"): field
    for field in plans["HakoAllocHeapReusePriorityDecision"].get("fields", [])
}
required_fields = {
    "route",
    "page_id",
    "selected_state",
    "requires_reactivate",
    "requires_fresh_page",
    "active_candidates",
    "recommitted_candidates",
    "retired_candidates",
    "decommitted_blocked",
}
missing_fields = sorted(name for name in required_fields if name not in decision_fields)
if missing_fields:
    raise SystemExit(f"missing reuse decision fields: {missing_fields}")

print("[m208-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-heap-reuse-priority-policy-proof' "$run_log"
rg -F -q 'setup_active=1,0,0' "$run_log"
rg -F -q 'active=1,1,1,0,0,1,1,0,0' "$run_log"
rg -F -q 'setup_recommitted=1,1,0,0' "$run_log"
rg -F -q 'recommitted=2,1,4,0,0,0,1,1,0' "$run_log"
rg -F -q 'setup_retired=1' "$run_log"
rg -F -q 'retired=3,0,2,1,0,0,0,1,0' "$run_log"
rg -F -q 'setup_fresh=1,0' "$run_log"
rg -F -q 'fresh=4,-1,0,0,1,0,0,0,1' "$run_log"
rg -F -q 'policy=4,1,1,1,1,1,0,4,-1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
