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
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.osvm_backed_fast_path_heap_box = "memory/osvm_backed_fast_path_heap_box.hako"' "$MODULE" "hako module must export M168 heap adapter"
guard_expect_in_file "$TAG" 'box HakoAllocOsVmBackedFastPathHeap' "$HEAP" "M168 adapter must own OSVM-backed orchestration"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.reservePage' "$HEAP" "M168 adapter must reserve through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.commitPage' "$HEAP" "M168 adapter must commit through page-source policy"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy.decommitPage' "$HEAP" "M168 adapter must decommit through page-source policy"
guard_expect_in_file "$TAG" 'me\.queue\.addPage\(page\)' "$HEAP" "M168 adapter must register backed pages through the queue owner"
guard_expect_in_file "$TAG" 'new HakoAllocPageModel' "$HEAP" "M168 adapter must still create page-local models"
guard_expect_in_file "$TAG" 'M168 OSVM page source composition' "$PLAN" "plan must retain M168 row"
guard_expect_in_file "$TAG" '293x-176 M168 Mimalloc OSVM Page-Source Composition' "$CARD" "missing M168 card"
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

if rg -n ': usize|HakoAllocUsizeFieldProbe|usize_field_probe' "$HEAP" "$APP" >/tmp/"$TAG".usize 2>&1; then
  echo "[$TAG] ERROR: M168 must not expand production usize field migration" >&2
  cat /tmp/"$TAG".usize >&2
  rm -f /tmp/"$TAG".usize
  exit 1
fi
rm -f /tmp/"$TAG".usize

if rg -n 'Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook|provider' "$HEAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M169+/M170+ or provider/hook ownership leaked into M168" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes' \
  src lang/c-abi/shims crates/nyash_kernel lang/src >/tmp/"$TAG".inactive_osvm_rows 2>&1; then
  echo "[$TAG] ERROR: OSVM unreserve/release rows must stay inactive in M168" >&2
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
    ("HakoAllocPageSourcePolicy.reservePage/1", "OsVmCoreBox.reserve_bytes_i64/1"),
    ("HakoAllocPageSourcePolicy.commitPage/2", "OsVmCoreBox.commit_bytes_i64/2"),
    ("HakoAllocPageSourcePolicy.decommitPage/2", "OsVmCoreBox.decommit_bytes_i64/2"),
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
