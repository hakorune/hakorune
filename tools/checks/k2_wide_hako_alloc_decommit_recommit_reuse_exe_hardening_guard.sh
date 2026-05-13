#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-decommit-recommit-reuse-exe-hardening"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-decommit-recommit-reuse-exe-hardening-proof/main.hako"
APP_README="apps/hako-alloc-decommit-recommit-reuse-exe-hardening-proof/README.md"
APP_TEST="apps/hako-alloc-decommit-recommit-reuse-exe-hardening-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-255-M210-DECOMMIT-RECOMMIT-REUSE-EXE-HARDENING.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
DECOMMIT_GUARD="lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako"
RECOMMIT_OWNER="lang/src/hako_alloc/memory/purge_recommit_heap_integration_box.hako"
DECOMMIT_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako"
RECOMMIT_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_recommit_adapter_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
LIFECYCLE="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
REUSE_PRIORITY="lang/src/hako_alloc/memory/heap_reuse_priority_box.hako"
LIFECYCLE_STATS="lang/src/hako_alloc/memory/lifecycle_stats_observer_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard.sh"

echo "[$TAG] checking M210 decommit/recommit/reuse EXE hardening"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$CURRENT_STATE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$DECOMMIT_GUARD" \
  "$RECOMMIT_OWNER" \
  "$DECOMMIT_ADAPTER" \
  "$RECOMMIT_ADAPTER" \
  "$PAGE_SOURCE" \
  "$LIFECYCLE" \
  "$REUSE_PRIORITY" \
  "$LIFECYCLE_STATS" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M210 card must be complete"
guard_expect_in_file "$TAG" 'M210 status:' "$PLAN" "mimalloc plan must record M210 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M210 guard"
guard_expect_in_file "$TAG" 'id = "M210"' "$PROOF_MANIFEST" "proof app manifest must list M210"
guard_expect_in_file "$TAG" 'latest_card = "293x-255-M210-DECOMMIT-RECOMMIT-REUSE-EXE-HARDENING"' "$CURRENT_STATE" "current state must point at M210 as latest card"
guard_expect_in_file "$TAG" 'current_blocker_token = "M211 purge candidate policy inventory"' "$CURRENT_STATE" "current state must advance to M211"
guard_expect_in_file "$TAG" 'HakoAllocPurgeStateAwareDecommitGuard' "$APP" "M210 proof must compose duplicate guard"
guard_expect_in_file "$TAG" 'HakoAllocRecommitHeapIntegration' "$APP" "M210 proof must compose recommit integration"
guard_expect_in_file "$TAG" 'HakoAllocPageLifecycleInvariantObserver' "$APP" "M210 proof must observe lifecycle states"
guard_expect_in_file "$TAG" 'HakoAllocHeapReusePriorityPolicy' "$APP" "M210 proof must consume reuse priority"
guard_expect_in_file "$TAG" 'HakoAllocLifecycleStatsObserverSurface' "$APP" "M210 proof must consume lifecycle stats"
guard_expect_in_file "$TAG" 'check "m210 decommit recommit reuse exe hardening"' "$APP" "M210 proof must use labelled check block"

if rg -n 'hako-alloc-decommit-recommit-reuse-exe-hardening-proof|DecommitRecommitReuseExeHardening' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M210 app/name matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|provider_|install_hook|global_allocator|NYASH_|std::env|env::|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$APP" "$APP_README" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: M210 proof must not widen provider/hook/OS-release/backend vocabulary" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'k2_wide_hako_alloc_decommit_recommit_reuse_exe_hardening_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M210 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m210_exe_hardening.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m210.mir.json"
exe_out="$tmp_dir/m210.exe"
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
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
    "HakoAllocRecommitHeapIntegration.attemptHeapPage/3",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
    "HakoAllocPageSourceRecommitAdapter.commitPage/2",
    "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3",
    "HakoAllocHeapReusePriorityPolicy.selectHeapPage/2",
    "HakoAllocLifecycleStatsObserverSurface.snapshot/2",
    "HakoAllocPageModel.reactivate/0",
    "HakoAllocPageQueue.directPageId/0",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

main = functions["main"]
required_methods = {
    ("HakoAllocPurgeStateAwareDecommitGuard", "attemptHeapPage"),
    ("HakoAllocRecommitHeapIntegration", "attemptHeapPage"),
    ("HakoAllocPageLifecycleInvariantObserver", "observeHeapPage"),
    ("HakoAllocHeapReusePriorityPolicy", "selectHeapPage"),
    ("HakoAllocLifecycleStatsObserverSurface", "snapshot"),
}
seen = set()
for callee in iter_calls(main):
    if callee.get("type") == "Method":
        seen.add((callee.get("box_name"), callee.get("name")))
missing_methods = sorted(required_methods - seen)
if missing_methods:
    raise SystemExit(f"main missing hardening calls: {missing_methods}")

print("[m210-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
if rg -n 'compat_replay=harness|unsupported_pure_shape|vm_only|silent_fallback_allowed=true' \
  "$build_log" >/tmp/"$TAG".fallback 2>&1; then
  echo "[$TAG] ERROR: M210 EXE hardening must not use VM/compat/silent fallback" >&2
  cat /tmp/"$TAG".fallback >&2
  rm -f /tmp/"$TAG".fallback
  exit 1
fi
rm -f /tmp/"$TAG".fallback

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-decommit-recommit-reuse-exe-hardening-proof' "$run_log"
rg -F -q 'cycle0=1,0,1,1,0,-1,0,1,1' "$run_log"
rg -F -q 'states0=1,2,3,4' "$run_log"
rg -F -q 'priority0=4,-1,1,2,0,4' "$run_log"
rg -F -q 'cycle1=1,0,1,0,1,1' "$run_log"
rg -F -q 'states1=3,4,4' "$run_log"
rg -F -q 'priority1=2,0,4,0,0' "$run_log"
rg -F -q 'marker=2,2,0' "$run_log"
rg -F -q 'guards=3,2,1,1,2,2,2,0' "$run_log"
rg -F -q 'stats=7,1,1,2,3,3,2,1,1' "$run_log"
rg -F -q 'totals=7,3,1' "$run_log"
rg -F -q 'heap=1,1,1,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
