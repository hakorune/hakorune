#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-347-POST-ARRAY-HANDLE-CACHE-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-346-ARRAY-SINGLE-THREAD-EXACT-HANDLE-CACHE.md"
TOOL="$ROOT_DIR/tools/allocator/array_post_handle_cache_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row347_array_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row347-post-array-cache-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row347-post-array-cache-owner] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-post-handle-cache-owner-refresh-v0"
require_line "$DOC" "input_contract=array-single-thread-exact-handle-cache-v0"
require_line "$DOC" "array_store_pct=45.00"
require_line "$DOC" "array_load_pct=12.87"
require_line "$DOC" "array_hash_pct=0.00"
require_line "$DOC" "array_total_pct=63.98"
require_line "$DOC" "hako_method_pct=35.71"
require_line "$DOC" "selected_boundary=array_slot_nativedirect_guard_surface"
require_line "$DOC" "next_diagnostic=array_slot_nativedirect_guard_surface"
require_line "$DOC" "selected_reason=array_helper_call_boundary_dominates_after_hash_removed"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_pattern "$TOOL" "array-post-handle-cache-owner-refresh-v0"
require_pattern "$TOOL" "array_slot_nativedirect_guard_surface"

cat >"$PERF_REPORT" <<'REPORT'
    45.00%  [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64         app.exe
    12.87%  [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64  app.exe
     6.52%  [.] HakoAllocObjectLifecycleAllocResult.recordSuccess/1                       app.exe
     6.50%  [.] HakoAllocObjectLifecycleReleaseResult.recordSuccess/2                     app.exe
     6.39%  [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1                app.exe
     6.22%  [.] HakoAllocPageModel.acquire_usize/1                                        app.exe
     6.11%  [.] nyash.array.slot_store_hii                                                app.exe
     5.82%  [.] HakoAllocObjectLifecyclePageQueue.beginSelection/0                        app.exe
     4.26%  [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0              app.exe
REPORT

"$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=array-post-handle-cache-owner-refresh-v0"
require_line "$REPORT" "input_contract=array-single-thread-exact-handle-cache-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "array_store_pct=45.00"
require_line "$REPORT" "array_load_pct=12.87"
require_line "$REPORT" "array_hash_pct=0.00"
require_line "$REPORT" "array_slot_helper_pct=6.11"
require_line "$REPORT" "array_total_pct=63.98"
require_line "$REPORT" "hako_method_pct=35.71"
require_line "$REPORT" "selected_boundary=array_slot_nativedirect_guard_surface"
require_line "$REPORT" "next_diagnostic=array_slot_nativedirect_guard_surface"
require_line "$REPORT" "selected_reason=array_helper_call_boundary_dominates_after_hash_removed"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
