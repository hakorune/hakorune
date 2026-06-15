#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-760-CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-759-CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_route_carrier_revalidation_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-route-carrier-revalidation-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-route-carrier-revalidation-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-route-carrier-revalidation-guard-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-route-carrier-revalidation-guard-surface] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-route-carrier-revalidation-guard-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-route-carrier-revalidation-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-route-carrier-revalidation-guard-surface-v0" \
  "source_evidence=296x-759,296x-758,296x-696,296x-699" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "route_carrier_residual_copy_count=13" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "prior_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite" \
  "prior_keeper_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite" \
  "prior_keeper_closed=1" \
  "prior_keeper_post_selected_keeper_candidate_count=0" \
  "current_probe_gap=call_operand_role_not_split_between_receiver_and_args" \
  "required_probe_field=call_operand_receiver_route_carrier_copy_count" \
  "required_probe_field=call_operand_arg_route_carrier_copy_count" \
  "receiver_post_target=0" \
  "arg_forwarding_enabled=0" \
  "arg_forwarding_policy=closed_until_explicit_arg_owner_selection" \
  "selected_next_action=call_operand_route_carrier_receiver_arg_split_probe" \
  "implementation_allowed=0" \
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

grep -F -q "CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001:" "$CARD" || {
  echo "[call-operand-route-carrier-revalidation-guard-surface] next receiver/arg split probe row is not documented" >&2
  exit 1
}

echo "[call-operand-route-carrier-revalidation-guard-surface] ok"
