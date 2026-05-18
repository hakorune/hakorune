#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-id-brand-first-pilot-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_657="docs/development/current/main/phases/phase-293x/293x-657-HAKO-ALLOC-ID-BRAND-002-ALLOCATOR-SCALAR-ID-BRAND-FIRST-PILOT.md"
CARD_658="docs/development/current/main/phases/phase-293x/293x-658-HAKO-ALLOC-ID-BRAND-003-ALLOCATOR-SCALAR-ID-BRAND-PILOT-CLOSEOUT-GUARD.md"
CARD_659="docs/development/current/main/phases/phase-293x/293x-659-MIMAP-145A-POST-ID-BRAND-PILOT-CLOSEOUT-ROW-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
PUBLIC_WRAPPER="tools/checks/k2_wide_hako_alloc_id_brand_first_pilot_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_id_brand_first_pilot_closeout_guard.sh"
PROOF_RUNNER="tools/checks/run_proof_app.sh"

echo "[$TAG] checking HAKO-ALLOC-ID-BRAND-003 allocator scalar ID brand pilot closeout"

guard_require_files \
  "$TAG" \
  "$CARD_657" \
  "$CARD_658" \
  "$CARD_659" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$GUARD_MANIFEST" \
  "$OWNER" \
  "$PUBLIC_WRAPPER" \
  "$IMPL_SCRIPT" \
  "$PROOF_RUNNER"

guard_require_exec_files "$TAG" "$PUBLIC_WRAPPER" "$IMPL_SCRIPT" "$PROOF_RUNNER"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_657" "brand pilot card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_658" "brand pilot closeout card must be landed"
guard_expect_in_file "$TAG" "MIMAP-145A" "$CARD_658" "closeout card must select next row"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_659" "next planning row must be selected current"

guard_expect_in_file "$TAG" "HAKO-ALLOC-ID-BRAND-002" "$GRANULARITY" "granularity must describe pilot row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-ID-BRAND-003" "$GRANULARITY" "granularity must describe closeout row"
guard_expect_in_file "$TAG" "MIMAP-145A" "$GRANULARITY" "granularity must name next planning row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-ID-BRAND-003" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-145A" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "HAKO-ALLOC-ID-BRAND-003" "$TASKBOARD" "taskboard must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-145A" "$TASKBOARD" "taskboard must name selected next row"
guard_expect_in_file "$TAG" "$PUBLIC_WRAPPER" "$INDEX" "check index must list this closeout guard"
guard_expect_in_file "$TAG" 'id = "hako-alloc-id-brand-first-pilot-closeout"' "$GUARD_MANIFEST" "guard manifest must include this closeout row"

guard_expect_in_file "$TAG" "brand SegmentId: i64" "$OWNER" "owner must declare SegmentId brand"
guard_expect_in_file "$TAG" "brand PageId: i64" "$OWNER" "owner must declare PageId brand"
guard_expect_in_file "$TAG" "brand BlockId: i64" "$OWNER" "owner must declare BlockId brand"
guard_expect_in_file "$TAG" "makeReuseToken\\(segment_id: SegmentId, page_id: PageId, reused_block_id: BlockId\\): i64" "$OWNER" "token helper must keep brand-typed parameter boundary"
guard_expect_in_file "$TAG" "me.makeReuseToken\\(SegmentId\\(segment_id\\), PageId\\(page_id\\), BlockId\\(reused_block_id\\)\\)" "$OWNER" "token helper call must use explicit brand constructors"
guard_expect_in_file "$TAG" "modeled_reuse_token: i64" "$OWNER" "token storage/report surface must stay scalar"

if rg -n ':[[:space:]]*(SegmentId|PageId|BlockId)' "$OWNER" | grep -v 'makeReuseToken(segment_id: SegmentId, page_id: PageId, reused_block_id: BlockId): i64' >/tmp/"$TAG".brand_widening 2>&1; then
  cat /tmp/"$TAG".brand_widening >&2
  rm -f /tmp/"$TAG".brand_widening
  guard_fail "$TAG" "brand pilot must not widen to fields, reports, typed locals, or extra helper signatures"
fi
rm -f /tmp/"$TAG".brand_widening

if rg -n 'brand[[:space:]]+(ReuseToken|ModeledReuseToken|AllocationToken|Pointer|Ptr|Generation)' "$OWNER" >/tmp/"$TAG".token_brand_widening 2>&1; then
  cat /tmp/"$TAG".token_brand_widening >&2
  rm -f /tmp/"$TAG".token_brand_widening
  guard_fail "$TAG" "brand pilot must not add token/pointer/generation vocabulary"
fi
rm -f /tmp/"$TAG".token_brand_widening

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "brand closeout must keep execution/concurrency/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-id-brand|SegmentId|PageId|BlockId' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "brand pilot must not leak into backend .inc matchers"
fi
rm -f /tmp/"$TAG".inc_leak

bash "$PROOF_RUNNER" --only MIMAP-142A >/tmp/"$TAG".proof 2>&1 || {
  cat /tmp/"$TAG".proof >&2
  rm -f /tmp/"$TAG".proof
  guard_fail "$TAG" "MIMAP-142A proof failed after brand pilot"
}
rm -f /tmp/"$TAG".proof

echo "[$TAG] ok"
