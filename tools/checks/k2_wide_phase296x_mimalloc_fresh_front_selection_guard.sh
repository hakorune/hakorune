#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-836-MIMALLOC-FRESH-FRONT-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-835-ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_fresh_front_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-fresh-front-selection] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-fresh-front-selection] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[mimalloc-fresh-front-selection] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-fresh-front-selection] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-fresh-front-selection] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-fresh-front-selection] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-fresh-front-selection-v0" \
  "source_evidence=296x-824,296x-835" \
  "row_kind=selection" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "fresh_front_selection_allowed=1" \
  "previous_front=object_lifecycle_body" \
  "previous_front_paused=1" \
  "candidate_front_count=14" \
  "selected_front=kilo_leaf_map_get_missing" \
  "selected_owner_family=map_missing_key_string_lookup_runtime_boundary" \
  "selected_reason=leaf_hako_slower_resident_kernel" \
  "primary_lane=resident_kernel" \
  "kernel_inner_runs=100" \
  "selected_ratio_kernel_cycles=0.01" \
  "selected_ratio_kernel_instr=0.01" \
  "selected_aot_status=ok" \
  "asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str" \
  "asm_top_symbol_0_percent=58.32" \
  "asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string" \
  "asm_top_symbol_1_percent=41.67" \
  "boot_startup_lane_reopened=0" \
  "product_nyrt_entry_changed=0" \
  "provider_activation_changed=0" \
  "backend_lowering_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-MISSING-KEY-OWNER-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not patch MapBox before owner inventory" \
  "do not optimize from process-total alone" \
  "do not select composite meso fronts before leaf owner inventory" \
  "do not infer a keeper from method/helper names" \
  "do not change product runtime or provider activation" \
  "do not resume objectLifecycleSmallAlloc without new Hako-slower evidence"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[mimalloc-fresh-front-selection] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[mimalloc-fresh-front-selection] ok"
