#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-local-free-retire"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
APP="apps/mimalloc-local-free-retire-proof/main.hako"
APP_TEST="apps/mimalloc-local-free-retire-proof/test.sh"
APP_README="apps/mimalloc-local-free-retire-proof/README.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-177-M169-MIMALLOC-LOCAL-FREE-RETIRE.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_local_free_retire.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_local_free_retire.err"

echo "[$TAG] checking M169 mimalloc local-free collection and retire"

guard_require_files \
  "$TAG" \
  "$PAGE_BOX" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$PLAN" \
  "$CARD" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'if 0 < local_free_top' "$PAGE_BOX" "page model must collect one local-free entry in acquire"
guard_expect_in_file "$TAG" 'if me\.used == 0' "$PAGE_BOX" "page model must record empty-page retire state on final local release"
guard_expect_in_file "$TAG" 'local_free_collect_count: i64 = 0' "$PAGE_BOX" "collection accounting must be explicit"
guard_expect_in_file "$TAG" 'local_free_collected_blocks: i64 = 0' "$PAGE_BOX" "collected block accounting must be explicit"
guard_expect_in_file "$TAG" 'retired: i64 = 0' "$PAGE_BOX" "retire state must be observable"
guard_expect_in_file "$TAG" 'retire_count: i64 = 0' "$PAGE_BOX" "retire accounting must be explicit"
guard_expect_in_file "$TAG" 'me\.local_free\.set\(local_free_top, block_id\)' "$PAGE_BOX" "local_free stack slots must be reusable after collection"
guard_expect_in_file "$TAG" 'me\.free\.set\(me\.free_top, block_id\)' "$PAGE_BOX" "free stack slots must be reusable after collection"
guard_expect_in_file "$TAG" 'mimalloc-local-free-retire-proof' "$APP_README" "proof README must describe M169 fixture"
guard_expect_in_file "$TAG" 'M169 local free collection and retire' "$PLAN" "plan must retain M169 row"
guard_expect_in_file "$TAG" '293x-177 M169 Mimalloc Local-Free Retire' "$CARD" "missing M169 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M169 guard"

if rg -n 'init[[:space:]]*\\{' "$PAGE_BOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: page model must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n ': usize|HakoAllocUsizeFieldProbe|usize_field_probe' "$PAGE_BOX" "$APP" >/tmp/"$TAG".usize 2>&1; then
  echo "[$TAG] ERROR: M169 production algorithm must stay on current i64 lane; usize probe remains isolated" >&2
  cat /tmp/"$TAG".usize >&2
  rm -f /tmp/"$TAG".usize
  exit 1
fi
rm -f /tmp/"$TAG".usize

if rg -n 'OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook|provider' "$PAGE_BOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M170+ or provider/hook ownership leaked into M169" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M169 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-local-free-retire|local_free_collect_count|retire_count' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M169 app/page matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-local-free-retire-proof$' "$OUT"
grep -q '^blocks=1,0,-1,1,-1$' "$OUT"
grep -q '^release=1,1,1$' "$OUT"
grep -q '^collect=1,1,0,0$' "$OUT"
grep -q '^state=0,0,2,1$' "$OUT"
grep -q '^counts=3,3,2,2,32,1,1,1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"

echo "[$TAG] ok"
