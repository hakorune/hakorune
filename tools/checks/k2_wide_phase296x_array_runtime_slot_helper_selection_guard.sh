#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-202-ARRAY-RUNTIME-SLOT-HELPER-SELECTION.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-201-LARGE-OWNER-REFRESH-AFTER-RESIDENCE-ZERO-NET.md"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[row202-array-slot-selection] missing line: $expected" >&2
    exit 1
  fi
}

require_text() {
  local expected="$1"
  if ! grep -Fq "$expected" "$CARD"; then
    echo "[row202-array-slot-selection] missing text: $expected" >&2
    exit 1
  fi
}

grep -q '^Status: Current$' "$CARD"
grep -q '^Status: Landed$' "$PREV"

require_line "selected_owner_family=array_runtime_slot_helper_lowering"
require_line "selected_next_diagnostic=array_runtime_slot_helper_cost_probe"
require_line "selected_reason=array_set_and_slot_store_are_dominant_after_typed_object_fast_lane"
require_line "diagnostic_owner=array_runtime_slot_store_i64_path"
require_line "array_runtime_slot_helper_selection=accepted"
require_line "next_diagnostic=array_runtime_slot_helper_cost_probe"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

require_text "array_runtime_set_idx_i64(handle, idx, value)"
require_text "array_slot_store_i64(handle, idx, value)"
require_text "with_array_box(handle, |arr| ...)"
require_text "arr.slot_store_i64_raw(idx, value)"
require_text "dominant_subowner=<facade_boundary|handle_cache_lookup|array_storage_write_lock|inline_i64_store|mixed>"

echo "[row202-array-slot-selection] ok"
