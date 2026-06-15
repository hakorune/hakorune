#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-704-MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-703-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-705-MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timer_alignment_probe_guard.sh"
TOOL="tools/allocator/hako_mimalloc_body_timer_alignment_probe.py"

[[ -f "$CARD" ]] || { echo "[mimalloc-body-timer-alignment] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-body-timer-alignment] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-body-timer-alignment] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[mimalloc-body-timer-alignment] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-body-timer-alignment] row704 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-body-timer-alignment] row703 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-body-timer-alignment] row705 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-body-timer-alignment] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-body-timer-alignment] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-body-timer-alignment-probe-v0"
require_line_in_file "$CARD" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line_in_file "$CARD" "source_evidence=296x-703"
require_line_in_file "$CARD" "baseline_body_elapsed_ratio=1.579"
require_line_in_file "$CARD" "scaled_in_process_repeat=65536"
require_line_in_file "$CARD" "scaled_hako_body_elapsed_ns=53000000"
require_line_in_file "$CARD" "scaled_c_body_elapsed_ns=25998914"
require_line_in_file "$CARD" "scaled_body_elapsed_ratio=2.039"
require_line_in_file "$CARD" "hako_timer_family=workload-body-env-now-ms-v0"
require_line_in_file "$CARD" "c_timer_family=workload-body-monotonic-v0"
require_line_in_file "$CARD" "timer_family_matched=0"
require_line_in_file "$CARD" "hako_timer_resolution_ns=1000000"
require_line_in_file "$CARD" "scaled_hako_timer_resolution_pct=1.887"
require_line_in_file "$CARD" "body_elapsed_ratio_precision_confidence=medium"
require_line_in_file "$CARD" "selected_next_owner=runtime_boundary_direct_probe"
require_line_in_file "$CARD" "selected_next_owner_confidence=medium"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "startup_lane_reopened=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=runtime_boundary_direct_probe"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-RUNTIME-BOUNDARY-DIRECT-PROBE-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-704"
require_line_in_file "$NEXT_CARD" "implementation_started=0"

echo "[mimalloc-body-timer-alignment] ok"
