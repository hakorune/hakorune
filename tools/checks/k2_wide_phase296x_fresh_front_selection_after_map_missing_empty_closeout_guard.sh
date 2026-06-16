#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fresh-front-selection-after-map-missing-empty-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-844-MIMALLOC-FRESH-FRONT-SELECTION-AFTER-MAP-MISSING-EMPTY-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-843-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fresh_front_selection_after_map_missing_empty_closeout_guard.sh"

for file in "$CARD" "$PREV_CARD"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
  grep -q '^Status: Landed$' "$file" || {
    echo "[$TAG] card must be Landed: $file" >&2
    exit 1
  }
done

grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[$TAG] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-fresh-front-selection-after-map-missing-empty-closeout-v0" \
  "source_evidence=296x-843,leaf-repeat3-2026-06-16" \
  "row_kind=selection" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "previous_front=kilo_leaf_map_get_missing" \
  "previous_front_closed=1" \
  "previous_front_route_winner_claim=1" \
  "candidate_front_count=4" \
  "selected_front=kilo_leaf_array_string_len" \
  "selected_owner_family=array_string_len_runtime_boundary_inventory" \
  "selected_reason=simpler_leaf_hako_slower_before_indexof" \
  "selected_ratio_instr=0.16" \
  "selected_ratio_cycles=0.10" \
  "selected_ratio_ms=0.40" \
  "selected_aot_status=ok" \
  "secondary_front=kilo_leaf_array_string_indexof_const" \
  "blocked_front=kilo_leaf_array_rmw_add1" \
  "blocked_front_reason=emit_helper_retry_failed" \
  "closed_front=kilo_leaf_map_get_missing" \
  "backend_lowering_changed=0" \
  "runtime_helper_changed=0" \
  "product_default_changed=0" \
  "benchmark_source_changed=0" \
  "helper_name_inference_enabled=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-OWNER-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not patch Array/String helpers before owner inventory" \
  "do not infer a keeper from helper or method names" \
  "do not select indexOf before the simpler length front is inventoried" \
  "do not reopen map missing unless fresh regression evidence appears" \
  "do not treat emit_helper_retry_failed as a perf owner" \
  "do not change product runtime or benchmark source"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
