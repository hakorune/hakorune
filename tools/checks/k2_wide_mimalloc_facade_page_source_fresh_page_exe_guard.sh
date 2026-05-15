#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-page-source-fresh-page-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-page-source-fresh-page-proof/main.hako"
APP_TEST="apps/mimalloc-facade-page-source-fresh-page-proof/test.sh"
APP_README="apps/mimalloc-facade-page-source-fresh-page-proof/README.md"
ADAPTER="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-383-MIMAP-021B-FACADE-PAGE-SOURCE-FRESH-PAGE-ATTACH.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-021B facade page-source fresh-page guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$ADAPTER" \
  "$FACADE" \
  "$PAGE_SOURCE" \
  "$PAGE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAttach' "$ADAPTER" "MIMAP-021B adapter owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAttachReport' "$ADAPTER" "MIMAP-021B report owner missing"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.reservePage' "$ADAPTER" "adapter must reserve through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.commitPage' "$ADAPTER" "adapter must commit through page-source policy"
guard_expect_in_file "$TAG" 'new HakoAllocPageModel' "$ADAPTER" "adapter must construct one modeled page"
guard_expect_in_file "$TAG" 'facade.objectLifecycleAddPage' "$ADAPTER" "adapter must attach through facade known-page API"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_page_source_box = "memory/object_lifecycle_facade_page_source_box.hako"' "$MODULE" "hako module must export MIMAP-021B adapter"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_page_source_box.hako' "$README" "memory README must name MIMAP-021B owner"
guard_expect_in_file "$TAG" 'MIMAP-021B' "$CARD" "MIMAP-021B card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-021B guard"

if rg -n 'objectLifecycleSmallAlloc|objectLifecycleRelease|objectLifecycleRealloc|objectLifecycleSmallAllocAligned|HakoAllocObjectLifecycleFacadePurge|HakoAllocBoundedPurge|HakoAllocPurgeState|decommitPage[[:space:]]*\(|recommit|unreserve|releasePage|PageMap|page_map|lookup[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ADAPTER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021B leaked behavior beyond fresh-page attach" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-page-source-fresh-page|HakoAllocObjectLifecycleFacadePageSource|objectLifecycleFacadePageSource' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021B matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap021b_facade_page_source.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap021b.mir.json"
exe_out="$tmp_dir/mimap021b.exe"
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
    "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5",
    "HakoAllocObjectLifecycleFacadePageSourceAttach.report/11",
    "HakoAllocObjectLifecycleFacadePageSourceAttach.reject/6",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAddPage/1",
    "HakoAllocObjectLifecycleFacade.objectLifecyclePageCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAddCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSelectPage/0",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "OsVmCoreBox.reserve_bytes_i64/1",
    "OsVmCoreBox.commit_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in (
    "HakoAllocObjectLifecycleFacadePageSourceAttach",
    "HakoAllocObjectLifecycleFacadePageSourceAttachReport",
    "HakoAllocObjectLifecycleFacade",
    "HakoAllocObjectLifecyclePageQueue",
    "HakoAllocPageModel",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

report_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAttachReport"].get("fields", [])
}
for field in (
    "status",
    "source_reserved",
    "source_committed",
    "added_page_id",
    "facade_page_count",
    "source_reject",
    "base",
    "bytes",
):
    if field not in report_fields:
        raise SystemExit(f"missing report field: {field}")

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

attach_fn = "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5"
require_global(attach_fn, "HakoAllocPageSourcePolicy.reservePage/1")
require_global(attach_fn, "HakoAllocPageSourcePolicy.commitPage/2")
require_method(attach_fn, "HakoAllocObjectLifecycleFacade", "objectLifecycleAddPage")
require_method(attach_fn, "HakoAllocObjectLifecycleFacade", "objectLifecyclePageCount")
require_method("main", "HakoAllocObjectLifecycleFacadePageSourceAttach", "attachFreshPage")
require_method("main", "HakoAllocObjectLifecycleFacade", "objectLifecycleSelectPage")

for fn_name in (attach_fn, "main"):
    fn = functions[fn_name]
    for callee in iter_calls(fn):
        box = callee.get("box_name") or ""
        name = callee.get("name") or ""
        target = f"{box}.{name}"
        forbidden_names = {
            "objectLifecycleSmallAlloc",
            "objectLifecycleReleaseBlock",
            "objectLifecycleReallocShrink",
            "objectLifecycleReallocGrow",
            "objectLifecycleSmallAllocAligned",
            "decommitPage",
            "unreserve",
            "releasePage",
        }
        if name in forbidden_names:
            raise SystemExit(f"forbidden call in {fn_name}: {target}")
        if any(part in target for part in ("PageMap", "RemoteFree", "Purge", "Atomic", "Tls")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")

print("[mimap021b-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"; then
  echo "[$TAG] ERROR: MIMAP-021B must not emit decommit" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-page-source-fresh-page-proof' "$run_log"
rg -F -q 'source_reserved=1' "$run_log"
rg -F -q 'source_committed=1' "$run_log"
rg -F -q 'source_reject=0' "$run_log"
rg -F -q 'added_page_id=310' "$run_log"
rg -F -q 'facade_page_count=1' "$run_log"
rg -F -q 'facade_add_count=1' "$run_log"
rg -F -q 'selected=310,2' "$run_log"
rg -F -q 'backing=' "$run_log"
rg -F -q 'counts=1,1,1,0' "$run_log"
rg -F -q 'shape=16' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"
