#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-page-model-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-page-model-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-page-model-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-page-model-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako"
FALLBACK="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-436-MIMAP-023A-FACADE-HUGE-PAGE-MODEL-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-023A facade huge-page model guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$FALLBACK" \
  "$HUGE_MODEL" \
  "$PAGE_MAP" \
  "$SIZE_CLASS" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugePageModelRoute' "$ROUTE" "MIMAP-023A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugePageModelReport' "$ROUTE" "MIMAP-023A report owner missing"
guard_expect_in_file "$TAG" 'alloc_miss_fallback: HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback' "$ROUTE" "route must preserve the MIMAP-021C fallback for non-huge requests"
guard_expect_in_file "$TAG" 'me\.huge_model = new HakoAllocHugePageModel\(me\.page_map\)' "$ROUTE" "route must reuse the M180 huge page model"
guard_expect_in_file "$TAG" 'SizeClassBox\.size_to_bin\(size\)' "$ROUTE" "route must classify with SizeClassBox"
guard_expect_in_file "$TAG" 'SizeClassBox\.huge_bin\(\)' "$ROUTE" "route must compare against huge bin"
guard_expect_in_file "$TAG" 'SizeClassBox\.bin_size\(SizeClassBox\.max_regular_bin\(\)\)' "$ROUTE" "route must use the same huge threshold as MIMAP-022B"
guard_expect_in_file "$TAG" 'me\.huge_model\.allocateHuge\(size, size\)' "$ROUTE" "huge route must delegate allocation to M180"
guard_expect_in_file "$TAG" 'me\.alloc_miss_fallback\.allocateOnMiss\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "non-huge path must forward through MIMAP-021C"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_page_model_box = "memory/object_lifecycle_facade_huge_page_model_box.hako"' "$MODULE" "hako module must export MIMAP-023A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_page_model_box.hako' "$README" "memory README must name MIMAP-023A owner"
guard_expect_in_file "$TAG" 'MIMAP-023A' "$CARD" "MIMAP-023A card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-023A guard"

if rg -n '\.lookup[[:space:]]*\(|markReleased[[:space:]]*\(|unregister[[:space:]]*\(|releaseHugePtr[[:space:]]*\(|unreserve|releasePage|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|HakoAllocHugeReleaseSeam|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-023A leaked behavior beyond facade huge-page model route" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-page-model|HakoAllocObjectLifecycleFacadeHugePageModel|object_lifecycle_facade_huge_page_model' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-023A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap023a_facade_huge_page_model.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap023a.mir.json"
exe_out="$tmp_dir/mimap023a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.birth/0",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.allocateWithHugePageModel/6",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.hugeThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.classifyRequest/1",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.initReport/1",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.routeHuge/2",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.copySmall/2",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.snapshotCounters/1",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6",
    "HakoAllocHugePageModel.allocateHuge/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute",
    "HakoAllocObjectLifecycleFacadeHugePageModelReport",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissReport",
    "HakoAllocHugePageModel",
    "HakoAllocHugePageMetaStore",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugePageModelReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "size_bin",
    "huge_threshold",
    "huge_routed",
    "huge_allocated",
    "huge_ptr",
    "huge_page_id",
    "huge_requested_size",
    "huge_committed_size",
    "huge_failure_kind",
    "huge_count",
    "huge_live_count",
    "huge_allocate_count",
    "page_map_entry_count",
    "page_map_register_count",
    "small_forwarded",
    "fallback_attempted",
    "small_source_status",
    "small_source_added_page_id",
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
    "final_ptr",
    "huge_attempt_count",
    "huge_success_count",
    "huge_failure_count",
    "small_forward_count",
    "fallback_attempt_count",
    "success_count",
    "failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-page model report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugePageModelRoute.allocateWithHugePageModel/6"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "initReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "classifyRequest")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "routeHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "copySmall")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "snapshotCounters")
require_method(route_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "allocateOnMiss")
require_method("HakoAllocObjectLifecycleFacadeHugePageModelRoute.routeHuge/2", "HakoAllocHugePageModel", "allocateHuge")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-page-model-proof' "$run_log"
rg -F -q 'huge=1,1,70000,1000' "$run_log"
rg -F -q 'huge_final=1,0,1000,0' "$run_log"
rg -F -q 'small=0,0,1,1' "$run_log"
rg -F -q 'small_source=1,602' "$run_log"
rg -F -q 'small_final=1,0,602,1' "$run_log"
rg -F -q 'route_counts=1,1,0,1,1,2,0' "$run_log"
rg -F -q 'page_map=1,1' "$run_log"
rg -F -q 'facade=1,602,1' "$run_log"
rg -F -q 'shape=6' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
