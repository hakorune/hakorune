#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-decommit-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-decommit-failfast-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-decommit-failfast-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_failfast_box.hako"
DECOMMIT_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako"
DECOMMIT_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-454-MIMAP-030A-FACADE-HUGE-DECOMMIT-FAILFAST.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"

echo "[$TAG] running MIMAP-030A facade huge decommit fail-fast guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$DECOMMIT_ROUTE" \
  "$DECOMMIT_ADAPTER" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README" \
  "$ROOT_README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute' "$ROUTE" "MIMAP-030A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeDecommitFailfastReport' "$ROUTE" "MIMAP-030A report owner missing"
guard_expect_in_file "$TAG" 'decommit_route: HakoAllocObjectLifecycleFacadeHugeDecommitRoute' "$ROUTE" "MIMAP-030A must reuse MIMAP-029A"
guard_expect_in_file "$TAG" 'me\.decommit_route\.allocateUnregisterDecommitHuge\(facade, size\)' "$ROUTE" "MIMAP-030A must start from MIMAP-029A success route"
guard_expect_in_file "$TAG" 'decommitted_bases: ArrayBox' "$ROUTE" "MIMAP-030A must own allocator-side decommit state"
guard_expect_in_file "$TAG" 'markSuccessfulDecommit' "$ROUTE" "MIMAP-030A must record successful decommit state"
guard_expect_in_file "$TAG" 'rejectDuplicateDecommit' "$ROUTE" "MIMAP-030A duplicate fail-fast entry missing"
guard_expect_in_file "$TAG" 'rejectStaleDecommit' "$ROUTE" "MIMAP-030A stale fail-fast entry missing"
guard_expect_in_file "$TAG" 'result\.adapter_calls_before_duplicate = me\.decommit_route\.decommit_adapter\.call_count' "$ROUTE" "MIMAP-030A must snapshot adapter count before duplicate"
guard_expect_in_file "$TAG" 'result\.adapter_calls_after_duplicate = me\.decommit_route\.decommit_adapter\.call_count' "$ROUTE" "MIMAP-030A must prove no duplicate adapter call"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.decommitPage\(base, bytes\)' "$DECOMMIT_ADAPTER" "M196 adapter remains the page-source decommit owner"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_decommit_failfast_box = "memory/object_lifecycle_facade_huge_decommit_failfast_box.hako"' "$MODULE" "hako module must export MIMAP-030A route"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_decommit_failfast_box.hako' "$README" "memory README must name MIMAP-030A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute' "$ROOT_README" "root hako_alloc README must name MIMAP-030A owner"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-030A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-030A guard"

if rg -n '\.decommitPage[[:space:]]*\(' "$ROUTE" "$APP" >/tmp/"$TAG".direct_adapter_call 2>&1; then
  echo "[$TAG] ERROR: MIMAP-030A must not call the page-source decommit adapter directly" >&2
  cat /tmp/"$TAG".direct_adapter_call >&2
  rm -f /tmp/"$TAG".direct_adapter_call
  exit 1
fi
rm -f /tmp/"$TAG".direct_adapter_call

if rg -n '\.lookup[[:space:]]*\(|\.unregister[[:space:]]*\(|markReleased[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-030A must reuse MIMAP-029A instead of direct page-map/model release calls" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|(^|[^A-Za-z0-9_])reservePage[[:space:]]*\(|(^|[^A-Za-z0-9_])commitPage[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".page_source_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-030A route/app must not directly call page-source allocation/decommit APIs" >&2
  cat /tmp/"$TAG".page_source_leak >&2
  rm -f /tmp/"$TAG".page_source_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_source_leak

if rg -n 'unreserve[A-Za-z0-9_]*[[:space:]]*\(|releasePage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-030A leaked behavior beyond facade huge decommit fail-fast diagnostics" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-decommit-failfast|HakoAllocObjectLifecycleFacadeHugeDecommitFailfast|object_lifecycle_facade_huge_decommit_failfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-030A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap030a_facade_huge_decommit_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap030a.mir.json"
exe_out="$tmp_dir/mimap030a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.proveHugeDecommitFailfast/4",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.hugeDecommitFailfastThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.markSuccessfulDecommit/2",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.rejectDuplicateDecommit/1",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.rejectStaleDecommit/3",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute.allocateUnregisterDecommitHuge/2",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastReport",
    "HakoAllocObjectLifecycleFacadeHugeDecommitRoute",
    "HakoAllocObjectLifecycleFacadeHugePageSourceRoute",
    "HakoAllocPageSourceDecommitAdapter",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute"].get("fields", [])
}
for name, declared in (
    ("decommit_route", "HakoAllocObjectLifecycleFacadeHugeDecommitRoute"),
    ("decommitted_bases", "ArrayBox"),
    ("decommitted_bytes", "ArrayBox"),
):
    field = route_fields.get(name)
    if field is None or field.get("declared_type") != declared or field.get("storage") != "handle":
        raise SystemExit(f"bad failfast route field {name}: {field}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeDecommitFailfastReport"].get("fields", [])
}
for field in (
    "first_status",
    "first_source_base",
    "first_source_bytes",
    "first_decommit_ok",
    "first_adapter_call_count",
    "mark_status",
    "marked_count",
    "duplicate_attempted",
    "duplicate_rejected",
    "adapter_calls_before_duplicate",
    "adapter_calls_after_duplicate",
    "stale_attempted",
    "stale_rejected",
    "adapter_calls_before_stale",
    "adapter_calls_after_stale",
    "no_unreserve",
    "no_recommit",
    "no_provider",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-decommit failfast report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.proveHugeDecommitFailfast/4"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitRoute", "allocateUnregisterDecommitHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute", "markSuccessfulDecommit")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute", "rejectDuplicateDecommit")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute", "rejectStaleDecommit")
for fn_name in (
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.rejectDuplicateDecommit/1",
    "HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute.rejectStaleDecommit/3",
):
    for callee in iter_calls(functions[fn_name]):
        name = callee.get("name") or ""
        box = callee.get("box_name") or ""
        if name == "decommitPage":
            raise SystemExit(f"failfast path must not call decommitPage: {fn_name} -> {box}.{name}")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-decommit-failfast-proof' "$run_log"
rg -F -q 'first=1,0,' "$run_log"
rg -F -q ',4194305,1,0' "$run_log"
rg -F -q 'huge=70000,1000,1,0,0' "$run_log"
rg -F -q 'adapter_first=1,1,0' "$run_log"
rg -F -q 'marker=0,1' "$run_log"
rg -F -q 'duplicate=1,1,5,' "$run_log"
rg -F -q ',4194305,1,1' "$run_log"
rg -F -q 'stale=1,1,7,99999,4194305,1,1' "$run_log"
rg -F -q 'route_counts=2,1,1,1,0' "$run_log"
rg -F -q 'stop=1,1,1' "$run_log"
rg -F -q 'final=1,1,0' "$run_log"
rg -F -q 'shape=41' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
