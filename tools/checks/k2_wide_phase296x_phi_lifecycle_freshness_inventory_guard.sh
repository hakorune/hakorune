#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-756-PHI-LIFECYCLE-FRESHNESS-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-755-PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_phi_lifecycle_freshness_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[phi-lifecycle-freshness-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[phi-lifecycle-freshness-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[phi-lifecycle-freshness-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[phi-lifecycle-freshness-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[phi-lifecycle-freshness-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[phi-lifecycle-freshness-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-phi-lifecycle-freshness-inventory-v0" \
  "input_contract=hako-mimalloc-phi-lifecycle-freshness-guard-surface-v0" \
  "source_evidence=296x-755,296x-750,296x-751,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "body_elapsed_ratio=79.586" \
  "copy_count=51" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "block_entry_route_none_count=7" \
  "block_entry_route_carrier_count=3" \
  "safe_candidate_count=0" \
  "closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family" \
  "phi_edge_reopen_gate=phi_lifecycle_proof" \
  "phi_edge_rewrite_uses_phi_lifecycle=0" \
  "phi_edge_rewrite_preserves_phi_inputs=0" \
  "phi_lifecycle_rewrite_proof_available=0" \
  "block_entry_route_none_reopen_gate=freshness_proof" \
  "block_entry_freshness_proof_available=0" \
  "variable_map_defined_only_invariant_preserved=1" \
  "phi_predecessor_remap_safe=0" \
  "block_entry_route_carrier_reopen_gate=route_specific_operand_policy" \
  "route_specific_operand_policy_available=0" \
  "arg_forwarding_enabled=0" \
  "helper_name_special_case=0" \
  "benchmark_name_branch_count=0" \
  "broad_local_ssa_coalescing_allowed=0" \
  "selected_owner=none" \
  "selected_owner_confidence=none" \
  "implementation_allowed=0" \
  "next_task=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-AFTER-PHI-FRESHNESS-NO-OWNER-001" \
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

for expected in \
  "proof_available=0" \
  "implementation_allowed=0" \
  "do not reopen this family from stale 750/753 evidence" \
  "MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-AFTER-PHI-FRESHNESS-NO-OWNER-001:"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[phi-lifecycle-freshness-inventory] missing inventory text: $expected" >&2
    exit 1
  }
done

echo "[phi-lifecycle-freshness-inventory] ok"
