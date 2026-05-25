#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-huge-osvm-slice"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako"
APP_TEST="apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/test.sh"
APP_README="apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/README.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
CARD="docs/development/current/main/phases/phase-294x/294x-58-MIMALLOC-COMPARISON-HUGE-OSVM-SLICE-PILOT.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh"

ROUTER="lang/src/hako_alloc/memory/huge_threshold_router_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_RELEASE="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
OSVM_HEAP="lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako"
OSVM_ROUTE="lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako"
RECOMMIT="lang/src/hako_alloc/memory/purge_recommit_heap_integration_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"

echo "[$TAG] checking hako_alloc mimalloc comparison huge/OSVM slice"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$ROUTER" \
  "$HUGE_MODEL" \
  "$HUGE_RELEASE" \
  "$OSVM_HEAP" \
  "$OSVM_ROUTE" \
  "$RECOMMIT" \
  "$PAGE_SOURCE"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'workload=huge-osvm-v1' "$APP" "V4 proof app must publish the huge/OSVM workload id"
guard_expect_in_file "$TAG" 'summary_fields=' "$APP" "V4 proof app must publish stable comparison summary fields"
guard_expect_in_file "$TAG" 'HakoAllocHugeThresholdRouter' "$APP" "V4 proof must consume the huge threshold router"
guard_expect_in_file "$TAG" 'HakoAllocHugePageModel' "$APP" "V4 proof must consume the huge page model"
guard_expect_in_file "$TAG" 'HakoAllocHugeReleaseSeam' "$APP" "V4 proof must consume the huge release seam"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathPurgeRoute' "$APP" "V4 proof must enter OSVM-backed alloc/free through the existing route owner"
guard_expect_in_file "$TAG" 'HakoAllocOsVmBackedFastPathHeap' "$APP" "V4 proof may observe the existing OSVM-backed heap"
guard_expect_in_file "$TAG" 'HakoAllocRecommitHeapIntegration' "$APP" "V4 proof must keep page-lifecycle verifier imports complete"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-006' "$CARD" "card must identify the V4 blocker token"
guard_expect_in_file "$TAG" 'V4' "$TASKBOARD" "taskboard must track V4"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$APP" "$INDEX" "check script index must list the V4 proof app"

if rg -n 'remote_free|RemoteFree|Tls|TLS|worker_local|Atomic|fetch_add|cas_|load_ordered|store_ordered|provider|hook|replacement|global_allocator|hako_mem_|externcall' \
  "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: huge/OSVM comparison slice leaked beyond V4 stop lines" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-mimalloc-comparison-huge-osvm-slice|huge-osvm-v1|HakoAllocMimallocComparisonHugeOsVm' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: huge/OSVM comparison slice leaked app/owner matcher into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_comparison_huge_osvm.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/v4.mir.json"
exe_out="$tmp_dir/v4.exe"
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
    "HakoAllocHugeThresholdRouter.allocateAlignedUsize/2",
    "HakoAllocHugeThresholdRouter.classifyAlignedRequestUsize/2",
    "HakoAllocHugePageModel.allocateHuge/2",
    "HakoAllocHugeReleaseSeam.releaseHugePtr/1",
    "HakoAllocOsVmFastPathPurgeRoute.allocate/1",
    "HakoAllocOsVmFastPathPurgeRoute.release/1",
    "HakoAllocOsVmBackedFastPathHeap.addFreshPage/0",
    "HakoAllocOsVmBackedFastPathHeap.pageBase/1",
    "HakoAllocOsVmBackedFastPathHeap.pageBackingBytes/1",
    "HakoAllocOsVmBackedFastPathHeap.decommitAll/0",
    "HakoAllocOsVmBackedFastPathHeap.requestedBytes/0",
    "HakoAllocOsVmBackedFastPathHeap.outstandingBlocks/0",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.reserve_bytes_usize/1",
    "OsVmCoreBox.commit_bytes_usize/2",
    "OsVmCoreBox.decommit_bytes_usize/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in (
    "HakoAllocHugeThresholdRouter",
    "HakoAllocHugePageModel",
    "HakoAllocHugeReleaseSeam",
    "HakoAllocOsVmBackedFastPathHeap",
    "HakoAllocOsVmBackedHandle",
    "HakoAllocOsVmPageBacking",
):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

def require_storage(box_name, field_name, declared, storage):
    fields = {field.get("name"): field for field in plans[box_name].get("fields", [])}
    field = fields.get(field_name)
    if field is None or field.get("declared_type") != declared or field.get("storage") != storage:
        raise SystemExit(f"{box_name}.{field_name} expected {declared}/{storage}, got {field}")

for name in (
    "small_route_count",
    "small_success_count",
    "small_reject_count",
    "huge_route_count",
    "huge_reject_count",
    "invalid_alignment_count",
    "invalid_size_count",
    "reject_count",
):
    require_storage("HakoAllocHugeThresholdRouter", name, "usize", "usize")
for name in (
    "last_padded_size",
    "last_huge_threshold",
):
    require_storage("HakoAllocHugeThresholdRouter", name, "usize", "usize")
for name in (
    "last_route_kind",
    "last_result_ptr",
    "last_good_size",
):
    require_storage("HakoAllocHugeThresholdRouter", name, "i64", "i64")

for name in (
    "huge_count",
    "live_count",
    "allocate_count",
    "release_count",
    "release_reject_count",
    "zero_reject_count",
    "commit_reject_count",
    "register_fail_count",
    "reject_count",
):
    require_storage("HakoAllocHugePageModel", name, "usize", "usize")
for name in (
    "last_requested_size",
    "last_committed_size",
):
    require_storage("HakoAllocHugePageModel", name, "usize", "usize")
for name in (
    "next_ptr",
    "last_result_ptr",
    "last_page_id",
    "last_failure_kind",
):
    require_storage("HakoAllocHugePageModel", name, "i64", "i64")

for name in (
    "release_count",
    "unregister_count",
    "lookup_miss_count",
    "not_huge_count",
    "model_reject_count",
    "reject_count",
):
    require_storage("HakoAllocHugeReleaseSeam", name, "usize", "usize")

for name in (
    "block_size",
    "page_capacity",
    "alloc_count",
    "release_count",
    "fallback_count",
    "page_create_count",
    "reject_count",
    "reserve_count",
    "commit_count",
    "decommit_count",
    "source_reject_count",
):
    require_storage("HakoAllocOsVmBackedFastPathHeap", name, "usize", "usize")

for name in ("bin", "next_page_id"):
    require_storage("HakoAllocOsVmBackedFastPathHeap", name, "i64", "i64")
require_storage("HakoAllocOsVmBackedFastPathHeap", "backing_count", "usize", "usize")

require_storage("HakoAllocOsVmBackedHandle", "requested_size", "usize", "usize")
require_storage("HakoAllocOsVmBackedHandle", "page_id", "i64", "i64")
require_storage("HakoAllocOsVmBackedHandle", "block_id", "i64", "i64")
require_storage("HakoAllocOsVmPageBacking", "bytes", "usize", "usize")
require_storage("HakoAllocOsVmPageBacking", "page_id", "i64", "i64")
require_storage("HakoAllocOsVmPageBacking", "base", "i64", "i64")

def require_method(owner_name, box_name, method, return_shape):
    routes = functions[owner_name].get("metadata", {}).get("lowering_plan", [])
    for route in routes:
        if (
            route.get("route_kind") == "user_box.method"
            and route.get("box_name") == box_name
            and route.get("method") == method
            and route.get("target_body_supported") is True
            and route.get("return_shape") == return_shape
        ):
            return
    raise SystemExit(f"missing method route in {owner_name}: {box_name}.{method} -> {return_shape}")

for box_name, method in (
    ("HakoAllocHugeThresholdRouter", "allocateAlignedUsize"),
    ("HakoAllocHugeThresholdRouter", "classifyAlignedRequestUsize"),
    ("HakoAllocHugePageModel", "allocateHuge"),
    ("HakoAllocHugeReleaseSeam", "releaseHugePtr"),
    ("HakoAllocOsVmFastPathPurgeRoute", "release"),
    ("HakoAllocOsVmBackedFastPathHeap", "addFreshPage"),
    ("HakoAllocOsVmBackedFastPathHeap", "pageBase"),
    ("HakoAllocOsVmBackedFastPathHeap", "pageBackingBytes"),
    ("HakoAllocOsVmBackedFastPathHeap", "decommitAll"),
    ("HakoAllocOsVmBackedFastPathHeap", "requestedBytes"),
    ("HakoAllocOsVmBackedFastPathHeap", "outstandingBlocks"),
):
    require_method("main", box_name, method, "scalar_i64")

print("[huge-osvm-slice-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-mimalloc-comparison-huge-osvm-slice-proof' "$run_log"
rg -F -q 'workload=huge-osvm-v1' "$run_log"
rg -F -q 'route=1,1,1,12000,2,0,2,1,1' "$run_log"
rg -F -q 'huge=1,70000,1000,4194305,4194305,1,1,0,0' "$run_log"
rg -F -q 'osvm=1,64,64,2,2,2,0,1' "$run_log"
rg -F -q 'summary_fields=4194321,4194433,0,2,6,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
