#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-page-source-alloc-miss-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako"
APP_TEST="apps/mimalloc-facade-page-source-alloc-miss-proof/test.sh"
APP_README="apps/mimalloc-facade-page-source-alloc-miss-proof/README.md"
FALLBACK="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"
ADAPTER="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-384-MIMAP-021C-FACADE-PAGE-SOURCE-ALLOC-MISS-FALLBACK.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-021C facade page-source alloc-miss fallback guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$FALLBACK" \
  "$ADAPTER" \
  "$FACADE" \
  "$REASON" \
  "$PAGE_SOURCE" \
  "$PAGE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback' "$FALLBACK" "MIMAP-021C fallback owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAllocMissReport' "$FALLBACK" "MIMAP-021C report owner missing"
guard_expect_in_file "$TAG" 'facade\.objectLifecycleSmallAlloc\(size\)' "$FALLBACK" "fallback must attempt/retry facade small allocation"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeReason\.small_no_page\(\)' "$FALLBACK" "fallback must gate only on small_no_page"
guard_expect_in_file "$TAG" 'new HakoAllocObjectLifecycleFacadePageSourceAttach\(\)' "$FALLBACK" "fallback must reuse MIMAP-021B attach adapter"
guard_expect_in_file "$TAG" 'attach\.attachFreshPage\(facade, page_id, block_size, capacity, reserved\)' "$FALLBACK" "fallback must attach one fresh page through adapter"
guard_expect_in_file "$TAG" 'fallback_attempt_count' "$FALLBACK" "fallback attempt counter missing"
guard_expect_in_file "$TAG" 'source_success_count' "$FALLBACK" "source success counter missing"
guard_expect_in_file "$TAG" 'retry_success_count' "$FALLBACK" "retry success counter missing"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_page_source_alloc_miss_box = "memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"' "$MODULE" "hako module must export MIMAP-021C fallback"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_page_source_alloc_miss_box.hako' "$README" "memory README must name MIMAP-021C owner"
guard_expect_in_file "$TAG" 'MIMAP-021C' "$CARD" "MIMAP-021C card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-021C guard"

if rg -n 'HakoAllocPageSourcePolicy|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|OsVm|OSVM|externcall' \
  "$FALLBACK" "$APP" >/tmp/"$TAG".direct_source 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C must reuse the 021B attach adapter instead of direct page-source/OSVM calls" >&2
  cat /tmp/"$TAG".direct_source >&2
  rm -f /tmp/"$TAG".direct_source
  exit 1
fi
rm -f /tmp/"$TAG".direct_source

if rg -n 'objectLifecycleRelease|objectLifecycleRealloc|objectLifecycleSmallAllocAligned|HakoAllocObjectLifecycleFacadePurge|HakoAllocBoundedPurge|HakoAllocPurgeState|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|unreserve|releasePage|PageMap|page_map|lookup[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$FALLBACK" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C leaked behavior beyond alloc-miss fallback" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-page-source-alloc-miss|HakoAllocObjectLifecycleFacadePageSourceAllocMiss|allocateOnMiss' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap021c_facade_page_source.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap021c.mir.json"
exe_out="$tmp_dir/mimap021c.exe"
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
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.snapshotCounters/1",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.copyFinalAllocation/2",
    "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocOk/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocBlockId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecyclePageCount/0",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "OsVmCoreBox.reserve_bytes_i64/1",
    "OsVmCoreBox.commit_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissReport",
    "HakoAllocObjectLifecycleFacadePageSourceAttach",
    "HakoAllocObjectLifecycleFacadePageSourceAttachReport",
    "HakoAllocObjectLifecycleFacade",
    "HakoAllocObjectLifecyclePageQueue",
    "HakoAllocPageModel",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAllocMissReport"].get("fields", [])
}
for field in (
    "status",
    "initial_ok",
    "initial_reason",
    "fallback_attempted",
    "source_status",
    "source_reserved",
    "source_committed",
    "source_reject",
    "source_added_page_id",
    "source_facade_page_count",
    "source_base",
    "source_bytes",
    "retry_ok",
    "retry_reason",
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
    "fallback_attempt_count",
    "source_success_count",
    "source_failure_count",
    "retry_success_count",
    "retry_failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing alloc-miss report field: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(fn_name, box_name, name):
    for callee in iter_calls(functions[fn_name]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn_name}")

def require_global(fn_name, symbol):
    routes = functions[fn_name].get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return
    raise SystemExit(f"missing generic-i64 route in {fn_name} -> {symbol}: {routes}")

fallback_fn = "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6"
for name in (
    "objectLifecycleSmallAlloc",
    "objectLifecycleAllocReason",
):
    require_method(fallback_fn, "HakoAllocObjectLifecycleFacade", name)
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAttach", "attachFreshPage")
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "snapshotCounters")
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "copyFinalAllocation")

copy_fn = "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.copyFinalAllocation/2"
for name in (
    "objectLifecycleAllocOk",
    "objectLifecycleAllocReason",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
):
    require_method(copy_fn, "HakoAllocObjectLifecycleFacade", name)

attach_fn = "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5"
require_global(attach_fn, "HakoAllocPageSourcePolicy.reservePage/1")
require_global(attach_fn, "HakoAllocPageSourcePolicy.commitPage/2")
require_method(attach_fn, "HakoAllocObjectLifecycleFacade", "objectLifecycleAddPage")

require_method("main", "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "allocateOnMiss")
for name in (
    "objectLifecyclePageCount",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
):
    require_method("main", "HakoAllocObjectLifecycleFacade", name)

for fn_name in (fallback_fn, "main"):
    fn = functions[fn_name]
    for callee in iter_calls(fn):
        box = callee.get("box_name") or ""
        name = callee.get("name") or ""
        target = f"{box}.{name}"
        forbidden_names = {
            "objectLifecycleReleaseBlock",
            "objectLifecycleReallocShrink",
            "objectLifecycleReallocGrow",
            "objectLifecycleSmallAllocAligned",
            "decommitPage",
            "recommitPage",
            "unreserve",
            "releasePage",
        }
        if name in forbidden_names:
            raise SystemExit(f"forbidden call in {fn_name}: {target}")
        if any(part in target for part in ("PageMap", "RemoteFree", "Purge", "Atomic", "Tls", "Provider")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")

print("[mimap021c-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"; then
  echo "[$TAG] ERROR: MIMAP-021C must not emit decommit" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-page-source-alloc-miss-proof' "$run_log"
rg -F -q 'initial=0,1' "$run_log"
rg -F -q 'fallback_attempted=1' "$run_log"
rg -F -q 'source=1,1,1,0' "$run_log"
rg -F -q 'source_page=411,1' "$run_log"
rg -F -q 'backing=' "$run_log"
rg -F -q 'retry=1,0' "$run_log"
rg -F -q 'final=1,0,411,1' "$run_log"
rg -F -q 'counts=1,1,0,1,0' "$run_log"
rg -F -q 'facade=1,411,1' "$run_log"
rg -F -q 'shape=14' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"
