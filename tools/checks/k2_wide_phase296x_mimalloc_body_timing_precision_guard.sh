#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-702-MIMALLOC-BODY-TIMING-PRECISION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-701-MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-703-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_precision_guard.sh"
TOOL="tools/allocator/hako_mimalloc_body_timing_precision.py"

[[ -f "$CARD" ]] || { echo "[mimalloc-body-timing-precision] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-body-timing-precision] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-body-timing-precision] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[mimalloc-body-timing-precision] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-body-timing-precision] row702 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-body-timing-precision] row701 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-body-timing-precision] row703 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-body-timing-precision] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-body-timing-precision] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-body-timing-precision-v0"
require_line_in_file "$CARD" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line_in_file "$CARD" "source_evidence=296x-701"
require_line_in_file "$CARD" "hako_timer_family=workload-body-env-now-ms-v0"
require_line_in_file "$CARD" "c_timer_family=workload-body-monotonic-v0"
require_line_in_file "$CARD" "timer_family_matched=0"
require_line_in_file "$CARD" "hako_timer_resolution_ns=1000000"
require_line_in_file "$CARD" "body_elapsed_ratio_raw=1.579"
require_line_in_file "$CARD" "body_elapsed_ratio_precision_confidence=low"
require_line_in_file "$CARD" "measurement_boundary_confidence=low"
require_line_in_file "$CARD" "selected_next_owner=runtime_boundary_inventory"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "startup_lane_reopened=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=mimalloc_runtime_boundary_inventory"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-702"
require_line_in_file "$NEXT_CARD" "implementation_started=0"

echo "[mimalloc-body-timing-precision] ok"
