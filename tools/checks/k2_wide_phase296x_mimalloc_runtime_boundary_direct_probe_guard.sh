#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-705-MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-704-MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_runtime_boundary_direct_probe_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-runtime-boundary-direct-probe] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-runtime-boundary-direct-probe] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-runtime-boundary-direct-probe] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-runtime-boundary-direct-probe] row705 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-runtime-boundary-direct-probe] row704 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-runtime-boundary-direct-probe] row706 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-runtime-boundary-direct-probe] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-runtime-boundary-direct-probe] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-runtime-boundary-direct-probe-v0"
require_line_in_file "$CARD" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line_in_file "$CARD" "source_evidence=296x-704"
require_line_in_file "$CARD" "perf_runs=20"
require_line_in_file "$CARD" "in_process_operation_repeat=65536"
require_line_in_file "$CARD" "body_elapsed_ns=58000000"
require_line_in_file "$CARD" "top_symbol=nyash_array_length_h"
require_line_in_file "$CARD" "top_symbol_percent=70.61"
require_line_in_file "$CARD" "selected_owner=generated_runtime_array_length_boundary"
require_line_in_file "$CARD" "selected_owner_confidence=medium"
require_line_in_file "$CARD" "compiler_lowering_implementation_started=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=direct_array_length_boundary_design"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-705"
require_line_in_file "$NEXT_CARD" "implementation_started=0"

echo "[mimalloc-runtime-boundary-direct-probe] ok"
