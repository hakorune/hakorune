#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-763-CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-762-CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_same_block_self_receiver_materialization_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-same-block-self-receiver-materialization-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-same-block-self-receiver-materialization-design] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-same-block-self-receiver-materialization-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-same-block-self-receiver-materialization-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-same-block-self-receiver-materialization-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-same-block-self-receiver-materialization-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-same-block-self-receiver-materialization-design-v0" \
  "source_evidence=296x-762,296x-685,296x-686,296x-687" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "same_block_self_receiver_materialization_count=2" \
  "prior_keeper_shape=same_block_root_receiver_operand_forwarding" \
  "prior_keeper_landed=1" \
  "prior_keeper_owner=LocalSSA::ensure_fallback_copy" \
  "existing_code_seam=LocalKind::Recv::can_forward_same_block_copy_root_to_receiver" \
  "current_receiver_residue_interpretation=receiver_pin_copy_not_additional_forwarding_candidate" \
  "selected_owner=none" \
  "selected_owner_reason=existing_same_block_receiver_keeper_already_landed_and_current_residue_is_the_receiver_pin_copy_itself" \
  "receiver_lane_closed=1" \
  "arg_forwarding_enabled=0" \
  "selected_next_action=call_operand_arg_residue_policy_selection" \
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

grep -F -q "CALL-OPERAND-ARG-RESIDUE-POLICY-SELECTION-001:" "$CARD" || {
  echo "[call-operand-same-block-self-receiver-materialization-design] next arg policy-selection row is not documented" >&2
  exit 1
}

echo "[call-operand-same-block-self-receiver-materialization-design] ok"
