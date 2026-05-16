#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-unregister-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-unregister-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-unregister-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-unregister-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_box.hako"
ALLOC_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako"
RELEASE_SEAM="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-442-MIMAP-026A-FACADE-HUGE-UNREGISTER-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-026A facade huge-unregister guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$ALLOC_ROUTE" \
  "$RELEASE_SEAM" \
  "$HUGE_MODEL" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnregisterRoute' "$ROUTE" "MIMAP-026A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnregisterReport' "$ROUTE" "MIMAP-026A report owner missing"
guard_expect_in_file "$TAG" 'alloc_route: HakoAllocObjectLifecycleFacadeHugePageModelRoute' "$ROUTE" "MIMAP-026A must reuse the MIMAP-023A route"
guard_expect_in_file "$TAG" 'release_seam: HakoAllocHugeReleaseSeam' "$ROUTE" "MIMAP-026A must compose the M181 release seam"
guard_expect_in_file "$TAG" 'new HakoAllocHugeReleaseSeam\(me\.alloc_route\.huge_model\)' "$ROUTE" "MIMAP-026A must bind M181 to the same huge model"
guard_expect_in_file "$TAG" 'me\.alloc_route\.allocateWithHugePageModel\(facade, size, page_id, block_size, capacity, reserved\)' "$ROUTE" "MIMAP-026A must allocate through MIMAP-023A"
guard_expect_in_file "$TAG" 'me\.release_seam\.releaseHugePtr\(result\.huge_ptr\)' "$ROUTE" "MIMAP-026A must release through M181"
guard_expect_in_file "$TAG" 'releaseHugePtr\(ptr\)' "$RELEASE_SEAM" "MIMAP-026A relies on existing M181 release seam"
guard_expect_in_file "$TAG" 'me\.page_map\.lookup\(ptr\)' "$RELEASE_SEAM" "M181 must own page-map lookup"
guard_expect_in_file "$TAG" 'me\.page_map\.unregister\(ptr\)' "$RELEASE_SEAM" "M181 must own page-map unregister"
guard_expect_in_file "$TAG" 'markReleased\(ptr\)' "$HUGE_MODEL" "M181 relies on existing M180 markReleased"
guard_expect_in_file "$TAG" 'entry\.live = 0' "$PAGE_MAP" "M171 page map must own unregister live-state transition"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unregister_box = "memory/object_lifecycle_facade_huge_unregister_box.hako"' "$MODULE" "hako module must export MIMAP-026A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unregister_box.hako' "$README" "memory README must name MIMAP-026A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-026A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-026A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|markReleased[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-026A must use M181 instead of direct page-map/model release calls" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'unreserve[A-Za-z0-9_]*[[:space:]]*\(|releasePage[[:space:]]*\(|decommit[A-Za-z0-9_]*[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-026A leaked behavior beyond facade huge-unregister route" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-unregister|HakoAllocObjectLifecycleFacadeHugeUnregister|object_lifecycle_facade_huge_unregister' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-026A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap026a_facade_huge_unregister.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap026a.mir.json"
exe_out="$tmp_dir/mimap026a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.allocateThenUnregisterHuge/6",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.hugeUnregisterThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.initHugeUnregisterReport/1",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.copyHugeUnregisterAllocation/2",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.snapshotHugeUnregisterCounters/1",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.unregisterHugeViaSeam/1",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.allocateWithHugePageModel/6",
    "HakoAllocHugeReleaseSeam.releaseHugePtr/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute",
    "HakoAllocObjectLifecycleFacadeHugeUnregisterReport",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute",
    "HakoAllocObjectLifecycleFacadeHugePageModelReport",
    "HakoAllocHugeReleaseSeam",
    "HakoAllocHugePageModel",
    "HakoAllocHugePageMetaStore",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeUnregisterReport"].get("fields", [])
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
    "unregister_attempted",
    "unregister_ok",
    "unregister_ptr",
    "unregister_page_id",
    "unregister_requested_size",
    "unregister_committed_size",
    "unregister_live_before",
    "unregister_live_after",
    "unregister_failure_kind",
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
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
    "final_ptr",
    "unregister_attempt_count",
    "unregister_success_count",
    "unregister_failure_count",
    "small_forward_count",
    "success_count",
    "failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-unregister report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.allocateThenUnregisterHuge/6"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute", "initHugeUnregisterReport")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "allocateWithHugePageModel")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute", "copyHugeUnregisterAllocation")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute", "unregisterHugeViaSeam")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnregisterRoute", "snapshotHugeUnregisterCounters")
require_method("HakoAllocObjectLifecycleFacadeHugeUnregisterRoute.unregisterHugeViaSeam/1", "HakoAllocHugeReleaseSeam", "releaseHugePtr")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-unregister-proof' "$run_log"
rg -F -q 'huge=1,1,70000,1000' "$run_log"
rg -F -q 'unregister=1,1,70000,1000,1,0,0' "$run_log"
rg -F -q 'unregister_meta=4194305,4194305,1,0,1,1,0' "$run_log"
rg -F -q 'page_map=1,0,1,1,0,1,0' "$run_log"
rg -F -q 'seam=1,1,0,0,0,0,0' "$run_log"
rg -F -q 'huge_final=1,0,1000,0' "$run_log"
rg -F -q 'small=0,0,1,1,0' "$run_log"
rg -F -q 'small_source=1,902' "$run_log"
rg -F -q 'small_final=1,0,902,1' "$run_log"
rg -F -q 'route_counts=1,1,0,1,2,0' "$run_log"
rg -F -q 'final_page_map=1,0,1,1,1' "$run_log"
rg -F -q 'facade=1,902,1' "$run_log"
rg -F -q 'shape=11' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
