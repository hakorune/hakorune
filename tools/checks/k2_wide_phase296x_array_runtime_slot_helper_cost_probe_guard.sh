#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-203-ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-202-ARRAY-RUNTIME-SLOT-HELPER-SELECTION.md"
TOOL="$ROOT/tools/allocator/array_runtime_slot_helper_cost_probe.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row203_array_slot_probe.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

grep -q '^Status: Current$' "$CARD"
grep -q '^Status: Landed$' "$PREV"
grep -q '^dominant_subowner=array_storage_write_lock$' "$CARD"
grep -q '^recommended_next=single_thread_array_store_backend$' "$CARD"
grep -q '^summary=ok$' "$CARD"

"$TOOL" --iterations 500000 --out "$REPORT"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$REPORT"; then
    echo "[row203-array-slot-probe] missing report line: $expected" >&2
    cat "$REPORT" >&2
    exit 1
  fi
}

require_regex() {
  local expected="$1"
  if ! grep -Eq "$expected" "$REPORT"; then
    echo "[row203-array-slot-probe] missing report regex: $expected" >&2
    cat "$REPORT" >&2
    exit 1
  fi
}

require_line "output_contract=array-runtime-slot-helper-cost-probe-v0"
require_regex '^valid_handle_idx_ns_per_op=[1-9][0-9]*$'
require_regex '^handle_cache_with_array_box_ns_per_op=[1-9][0-9]*$'
require_regex '^array_storage_write_lock_ns_per_op=[1-9][0-9]*$'
require_regex '^inline_i64_store_ns_per_op=[1-9][0-9]*$'
require_regex '^array_slot_store_i64_ns_per_op=[1-9][0-9]*$'
require_regex '^array_runtime_set_idx_i64_ns_per_op=[1-9][0-9]*$'
require_line "dominant_subowner=array_storage_write_lock"
require_line "recommended_next=single_thread_array_store_backend"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

cat "$REPORT"
