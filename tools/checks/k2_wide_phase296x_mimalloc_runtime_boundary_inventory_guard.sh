#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-703-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-702-MIMALLOC-BODY-TIMING-PRECISION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_runtime_boundary_inventory_guard.sh"
TOOL="tools/allocator/hako_mimalloc_runtime_boundary_inventory.py"

[[ -f "$CARD" ]] || { echo "[mimalloc-runtime-boundary-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-runtime-boundary-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[mimalloc-runtime-boundary-inventory] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-runtime-boundary-inventory] row703 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-runtime-boundary-inventory] row702 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-runtime-boundary-inventory] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-runtime-boundary-inventory] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-runtime-boundary-inventory-v0"
require_line_in_file "$CARD" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line_in_file "$CARD" "source_evidence=296x-702"
require_line_in_file "$CARD" "body_elapsed_ratio_raw=1.579"
require_line_in_file "$CARD" "measurement_boundary_confidence=low"
require_line_in_file "$CARD" "box_method_boundary_visible=1"
require_line_in_file "$CARD" "routeplan_slow_dynamic_hit_count=unknown"
require_line_in_file "$CARD" "object_refcount_boundary_visible=1"
require_line_in_file "$CARD" "host_handle_boundary_visible=1"
require_line_in_file "$CARD" "runtime_helper_call_boundary_visible=1"
require_line_in_file "$CARD" "generated_runtime_boundary_visible=1"
require_line_in_file "$CARD" "selected_owner=none"
require_line_in_file "$CARD" "selected_owner_confidence=low"
require_line_in_file "$CARD" "closed_world_routeplan_allowed=0"
require_line_in_file "$CARD" "exact_aot_specialization_selected=0"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "startup_lane_reopened=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "next_task=body_timer_alignment_or_boundary_probe"
require_line_in_file "$CARD" "summary=ok"

echo "[mimalloc-runtime-boundary-inventory] ok"
