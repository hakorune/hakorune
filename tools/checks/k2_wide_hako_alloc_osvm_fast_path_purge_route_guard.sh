#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-fast-path-purge-route"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-osvm-fast-path-purge-route-proof/main.hako"
APP_README="apps/hako-alloc-osvm-fast-path-purge-route-proof/README.md"
APP_TEST="apps/hako-alloc-osvm-fast-path-purge-route-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-516-MIMAP-042A-OSVM-FAST-PATH-BOUNDED-PURGE-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako"
HEAP="lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako"
DUPLICATE_GUARD="lang/src/hako_alloc/memory/purge_state_aware_decommit_box.hako"
SCHEDULER="lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh"

echo "[$TAG] checking MIMAP-042A OSVM-backed fast-path bounded purge route"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$HEAP" \
  "$DUPLICATE_GUARD" \
  "$SCHEDULER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-042A card must be landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-042A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-042A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-042A"
guard_expect_in_file "$TAG" 'memory.osvm_fast_path_purge_route_box = "memory/osvm_fast_path_purge_route_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-042A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_purge_route_box.hako` owns MIMAP-042A' "$MEMORY_README" "memory README must define MIMAP-042A owner"
guard_expect_in_file "$TAG" 'box HakoAllocOsVmFastPathPurgeRoute' "$OWNER" "MIMAP-042A owner box must exist"
guard_expect_in_file "$TAG" 'heap: HakoAllocOsVmBackedFastPathHeap' "$OWNER" "MIMAP-042A must own OSVM-backed heap handle"
guard_expect_in_file "$TAG" 'decommit_guard: HakoAllocPurgeStateAwareDecommitGuard' "$OWNER" "MIMAP-042A must own state-aware guard handle"
guard_expect_in_file "$TAG" 'scheduler: HakoAllocBoundedPurgeDecommitScheduler' "$OWNER" "MIMAP-042A must own bounded scheduler handle"
guard_expect_in_file "$TAG" 'allocate\(size\)' "$OWNER" "MIMAP-042A must expose allocation route"
guard_expect_in_file "$TAG" 'release\(handle\)' "$OWNER" "MIMAP-042A must expose release route"
guard_expect_in_file "$TAG" 'purgeOne\(max_scan_pages\)' "$OWNER" "MIMAP-042A must expose bounded purge route"
guard_expect_in_file "$TAG" 'me.scheduler.run\(me.heap, me.decommit_guard, max_scan_pages\)' "$OWNER" "MIMAP-042A must delegate purge through M212"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(' \
  "$OWNER" "$APP" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-042A route/proof must not call page-source, OSVM, unreserve, or recommit seams directly" >&2
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler|Atomic|Tls|RemoteFree|remote_free|worker_local' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: MIMAP-042A route/proof must stay provider/backend/concurrency vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-osvm-fast-path-purge-route-proof|HakoAllocOsVmFastPathPurgeRoute|osvm_fast_path_purge_route' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-042A app/route matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap042a_osvm_fast_path_purge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap042a.mir.json"
exe_out="$tmp_dir/mimap042a.exe"
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
    "HakoAllocOsVmFastPathPurgeRoute.allocate/1",
    "HakoAllocOsVmFastPathPurgeRoute.release/1",
    "HakoAllocOsVmFastPathPurgeRoute.snapshotPurge/1",
    "HakoAllocOsVmFastPathPurgeRoute.purgeOne/1",
    "HakoAllocOsVmBackedFastPathHeap.allocate/1",
    "HakoAllocOsVmBackedFastPathHeap.release/1",
    "HakoAllocBoundedPurgeDecommitScheduler.run/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in (
    "HakoAllocOsVmFastPathPurgeRoute",
    "HakoAllocOsVmBackedFastPathHeap",
    "HakoAllocPurgeStateAwareDecommitGuard",
    "HakoAllocBoundedPurgeDecommitScheduler",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocOsVmFastPathPurgeRoute"].get("fields", [])
}
for name in ("heap", "decommit_guard", "scheduler"):
    if name not in route_fields:
        raise SystemExit(f"missing route field: {name}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def call_pairs(fn_name):
    return {
        (callee.get("box_name"), callee.get("name"))
        for callee in iter_calls(functions[fn_name])
    }

expected_calls = {
    "HakoAllocOsVmFastPathPurgeRoute.allocate/1": {
        ("HakoAllocOsVmBackedFastPathHeap", "allocate"),
    },
    "HakoAllocOsVmFastPathPurgeRoute.release/1": {
        ("HakoAllocOsVmBackedFastPathHeap", "release"),
    },
    "HakoAllocOsVmFastPathPurgeRoute.purgeOne/1": {
        ("HakoAllocBoundedPurgeDecommitScheduler", "run"),
        ("HakoAllocOsVmFastPathPurgeRoute", "snapshotPurge"),
    },
}
for fn_name, expected in expected_calls.items():
    seen = call_pairs(fn_name)
    missing_calls = sorted(expected - seen)
    if missing_calls:
        raise SystemExit(f"{fn_name} missing required seam calls: {missing_calls}")

for fn_name in (
    "main",
    "HakoAllocOsVmFastPathPurgeRoute.allocate/1",
    "HakoAllocOsVmFastPathPurgeRoute.release/1",
    "HakoAllocOsVmFastPathPurgeRoute.purgeOne/1",
):
    for callee in iter_calls(functions[fn_name]):
        if callee.get("name") in {
            "reservePage",
            "commitPage",
            "decommitPage",
            "unreserve",
            "releasePage",
            "attemptHeapPage",
        } and callee.get("box_name") in {
            "HakoAllocPageSourcePolicy",
            "OsVmCoreBox",
            "HakoAllocPurgeHeapDecommitIntegration",
            "HakoAllocPurgeStateAwareDecommitGuard",
        }:
            raise SystemExit(f"MIMAP-042A route/proof bypassed scheduler seam: {callee}")

print("[mimap042a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-osvm-fast-path-purge-route-proof' "$run_log"
rg -F -q 'alloc=1,1,1,1' "$run_log"
rg -F -q 'first=0,0,1,1,1,1,0' "$run_log"
rg -F -q 'duplicate=1,2,1,1,0,1,1' "$run_log"
rg -F -q 'route=2,1,1,1,2' "$run_log"
rg -F -q 'heap=1,1,0,1,1' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
