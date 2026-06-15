#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-762-CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-761-CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_receiver_residue_classification_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-receiver-residue-classification] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-receiver-residue-classification] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-receiver-residue-classification] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-receiver-residue-classification] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-receiver-residue-classification] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-receiver-residue-classification] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-receiver-residue-classification-v0" \
  "source_evidence=296x-761" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "call_operand_receiver_route_carrier_copy_count=2" \
  "call_operand_arg_route_carrier_copy_count=11" \
  "receiver_sample_0_block=block_596" \
  "receiver_sample_0_dst=62" \
  "receiver_sample_0_src=0" \
  "receiver_sample_0_callee=HakoAllocObjectLifecycleFacade.smallAcquireFailedReason/0" \
  "receiver_sample_0_class=same_block_self_receiver_materialization" \
  "receiver_sample_1_block=block_597" \
  "receiver_sample_1_dst=78" \
  "receiver_sample_1_src=0" \
  "receiver_sample_1_callee=HakoAllocObjectLifecycleFacade.recordSmallAllocSuccess/1" \
  "receiver_sample_1_class=same_block_self_receiver_materialization" \
  "same_block_self_receiver_materialization_count=2" \
  "prior_cfg_stable_receiver_keeper_reopen=0" \
  "selected_policy_family=same_block_self_receiver_materialization_rewrite" \
  "selected_next_action=call_operand_same_block_self_receiver_materialization_design" \
  "implementation_allowed=0" \
  "design_required=1" \
  "arg_forwarding_enabled=0" \
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

grep -F -q "CALL-OPERAND-SAME-BLOCK-SELF-RECEIVER-MATERIALIZATION-DESIGN-001:" "$CARD" || {
  echo "[call-operand-receiver-residue-classification] next same-block self receiver design row is not documented" >&2
  exit 1
}

echo "[call-operand-receiver-residue-classification] ok"
