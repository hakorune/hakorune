#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-metadata-store-indexed-read"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

ALIGNED_STORE="lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako"
HUGE_STORE="lang/src/hako_alloc/memory/huge_page_meta_store_box.hako"
HUGE_MODEL="lang/src/hako_alloc/memory/huge_page_model_box.hako"
CARD="$(guard_require_phase293x_card "$TAG" "293x-223-C206E-METADATA-STORE-INDEXED-READ-CLEANUP.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_metadata_store_indexed_read_guard.sh"

echo "[$TAG] checking C206e metadata-store indexed read cleanup"

guard_require_files \
  "$TAG" \
  "$ALIGNED_STORE" \
  "$HUGE_STORE" \
  "$HUGE_MODEL" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C206e card must be complete"
guard_expect_in_file "$TAG" 'C206e status:' "$PLAN" "mimalloc plan must record C206e status"
guard_expect_in_file "$TAG" '`C206e` is complete as `293x-223`' "$RECORD_SSOT" "record SSOT must mark C206e complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C206e guard"

guard_expect_in_file "$TAG" 'alignmentAt\(index\)' "$ALIGNED_STORE" "aligned store must expose indexed alignment read"
guard_expect_in_file "$TAG" 'paddedSizeAt\(index\)' "$ALIGNED_STORE" "aligned store must expose indexed padded-size read"
guard_expect_in_file "$TAG" 'return me\.alignmentAt\(index\)' "$ALIGNED_STORE" "alignmentFor must delegate through indexed read"
guard_expect_in_file "$TAG" 'return me\.paddedSizeAt\(index\)' "$ALIGNED_STORE" "paddedSizeFor must delegate through indexed read"

guard_expect_in_file "$TAG" 'pageIdAt\(index\)' "$HUGE_STORE" "huge store must expose indexed page-id read"
guard_expect_in_file "$TAG" 'requestedSizeAt\(index\)' "$HUGE_STORE" "huge store must expose indexed requested-size read"
guard_expect_in_file "$TAG" 'committedSizeAt\(index\)' "$HUGE_STORE" "huge store must expose indexed committed-size read"
guard_expect_in_file "$TAG" 'markReleasedAt\(index\)' "$HUGE_STORE" "huge store must expose indexed release update"
guard_expect_in_file "$TAG" 'return me\.pageIdAt\(index\)' "$HUGE_STORE" "pageIdFor must delegate through indexed read"
guard_expect_in_file "$TAG" 'return me\.requestedSizeAt\(index\)' "$HUGE_STORE" "requestedSizeFor must delegate through indexed read"
guard_expect_in_file "$TAG" 'return me\.committedSizeAt\(index\)' "$HUGE_STORE" "committedSizeFor must delegate through indexed read"
guard_expect_in_file "$TAG" 'return me\.markReleasedAt\(index\)' "$HUGE_STORE" "markReleased must delegate through indexed release update"

guard_expect_in_file "$TAG" 'return me\.meta_store\.pageIdAt\(index\)' "$HUGE_MODEL" "huge model must use resolved-index page-id read"
guard_expect_in_file "$TAG" 'return me\.meta_store\.requestedSizeAt\(index\)' "$HUGE_MODEL" "huge model must use resolved-index requested-size read"
guard_expect_in_file "$TAG" 'return me\.meta_store\.committedSizeAt\(index\)' "$HUGE_MODEL" "huge model must use resolved-index committed-size read"
guard_expect_in_file "$TAG" 'me\.meta_store\.markReleasedAt\(index\)' "$HUGE_MODEL" "huge model must use resolved-index release update"

if rg -n 'meta_store\.(pageIdFor|requestedSizeFor|committedSizeFor|markReleased)\(ptr\)' \
  "$HUGE_MODEL" >/tmp/"$TAG".repeat_lookup 2>&1; then
  echo "[$TAG] ERROR: huge model must not re-enter pointer-based metadata APIs after resolving index" >&2
  cat /tmp/"$TAG".repeat_lookup >&2
  rm -f /tmp/"$TAG".repeat_lookup
  exit 1
fi
rm -f /tmp/"$TAG".repeat_lookup

if rg -n 'ArrayStorage::InlineRecord|InlineRecord|provider|hook|hako_mem_|externcall|memcpy|copy_bytes|aligned_alloc|OSVM|OsVm' \
  "$ALIGNED_STORE" "$HUGE_STORE" "$HUGE_MODEL" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: C206e leaked beyond metadata-store cleanup scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

echo "[$TAG] ok"
