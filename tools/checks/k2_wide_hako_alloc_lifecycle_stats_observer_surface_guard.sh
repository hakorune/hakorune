#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-lifecycle-stats-observer-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-lifecycle-stats-observer-surface-proof/main.hako"
APP_README="apps/hako-alloc-lifecycle-stats-observer-surface-proof/README.md"
APP_TEST="apps/hako-alloc-lifecycle-stats-observer-surface-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-254-M209-LIFECYCLE-STATS-OBSERVER-SURFACE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/lifecycle_stats_observer_box.hako"
LIFECYCLE="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
REUSE_POLICY="lang/src/hako_alloc/memory/heap_reuse_priority_box.hako"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh"

echo "[$TAG] checking M209 lifecycle stats observer surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$LIFECYCLE" \
  "$REUSE_POLICY" \
  "$CURRENT_STATE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M209 card must be complete"
guard_expect_in_file "$TAG" 'M209 status:' "$PLAN" "mimalloc plan must record M209 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M209 guard"
guard_expect_in_file "$TAG" 'id = "M209"' "$PROOF_MANIFEST" "proof app manifest must list M209"
guard_expect_in_file "$TAG" 'latest_card = "293x-254-M209-LIFECYCLE-STATS-OBSERVER-SURFACE"' "$CURRENT_STATE" "current state must point at M209 as latest card"
guard_expect_in_file "$TAG" 'current_blocker_token = "M210 decommit/recommit/reuse EXE hardening"' "$CURRENT_STATE" "current state must advance to M210"
guard_expect_in_file "$TAG" 'memory.lifecycle_stats_observer_box = "memory/lifecycle_stats_observer_box.hako"' "$MODULE" "hako_alloc module must export M209 owner"
guard_expect_in_file "$TAG" 'box HakoAllocLifecycleStatsSnapshot' "$OWNER" "M209 snapshot box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocLifecycleStatsObserverSurface' "$OWNER" "M209 surface box must exist"
guard_expect_in_file "$TAG" 'snapshot' "$OWNER" "M209 surface must expose snapshot"
guard_expect_in_file "$TAG" 'totalObservedStates' "$OWNER" "M209 snapshot must expose observed state total"
guard_expect_in_file "$TAG" 'totalReusePicks' "$OWNER" "M209 snapshot must expose reuse pick total"
guard_expect_in_file "$TAG" 'totalBlockedOrMissing' "$OWNER" "M209 snapshot must expose blocked/missing total"
guard_expect_in_file "$TAG" 'owns M209 lifecycle stats observer surface' "$MEMORY_README" "memory README must define M209 owner"
guard_expect_in_file "$TAG" 'HakoAllocLifecycleStatsObserverSurface' "$APP" "M209 proof must construct stats surface"
guard_expect_in_file "$TAG" 'check "m209 lifecycle stats observer"' "$APP" "M209 proof must use labelled check block"

if rg -n 'observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|acquire[[:space:]]*\(|releaseLocal[[:space:]]*\(|reactivate[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: M209 stats surface must stay read-only and must not observe, select, mutate pages, or source memory" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$OWNER" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: M209 stats owner must stay options/provider/backend vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-lifecycle-stats-observer-surface-proof|HakoAllocLifecycleStats' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M209 app/stats matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M209 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m209_lifecycle_stats.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m209.mir.json"
exe_out="$tmp_dir/m209.exe"
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
    "HakoAllocLifecycleStatsObserverSurface.snapshot/2",
    "HakoAllocLifecycleStatsSnapshot.totalObservedStates/0",
    "HakoAllocLifecycleStatsSnapshot.totalReusePicks/0",
    "HakoAllocLifecycleStatsSnapshot.totalBlockedOrMissing/0",
    "HakoAllocHeapReusePriorityPolicy.selectHeapPage/2",
    "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocLifecycleStatsSnapshot", "HakoAllocLifecycleStatsObserverSurface"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

snapshot_fields = {
    field.get("name"): field
    for field in plans["HakoAllocLifecycleStatsSnapshot"].get("fields", [])
}
required_fields = {
    "observe_count",
    "missing_count",
    "active_count",
    "retired_count",
    "decommitted_count",
    "recommitted_count",
    "last_page_id",
    "last_state",
    "select_count",
    "active_pick_count",
    "recommitted_pick_count",
    "retired_pick_count",
    "fresh_pick_count",
    "decommitted_skip_count",
    "missing_skip_count",
    "last_route",
    "last_selected_page_id",
}
missing_fields = sorted(name for name in required_fields if name not in snapshot_fields)
if missing_fields:
    raise SystemExit(f"missing lifecycle stats fields: {missing_fields}")

for name in required_fields:
    field = snapshot_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad lifecycle stats field {name}: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

snapshot_fn = functions["HakoAllocLifecycleStatsObserverSurface.snapshot/2"]
for callee in iter_calls(snapshot_fn):
    name = callee.get("name")
    box_name = callee.get("box_name")
    if name in {"observeHeapPage", "selectHeapPage", "acquire", "releaseLocal", "attemptHeapPage"}:
        raise SystemExit(f"snapshot must not call behavior method: {box_name}.{name}")

print("[m209-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-lifecycle-stats-observer-surface-proof' "$run_log"
rg -F -q 'lifecycle=6,0,1,2,1,2,0,3' "$run_log"
rg -F -q 'reuse=4,1,1,1,1,1,0,4,-1' "$run_log"
rg -F -q 'totals=6,4,1' "$run_log"
rg -F -q 'policy=4,1,1,1,1,1,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
