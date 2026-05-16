#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-unregister-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-unregister-failfast-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-unregister-failfast-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-unregister-failfast-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_failfast_box.hako"
UNREGISTER_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_box.hako"
RELEASE_SEAM="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-444-MIMAP-027A-FACADE-HUGE-UNREGISTER-FAILFAST-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-027A facade huge-unregister fail-fast guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$UNREGISTER_ROUTE" \
  "$RELEASE_SEAM" \
  "$HUGE_MODEL" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute' "$ROUTE" "MIMAP-027A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastReport' "$ROUTE" "MIMAP-027A report owner missing"
guard_expect_in_file "$TAG" 'unregister_route: HakoAllocObjectLifecycleFacadeHugeUnregisterRoute' "$ROUTE" "MIMAP-027A must reuse the MIMAP-026A route"
guard_expect_in_file "$TAG" 'me\.unregister_route\.allocateThenUnregisterHuge\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "MIMAP-027A must start from MIMAP-026A"
guard_expect_in_file "$TAG" 'me\.unregister_route\.release_seam\.releaseHugePtr\(result\.first_ptr\)' "$ROUTE" "MIMAP-027A must reject double-unregister through M181"
guard_expect_in_file "$TAG" 'me\.unregister_route\.release_seam\.releaseHugePtr\(stale_ptr\)' "$ROUTE" "MIMAP-027A must reject stale pointer through M181"
guard_expect_in_file "$TAG" 'releaseHugePtr\(ptr\)' "$RELEASE_SEAM" "MIMAP-027A relies on existing M181 release seam"
guard_expect_in_file "$TAG" 'me\.page_map\.lookup\(ptr\)' "$RELEASE_SEAM" "M181 must own page-map lookup"
guard_expect_in_file "$TAG" 'markReleased\(ptr\)' "$HUGE_MODEL" "M181 relies on existing M180 markReleased"
guard_expect_in_file "$TAG" 'entry\.live = 0' "$PAGE_MAP" "M171 page map must own unregister live-state transition"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unregister_failfast_box = "memory/object_lifecycle_facade_huge_unregister_failfast_box.hako"' "$MODULE" "hako module must export MIMAP-027A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unregister_failfast_box.hako' "$README" "memory README must name MIMAP-027A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-027A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-027A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|markReleased[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-027A must use M181 instead of direct page-map/model release calls" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'unreserve[A-Za-z0-9_]*[[:space:]]*\(|releasePage[[:space:]]*\(|decommit[A-Za-z0-9_]*[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-027A leaked behavior beyond facade huge-unregister fail-fast diagnostics" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-unregister-failfast|HakoAllocObjectLifecycleFacadeHugeUnregisterFailfast|object_lifecycle_facade_huge_unregister_failfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-027A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap027a_facade_huge_unregister_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap027a.mir.json"
exe_out="$tmp_dir/mimap027a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.proveHugeUnregisterFailfast/7",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.hugeUnregisterFailfastThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.initHugeUnregisterFailfastReport/1",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.copyFirstUnregisterResult/2",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.snapshotHugeUnregisterFailfastCounters/1",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.rejectDoubleUnregisteredPtr/1",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.rejectStaleHugePtr/2",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.allocateThenUnregisterHuge/6",
    "HakoAllocHugeReleaseSeam.releaseHugePtr/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastReport",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterReport",
    "HakoAllocHugeReleaseSeam",
    "HakoAllocHugePageModel",
    "HakoAllocHugePageMetaStore",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "huge_threshold",
    "first_unregister_ok",
    "first_ptr",
    "first_page_id",
    "first_huge_live_after",
    "first_page_map_live_after",
    "first_page_map_unregister_count",
    "first_seam_release_count",
    "first_seam_unregister_count",
    "double_attempted",
    "double_ok",
    "double_ptr",
    "double_page_map_live_before",
    "double_page_map_live_after",
    "double_lookup_miss_count",
    "double_failure_kind",
    "stale_attempted",
    "stale_ok",
    "stale_ptr",
    "stale_page_map_live_before",
    "stale_page_map_live_after",
    "stale_lookup_miss_count",
    "stale_failure_kind",
    "huge_count",
    "huge_live_count",
    "huge_allocate_count",
    "huge_release_count",
    "huge_release_reject_count",
    "page_map_entry_count",
    "page_map_live_count",
    "page_map_register_count",
    "page_map_lookup_count",
    "page_map_lookup_miss_count",
    "page_map_unregister_count",
    "page_map_reject_count",
    "seam_release_count",
    "seam_unregister_count",
    "seam_lookup_miss_count",
    "seam_not_huge_count",
    "seam_model_reject_count",
    "seam_reject_count",
    "seam_last_failure_kind",
    "failfast_attempt_count",
    "double_reject_count",
    "stale_reject_count",
    "success_count",
    "failure_count",
    "final_ok",
    "final_reason",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-unregister failfast report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.proveHugeUnregisterFailfast/7"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute", "initHugeUnregisterFailfastReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute", "allocateThenUnregisterHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute", "copyFirstUnregisterResult")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute", "rejectDoubleUnregisteredPtr")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute", "rejectStaleHugePtr")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute", "snapshotHugeUnregisterFailfastCounters")
require_method("HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.rejectDoubleUnregisteredPtr/1", "HakoAllocHugeReleaseSeam", "releaseHugePtr")
require_method("HakoAllocObjectLifecycleFacadeHugeUnregisterFailfastRoute.rejectStaleHugePtr/2", "HakoAllocHugeReleaseSeam", "releaseHugePtr")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-unregister-failfast-proof' "$run_log"
rg -F -q 'first=1,70000,1000,0,0,1,1,1' "$run_log"
rg -F -q 'double=1,0,70000,0,0,1,1' "$run_log"
rg -F -q 'stale=1,0,99999,0,0,2,1' "$run_log"
rg -F -q 'huge=1,0,1,1,0' "$run_log"
rg -F -q 'page_map=1,0,1,3,2,1,0' "$run_log"
rg -F -q 'seam=1,1,2,0,0,2,1' "$run_log"
rg -F -q 'route_counts=2,1,1,1,0' "$run_log"
rg -F -q 'final=1,1,0' "$run_log"
rg -F -q 'shape=8' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
