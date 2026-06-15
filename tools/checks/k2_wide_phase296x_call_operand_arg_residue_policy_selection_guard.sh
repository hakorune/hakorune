#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-764-CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-763-CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_arg_residue_policy_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-arg-residue-policy-selection] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-arg-residue-policy-selection] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-arg-residue-policy-selection] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-arg-residue-policy-selection] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-arg-residue-policy-selection] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-arg-residue-policy-selection] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-arg-residue-policy-selection-v0" \
  "source_evidence=296x-763,296x-761,296x-685,296x-691" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "call_operand_arg_route_carrier_copy_count=11" \
  "arg_acquire_usize_copy_count=3" \
  "arg_record_failure_after_selected_page_copy_count=5" \
  "arg_record_failure_no_selection_copy_count=2" \
  "arg_record_small_alloc_success_copy_count=1" \
  "prior_arg_same_block_root_candidate_count=7" \
  "prior_safe_arg_candidate_count=1" \
  "prior_rejected_arg_forwarding_count=9" \
  "arg_forwarding_enabled=0" \
  "selected_owner=none" \
  "selected_owner_reason=arg_residue_spans_size_and_result_value_arguments_without_a_single_safe_receiver_like_owner" \
  "call_operand_lane_closed=1" \
  "selected_next_action=call_operand_route_carrier_closeout_and_fresh_owner_selection" \
  "implementation_allowed=0" \
  "design_opens_implementation=0" \
  "measurement_required=0" \
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

grep -F -q "CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001:" "$CARD" || {
  echo "[call-operand-arg-residue-policy-selection] next closeout row is not documented" >&2
  exit 1
}

echo "[call-operand-arg-residue-policy-selection] ok"
