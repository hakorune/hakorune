#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-748-POST-MIR-CALL-COMPARE-OPERAND-FORWARDING-OWNER-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-747-MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001.md"
TOOL="tools/allocator/hako_mimalloc_post_mir_call_compare_operand_forwarding_owner_refresh.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_mir_call_compare_operand_forwarding_owner_refresh_guard.sh"

[[ -f "$CARD" ]] || { echo "[post-mir-call-compare-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-mir-call-compare-owner] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[post-mir-call-compare-owner] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[post-mir-call-compare-owner] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[post-mir-call-compare-owner] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[post-mir-call-compare-owner] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[post-mir-call-compare-owner] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-post-mir-call-compare-operand-forwarding-owner-refresh-v0" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "source_evidence=296x-747" \
  "hako_body_elapsed_ns=365000000" \
  "c_body_elapsed_ns=3727908" \
  "body_elapsed_ratio=97.910" \
  "copy_count=51" \
  "local_ssa_copy_materialization_copy_count=20" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "expression_materialization_copy_count=1" \
  "mir_call_origin_copy_count=0" \
  "const_origin_copy_count=1" \
  "dominant_dynamic_owner=local_ssa_copy_materialization" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "dominant_expression_origin=const" \
  "selected_next_owner=local_ssa_block_entry_phi_edge_copy_family" \
  "selected_owner_confidence=medium" \
  "selected_reason=compare_operand_family_removed_and_residue_moved_to_block_entry_phi_edge_copies" \
  "next_task=local_ssa_block_entry_phi_edge_copy_design" \
  "implementation_allowed=0" \
  "design_required=1" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-COPY-DESIGN-001:" "$CARD" || {
  echo "[post-mir-call-compare-owner] next design row is not documented" >&2
  exit 1
}

echo "[post-mir-call-compare-owner] ok"
