#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-unreserve-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-huge-unreserve-failfast-proof/main.hako"
APP_TEST="apps/mimalloc-facade-huge-unreserve-failfast-proof/test.sh"
APP_README="apps/mimalloc-facade-huge-unreserve-failfast-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako"
UNRESERVE_ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_box.hako"
BACKING_SET="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_backing_set_box.hako"
UNRESERVE_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-463-MIMAP-035A-FACADE-HUGE-UNRESERVE-FAILFAST.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"

echo "[$TAG] running MIMAP-035A facade huge unreserve fail-fast guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ROUTE" \
  "$UNRESERVE_ROUTE" \
  "$BACKING_SET" \
  "$UNRESERVE_ADAPTER" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README" \
  "$ROOT_README"
guard_require_exec_files "$TAG" "$APP_TEST" "$0"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute' "$ROUTE" "MIMAP-035A route owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastReport' "$ROUTE" "MIMAP-035A report owner missing"
guard_expect_in_file "$TAG" 'unreserve_route: HakoAllocObjectLifecycleFacadeHugeUnreserveRoute' "$ROUTE" "MIMAP-035A must reuse MIMAP-034A"
guard_expect_in_file "$TAG" 'unreserved_backings: HakoAllocObjectLifecycleFacadeHugeBackingSet' "$ROUTE" "MIMAP-035A must delegate backing state to MIMAP-037A helper"
guard_expect_in_file "$TAG" 'me\.unreserve_route\.allocateUnregisterDecommitUnreserveHuge\(facade, size\)' "$ROUTE" "MIMAP-035A must start from MIMAP-034A success route"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeBackingSet' "$BACKING_SET" "MIMAP-037A backing-set helper missing"
guard_expect_in_file "$TAG" 'mark\(base, bytes\)' "$BACKING_SET" "MIMAP-037A backing-set helper must own mark"
guard_expect_in_file "$TAG" 'markSuccessfulUnreserve' "$ROUTE" "MIMAP-035A must record successful unreserve state"
guard_expect_in_file "$TAG" 'rejectDuplicateUnreserve' "$ROUTE" "MIMAP-035A duplicate fail-fast entry missing"
guard_expect_in_file "$TAG" 'rejectStaleUnreserve' "$ROUTE" "MIMAP-035A stale fail-fast entry missing"
guard_expect_in_file "$TAG" 'result\.adapter_calls_before_duplicate = me\.unreserve_route\.unreserve_adapter\.call_count' "$ROUTE" "MIMAP-035A must snapshot adapter count before duplicate"
guard_expect_in_file "$TAG" 'result\.adapter_calls_after_duplicate = me\.unreserve_route\.unreserve_adapter\.call_count' "$ROUTE" "MIMAP-035A must prove no duplicate adapter call"
guard_expect_in_file "$TAG" 'result\.adapter_calls_before_stale = me\.unreserve_route\.unreserve_adapter\.call_count' "$ROUTE" "MIMAP-035A must snapshot adapter count before stale"
guard_expect_in_file "$TAG" 'result\.adapter_calls_after_stale = me\.unreserve_route\.unreserve_adapter\.call_count' "$ROUTE" "MIMAP-035A must prove no stale adapter call"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.unreservePage\(base, bytes\)' "$UNRESERVE_ADAPTER" "MIMAP-033A adapter remains the page-source unreserve owner"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unreserve_failfast_box = "memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako"' "$MODULE" "hako module must export MIMAP-035A route"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_backing_set_box = "memory/object_lifecycle_facade_huge_backing_set_box.hako"' "$MODULE" "hako module must export MIMAP-037A helper"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unreserve_failfast_box.hako' "$README" "memory README must name MIMAP-035A owner"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_backing_set_box.hako' "$README" "memory README must name MIMAP-037A helper"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute' "$ROOT_README" "root hako_alloc README must name MIMAP-035A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeBackingSet' "$ROOT_README" "root hako_alloc README must name MIMAP-037A helper"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-035A card must be landed after implementation"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-035A guard"

if rg -n '\.unreservePage[[:space:]]*\(' "$ROUTE" "$APP" >/tmp/"$TAG".direct_adapter_call 2>&1; then
  echo "[$TAG] ERROR: MIMAP-035A must not call the page-source unreserve adapter directly on fail-fast paths" >&2
  cat /tmp/"$TAG".direct_adapter_call >&2
  rm -f /tmp/"$TAG".direct_adapter_call
  exit 1
fi
rm -f /tmp/"$TAG".direct_adapter_call

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|(^|[^A-Za-z0-9_])reservePage[[:space:]]*\(|(^|[^A-Za-z0-9_])commitPage[[:space:]]*\(|(^|[^A-Za-z0-9_])decommitPage[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".page_source_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-035A route/app must not directly call page-source or OSVM APIs" >&2
  cat /tmp/"$TAG".page_source_leak >&2
  rm -f /tmp/"$TAG".page_source_leak
  exit 1
fi
rm -f /tmp/"$TAG".page_source_leak

if rg -n 'recommit[A-Za-z0-9_]*[[:space:]]*\(|purge[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-035A leaked behavior beyond facade huge unreserve fail-fast diagnostics" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-huge-unreserve-failfast|HakoAllocObjectLifecycleFacadeHugeUnreserveFailfast|object_lifecycle_facade_huge_unreserve_failfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-035A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap035a_facade_huge_unreserve_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap035a.mir.json"
exe_out="$tmp_dir/mimap035a.exe"
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
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.proveHugeUnreserveFailfast/4",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.hugeUnreserveFailfastThreshold/0",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.markSuccessfulUnreserve/2",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.rejectDuplicateUnreserve/1",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.rejectStaleUnreserve/3",
    "HakoAllocObjectLifecycleFacadeHugeBackingSet.length/0",
    "HakoAllocObjectLifecycleFacadeHugeBackingSet.find/2",
    "HakoAllocObjectLifecycleFacadeHugeBackingSet.has/2",
    "HakoAllocObjectLifecycleFacadeHugeBackingSet.mark/2",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute.allocateUnregisterDecommitUnreserveHuge/2",
    "HakoAllocPageSourceUnreserveAdapter.unreservePage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastReport",
    "HakoAllocObjectLifecycleFacadeHugeBackingSet",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute",
    "HakoAllocPageSourceUnreserveAdapter",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute"].get("fields", [])
}
for name, declared in (
    ("unreserve_route", "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute"),
    ("unreserved_backings", "HakoAllocObjectLifecycleFacadeHugeBackingSet"),
):
    field = route_fields.get(name)
    if field is None or field.get("declared_type") != declared or field.get("storage") != "handle":
        raise SystemExit(f"bad failfast route field {name}: {field}")

helper_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadeHugeBackingSet"].get("fields", [])
}
for name in ("bases", "bytes_values"):
    field = helper_fields.get(name)
    if field is None or field.get("declared_type") != "ArrayBox" or field.get("storage") != "handle":
        raise SystemExit(f"bad backing-set helper field {name}: {field}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastReport"].get("fields", [])
}
for field in (
    "first_status",
    "first_source_base",
    "first_source_bytes",
    "first_unreserve_ok",
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
    "no_direct_page_source",
    "no_recommit",
    "no_provider",
):
    if field not in report_fields:
        raise SystemExit(f"missing huge-unreserve failfast report field: {field}")

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

route_fn = "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.proveHugeUnreserveFailfast/4"
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveRoute", "allocateUnregisterDecommitUnreserveHuge")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute", "markSuccessfulUnreserve")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute", "rejectDuplicateUnreserve")
require_method(route_fn, "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute", "rejectStaleUnreserve")
for fn_name in (
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.rejectDuplicateUnreserve/1",
    "HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute.rejectStaleUnreserve/3",
):
    for callee in iter_calls(functions[fn_name]):
        name = callee.get("name") or ""
        box = callee.get("box_name") or ""
        if name == "unreservePage":
            raise SystemExit(f"failfast path must not call unreservePage: {fn_name} -> {box}.{name}")

print("[mimap035a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-huge-unreserve-failfast-proof' "$run_log"
rg -F -q 'first=1,0,' "$run_log"
rg -F -q ',4194305,0' "$run_log"
rg -F -q 'huge=70000,1000' "$run_log"
rg -F -q 'decommit=1,1' "$run_log"
rg -F -q 'unreserve_first=1,1' "$run_log"
rg -F -q 'adapter_first=1,1,0' "$run_log"
rg -F -q 'marker=0,1' "$run_log"
rg -F -q 'duplicate=1,1,5,' "$run_log"
rg -F -q ',4194305,1,1' "$run_log"
rg -F -q 'stale=1,1,7,99999,4194305,1,1' "$run_log"
rg -F -q 'route_counts=2,1,1,1,0' "$run_log"
rg -F -q 'stop=1,1,1' "$run_log"
rg -F -q 'final=1,1,0' "$run_log"
rg -F -q 'shape=47' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"
echo "[$TAG] ok"
