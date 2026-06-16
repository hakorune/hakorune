#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="leaf-array-string-len-next-owner-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-848-MIMALLOC-LEAF-ARRAY-STRING-LEN-NEXT-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-847-MIMALLOC-LEAF-ARRAY-STRING-LEN-READY-PATH-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_leaf_array_string_len_next_owner_selection_guard.sh"

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
  "output_contract=hako-mimalloc-leaf-array-string-len-next-owner-selection-v0" \
  "source_evidence=296x-847,code-inspection-2026-06-16" \
  "row_kind=owner_selection" \
  "target_front=kilo_leaf_array_string_len" \
  "previous_attempt=array_string_len_readonly_ready_path" \
  "previous_attempt_kept=0" \
  "previous_attempt_reverted=1" \
  "previous_nonkeeper_reason=cycles_and_wall_time_regressed" \
  "ny_main_hot_loop_call_alias_pair=nyash.array.string_len_hi,hako.array_text.slot_len" \
  "ny_main_hot_loop_aliases_share_body=1" \
  "backend_array_string_len_direct_helper_enabled=1" \
  "array_string_len_window_routes_exist=1" \
  "helper_body_owner=crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs" \
  "handle_cache_owner=crates/nyash_kernel/src/plugin/array_handle_cache.rs" \
  "array_text_storage_owner=src/boxes/array/ops/text.rs" \
  "selected_owner=array_text_slot_len_loop_local_session_boundary" \
  "selected_owner_confidence=medium" \
  "implementation_allowed=0" \
  "helper_name_inference_enabled=0" \
  "source_hako_changed=0" \
  "mirbuilder_changed=0" \
  "backend_lowering_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not reapply the ready-path change" \
  "do not infer from hako.array_text.slot_len or nyash.array.string_len_hi by name alone" \
  "do not treat alias symbol spelling as owner evidence" \
  "do not patch ArrayBox storage" \
  "do not change indexOf/store/write paths" \
  "do not add a backend loop session without a guard surface" \
  "do not touch MIRBuilder object management" \
  "do not claim keeper without measuring this exact front"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

echo "[$TAG] ok"
