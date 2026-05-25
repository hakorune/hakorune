#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-mimalloc-facade-page-source-alloc-miss-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako"
APP_TEST="apps/mimalloc-facade-page-source-alloc-miss-proof/test.sh"
APP_README="apps/mimalloc-facade-page-source-alloc-miss-proof/README.md"
FALLBACK="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"
ADAPTER="lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-384-MIMAP-021C-FACADE-PAGE-SOURCE-ALLOC-MISS-FALLBACK.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
MODULE_INDEX="lang/src/hako_alloc/memory/MODULE_INDEX.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh"
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
DIRECT_SOURCE_LOG="$ARTIFACT_DIR/direct_source.log"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
INC_LOG="$ARTIFACT_DIR/inc_leak.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$DIRECT_SOURCE_LOG" "$FORBIDDEN_LOG" "$INC_LOG"

echo "[$TAG] running MIMAP-021C facade page-source alloc-miss fallback guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$FALLBACK" \
  "$ADAPTER" \
  "$FACADE" \
  "$REASON" \
  "$PAGE_SOURCE" \
  "$PAGE" \
  "$MODULE" \
  "$CARD" \
  "$INDEX" \
  "$README" \
  "$MODULE_INDEX"

guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback' "$FALLBACK" "MIMAP-021C fallback owner missing"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadePageSourceAllocMissReport' "$FALLBACK" "MIMAP-021C report owner missing"
guard_expect_in_file "$TAG" 'reserve_count: usize = 0' "$ADAPTER" "attach reserve counter must be exact usize"
guard_expect_in_file "$TAG" 'commit_count: usize = 0' "$ADAPTER" "attach commit counter must be exact usize"
guard_expect_in_file "$TAG" 'attach_count: usize = 0' "$ADAPTER" "attach counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$ADAPTER" "attach reject counter must be exact usize"
guard_expect_in_file "$TAG" 'facade\.objectLifecycleSmallAlloc\(size\)' "$FALLBACK" "fallback must attempt/retry facade small allocation"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeReason\.small_no_page\(\)' "$FALLBACK" "fallback must gate only on small_no_page"
guard_expect_in_file "$TAG" 'new HakoAllocObjectLifecycleFacadePageSourceAttach\(\)' "$FALLBACK" "fallback must reuse MIMAP-021B attach adapter"
guard_expect_in_file "$TAG" 'attach\.attachFreshPage\(facade, page_id, block_size, capacity, reserved\)' "$FALLBACK" "fallback must attach one fresh page through adapter"
guard_expect_in_file "$TAG" 'fallback_attempt_count: usize = 0' "$FALLBACK" "fallback attempt counter must be exact usize"
guard_expect_in_file "$TAG" 'source_success_count: usize = 0' "$FALLBACK" "source success counter must be exact usize"
guard_expect_in_file "$TAG" 'source_failure_count: usize = 0' "$FALLBACK" "source failure counter must be exact usize"
guard_expect_in_file "$TAG" 'retry_success_count: usize = 0' "$FALLBACK" "retry success counter must be exact usize"
guard_expect_in_file "$TAG" 'retry_failure_count: usize = 0' "$FALLBACK" "retry failure counter must be exact usize"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_page_source_alloc_miss_box = "memory/object_lifecycle_facade_page_source_alloc_miss_box.hako"' "$MODULE" "hako module must export MIMAP-021C fallback"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_page_source_alloc_miss_box.hako' "$MODULE_INDEX" "memory module index must name MIMAP-021C owner"
guard_expect_in_file "$TAG" 'MIMAP-021C' "$CARD" "MIMAP-021C card missing"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-021C guard"

if rg -n 'HakoAllocPageSourcePolicy|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|OsVm|OSVM|externcall' \
  "$FALLBACK" "$APP" >"$DIRECT_SOURCE_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C must reuse the 021B attach adapter instead of direct page-source/OSVM calls" >&2
  cat "$DIRECT_SOURCE_LOG" >&2
  rm -f "$DIRECT_SOURCE_LOG"
  exit 1
fi
rm -f "$DIRECT_SOURCE_LOG"

if rg -n 'objectLifecycleRelease|objectLifecycleRealloc|objectLifecycleSmallAllocAligned|HakoAllocObjectLifecycleFacadePurge|HakoAllocBoundedPurge|HakoAllocPurgeState|decommitPage[[:space:]]*\(|recommit[A-Za-z0-9_]*[[:space:]]*\(|unreserve|releasePage|PageMap|page_map|lookup[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(|Tls|Atomic|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$FALLBACK" "$APP" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C leaked behavior beyond alloc-miss fallback" >&2
  cat "$FORBIDDEN_LOG" >&2
  rm -f "$FORBIDDEN_LOG"
  exit 1
fi
rm -f "$FORBIDDEN_LOG"

if rg -n 'mimalloc-facade-page-source-alloc-miss|HakoAllocObjectLifecycleFacadePageSourceAllocMiss|allocateOnMiss' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-021C matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap021c.mir.json"
exe_out="$TMP_DIR/mimap021c.exe"
build_log="$TMP_DIR/build.log"
run_log="$TMP_DIR/run.log"

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
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.snapshotCounters/1",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.copyFinalAllocation/2",
    "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocOk/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocBlockId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecyclePageCount/0",
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
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback",
    "HakoAllocObjectLifecycleFacadePageSourceAllocMissReport",
    "HakoAllocObjectLifecycleFacadePageSourceAttach",
    "HakoAllocObjectLifecycleFacadePageSourceAttachReport",
    "HakoAllocObjectLifecycleFacade",
    "HakoAllocObjectLifecyclePageQueue",
    "HakoAllocPageModel",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

attach_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAttach"].get("fields", [])
}
for field in ("reserve_count", "commit_count", "attach_count", "reject_count"):
    attach_field = attach_fields.get(field)
    if attach_field is None or attach_field.get("declared_type") != "usize" or attach_field.get("storage") != "usize":
        raise SystemExit(f"attach {field} must be exact usize storage: {attach_field}")

attach_report_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAttachReport"].get("fields", [])
}
for field in (
    "status",
    "added_page_id",
    "base",
):
    attach_report_field = attach_report_fields.get(field)
    if attach_report_field is None or attach_report_field.get("declared_type") != "i64" or attach_report_field.get("storage") != "i64":
        raise SystemExit(f"attach report {field} must remain signed storage: {attach_report_field}")

for field in (
    "source_reserved",
    "source_committed",
    "facade_page_count",
    "source_reject",
):
    attach_report_field = attach_report_fields.get(field)
    if attach_report_field is None or attach_report_field.get("declared_type") != "usize" or attach_report_field.get("storage") != "usize":
        raise SystemExit(f"attach report mirror {field} must be exact usize storage: {attach_report_field}")

for field in (
    "bytes",
    "block_size",
    "capacity",
    "reserved",
):
    attach_report_field = attach_report_fields.get(field)
    if attach_report_field is None or attach_report_field.get("declared_type") != "usize" or attach_report_field.get("storage") != "usize":
        raise SystemExit(f"attach report payload {field} must be exact usize storage: {attach_report_field}")

fallback_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback"].get("fields", [])
}
for field in (
    "fallback_attempt_count",
    "source_success_count",
    "source_failure_count",
    "retry_success_count",
    "retry_failure_count",
):
    fallback_field = fallback_fields.get(field)
    if fallback_field is None or fallback_field.get("declared_type") != "usize" or fallback_field.get("storage") != "usize":
        raise SystemExit(f"fallback {field} must be exact usize storage: {fallback_field}")

report_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadePageSourceAllocMissReport"].get("fields", [])
}
for field in (
    "status",
    "initial_ok",
    "initial_reason",
    "fallback_attempted",
    "source_status",
    "source_added_page_id",
    "source_base",
    "retry_ok",
    "retry_reason",
    "final_ok",
    "final_reason",
    "final_page_id",
    "final_block_id",
):
    report_field = report_fields.get(field)
    if report_field is None or report_field.get("declared_type") != "i64" or report_field.get("storage") != "i64":
        raise SystemExit(f"alloc-miss report {field} must remain signed storage: {report_field}")

for field in (
    "source_reserved",
    "source_committed",
    "source_reject",
    "source_facade_page_count",
):
    report_field = report_fields.get(field)
    if report_field is None or report_field.get("declared_type") != "usize" or report_field.get("storage") != "usize":
        raise SystemExit(f"alloc-miss source mirror {field} must be exact usize storage: {report_field}")

for field in (
    "source_bytes",
):
    report_field = report_fields.get(field)
    if report_field is None or report_field.get("declared_type") != "usize" or report_field.get("storage") != "usize":
        raise SystemExit(f"alloc-miss source payload {field} must be exact usize storage: {report_field}")

for field in (
    "fallback_attempt_count",
    "source_success_count",
    "source_failure_count",
    "retry_success_count",
    "retry_failure_count",
):
    report_field = report_fields.get(field)
    if report_field is None or report_field.get("declared_type") != "usize" or report_field.get("storage") != "usize":
        raise SystemExit(f"alloc-miss report mirror {field} must be exact usize storage: {report_field}")

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

fallback_fn = "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.allocateOnMiss/6"
for name in (
    "objectLifecycleSmallAlloc",
    "objectLifecycleAllocReason",
):
    require_method(fallback_fn, "HakoAllocObjectLifecycleFacade", name)
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAttach", "attachFreshPage")
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "snapshotCounters")
require_method(fallback_fn, "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "copyFinalAllocation")

copy_fn = "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.copyFinalAllocation/2"
for name in (
    "objectLifecycleAllocOk",
    "objectLifecycleAllocReason",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
):
    require_method(copy_fn, "HakoAllocObjectLifecycleFacade", name)

attach_fn = "HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage/5"
require_global(attach_fn, "HakoAllocPageSourcePolicy.reservePage/1")
require_global(attach_fn, "HakoAllocPageSourcePolicy.commitPage/2")
require_method(attach_fn, "HakoAllocObjectLifecycleFacade", "objectLifecycleAddPage")

require_method("main", "HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback", "allocateOnMiss")
for name in (
    "objectLifecyclePageCount",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
):
    require_method("main", "HakoAllocObjectLifecycleFacade", name)

for fn_name in (fallback_fn, "main"):
    fn = functions[fn_name]
    for callee in iter_calls(fn):
        box = callee.get("box_name") or ""
        name = callee.get("name") or ""
        target = f"{box}.{name}"
        forbidden_names = {
            "objectLifecycleReleaseBlock",
            "objectLifecycleReallocShrink",
            "objectLifecycleReallocGrow",
            "objectLifecycleSmallAllocAligned",
            "decommitPage",
            "recommitPage",
            "unreserve",
            "releasePage",
        }
        if name in forbidden_names:
            raise SystemExit(f"forbidden call in {fn_name}: {target}")
        if any(part in target for part in ("PageMap", "RemoteFree", "Purge", "Atomic", "Tls", "Provider")):
            raise SystemExit(f"forbidden owner in {fn_name}: {target}")

print("[mimap021c-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"; then
  echo "[$TAG] ERROR: MIMAP-021C must not emit decommit" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-page-source-alloc-miss-proof' "$run_log"
rg -F -q 'initial=0,1' "$run_log"
rg -F -q 'fallback_attempted=1' "$run_log"
rg -F -q 'source=1,1,1,0' "$run_log"
rg -F -q 'source_page=411,1' "$run_log"
rg -F -q 'backing=' "$run_log"
rg -F -q 'retry=1,0' "$run_log"
rg -F -q 'final=1,0,411,1' "$run_log"
rg -F -q 'counts=1,1,0,1,0' "$run_log"
rg -F -q 'facade=1,411,1' "$run_log"
rg -F -q 'shape=14' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"
