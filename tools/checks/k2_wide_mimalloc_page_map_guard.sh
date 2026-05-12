#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-map"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-map-proof/main.hako"
APP_TEST="apps/mimalloc-page-map-proof/test.sh"
APP_README="apps/mimalloc-page-map-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-179-M171-MIMALLOC-PAGE-MAP-MODEL.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_map_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map.err"

echo "[$TAG] checking M171 mimalloc page-map model"

guard_require_files \
  "$TAG" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.page_map_box = "memory/page_map_box.hako"' "$MODULE" "hako module must export page_map_box"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapEntry' "$PAGE_MAP" "page-map entry owner must exist"
guard_expect_in_file "$TAG" 'box HakoAllocPageMap' "$PAGE_MAP" "page-map owner must exist"
guard_expect_in_file "$TAG" 'register\(ptr, page_id, block_id\)' "$PAGE_MAP" "page-map must expose register"
guard_expect_in_file "$TAG" 'lookup\(ptr\)' "$PAGE_MAP" "page-map must expose lookup"
guard_expect_in_file "$TAG" 'unregister\(ptr\)' "$PAGE_MAP" "page-map must expose unregister"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_box as HakoAllocPageMapBox' "$APP" "proof app must import page_map_box"
guard_expect_in_file "$TAG" 'M171 page-map model' "$PLAN" "plan must retain M171 row"
guard_expect_in_file "$TAG" '293x-179 M171 Mimalloc Page-Map Model' "$CARD" "missing M171 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M171 guard"

if rg -n 'init[[:space:]]*\{' "$PAGE_MAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M171 page-map must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'externcall|hako_mem_|OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|provider|hook|replacement|realloc|releaseLocal|HakoAllocPageModel|HakoAllocRemoteFreePolicy' \
  "$PAGE_MAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M171 leaked out of pure page-map model scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-page-map|HakoAllocPageMap|page_map_box' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: page-map app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-page-map-proof$' "$OUT"
grep -q '^register=1,1,0,1,0$' "$OUT"
grep -q '^unregister=1,0$' "$OUT"
grep -q '^shape=5$' "$OUT"
grep -q '^counts=3,2,3,5,2,1,3$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cat "$OUT"

echo "[$TAG] ok"
