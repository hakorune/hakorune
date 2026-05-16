#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-page-source-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-page-source-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-page-source-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-page-source-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_source_box.hako"
HUGE_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-446-MIMAP-028A-FACADE-HUGE-PAGE-SOURCE-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"

echo "[$TAG] running MIMAP-028A facade huge page-source guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$HUGE_ROUTE" \
  "$PAGE_SOURCE" \
  "$HUGE_MODEL" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README" \
  "$ROOT_README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugePageSourceRoute' "$ROUTE" "MIMAP-028A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugePageSourceReport' "$ROUTE" "MIMAP-028A report owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeBackingSource' "$ROUTE" "MIMAP-028A backing source owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeBackingReport' "$ROUTE" "MIMAP-028A backing report owner missing"
guard_expect_in_file "$TAG" 'huge_route: HakoAllocObjectLifecycleFacadeHugePageModelRoute' "$ROUTE" "route must compose the MIMAP-023A huge route"
guard_expect_in_file "$TAG" 'backing_source: HakoAllocObjectLifecycleFacadeHugeBackingSource' "$ROUTE" "route must compose the backing source owner"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.reservePage' "$ROUTE" "route must reserve through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.commitPage' "$ROUTE" "route must commit through page-source policy"
guard_expect_in_file "$TAG" 'me\.huge_route\.allocateWithHugePageModel' "$ROUTE" "route must delegate huge allocation to MIMAP-023A"
guard_expect_in_file "$TAG" 'release_count: i64 = 0' "$ROUTE" "route must expose inactive release counter"
guard_expect_in_file "$TAG" 'unregister_count: i64 = 0' "$ROUTE" "route must expose inactive unregister counter"
guard_expect_in_file "$TAG" 'decommit_count: i64 = 0' "$ROUTE" "route must expose inactive decommit counter"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_page_source_box = "memory/object_lifecycle_facade_huge_page_source_box.hako"' "$MODULE" "hako module must export MIMAP-028A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_page_source_box.hako' "$README" "memory README must name MIMAP-028A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugePageSourceRoute' "$ROOT_README" "root hako_alloc README must name MIMAP-028A owner"
guard_expect_in_file "$TAG" 'MIMAP-028A' "$CARD" "MIMAP-028A card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-028A guard"

if rg -n '\.lookup[[:space:]]*\(|markReleased[[:space:]]*\(|unregister[[:space:]]*\(|releaseHugePtr[[:space:]]*\(|releasePage|unreserve|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|HakoAllocHugeReleaseSeam|objectLifecycleRelease|objectLifecycleRealloc|objectLifecycleSmallAllocAligned|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-028A leaked behavior beyond huge page-source backing" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-page-source|HakoAllocObjectLifecycleFacadeHugePageSource|object_lifecycle_facade_huge_page_source' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-028A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap028a_facade_huge_page_source.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap028a.mir.json"
exe_out="$tmp_dir/mimap028a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.allocateHugeWithPageSource/2",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.initHugePageSourceReport/1",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.copySource/2",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.copyHuge/2",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.snapshotCounters/1",
    "HakoAllocObjectLifecycleFacadeHugeBackingSource.reserveCommitBacking/1",
    "HakoAllocObjectLifecycleFacadeHugeBackingSource.snapshot/1",
    "HakoAllocObjectLifecycleFacadeHugePageModelRoute.allocateWithHugePageModel/6",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "HakoAllocHugePageModel.allocateHuge/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute",
    "HakoAllocObjectLifecycleFacadeHugePageSourceReport",
    "HakoAllocObjectLifecycleFacadeHugeBackingSource",
    "HakoAllocObjectLifecycleFacadeHugeBackingReport",
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
    for field in plans["HakoAllocObjectLifecycleFacadeHugePageSourceReport"].get("fields", [])
}
for field in (
    "status",
    "requested_size",
    "source_attempted",
    "source_status",
    "source_reserved",
    "source_committed",
    "source_reject",
    "source_base",
    "source_bytes",
    "huge_allocated",
    "huge_ptr",
    "huge_page_id",
    "huge_requested_size",
    "huge_committed_size",
    "page_map_entry_count",
    "page_map_register_count",
    "release_count",
    "unregister_count",
    "decommit_count",
    "source_attempt_count",
    "source_success_count",
    "source_failure_count",
    "huge_success_count",
    "huge_failure_count",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-page-source report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.allocateHugeWithPageSource/2"
source_fn = "HakoAllocObjectLifecycleFacadeHugeBackingSource.reserveCommitBacking/1"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeBackingSource", "reserveCommitBacking")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageModelRoute", "allocateWithHugePageModel")
require_global(source_fn, "HakoAllocPageSourcePolicy.reservePage/1")
require_global(source_fn, "HakoAllocPageSourcePolicy.commitPage/2")

for fn_name in (route_fn, source_fn, "main"):
    for callee in iter_calls(functions[fn_name]):
        box = callee.get("box_name") or ""
        name = callee.get("name") or ""
        target = f"{box}.{name}"
        forbidden_names = {
            "markReleased",
            "releaseHugePtr",
            "unregister",
            "decommitPage",
            "releasePage",
            "unreserve",
            "objectLifecycleReleaseBlock",
            "objectLifecycleReallocShrink",
            "objectLifecycleReallocGrow",
            "objectLifecycleSmallAllocAligned",
        }
        if name in forbidden_names:
            raise SystemExit(f"forbidden call in {fn_name}: {target}")
        if any(part in target for part in ("HugeReleaseSeam", "RemoteFree", "Purge", "Atomic", "Tls")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"; then
  echo "[$TAG] ERROR: MIMAP-028A must not emit decommit" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-page-source-proof' "$run_log"
rg -F -q 'source=1,1,1,1,0,0' "$run_log"
rg -F -q 'backing=' "$run_log"
rg -F -q 'huge=1,1,70000,1000' "$run_log"
rg -F -q 'final=1,0,1000,70000' "$run_log"
rg -F -q 'page_map=1,1' "$run_log"
rg -F -q 'inactive=0,0,0' "$run_log"
rg -F -q 'route_counts=1,1,0,1,0' "$run_log"
rg -F -q 'shape=9' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
