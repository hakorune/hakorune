#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-765-CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-764-CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_route_carrier_closeout_and_fresh_owner_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-route-carrier-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-route-carrier-closeout] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-route-carrier-closeout] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-route-carrier-closeout] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-route-carrier-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-route-carrier-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-route-carrier-closeout-and-fresh-owner-selection-v0" \
  "source_evidence=296x-764,296x-763,296x-761,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "last_body_timing_source=296x-753" \
  "last_hako_body_elapsed_ns=374000000" \
  "last_c_body_elapsed_ns=4699328" \
  "last_body_elapsed_ratio=79.586" \
  "copy_count=51" \
  "call_operand_route_carrier_copy_count=13" \
  "call_operand_receiver_route_carrier_copy_count=2" \
  "call_operand_arg_route_carrier_copy_count=11" \
  "receiver_lane_closed=1" \
  "arg_lane_closed=1" \
  "call_operand_route_carrier_lane_closed=1" \
  "selected_owner=none" \
  "selected_owner_reason=receiver_surface_already_has_landed_seam_and_arg_surface_has_no_single_safe_owner" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_next_action=mimalloc_body_timing_rebaseline_after_call_operand_closeout" \
  "implementation_allowed=0" \
  "measurement_required=1" \
  "winner_claim=0" \
  "startup_lane_reopened=0" \
  "source_hako_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "product_default_changed=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001:" "$CARD" || {
  echo "[call-operand-route-carrier-closeout] next rebaseline row is not documented" >&2
  exit 1
}

echo "[call-operand-route-carrier-closeout] ok"
