#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-708-MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-707-MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_array_len_helper_borrowed_ready_implementation_guard.sh"
IMPL="crates/nyash_kernel/src/plugin/array_compat.rs"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-len-borrowed-ready] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-len-borrowed-ready] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-array-len-borrowed-ready] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$IMPL" ]] || { echo "[mimalloc-array-len-borrowed-ready] missing impl: $IMPL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-array-len-borrowed-ready] row708 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-array-len-borrowed-ready] row707 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-array-len-borrowed-ready] row709 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-array-len-borrowed-ready] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-array-len-borrowed-ready] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-array-len-helper-borrowed-ready-implementation-v0"
require_line_in_file "$CARD" "source_evidence=296x-707"
require_line_in_file "$CARD" "target_symbol=nyash_array_length_h"
require_line_in_file "$CARD" "helper_abi_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "array_length_helper_uses_borrowed_ready=1"
require_line_in_file "$CARD" "nyash_array_length_h_validation_green=1"
require_line_in_file "$CARD" "body_timing_remeasured=1"
require_line_in_file "$CARD" "perf_runs=10"
require_line_in_file "$CARD" "in_process_operation_repeat=65536"
require_line_in_file "$CARD" "body_elapsed_ns_before=54000000"
require_line_in_file "$CARD" "body_elapsed_ns_after=53000000"
require_line_in_file "$CARD" "top_symbol_percent_before=72.06"
require_line_in_file "$CARD" "top_symbol_percent_after=68.13"
require_line_in_file "$CARD" "winner_claim=1"
require_line_in_file "$CARD" "next_task=MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-708"
require_line_in_file "$NEXT_CARD" "implementation_started=0"

grep -q 'with_array_box_ready(handle, |arr| arr.len() as i64).unwrap_or(0)' "$IMPL" || {
  echo "[mimalloc-array-len-borrowed-ready] nyash_array_length_h must use with_array_box_ready" >&2
  exit 1
}

grep -q 'pub extern "C" fn nyash_array_length_h(handle: i64) -> i64' "$IMPL" || {
  echo "[mimalloc-array-len-borrowed-ready] helper ABI changed or missing" >&2
  exit 1
}

echo "[mimalloc-array-len-borrowed-ready] ok"
