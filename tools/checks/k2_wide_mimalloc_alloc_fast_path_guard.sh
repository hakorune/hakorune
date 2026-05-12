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
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'memory.alloc_fast_path_heap_box = "memory/alloc_fast_path_heap_box.hako"' "$MODULE" "hako module must export alloc fast path heap"
guard_expect_in_file "$TAG" 'box HakoAllocFastPathHeap' "$FAST_HEAP" "fast path heap must own M167 orchestration"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$FAST_HEAP" "fast path heap must compose page model"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_queue_box as HakoAllocPageQueueBox' "$FAST_HEAP" "fast path heap must compose page queue"
guard_expect_in_file "$TAG" 'me.queue.selectPage()' "$FAST_HEAP" "fast path must select pages through the queue owner"
guard_expect_in_file "$TAG" 'page\.acquire\(size\)' "$FAST_HEAP" "fast path must pop blocks through the page owner"
guard_expect_in_file "$TAG" 'fallback_count: i64 = 0' "$FAST_HEAP" "fallback accounting must be explicit"
guard_expect_in_file "$TAG" 'M167 alloc fast path plus generic fallback' "$PLAN" "plan must retain M167 row"
guard_expect_in_file "$TAG" '293x-175 M167 Mimalloc Alloc Fast Path' "$CARD" "missing M167 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M167 guard"

if rg -n 'init[[:space:]]*\\{' "$FAST_HEAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M167 heap must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n ': usize|HakoAllocUsizeFieldProbe|usize_field_probe' "$FAST_HEAP" "$APP" >/tmp/"$TAG".usize 2>&1; then
  echo "[$TAG] ERROR: M167 production algorithm must stay on current i64 lane; usize probe remains isolated" >&2
  cat /tmp/"$TAG".usize >&2
  rm -f /tmp/"$TAG".usize
  exit 1
fi
rm -f /tmp/"$TAG".usize

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

echo "[$TAG] ok"
