#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-alloc-fast-path"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

FAST_HEAP="lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
QUEUE_BOX="lang/src/hako_alloc/memory/page_queue_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-alloc-fast-path-proof/main.hako"
APP_TEST="apps/mimalloc-alloc-fast-path-proof/test.sh"
APP_README="apps/mimalloc-alloc-fast-path-proof/README.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-175-M167-MIMALLOC-ALLOC-FAST-PATH.md"
USIZE_SIZE_CARD="docs/development/current/main/phases/phase-294x/294x-50-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-SIZE-CAPACITY.md"
USIZE_HANDLE_SIZE_CARD="docs/development/current/main/phases/phase-294x/294x-51-HAKO-ALLOC-USIZE-FAST-PATH-HANDLE-REQUESTED-SIZE.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_alloc_fast_path.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_alloc_fast_path.err"

echo "[$TAG] checking M167 mimalloc alloc fast path"

guard_require_files \
  "$TAG" \
  "$FAST_HEAP" \
  "$PAGE_BOX" \
  "$QUEUE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$PLAN" \
  "$CARD" \
  "$USIZE_SIZE_CARD" \
  "$USIZE_HANDLE_SIZE_CARD" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'memory.alloc_fast_path_heap_box = "memory/alloc_fast_path_heap_box.hako"' "$MODULE" "hako module must export alloc fast path heap"
guard_expect_in_file "$TAG" 'box HakoAllocFastPathHeap' "$FAST_HEAP" "fast path heap must own M167 orchestration"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$FAST_HEAP" "fast path heap must compose page model"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_queue_box as HakoAllocPageQueueBox' "$FAST_HEAP" "fast path heap must compose page queue"
guard_expect_in_file "$TAG" 'me.queue.selectPage()' "$FAST_HEAP" "fast path must select pages through the queue owner"
guard_expect_in_file "$TAG" 'page\.acquire\(size\)' "$FAST_HEAP" "fast path must pop blocks through the page owner"
guard_expect_in_file "$TAG" 'bin: i64' "$FAST_HEAP" "bin must remain signed route/index metadata"
guard_expect_in_file "$TAG" 'block_size: usize' "$FAST_HEAP" "block_size must be exact usize size-class metadata"
guard_expect_in_file "$TAG" 'page_capacity: usize' "$FAST_HEAP" "page_capacity must be exact usize capacity metadata"
guard_expect_fixed_in_file "$TAG" 'birth(bin, block_size: usize, page_capacity: usize)' "$FAST_HEAP" "fast path heap birth must carry exact size/capacity parameters"
guard_expect_in_file "$TAG" 'requested_size: usize' "$FAST_HEAP" "fast path handle requested_size must be exact usize"
guard_expect_fixed_in_file "$TAG" 'birth(page_id, block_id, requested_size: usize)' "$FAST_HEAP" "fast path handle birth must carry exact requested_size"
guard_expect_in_file "$TAG" 'next_page_id: i64 = 0' "$FAST_HEAP" "next_page_id must remain signed index metadata"
guard_expect_in_file "$TAG" 'alloc_count: usize = 0' "$FAST_HEAP" "alloc accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'release_count: usize = 0' "$FAST_HEAP" "release accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'fallback_count: usize = 0' "$FAST_HEAP" "fallback accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'page_create_count: usize = 0' "$FAST_HEAP" "page creation accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$FAST_HEAP" "reject accounting must be exact usize storage"
guard_expect_in_file "$TAG" 'M167 alloc fast path plus generic fallback' "$PLAN" "plan must retain M167 row"
guard_expect_in_file "$TAG" '293x-175 M167 Mimalloc Alloc Fast Path' "$CARD" "missing M167 card"
guard_expect_in_file "$TAG" '294x-50 Hako Alloc Usize Fast Path Heap Size Capacity' "$USIZE_SIZE_CARD" "missing 294x-50 usize size/capacity card"
guard_expect_in_file "$TAG" '294x-51 Hako Alloc Usize Fast Path Handle Requested Size' "$USIZE_HANDLE_SIZE_CARD" "missing 294x-51 usize handle requested-size card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M167 guard"

if rg -n 'init[[:space:]]*\\{' "$FAST_HEAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M167 heap must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'HakoAllocUsizeFieldProbe|usize_field_probe' "$FAST_HEAP" "$APP" >/tmp/"$TAG".usize_probe 2>&1; then
  echo "[$TAG] ERROR: M167 production algorithm must not depend on the usize probe owner" >&2
  cat /tmp/"$TAG".usize_probe >&2
  rm -f /tmp/"$TAG".usize_probe
  exit 1
fi
rm -f /tmp/"$TAG".usize_probe

if rg -n ': usize' "$APP" >/tmp/"$TAG".usize_app 2>&1; then
  echo "[$TAG] ERROR: M167 proof app must not introduce extra usize locals or fields" >&2
  cat /tmp/"$TAG".usize_app >&2
  rm -f /tmp/"$TAG".usize_app
  exit 1
fi
rm -f /tmp/"$TAG".usize_app

if rg -n 'OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook|provider' "$FAST_HEAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M168+ or provider/hook ownership leaked into M167" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M167 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-alloc-fast-path|HakoAllocFastPath|alloc_fast_path_heap' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: alloc fast path matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-alloc-fast-path-proof$' "$OUT"
grep -q '^handles=0:1,0:0,1:1$' "$OUT"
grep -q '^release=1,0$' "$OUT"
grep -q '^heap_counts=3,1,1,2,2$' "$OUT"
grep -q '^queue_counts=2,3,2,1,1$' "$OUT"
grep -q '^totals=2,48$' "$OUT"
grep -q '^shape=12$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"

MIR_JSON="${TMPDIR:-/tmp}/hakorune_mimalloc_alloc_fast_path.mir.json"
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend mir --emit-mir-json "$MIR_JSON" "$ROOT_DIR/$APP" >/tmp/"$TAG".mir.out 2>/tmp/"$TAG".mir.err

python3 - "$MIR_JSON" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocFastPathHeap", "HakoAllocFastPathHandle", "HakoAllocPageQueue", "HakoAllocPageModel"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

heap_fields = {
    field.get("name"): field
    for field in plans["HakoAllocFastPathHeap"].get("fields", [])
}
for field_name in (
    "block_size",
    "page_capacity",
    "alloc_count",
    "release_count",
    "fallback_count",
    "page_create_count",
    "reject_count",
):
    field = heap_fields.get(field_name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"fast path heap {field_name} must be exact usize storage: {field}")

for field_name in ("bin", "next_page_id"):
    field = heap_fields.get(field_name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"fast path heap {field_name} must remain i64 storage: {field}")

handle_fields = {
    field.get("name"): field
    for field in plans["HakoAllocFastPathHandle"].get("fields", [])
}
for field_name in ("page_id", "block_id", "requested_size"):
    field = handle_fields.get(field_name)
    if field_name == "requested_size":
        if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
            raise SystemExit(f"fast path handle {field_name} must be exact usize storage: {field}")
        continue
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"fast path handle {field_name} must remain i64 storage: {field}")
PY

echo "[$TAG] ok"
