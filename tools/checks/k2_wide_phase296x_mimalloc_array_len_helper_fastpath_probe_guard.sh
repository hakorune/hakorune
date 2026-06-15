#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-707-MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-708-MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_array_len_helper_fastpath_probe_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-len-fastpath-probe] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-len-fastpath-probe] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-array-len-fastpath-probe] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-array-len-fastpath-probe] row707 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-array-len-fastpath-probe] row706 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-array-len-fastpath-probe] row708 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-array-len-fastpath-probe] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-array-len-fastpath-probe] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-array-len-helper-fastpath-probe-v0"
require_line_in_file "$CARD" "target_symbol=nyash_array_length_h"
require_line_in_file "$CARD" "source_evidence=296x-706"
require_line_in_file "$CARD" "mir_route_already_array_slot_len=1"
require_line_in_file "$CARD" "perf_runs=10"
require_line_in_file "$CARD" "in_process_operation_repeat=65536"
require_line_in_file "$CARD" "top_symbol=nyash_array_length_h"
require_line_in_file "$CARD" "top_symbol_percent=72.06"
require_line_in_file "$CARD" "helper_abi_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "selected_owner=host_handle_lookup"
require_line_in_file "$CARD" "selected_owner_confidence=medium"
require_line_in_file "$CARD" "implementation_allowed=1"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-707"
if grep -q '^Status: Active$' "$NEXT_CARD"; then
  require_line_in_file "$NEXT_CARD" "implementation_started=0"
else
  require_line_in_file "$NEXT_CARD" "output_contract=hako-mimalloc-array-len-helper-borrowed-ready-implementation-v0"
  require_line_in_file "$NEXT_CARD" "summary=ok"
fi

echo "[mimalloc-array-len-fastpath-probe] ok"
