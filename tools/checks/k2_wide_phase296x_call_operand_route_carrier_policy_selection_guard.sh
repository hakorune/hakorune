#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-759-CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-758-ROUTE-CARRIER-RESIDUAL-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_route_carrier_policy_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-route-carrier-policy-selection] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-route-carrier-policy-selection] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[call-operand-route-carrier-policy-selection] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[call-operand-route-carrier-policy-selection] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[call-operand-route-carrier-policy-selection] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[call-operand-route-carrier-policy-selection] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-call-operand-route-carrier-policy-selection-v0" \
  "source_evidence=296x-758,296x-690,296x-691,296x-693,296x-694" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "route_carrier_residual_copy_count=13" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "prior_receiver_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite" \
  "prior_receiver_keeper_candidate_count=13" \
  "prior_localssa_emission_time_rejected=1" \
  "prior_cfg_stable_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite" \
  "arg_forwarding_enabled=0" \
  "unknown_root_forwarding_enabled=0" \
  "helper_name_special_case=0" \
  "benchmark_name_special_case=0" \
  "selected_policy_family=cfg_stable_call_operand_route_carrier_revalidation" \
  "selected_policy_candidate_count=13" \
  "selected_next_action=call_operand_route_carrier_revalidation_guard_surface" \
  "implementation_allowed=0" \
  "guard_surface_required=1" \
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

grep -F -q "CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001:" "$CARD" || {
  echo "[call-operand-route-carrier-policy-selection] next revalidation guard-surface row is not documented" >&2
  exit 1
}

echo "[call-operand-route-carrier-policy-selection] ok"
