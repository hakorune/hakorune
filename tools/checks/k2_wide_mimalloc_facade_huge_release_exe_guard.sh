#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-release-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-release-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-release-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-release-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako"
ALLOC_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-438-MIMAP-024A-FACADE-HUGE-RELEASE-METADATA-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-024A facade huge-release metadata guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$ALLOC_ROUTE" \
  "$HUGE_MODEL" \
  "$HUGE_STORE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeReleaseRoute' "$ROUTE" "MIMAP-024A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeReleaseReport' "$ROUTE" "MIMAP-024A report owner missing"
guard_expect_in_file "$TAG" 'alloc_route: HakoAllocObjectLifecycleFacadeHugePageModelRoute' "$ROUTE" "MIMAP-024A must reuse the MIMAP-023A route"
guard_expect_in_file "$TAG" 'me\.alloc_route\.allocateWithHugePageModel\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "MIMAP-024A must allocate through MIMAP-023A"
guard_expect_in_file "$TAG" 'me\.alloc_route\.huge_model\.markReleased\(result\.huge_ptr\)' "$ROUTE" "MIMAP-024A must release through the M180 metadata seam"
guard_expect_in_file "$TAG" 'markReleased\(ptr\)' "$HUGE_MODEL" "MIMAP-024A relies on existing M180 markReleased"
guard_expect_in_file "$TAG" 'live_flags\.set\(index, 0\)' "$HUGE_STORE" "MIMAP-024A must clear live state through C205d store"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_release_box = "memory/object_lifecycle_facade_huge_release_box.hako"' "$MODULE" "hako module must export MIMAP-024A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_release_box.hako' "$README" "memory README must name MIMAP-024A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-024A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-024A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|releaseHugePtr[[:space:]]*\(|unreserve|releasePage|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|HakoAllocHugeReleaseSeam|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-024A leaked behavior beyond facade huge-release metadata route" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-release|HakoAllocObjectLifecycleFacadeHugeRelease|object_lifecycle_facade_huge_release' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-024A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap024a_facade_huge_release.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap024a.mir.json"
exe_out="$tmp_dir/mimap024a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.allocateThenReleaseHuge/6",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.hugeReleaseThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.initReleaseReport/1",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.copyReleaseAllocation/2",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.snapshotReleaseCounters/1",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.releaseHugeMetadata/1",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.allocateWithHugePageModel/6",
    "HakoAllocHugePageModel.markReleased/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute",
    "HakoAllocObjectLifecycleFacadeHugeReleaseReport",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute",
    "HakoAllocObjectLifecycleFacadeHugePageModelReport",
    "HakoAllocHugePageModel",
    "HakoAllocHugePageMetaStore",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeReleaseReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "huge_threshold",
    "huge_routed",
    "huge_allocated",
    "huge_ptr",
    "huge_page_id",
    "huge_requested_size",
    "huge_committed_size",
    "allocation_final_ok",
    "allocation_final_reason",
    "small_forwarded",
    "fallback_attempted",
    "small_source_status",
    "small_source_added_page_id",
    "release_attempted",
    "release_ok",
    "release_ptr",
    "release_page_id",
    "release_requested_size",
    "release_committed_size",
    "release_live_before",
    "release_live_after",
    "release_failure_kind",
    "huge_count",
    "huge_live_count",
    "huge_allocate_count",
    "huge_release_count",
    "huge_release_reject_count",
    "page_map_entry_count",
    "page_map_live_count",
    "page_map_register_count",
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
    "final_ptr",
    "release_attempt_count",
    "release_success_count",
    "release_failure_count",
    "small_forward_count",
    "success_count",
    "failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-release report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.allocateThenReleaseHuge/6"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseRoute", "initReleaseReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "allocateWithHugePageModel")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseRoute", "copyReleaseAllocation")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseRoute", "releaseHugeMetadata")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseRoute", "snapshotReleaseCounters")
require_method("HakoAllocObjectLifecycleFacadeHugeReleaseRoute.releaseHugeMetadata/1", "HakoAllocHugePageModel", "markReleased")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-release-proof' "$run_log"
rg -F -q 'huge=1,1,70000,1000' "$run_log"
rg -F -q 'release=1,1,70000,1000,1,0' "$run_log"
rg -F -q 'release_meta=4194305,4194305,1,0,1,1,0' "$run_log"
rg -F -q 'huge_final=1,0,1000,0' "$run_log"
rg -F -q 'small=0,0,1,1,0' "$run_log"
rg -F -q 'small_source=1,702' "$run_log"
rg -F -q 'small_final=1,0,702,1' "$run_log"
rg -F -q 'release_counts=1,1,0,1,2,0' "$run_log"
rg -F -q 'page_map=1,1,1' "$run_log"
rg -F -q 'facade=1,702,1' "$run_log"
rg -F -q 'shape=9' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
