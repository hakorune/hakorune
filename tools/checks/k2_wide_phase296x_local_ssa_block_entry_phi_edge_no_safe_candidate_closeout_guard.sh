#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-751-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-750-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-CANDIDATE-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_block_entry_phi_edge_no_safe_candidate_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[local-ssa-block-entry-phi-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[local-ssa-block-entry-phi-closeout] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[local-ssa-block-entry-phi-closeout] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[local-ssa-block-entry-phi-closeout] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[local-ssa-block-entry-phi-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[local-ssa-block-entry-phi-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-no-safe-candidate-closeout-v0" \
  "input_contract=hako-mimalloc-local-ssa-block-entry-phi-edge-copy-candidate-probe-v0" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "source_evidence=296x-750" \
  "copy_count=51" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "block_entry_route_none_count=7" \
  "block_entry_route_carrier_count=3" \
  "safe_candidate_count=0" \
  "selected_policy=none" \
  "family_closed_as_non_keeper=1" \
  "phi_edge_owner_required=phi_lifecycle" \
  "block_entry_route_carrier_owner_required=route_specific_operand_policy" \
  "block_entry_route_none_owner_required=freshness_proof" \
  "phi_edge_optimization_allowed=0" \
  "block_entry_route_carrier_optimization_allowed=0" \
  "block_entry_route_none_optimization_allowed=0" \
  "local_ssa_broad_copy_coalescing_allowed=0" \
  "freshness_proof_available=0" \
  "phi_lifecycle_changed=0" \
  "cfg_changed=0" \
  "copy_emission_ssot_preserved=1" \
  "next_task=fresh_owner_selection_after_local_ssa_no_safe_candidate" \
  "implementation_allowed=0" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001:" "$CARD" || {
  echo "[local-ssa-block-entry-phi-closeout] fresh owner selection row is not documented" >&2
  exit 1
}

echo "[local-ssa-block-entry-phi-closeout] ok"
