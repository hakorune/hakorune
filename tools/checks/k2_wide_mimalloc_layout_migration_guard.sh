#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-layout-migration"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
LAYOUT="lang/src/hako_alloc/memory/layout_box.hako"
PAGE_HEAP="lang/src/hako_alloc/memory/page_heap_box.hako"
M163_APP_TEST="apps/mimalloc-size-class-policy-proof/test.sh"
M164_CARD="docs/development/current/main/phases/phase-293x/293x-165-M164-MIMALLOC-LAYOUT-MIGRATION-CLOSEOUT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_layout_migration_guard.sh"

echo "[$TAG] checking M164 mimalloc layout migration closeout"

guard_require_files \
  "$TAG" \
  "$SIZE_CLASS" \
  "$LAYOUT" \
  "$PAGE_HEAP" \
  "$M163_APP_TEST" \
  "$M164_CARD" \
  "$PLAN" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'static box SizeClassBox' "$SIZE_CLASS" "SizeClassBox must remain the size-class owner"
guard_expect_in_file "$TAG" 'size_to_bin' "$SIZE_CLASS" "SizeClassBox must own size_to_bin"
guard_expect_in_file "$TAG" 'bin_size' "$SIZE_CLASS" "SizeClassBox must own bin_size"
guard_expect_in_file "$TAG" 'legacy two-class compatibility facade' "$LAYOUT" "LayoutBox must document compatibility-facade ownership"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.size_class_box as SizeClassBox' "$LAYOUT" "LayoutBox must import SizeClassBox"
guard_expect_in_file "$TAG" 'SizeClassBox.normalize_size' "$LAYOUT" "LayoutBox normalize_size must delegate"
guard_expect_in_file "$TAG" 'SizeClassBox.good_size' "$LAYOUT" "LayoutBox class_id must use SizeClassBox.good_size"
guard_expect_in_file "$TAG" 'SizeClassBox.bin_size.4' "$LAYOUT" "LayoutBox small class_size must delegate"
guard_expect_in_file "$TAG" 'SizeClassBox.bin_size.8' "$LAYOUT" "LayoutBox medium class_size must delegate"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.layout_box as LayoutBox' "$PAGE_HEAP" "page heap should consume the compatibility facade"
guard_expect_in_file "$TAG" 'LayoutBox.class_size.0' "$PAGE_HEAP" "page heap should keep legacy small-page callsite"
guard_expect_in_file "$TAG" 'LayoutBox.class_size.1' "$PAGE_HEAP" "page heap should keep legacy medium-page callsite"
guard_expect_in_file "$TAG" 'free_stack: ArrayBox = new ArrayBox\(\)' "$PAGE_HEAP" "page heap should initialize fixed page arrays at declaration site"
guard_expect_in_file "$TAG" 'small_page: HakoAllocPage = new HakoAllocPage' "$PAGE_HEAP" "heap small page should use stored field initializer"
guard_expect_in_file "$TAG" 'M164 layout migration closeout' "$PLAN" "plan must retain M164 row"
guard_expect_in_file "$TAG" '293x-165 M164 Mimalloc Layout Migration Closeout' "$M164_CARD" "missing M164 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M164 guard"

if rg -n 'init[[:space:]]*\\{' "$PAGE_HEAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: page_heap_box should use stored fields/initializers, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'class_id\(size\).*if .*<=|if n <= 32|if n <= 64|return 32|return 64' "$LAYOUT" >/tmp/"$TAG".layout 2>&1; then
  echo "[$TAG] ERROR: stale direct size-class ownership remains in LayoutBox" >&2
  cat /tmp/"$TAG".layout >&2
  rm -f /tmp/"$TAG".layout
  exit 1
fi
rm -f /tmp/"$TAG".layout

if rg -F -q 'SizeClassBox' "$PAGE_HEAP"; then
  guard_fail "$TAG" "page_heap_box must not bypass LayoutBox during M164 closeout"
fi

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M164 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-layout-migration|SizeClassBox|LayoutBox|MI_SIZE_CLASS|MI_CLASS_CAP' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: layout migration matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

bash "$M163_APP_TEST"
apps/mimalloc-lite/test.sh
apps/allocator-stress/test.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh

echo "[$TAG] ok"
