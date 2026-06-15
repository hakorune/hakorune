#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-749-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-748-POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_block_entry_phi_edge_copy_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-ssa-block-entry-phi-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-ssa-block-entry-phi-design] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-ssa-block-entry-phi-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[local-ssa-block-entry-phi-design] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-ssa-block-entry-phi-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-ssa-block-entry-phi-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-design-v0" \
  "source_evidence=296x-748" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "selected_next_owner=local_ssa_block_entry_phi_edge_copy_family" \
  "selected_owner_confidence=medium" \
  "copy_count=51" \
  "local_ssa_copy_materialization_copy_count=20" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "dominant_dynamic_owner=local_ssa_copy_materialization" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "selected_design=split_inventory_before_implementation" \
  "phi_edge_optimization_allowed=0" \
  "block_entry_optimization_allowed=0" \
  "local_ssa_broad_copy_coalescing_allowed=0" \
  "phi_lifecycle_changed=0" \
  "cfg_changed=0" \
  "copy_emission_ssot_preserved=1" \
  "candidate_probe_required=1" \
  "next_task=local_ssa_block_entry_phi_edge_copy_candidate_probe" \
  "implementation_allowed=0" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001:" "$CARD" || {
  echo "[local-ssa-block-entry-phi-design] next candidate probe row is not documented" >&2
  exit 1
}

echo "[local-ssa-block-entry-phi-design] ok"
