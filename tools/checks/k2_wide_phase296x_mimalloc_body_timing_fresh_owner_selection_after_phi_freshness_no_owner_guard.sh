#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-757-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-AFTER-PHI-FRESHNESS-NO-OWNER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-756-PHI-LIFECYCLE-FRESHNESS-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_fresh_owner_selection_after_phi_freshness_no_owner_guard.sh"

[[ -f "$CARD" ]] || { echo "[fresh-owner-after-phi-freshness-no-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[fresh-owner-after-phi-freshness-no-owner] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[fresh-owner-after-phi-freshness-no-owner] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[fresh-owner-after-phi-freshness-no-owner] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[fresh-owner-after-phi-freshness-no-owner] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[fresh-owner-after-phi-freshness-no-owner] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-body-timing-fresh-owner-selection-after-phi-freshness-no-owner-v0" \
  "source_evidence=296x-753,296x-756" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "hako_body_elapsed_ns=374000000" \
  "c_body_elapsed_ns=4699328" \
  "body_elapsed_ratio=79.586" \
  "gap_owner=compiler_lowering" \
  "gap_confidence=medium" \
  "selected_mir_body_owner=local_ssa_copy_materialization" \
  "selected_owner_confidence=high" \
  "copy_count=51" \
  "local_ssa_copy_materialization_copy_count=20" \
  "closed_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_family_phi_edge_copy_count=18" \
  "closed_family_block_entry_copy_count=10" \
  "closed_family_safe_candidate_count=0" \
  "closed_family_selected_owner=none" \
  "closed_family_implementation_allowed=0" \
  "remaining_route_carrier_copy_count=13" \
  "remaining_compare_operand_route_carrier_copy_count=0" \
  "expression_materialization_copy_count=1" \
  "dominant_expression_origin=const" \
  "mir_call_origin_copy_count=0" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_next_action=route_carrier_residual_inventory" \
  "selected_next_action_reason=phi_freshness_family_closed_but_route_carrier_residue_remains_and_needs_role_inventory_before_implementation" \
  "implementation_allowed=0" \
  "measurement_required=0" \
  "inventory_required=1" \
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

grep -F -q "ROUTE-CARRIER-RESIDUAL-INVENTORY-001:" "$CARD" || {
  echo "[fresh-owner-after-phi-freshness-no-owner] next inventory row is not documented" >&2
  exit 1
}

echo "[fresh-owner-after-phi-freshness-no-owner] ok"
