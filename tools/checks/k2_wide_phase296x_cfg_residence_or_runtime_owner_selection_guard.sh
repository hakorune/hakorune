#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-198-CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-197-MIR-TYPED-FIELD-RESIDENCE-ERASURE-FEASIBILITY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row198-owner-selection] missing line in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_line "$CARD" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$CARD" "selected_next_owner=cfg_aware_typed_field_residence_design"
require_line "$CARD" "selection_reason=runtime_fast_lane_already_accepted_but_helper_calls_remain_large"
require_line "$CARD" "rejected_next_owner=retry_block_local_typed_field_residence"
require_line "$CARD" "rejected_next_owner_reason=net_helper_call_delta_is_zero"
require_line "$CARD" "runtime_owner_status=single_thread_exact_store_landed_as_floor"
require_line "$CARD" "transform_open=0"
require_line "$CARD" "winner_claim=0"
require_line "$CARD" "replacement_active=0"
require_line "$CARD" "hook_installed=0"
require_line "$CARD" "global_allocator=0"
require_line "$CARD" "summary=ok"

echo "[row198-owner-selection] ok"
