#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-queue"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

QUEUE_BOX="lang/src/hako_alloc/memory/page_queue_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-queue-proof/main.hako"
APP_TEST="apps/mimalloc-page-queue-proof/test.sh"
APP_README="apps/mimalloc-page-queue-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-167-M166-MIMALLOC-PAGE-QUEUE-DIRECT-CACHE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_queue_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_queue.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_queue.err"

echo "[$TAG] checking M166 mimalloc page queue/direct-cache"

guard_require_files \
  "$TAG" \
  "$QUEUE_BOX" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'box HakoAllocPageQueue' "$QUEUE_BOX" "HakoAllocPageQueue must own page selection"
guard_expect_in_file "$TAG" 'pages: ArrayBox = new ArrayBox\(\)' "$QUEUE_BOX" "page queue must initialize pages as a stored member"
guard_expect_in_file "$TAG" 'direct_page_index: i64 = -1' "$QUEUE_BOX" "page queue must initialize direct-page cache state"
guard_expect_in_file "$TAG" 'selectPage' "$QUEUE_BOX" "page queue must expose selectPage"
guard_expect_in_file "$TAG" 'refreshDirectPage' "$QUEUE_BOX" "page queue must expose refreshDirectPage"
guard_expect_in_file "$TAG" 'freeCount' "$QUEUE_BOX" "page queue must observe page availability only"
guard_expect_in_file "$TAG" 'memory.page_queue_box = "memory/page_queue_box.hako"' "$MODULE" "hako module must export page_queue_box"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_queue_box as HakoAllocPageQueueBox' "$APP" "proof app must import page_queue_box"
guard_expect_in_file "$TAG" 'M166 page queue and direct-page cache' "$PLAN" "plan must retain M166 row"
guard_expect_in_file "$TAG" '293x-167 M166 Mimalloc Page Queue Direct Cache' "$CARD" "missing M166 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M166 guard"

if rg -n 'init[[:space:]]*\\{' "$QUEUE_BOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: new page queue must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n '\.acquire\(' "$QUEUE_BOX" >/tmp/"$TAG".acquire 2>&1; then
  echo "[$TAG] ERROR: M166 queue must choose pages, not pop allocation blocks" >&2
  cat /tmp/"$TAG".acquire >&2
  rm -f /tmp/"$TAG".acquire
  exit 1
fi
rm -f /tmp/"$TAG".acquire

if rg -n 'OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook' "$QUEUE_BOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M166+ or substrate ownership leaked into page queue" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M166 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-page-queue|HakoAllocPageQueue|page_queue_box|direct_page_index' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: page queue matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-page-queue-proof$' "$OUT"
grep -q '^entries=0,1,2$' "$OUT"
grep -q '^ids=10,11,-1,12$' "$OUT"
grep -q '^direct=2,12$' "$OUT"
grep -q '^counts=3,4,2,2,1$' "$OUT"
grep -q '^shape=9$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"

echo "[$TAG] ok"
