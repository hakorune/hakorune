#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-osvm-page-source-composition"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

HEAP="lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako"
FAST_HEAP="lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako"
PAGE_SOURCE_POLICY="lang/src/hako_alloc/memory/page_source_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-osvm-page-source-composition-proof/main.hako"
APP_TEST="apps/mimalloc-osvm-page-source-composition-proof/test.sh"
APP_README="apps/mimalloc-osvm-page-source-composition-proof/README.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-176-M168-MIMALLOC-OSVM-PAGE-SOURCE-COMPOSITION.md"
USIZE_HANDLE_SIZE_CARD="docs/development/current/main/phases/phase-294x/294x-52-HAKO-ALLOC-USIZE-OSVM-BACKED-HANDLE-REQUESTED-SIZE.md"
USIZE_OSVM_BYTE_CARD="docs/development/current/main/phases/phase-294x/294x-54-HAKO-ALLOC-USIZE-OSVM-BACKED-BYTE-LENGTH-SEAM.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh"

echo "[$TAG] running M168 mimalloc OSVM page-source composition guard"

guard_require_files \
  "$TAG" \
  "$HEAP" \
  "$FAST_HEAP" \
  "$PAGE_SOURCE_POLICY" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$PLAN" \
  "$CARD" \
  "$USIZE_HANDLE_SIZE_CARD" \
  "$USIZE_OSVM_BYTE_CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.osvm_backed_fast_path_heap_box = "memory/osvm_backed_fast_path_heap_box.hako"' "$MODULE" "hako module must export M168 heap adapter"
guard_expect_in_file "$TAG" 'box HakoAllocOsVmBackedFastPathHeap' "$HEAP" "M168 adapter must own OSVM-backed orchestration"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.reservePage' "$HEAP" "M168 adapter must reserve through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.commitPage' "$HEAP" "M168 adapter must commit through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.decommitPage' "$HEAP" "M168 adapter must decommit through page-source policy"
guard_expect_in_file "$TAG" 'reserve_bytes_usize' "$PAGE_SOURCE_POLICY" "page-source policy must use exact usize reserve facade"
guard_expect_in_file "$TAG" 'commit_bytes_usize' "$PAGE_SOURCE_POLICY" "page-source policy must use exact usize commit facade"
guard_expect_in_file "$TAG" 'decommit_bytes_usize' "$PAGE_SOURCE_POLICY" "page-source policy must use exact usize decommit facade"
guard_expect_in_file "$TAG" 'me\.queue\.addPage\(page\)' "$HEAP" "M168 adapter must register backed pages through the queue owner"
guard_expect_in_file "$TAG" 'new HakoAllocPageModel' "$HEAP" "M168 adapter must still create page-local models"
guard_expect_in_file "$TAG" 'bin: i64' "$HEAP" "bin must remain signed route/index metadata"
guard_expect_in_file "$TAG" 'block_size: usize' "$HEAP" "block_size must be exact usize size-class metadata"
guard_expect_in_file "$TAG" 'page_capacity: usize' "$HEAP" "page_capacity must be exact usize capacity metadata"
guard_expect_fixed_in_file "$TAG" 'birth(bin, block_size: usize, page_capacity: usize)' "$HEAP" "OSVM-backed heap birth must carry exact size/capacity parameters"
guard_expect_in_file "$TAG" 'next_page_id: i64 = 0' "$HEAP" "next_page_id must remain signed index metadata"
guard_expect_in_file "$TAG" 'backing_count: usize = 0' "$HEAP" "backing_count must be exact usize backing-array length storage"
guard_expect_in_file "$TAG" 'alloc_count: usize = 0' "$HEAP" "alloc accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'release_count: usize = 0' "$HEAP" "release accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'fallback_count: usize = 0' "$HEAP" "fallback accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'page_create_count: usize = 0' "$HEAP" "page creation accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$HEAP" "reject accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'reserve_count: usize = 0' "$HEAP" "reserve accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'commit_count: usize = 0' "$HEAP" "commit accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'decommit_count: usize = 0' "$HEAP" "decommit accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'source_reject_count: usize = 0' "$HEAP" "source reject accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'requested_size: usize' "$HEAP" "OSVM-backed handle requested_size must be exact usize"
guard_expect_fixed_in_file "$TAG" 'birth(page_id, block_id, requested_size: usize)' "$HEAP" "OSVM-backed handle birth must carry exact requested_size"
guard_expect_in_file "$TAG" 'bytes: usize' "$HEAP" "OSVM page backing bytes must be exact usize"
guard_expect_in_file "$TAG" 'M168 OSVM page source composition' "$PLAN" "plan must retain M168 row"
guard_expect_in_file "$TAG" '293x-176 M168 Mimalloc OSVM Page-Source Composition' "$CARD" "missing M168 card"
guard_expect_in_file "$TAG" '294x-52 Hako Alloc Usize OSVM Backed Handle Requested Size' "$USIZE_HANDLE_SIZE_CARD" "missing 294x-52 usize OSVM-backed handle requested-size card"
guard_expect_in_file "$TAG" '294x-54 Hako Alloc Usize OSVM Backed Byte-Length Seam' "$USIZE_OSVM_BYTE_CARD" "missing 294x-54 usize OSVM-backed byte-length seam card"
guard_expect_in_file "$TAG" 'scalar-return proof seam' "$CARD" "M168 card must document addFreshPage as a proof-only scalar seam"
guard_expect_in_file "$TAG" 'semantic allocator API' "$CARD" "M168 card must preserve allocate(size) as the semantic API"
guard_expect_in_file "$TAG" 'object-return allocation surface' "$CARD" "M168 card must preserve object-return allocation semantics"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M168 guard"

if rg -n 'init[[:space:]]*\\{' "$HEAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M168 heap adapter must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'OSVM|OsVm|page_source|PageSource|reservePage|commitPage|decommitPage' "$FAST_HEAP" >/tmp/"$TAG".m167_leak 2>&1; then
  echo "[$TAG] ERROR: M168 OSVM sourcing leaked into the M167 fast-path heap" >&2
  cat /tmp/"$TAG".m167_leak >&2
  rm -f /tmp/"$TAG".m167_leak
  exit 1
fi
rm -f /tmp/"$TAG".m167_leak

if rg -n 'HakoAllocUsizeFieldProbe|usize_field_probe' "$HEAP" "$APP" >/tmp/"$TAG".usize_probe 2>&1; then
  echo "[$TAG] ERROR: M168 must not depend on the usize probe owner" >&2
  cat /tmp/"$TAG".usize_probe >&2
  rm -f /tmp/"$TAG".usize_probe
  exit 1
fi
rm -f /tmp/"$TAG".usize_probe

if rg -n ': usize' "$APP" >/tmp/"$TAG".usize_app 2>&1; then
  echo "[$TAG] ERROR: M168 proof app must not introduce extra usize locals or fields" >&2
  cat /tmp/"$TAG".usize_app >&2
  rm -f /tmp/"$TAG".usize_app
  exit 1
fi
rm -f /tmp/"$TAG".usize_app

if rg -n 'Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook|provider' "$HEAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M169+/M170+ or provider/hook ownership leaked into M168" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes' \
  "$HEAP" "$APP" >/tmp/"$TAG".inactive_osvm_rows 2>&1; then
  echo "[$TAG] ERROR: M168 heap/app must not own OSVM unreserve/release behavior" >&2
  cat /tmp/"$TAG".inactive_osvm_rows >&2
  rm -f /tmp/"$TAG".inactive_osvm_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_osvm_rows

if rg -n 'mimalloc-osvm-page-source-composition|HakoAllocOsVmBackedFastPathHeap|osvm_backed_fast_path' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M168 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m168_osvm_page_source.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m168.mir.json"
exe_out="$tmp_dir/m168.exe"
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
    "HakoAllocOsVmBackedFastPathHeap.addBackedPage/0",
    "HakoAllocOsVmBackedFastPathHeap.addFreshPage/0",
    "HakoAllocOsVmBackedFastPathHeap.decommitPage/1",
    "HakoAllocOsVmBackedFastPathHeap.decommitAll/0",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.reserve_bytes_usize/1",
    "OsVmCoreBox.commit_bytes_usize/2",
    "OsVmCoreBox.decommit_bytes_usize/2",
    "OsVmCoreBox.reserve_bytes_i64/1",
    "OsVmCoreBox.commit_bytes_i64/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in (
    "HakoAllocOsVmBackedFastPathHeap",
    "HakoAllocOsVmBackedHandle",
    "HakoAllocOsVmPageBacking",
):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

heap_fields = {
    field.get("name"): field
    for field in plans["HakoAllocOsVmBackedFastPathHeap"].get("fields", [])
}
for field_name in (
    "block_size",
    "page_capacity",
    "alloc_count",
    "release_count",
    "fallback_count",
    "page_create_count",
    "reject_count",
    "reserve_count",
    "commit_count",
    "decommit_count",
    "source_reject_count",
):
    field = heap_fields.get(field_name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"osvm-backed fast path heap {field_name} must be exact usize storage: {field}")

for field_name in ("bin", "next_page_id"):
    field = heap_fields.get(field_name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"osvm-backed fast path heap {field_name} must remain i64 storage: {field}")

field = heap_fields.get("backing_count")
if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
    raise SystemExit(f"osvm-backed fast path heap backing_count must be exact usize storage: {field}")

for box_name, field_names in (
    ("HakoAllocOsVmBackedHandle", ("page_id", "block_id", "requested_size")),
    ("HakoAllocOsVmPageBacking", ("page_id", "base", "bytes")),
):
    fields = {
        field.get("name"): field
        for field in plans[box_name].get("fields", [])
    }
    for field_name in field_names:
        field = fields.get(field_name)
        if box_name == "HakoAllocOsVmBackedHandle" and field_name == "requested_size":
            if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
                raise SystemExit(f"{box_name}.{field_name} must be exact usize storage: {field}")
            continue
        if field_name == "bytes":
            if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
                raise SystemExit(f"{box_name}.{field_name} must be exact usize storage: {field}")
            continue
        if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
            raise SystemExit(f"{box_name}.{field_name} must remain i64 storage: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_global(owner_name, symbol):
    routes = functions[owner_name].get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return
    raise SystemExit(f"missing generic-i64 route in {owner_name} -> {symbol}: {routes}")

for owner_name, symbol in (
    ("HakoAllocOsVmBackedFastPathHeap.addBackedPage/0", "HakoAllocPageSourcePolicy.reservePage/1"),
    ("HakoAllocOsVmBackedFastPathHeap.addBackedPage/0", "HakoAllocPageSourcePolicy.commitPage/2"),
    ("HakoAllocOsVmBackedFastPathHeap.decommitPage/1", "HakoAllocPageSourcePolicy.decommitPage/2"),
    ("HakoAllocPageSourcePolicy.reservePage/1", "OsVmCoreBox.reserve_bytes_usize/1"),
    ("HakoAllocPageSourcePolicy.commitPage/2", "OsVmCoreBox.commit_bytes_usize/2"),
    ("HakoAllocPageSourcePolicy.decommitPage/2", "OsVmCoreBox.decommit_bytes_usize/2"),
):
    require_global(owner_name, symbol)

main = functions["main"]
method_calls = {
    (callee.get("box_name"), callee.get("name"))
    for callee in iter_calls(main)
    if callee.get("type") == "Method"
}
for method in (
    ("HakoAllocOsVmBackedFastPathHeap", "addFreshPage"),
    ("HakoAllocOsVmBackedFastPathHeap", "pageBase"),
    ("HakoAllocOsVmBackedFastPathHeap", "pageBackingBytes"),
    ("HakoAllocOsVmBackedFastPathHeap", "decommitAll"),
):
    if method not in method_calls:
        raise SystemExit(f"missing main method call: {method}")

print("[m168-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-osvm-page-source-composition-proof' "$run_log"
rg -F -q 'page_ids=0,1' "$run_log"
rg -F -q 'heap_counts=0,0,0,2,0' "$run_log"
rg -F -q 'queue_counts=2,2' "$run_log"
rg -F -q 'source_counts=2,2,2,0' "$run_log"
rg -F -q 'cleanup=1' "$run_log"
rg -F -q 'shape=10' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"
