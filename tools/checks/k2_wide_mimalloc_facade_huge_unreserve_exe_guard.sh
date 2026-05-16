#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-unreserve-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-unreserve-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-unreserve-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-unreserve-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_box.hako"
DECOMMIT_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako"
UNRESERVE_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-461-MIMAP-034A-FACADE-HUGE-UNRESERVE-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"

echo "[$TAG] running MIMAP-034A facade huge unreserve guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$DECOMMIT_ROUTE" \
  "$UNRESERVE_ADAPTER" \
  "$PAGE_SOURCE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README" \
  "$ROOT_README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnreserveRoute' "$ROUTE" "MIMAP-034A route owner missing"
guard_expect_in_file "$TAG" 'allocateUnregisterDecommitUnreserveHuge\(facade, size\): i64' "$ROUTE" "MIMAP-034A route must publish a scalar return contract"
guard_expect_in_file "$TAG" 'decommit_route: HakoAllocObjectLifecycleFacadeHugeDecommitRoute' "$ROUTE" "MIMAP-034A must reuse MIMAP-029A"
guard_expect_in_file "$TAG" 'unreserve_adapter: HakoAllocPageSourceUnreserveAdapter' "$ROUTE" "MIMAP-034A must reuse MIMAP-033A adapter"
guard_expect_in_file "$TAG" 'me\.decommit_route\.allocateUnregisterDecommitHuge\(facade, size\)' "$ROUTE" "MIMAP-034A must start from MIMAP-029A"
guard_expect_in_file "$TAG" 'me\.unreserve_adapter\.unreservePage\(me\.last_decommit_base, me\.last_decommit_bytes\)' "$ROUTE" "MIMAP-034A must unreserve the decommitted backing range"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.unreservePage\(base, bytes\)' "$UNRESERVE_ADAPTER" "MIMAP-033A adapter must delegate to page-source unreserve"
guard_expect_in_file "$TAG" 'OsVmCoreBox\.unreserve_bytes_i64\(base, bytes\)' "$PAGE_SOURCE" "page-source policy must delegate to OSVM unreserve"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unreserve_box = "memory/object_lifecycle_facade_huge_unreserve_box.hako"' "$MODULE" "hako module must export MIMAP-034A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unreserve_box.hako' "$README" "memory README must name MIMAP-034A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeUnreserveRoute' "$ROOT_README" "root hako_alloc README must name MIMAP-034A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-034A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-034A guard"

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|markReleased[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_release_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-034A must reuse MIMAP-029A instead of direct page-map/model release calls" >&2
  cat /tmp/"$TAG".direct_release_leak >&2
  rm -f /tmp/"$TAG".direct_release_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_release_leak

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|(^|[^A-Za-z0-9_])reservePage[[:space:]]*\(|(^|[^A-Za-z0-9_])commitPage[[:space:]]*\(|(^|[^A-Za-z0-9_])decommitPage[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".page_source_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-034A route/app must not directly call page-source or OSVM APIs" >&2
  cat /tmp/"$TAG".page_source_leak >&2
  rm -f /tmp/"$TAG".page_source_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_source_leak

if rg -n 'duplicate[A-Za-z0-9_]*[[:space:]]*\(|stale[A-Za-z0-9_]*[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-034A leaked behavior beyond facade huge unreserve success route" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-unreserve|HakoAllocObjectLifecycleFacadeHugeUnreserve|object_lifecycle_facade_huge_unreserve' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-034A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap034a_facade_huge_unreserve.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap034a.mir.json"
exe_out="$tmp_dir/mimap034a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.allocateUnregisterDecommitUnreserveHuge/2",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.hugeUnreserveThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.resetHugeUnreserveState/1",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.snapshotHugeUnreserveState/0",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.allocateUnregisterDecommitHuge/2",
    "HakoAllocPageSourceUnreserveAdapter.unreservePage/2",
    "HakoAllocPageSourcePolicy.unreservePage/2",
    "OsVmCoreBox.unreserve_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute",
    "HakoAllocPageSourceUnreserveAdapter",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadeHugeUnreserveRoute"].get("fields", [])
}
for name, declared in (
    ("decommit_route", "HakoAllocObjectLifecycleFacadeHugeDecommitRoute"),
    ("unreserve_adapter", "HakoAllocPageSourceUnreserveAdapter"),
):
    field = route_fields.get(name)
    if field is None or field.get("declared_type") != declared or field.get("storage") != "handle":
        raise SystemExit(f"bad huge-unreserve route field {name}: {field}")

for field in (
    "last_source_base",
    "last_source_bytes",
    "last_decommit_status",
    "last_decommit_ok",
    "last_decommit_base",
    "last_decommit_bytes",
    "last_unreserve_attempted",
    "last_unreserve_ok",
    "last_unreserve_base",
    "last_unreserve_bytes",
    "last_unreserve_rc",
    "last_adapter_call_count",
    "last_adapter_success_count",
    "last_adapter_reject_count",
    "no_extra_diagnostics",
    "no_recommit",
    "no_provider",
):
    if field not in route_fields:
        raise SystemExit(f"missing huge-unreserve route field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.allocateUnregisterDecommitUnreserveHuge/2"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitRoute", "allocateUnregisterDecommitHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute", "resetHugeUnreserveState")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute", "snapshotHugeUnreserveState")
require_method(route_fn, "HakoAllocPageSourceUnreserveAdapter", "unreservePage")
require_global("HakoAllocPageSourceUnreserveAdapter.unreservePage/2", "HakoAllocPageSourcePolicy.unreservePage/2")
require_global("HakoAllocPageSourcePolicy.unreservePage/2", "OsVmCoreBox.unreserve_bytes_i64/2")

routes = functions["OsVmCoreBox.unreserve_bytes_i64/2"].get("metadata", {}).get("extern_call_routes", [])
for route in routes:
    if (
        route.get("route_id") == "extern.hako_osvm.unreserve_bytes_i64"
        and route.get("core_op") == "HakoOsvmUnreserveBytesI64"
        and route.get("symbol") == "hako_osvm_unreserve_bytes_i64"
        and route.get("return_shape") == "scalar_i64"
        and route.get("value_demand") == "runtime_i64"
        and route.get("effects") == ["hako.osvm.unreserve"]
    ):
        break
else:
    raise SystemExit(f"missing OSVM unreserve extern route: {routes}")

print("[mimap034a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-unreserve-proof' "$run_log"
rg -F -q 'huge=70000,1000' "$run_log"
rg -F -q 'decommit=1,1' "$run_log"
rg -F -q 'unreserve=1,1' "$run_log"
rg -F -q 'adapter=1,1,0' "$run_log"
rg -F -q 'route_counts=1,1,0' "$run_log"
rg -F -q 'stop=1,1,1' "$run_log"
rg -F -q 'final=1,0,1' "$run_log"
rg -F -q 'shape=27' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"
