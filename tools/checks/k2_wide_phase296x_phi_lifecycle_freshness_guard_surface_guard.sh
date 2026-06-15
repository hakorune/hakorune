#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-755-PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-754-PHI-LIFECYCLE-FRESHNESS-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_phi_lifecycle_freshness_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[phi-lifecycle-freshness-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[phi-lifecycle-freshness-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[phi-lifecycle-freshness-guard-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[phi-lifecycle-freshness-guard-surface] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[phi-lifecycle-freshness-guard-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[phi-lifecycle-freshness-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-phi-lifecycle-freshness-guard-surface-v0" \
  "input_contract=hako-mimalloc-phi-lifecycle-freshness-design-v0" \
  "source_evidence=296x-754" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "body_elapsed_ratio=79.586" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "block_entry_route_none_count=7" \
  "block_entry_route_carrier_count=3" \
  "closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_nonkeeper_safe_candidate_count=0" \
  "guard_surface_defined=1" \
  "phi_edge_reopen_gate=phi_lifecycle_proof" \
  "block_entry_route_none_reopen_gate=freshness_proof" \
  "block_entry_route_carrier_reopen_gate=route_specific_operand_policy" \
  "variable_map_defined_only_guard_required=1" \
  "phi_lifecycle_mutation_guard_required=1" \
  "local_ssa_broad_coalescing_guard_required=1" \
  "cfg_semantics_changed_allowed=0" \
  "freshness_proof_available=0" \
  "phi_lifecycle_rewrite_proof_available=0" \
  "route_specific_operand_policy_available=0" \
  "implementation_allowed=0" \
  "next_task=PHI-LIFECYCLE-FRESHNESS-INVENTORY-001" \
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
  "phi_edge_rewrite_uses_phi_lifecycle=0|1" \
  "block_entry_freshness_proof_available=0|1" \
  "route_specific_operand_policy_available=0|1" \
  "do not implement from this guard-surface row" \
  "PHI-LIFECYCLE-FRESHNESS-INVENTORY-001:"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[phi-lifecycle-freshness-guard-surface] missing guard text: $expected" >&2
    exit 1
  }
done

echo "[phi-lifecycle-freshness-guard-surface] ok"
