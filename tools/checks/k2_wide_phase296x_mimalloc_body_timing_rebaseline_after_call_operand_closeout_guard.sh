#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-766-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-765-CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_rebaseline_after_call_operand_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-body-rebaseline-after-call-operand-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-body-rebaseline-after-call-operand-closeout] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-body-rebaseline-after-call-operand-closeout] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-body-rebaseline-after-call-operand-closeout] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-body-rebaseline-after-call-operand-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-body-rebaseline-after-call-operand-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-body-timing-rebaseline-after-call-operand-closeout-v0" \
  "source_evidence=296x-765,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "hako_body_elapsed_ns=7000000" \
  "c_body_elapsed_ns=3274627" \
  "body_elapsed_gap_ns=3725373" \
  "body_elapsed_ratio=2.138" \
  "hako_external_elapsed_ms=10" \
  "c_external_elapsed_ms=10" \
  "external_elapsed_ratio=1.000" \
  "gap_owner=measurement_harness" \
  "gap_confidence=low" \
  "evidence_quality=single_sample_small_gap" \
  "gap_reason=body_gap_not_large_enough_for_owner" \
  "copy_count=51" \
  "local_like_copy_count=20" \
  "backend_route_carrier_copy_count=19" \
  "route_aware_candidate_copy_count=19" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "dominant_route_carrier_role=call_operand" \
  "call_operand_route_carrier_copy_count=13" \
  "call_operand_receiver_route_carrier_copy_count=2" \
  "call_operand_arg_route_carrier_copy_count=11" \
  "receiver_lane_closed=1" \
  "arg_lane_closed=1" \
  "call_operand_route_carrier_lane_closed=1" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_owner=none" \
  "selected_owner_reason=body_gap_not_large_enough_for_compiler_lowering_owner" \
  "selected_next_action=measurement_hygiene_refresh" \
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

grep -F -q "MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001:" "$CARD" || {
  echo "[mimalloc-body-rebaseline-after-call-operand-closeout] next measurement hygiene row is not documented" >&2
  exit 1
}

echo "[mimalloc-body-rebaseline-after-call-operand-closeout] ok"
