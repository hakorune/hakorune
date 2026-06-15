#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-758-ROUTE-CARRIER-RESIDUAL-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-757-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-AFTER-PHI-FRESHNESS-NO-OWNER-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_route_carrier_residual_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[route-carrier-residual-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[route-carrier-residual-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[route-carrier-residual-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[route-carrier-residual-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[route-carrier-residual-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[route-carrier-residual-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-route-carrier-residual-inventory-v0" \
  "source_evidence=296x-757,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "body_elapsed_ratio=79.586" \
  "copy_count=51" \
  "local_ssa_copy_materialization_copy_count=20" \
  "closed_phi_freshness_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_phi_freshness_implementation_allowed=0" \
  "route_carrier_residual_copy_count=13" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "block_entry_route_carrier_count=3" \
  "phi_edge_route_carrier_count=8" \
  "dominant_route_carrier_role=call_operand" \
  "selected_role=call_operand" \
  "selected_role_confidence=medium" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_next_action=call_operand_route_carrier_policy_selection" \
  "selected_next_action_reason=call_operand_route_carrier_is_the_only_nonzero_route_carrier_role_after_compare_operand_and_phi_freshness_are_closed" \
  "implementation_allowed=0" \
  "policy_selection_required=1" \
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

grep -F -q "CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001:" "$CARD" || {
  echo "[route-carrier-residual-inventory] next policy-selection row is not documented" >&2
  exit 1
}

echo "[route-carrier-residual-inventory] ok"
