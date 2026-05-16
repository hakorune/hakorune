#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-decommit-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-decommit-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-decommit-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-decommit-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako"
PAGE_SOURCE_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_source_box.hako"
RELEASE_SEAM="lang/src/hako_alloc/memory/huge_release_seam_box.hako"
DECOMMIT_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
HUGE_META_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-448-MIMAP-029A-FACADE-HUGE-DECOMMIT-ROUTE.md"
SELECTION_CARD="docs/development/current/main/phases/phase-293x/293x-447-MIMAP-028B-POST-BACKED-HUGE-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"

echo "[$TAG] running MIMAP-029A facade huge decommit guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$PAGE_SOURCE_ROUTE" \
  "$RELEASE_SEAM" \
  "$DECOMMIT_ADAPTER" \
  "$PAGE_SOURCE" \
  "$HUGE_MODEL" \
  "$HUGE_META_STORE" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$CARD" \
  "$SELECTION_CARD" \
  "$INDEX" \
  "$README" \
  "$ROOT_README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeDecommitRoute' "$ROUTE" "MIMAP-029A route owner missing"
guard_expect_in_file "$TAG" 'allocateUnregisterDecommitHuge\(facade, size\): i64' "$ROUTE" "MIMAP-029A route must publish a scalar return contract"
guard_expect_in_file "$TAG" 'page_source_route: HakoAllocObjectLifecycleFacadeHugePageSourceRoute' "$ROUTE" "MIMAP-029A must reuse MIMAP-028A route"
guard_expect_in_file "$TAG" 'release_seam: HakoAllocHugeReleaseSeam' "$ROUTE" "MIMAP-029A must compose M181 release seam"
guard_expect_in_file "$TAG" 'new HakoAllocHugeReleaseSeam\(me\.page_source_route\.huge_route\.huge_model\)' "$ROUTE" "MIMAP-029A must bind M181 to the MIMAP-028A huge model"
guard_expect_in_file "$TAG" 'decommit_adapter: HakoAllocPageSourceDecommitAdapter' "$ROUTE" "MIMAP-029A must reuse M196 decommit adapter"
guard_expect_in_file "$TAG" 'me\.page_source_route\.allocateHugeWithPageSource\(facade, size\)' "$ROUTE" "MIMAP-029A must allocate through MIMAP-028A"
guard_expect_in_file "$TAG" 'me\.release_seam\.releaseHugePtr\(alloc_result\.huge_ptr\)' "$ROUTE" "MIMAP-029A must unregister through M181"
guard_expect_in_file "$TAG" 'me\.decommit_adapter\.decommitPage\(alloc_result\.source_base, alloc_result\.source_bytes\)' "$ROUTE" "MIMAP-029A must decommit the MIMAP-028A backing range"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.decommitPage\(base, bytes\)' "$DECOMMIT_ADAPTER" "M196 adapter must delegate to page-source decommit"
guard_expect_in_file "$TAG" 'committedSizeAt\(index\): i64' "$HUGE_META_STORE" "huge meta store scalar accessors must publish stable return contracts"
guard_expect_in_file "$TAG" 'requestedSizeAt\(index\): i64' "$HUGE_META_STORE" "huge meta store scalar accessors must publish stable return contracts"
guard_expect_in_file "$TAG" 'local committed_size: i64 = me\.committed_sizes\.get\(index\)' "$HUGE_META_STORE" "backend MIR must see typed scalar column reads"
guard_expect_in_file "$TAG" 'local requested_size: i64 = me\.requested_sizes\.get\(index\)' "$HUGE_META_STORE" "backend MIR must see typed scalar column reads"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_decommit_box = "memory/object_lifecycle_facade_huge_decommit_box.hako"' "$MODULE" "hako module must export MIMAP-029A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_decommit_box.hako' "$README" "memory README must name MIMAP-029A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeDecommitRoute' "$ROOT_README" "root hako_alloc README must name MIMAP-029A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-029A card must be landed after implementation"
guard_expect_in_file "$TAG" 'USERBOX-METHOD-COMPOSITE-001' "$CARD" "MIMAP-029A card must pin pure-first sidecar trigger"
guard_expect_in_file "$TAG" 'USERBOX-METHOD-COMPOSITE-001' "$SELECTION_CARD" "MIMAP-028B card must pin pure-first sidecar trigger"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-029A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|markReleased[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-029A must use M181 instead of direct page-map/model release calls" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|(^|[^A-Za-z0-9_])reservePage[[:space:]]*\(|(^|[^A-Za-z0-9_])commitPage[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".page_source_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-029A route/app must use MIMAP-028A and M196 instead of direct page-source calls" >&2
  cat /tmp/"$TAG".page_source_leak >&2
  rm -f /tmp/"$TAG".page_source_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_source_leak

if rg -n 'unreserve[A-Za-z0-9_]*[[:space:]]*\(|releasePage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-029A leaked behavior beyond facade huge decommit route" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-decommit|HakoAllocObjectLifecycleFacadeHugeDecommit|object_lifecycle_facade_huge_decommit' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-029A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap029a_facade_huge_decommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap029a.mir.json"
exe_out="$tmp_dir/mimap029a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.allocateUnregisterDecommitHuge/2",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.hugeDecommitThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.resetHugeDecommitState/1",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.snapshotHugeDecommitState/0",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute.allocateHugeWithPageSource/2",
    "HakoAllocHugeReleaseSeam.releaseHugePtr/1",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute",
    "HakoAllocObjectLifecycleFacadeHugePageSourceReport",
    "HakoAllocHugeReleaseSeam",
    "HakoAllocPageSourceDecommitAdapter",
    "HakoAllocHugePageModel",
    "HakoAllocPageMap",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeDecommitRoute"].get("fields", [])
}
for field in (
    "last_source_base",
    "last_source_bytes",
    "last_source_status",
    "last_source_commit_rc",
    "last_huge_ptr",
    "last_huge_page_id",
    "last_unregister_ok",
    "last_unregister_live_before",
    "last_unregister_live_after",
    "last_decommit_ok",
    "last_decommit_base",
    "last_decommit_bytes",
    "last_decommit_rc",
    "last_page_map_unregister_count",
    "last_seam_unregister_count",
    "last_adapter_call_count",
    "success_count",
):
    if field not in route_fields:
        raise SystemExit(f"missing huge-decommit route field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.allocateUnregisterDecommitHuge/2"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugePageSourceRoute", "allocateHugeWithPageSource")
require_method(route_fn, "HakoAllocHugeReleaseSeam", "releaseHugePtr")
require_method(route_fn, "HakoAllocPageSourceDecommitAdapter", "decommitPage")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitRoute", "resetHugeDecommitState")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitRoute", "snapshotHugeDecommitState")
require_global("HakoAllocPageSourceDecommitAdapter.decommitPage/2", "HakoAllocPageSourcePolicy.decommitPage/2")
require_global("HakoAllocPageSourcePolicy.decommitPage/2", "OsVmCoreBox.decommit_bytes_i64/2")

for fn_name in (route_fn, "main"):
    for callee in iter_calls(functions[fn_name]):
        name = callee.get("name") or ""
        box = callee.get("box_name") or ""
        target = f"{box}.{name}"
        if name in {"lookup", "unregister", "markReleased", "unreserve", "releasePage"}:
            raise SystemExit(f"forbidden direct call in {fn_name}: {target}")
        if any(part in target for part in ("RemoteFree", "Atomic", "Tls")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-decommit-proof' "$run_log"
rg -F -q 'source=1,' "$run_log"
rg -F -q ',4194305,0' "$run_log"
rg -F -q 'huge=70000,1000,4194305,4194305,1' "$run_log"
rg -F -q 'unregister=1,1,1,0,0' "$run_log"
rg -F -q 'decommit=1,1,' "$run_log"
rg -F -q ',4194305,0' "$run_log"
rg -F -q 'counts=0,0,1,1,1,0,1,1,0' "$run_log"
rg -F -q 'route_counts=1,1,0' "$run_log"
rg -F -q 'final=1,0,1' "$run_log"
rg -F -q 'shape=34' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
