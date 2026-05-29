#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-345-POST-DIRECT-SLOT-SUPPORTED-STORAGE-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-344-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-IMPLEMENTATION.md"
TOOL="$ROOT_DIR/tools/allocator/direct_slot_post_supported_storage_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row345_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row345-post-supported-storage-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row345-post-supported-storage-owner] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-post-supported-storage-owner-refresh-v0"
require_line "$DOC" "input_contract=direct-slot-supported-storage-nativedirect-implementation-v0"
require_line "$DOC" "field_helper_pct=0.00"
require_line "$DOC" "array_store_pct=38.21"
require_line "$DOC" "array_load_pct=10.67"
require_line "$DOC" "array_hash_pct=39.55"
require_line "$DOC" "array_total_pct=95.52"
require_line "$DOC" "selected_boundary=array_single_thread_exact_handle_cache"
require_line "$DOC" "next_diagnostic=array_single_thread_exact_handle_cache"
require_line "$DOC" "selected_reason=array_single_thread_hash_lookup_dominates_after_direct_slot_supported_storage"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_pattern "$TOOL" "direct-slot-post-supported-storage-owner-refresh-v0"
require_pattern "$TOOL" "array_single_thread_exact_handle_cache"
require_pattern "$TOOL" "array_slot_backend::single_thread_store_i64"

cat >"$PERF_REPORT" <<'REPORT'
    38.21%  [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64         app.exe
    34.34%  [.] core::hash::BuildHasher::hash_one                                         app.exe
    10.67%  [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64  app.exe
     5.21%  [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write                 app.exe
     4.17%  [.] nyash.array.slot_load_store_i64_hihi                                      app.exe
     2.92%  [.] nyash.array.slot_store_hii                                                app.exe
REPORT

"$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=direct-slot-post-supported-storage-owner-refresh-v0"
require_line "$REPORT" "input_contract=direct-slot-supported-storage-nativedirect-implementation-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "field_helper_pct=0.00"
require_line "$REPORT" "array_store_pct=38.21"
require_line "$REPORT" "array_load_pct=10.67"
require_line "$REPORT" "array_hash_pct=39.55"
require_line "$REPORT" "array_direct_op_pct=4.17"
require_line "$REPORT" "array_slot_helper_pct=2.92"
require_line "$REPORT" "array_total_pct=95.52"
require_line "$REPORT" "selected_boundary=array_single_thread_exact_handle_cache"
require_line "$REPORT" "next_diagnostic=array_single_thread_exact_handle_cache"
require_line "$REPORT" "selected_reason=array_single_thread_hash_lookup_dominates_after_direct_slot_supported_storage"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
