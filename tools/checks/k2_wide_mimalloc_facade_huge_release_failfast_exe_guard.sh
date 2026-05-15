#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-release-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-release-failfast-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-release-failfast-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-release-failfast-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_failfast_box.hako"
RELEASE_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-440-MIMAP-025A-FACADE-HUGE-RELEASE-FAILFAST-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-025A facade huge-release fail-fast guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$RELEASE_ROUTE" \
  "$HUGE_MODEL" \
  "$HUGE_STORE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute' "$ROUTE" "MIMAP-025A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeReleaseFailfastReport' "$ROUTE" "MIMAP-025A report owner missing"
guard_expect_in_file "$TAG" 'release_route: HakoAllocObjectLifecycleFacadeHugeReleaseRoute' "$ROUTE" "MIMAP-025A must reuse the MIMAP-024A route"
guard_expect_in_file "$TAG" 'me\.release_route\.allocateThenReleaseHuge\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "MIMAP-025A must start from MIMAP-024A"
guard_expect_in_file "$TAG" 'me\.release_route\.alloc_route\.huge_model\.markReleased\(result\.first_ptr\)' "$ROUTE" "MIMAP-025A must reject double release through M180"
guard_expect_in_file "$TAG" 'me\.release_route\.alloc_route\.huge_model\.markReleased\(stale_ptr\)' "$ROUTE" "MIMAP-025A must reject stale release through M180"
guard_expect_in_file "$TAG" 'markReleased\(ptr\)' "$HUGE_MODEL" "MIMAP-025A relies on existing M180 markReleased"
guard_expect_in_file "$TAG" 'live_flags\.set\(index, 0\)' "$HUGE_STORE" "MIMAP-025A must keep C205d live-state clearing owner"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_release_failfast_box = "memory/object_lifecycle_facade_huge_release_failfast_box.hako"' "$MODULE" "hako module must export MIMAP-025A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_release_failfast_box.hako' "$README" "memory README must name MIMAP-025A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-025A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-025A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|releaseHugePtr[[:space:]]*\(|unreserve|releasePage|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|HakoAllocHugeReleaseSeam|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-025A leaked behavior beyond facade huge-release fail-fast diagnostics" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-release-failfast|HakoAllocObjectLifecycleFacadeHugeReleaseFailfast|object_lifecycle_facade_huge_release_failfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-025A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap025a_facade_huge_release_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap025a.mir.json"
exe_out="$tmp_dir/mimap025a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.proveHugeReleaseFailfast/7",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.hugeReleaseFailfastThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.initFailfastReport/1",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.copyFirstReleaseResult/2",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.snapshotFailfastCounters/1",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.rejectDoubleReleasedPtr/1",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.rejectStaleHugePtr/2",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute.allocateThenReleaseHuge/6",
    "HakoAllocHugePageModel.markReleased/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute",
    "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastReport",
    "HakoAllocObjectLifecycleFacadeHugeReleaseRoute",
    "HakoAllocObjectLifecycleFacadeHugeReleaseReport",
    "HakoAllocHugePageModel",
    "HakoAllocHugePageMetaStore",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeReleaseFailfastReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "huge_threshold",
    "first_release_ok",
    "first_ptr",
    "first_page_id",
    "first_live_after",
    "first_release_count",
    "double_attempted",
    "double_ok",
    "double_ptr",
    "double_live_before",
    "double_live_after",
    "double_failure_kind",
    "stale_attempted",
    "stale_ok",
    "stale_ptr",
    "stale_live_before",
    "stale_live_after",
    "stale_failure_kind",
    "huge_count",
    "huge_live_count",
    "huge_allocate_count",
    "huge_release_count",
    "huge_release_reject_count",
    "huge_reject_count",
    "page_map_entry_count",
    "page_map_live_count",
    "page_map_register_count",
    "failfast_attempt_count",
    "double_reject_count",
    "stale_reject_count",
    "success_count",
    "failure_count",
    "final_ok",
    "final_reason",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-release failfast report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.proveHugeReleaseFailfast/7"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute", "initFailfastReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseRoute", "allocateThenReleaseHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute", "copyFirstReleaseResult")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute", "rejectDoubleReleasedPtr")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute", "rejectStaleHugePtr")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute", "snapshotFailfastCounters")
require_method("HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.rejectDoubleReleasedPtr/1", "HakoAllocHugePageModel", "markReleased")
require_method("HakoAllocObjectLifecycleFacadeHugeReleaseFailfastRoute.rejectStaleHugePtr/2", "HakoAllocHugePageModel", "markReleased")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-release-failfast-proof' "$run_log"
rg -F -q 'first=1,70000,1000,0,1' "$run_log"
rg -F -q 'double=1,0,70000,0,0,1' "$run_log"
rg -F -q 'stale=1,0,99999,0,0,2' "$run_log"
rg -F -q 'huge=1,0,1,1,2,2' "$run_log"
rg -F -q 'page_map=1,1,1' "$run_log"
rg -F -q 'route_counts=2,1,1,1,0' "$run_log"
rg -F -q 'final=1,1,0' "$run_log"
rg -F -q 'shape=7' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
