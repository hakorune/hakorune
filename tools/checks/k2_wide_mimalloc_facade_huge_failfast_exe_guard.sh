#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-failfast-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-failfast-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-failfast-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_failfast_box.hako"
FALLBACK="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-434-MIMAP-022B-FACADE-HUGE-REQUEST-FAILFAST-ROUTING.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-022B facade huge-request fail-fast guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$FALLBACK" \
  "$FACADE" \
  "$REASON" \
  "$SIZE_CLASS" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeFailfastRoute' "$ROUTE" "MIMAP-022B route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeFailfastReport' "$ROUTE" "MIMAP-022B report owner missing"
guard_expect_in_file "$TAG" 'SizeClassBox\.size_to_bin\(size\)' "$ROUTE" "route must classify with SizeClassBox"
guard_expect_in_file "$TAG" 'SizeClassBox\.huge_bin\(\)' "$ROUTE" "route must compare against huge bin"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeReason\.small_huge_request\(\)' "$ROUTE" "route must use facade reason SSOT for huge rejection"
guard_expect_in_file "$TAG" 'alloc_miss_fallback\.allocateOnMiss\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "non-huge path must forward to MIMAP-021C fallback"
guard_expect_in_file "$TAG" 'small_huge_request' "$REASON" "facade reason SSOT must name huge request rejection"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_failfast_box = "memory/object_lifecycle_facade_huge_failfast_box.hako"' "$MODULE" "hako module must export MIMAP-022B route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_failfast_box.hako' "$README" "memory README must name MIMAP-022B owner"
guard_expect_in_file "$TAG" 'MIMAP-022B' "$CARD" "MIMAP-022B card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-022B guard"

if rg -n 'HakoAllocPageSourcePolicy|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|OsVm|OSVM|externcall' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_source 2>&1; then
  echo "[$TAG] ERROR: MIMAP-022B must reach page-source only through the 021C fallback" >&2
  cat /tmp/"$TAG".direct_source >&2
  rm -f /tmp/"$TAG".direct_source
  exit 1
fi
rm -f /tmp/"$TAG".direct_source

if rg -n 'objectLifecycleRelease|objectLifecycleRealloc|objectLifecycleSmallAllocAligned|HakoAllocObjectLifecycleFacadePurge|HakoAllocBoundedPurge|HakoAllocPurgeState|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|unreserve|releasePage|PageMap|page_map|lookup[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|HugePage' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-022B leaked behavior beyond huge fail-fast routing" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-failfast|HakoAllocObjectLifecycleFacadeHugeFailfast|objectLifecycleFacadeHugeFailfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-022B matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap022b_facade_huge_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap022b.mir.json"
exe_out="$tmp_dir/mimap022b.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.allocateWithHugeFailfast/6",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.hugeThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.classifyRequest/1",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.initReport/1",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.rejectHuge/1",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.copyFallback/2",
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.snapshotCounters/1",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecyclePageCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocBlockId/0",
    "SizeClassBox.size_to_bin/1",
    "SizeClassBox.huge_bin/0",
    "SizeClassBox.max_regular_bin/0",
    "SizeClassBox.bin_size/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeFailfastRoute",
    "HakoAllocObjectLifecycleFacadeHugeFailfastReport",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissReport",
    "HakoAllocObjectLifecycleFacadePageSourceAttach",
    "HakoAllocObjectLifecycleFacade",
    "HakoAllocObjectLifecyclePageQueue",
    "HakoAllocPageModel",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeFailfastReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "size_bin",
    "huge_threshold",
    "huge_rejected",
    "small_forwarded",
    "fallback_attempted",
    "initial_ok",
    "initial_reason",
    "source_status",
    "source_reserved",
    "source_committed",
    "source_reject",
    "source_added_page_id",
    "source_facade_page_count",
    "retry_ok",
    "retry_reason",
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
    "huge_reject_count",
    "small_forward_count",
    "fallback_attempt_count",
    "success_count",
    "failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge fail-fast report field: {field}")

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

def require_global_generic_i64(fn_name, symbol):
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

def require_global_scalar(fn_name, symbol):
    routes = functions[fn_name].get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("return_shape") == "ScalarI64"
            and route.get("emit_kind") == "direct_function_call"
        ):
            return
    raise SystemExit(f"missing scalar route in {fn_name} -> {symbol}: {routes}")

route_fn = "HakoAllocObjectLifecycleFacadeHugeFailfastRoute.allocateWithHugeFailfast/6"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "initReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "classifyRequest")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "rejectHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "copyFallback")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "snapshotCounters")
require_method(route_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "allocateOnMiss")

require_global_generic_i64("HakoAllocObjectLifecycleFacadeHugeFailfastRoute.classifyRequest/1", "SizeClassBox.size_to_bin/1")
require_global_scalar("HakoAllocObjectLifecycleFacadeHugeFailfastRoute.classifyRequest/1", "SizeClassBox.huge_bin/0")
require_global_generic_i64("HakoAllocObjectLifecycleFacadeHugeFailfastRoute.initReport/1", "SizeClassBox.size_to_bin/1")
require_global_scalar("HakoAllocObjectLifecycleFacadeHugeFailfastRoute.hugeThreshold/0", "SizeClassBox.max_regular_bin/0")
require_global_generic_i64("HakoAllocObjectLifecycleFacadeHugeFailfastRoute.hugeThreshold/0", "SizeClassBox.bin_size/1")

require_method("main", "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "hugeThreshold")
require_method("main", "HakoAllocObjectLifecycleFacadeHugeFailfastRoute", "allocateWithHugeFailfast")
for name in (
    "objectLifecyclePageCount",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
):
    require_method("main", "HakoAllocObjectLifecycleFacade", name)

for fn_name in (route_fn, "main"):
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
        if any(part in target for part in ("PageMap", "RemoteFree", "Purge", "Atomic", "Tls", "HugePage")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")

print("[mimap022b-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"; then
  echo "[$TAG] ERROR: MIMAP-022B must not emit decommit" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-failfast-proof' "$run_log"
rg -F -q 'huge=1,0,0,0,6' "$run_log"
rg -F -q 'huge_source=0,0,0,0' "$run_log"
rg -F -q 'huge_size=' "$run_log"
rg -F -q 'small=0,1,1' "$run_log"
rg -F -q 'small_source=1,1,1,0' "$run_log"
rg -F -q 'small_final=1,0,502,1' "$run_log"
rg -F -q 'route_counts=1,1,1,1,1' "$run_log"
rg -F -q 'facade=1,502,1' "$run_log"
rg -F -q 'shape=18' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"
