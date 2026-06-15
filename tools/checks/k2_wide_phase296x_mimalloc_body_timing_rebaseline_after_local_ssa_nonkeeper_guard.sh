#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-752-FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_rebaseline_after_local_ssa_nonkeeper_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-body-timing-rebaseline-after-local-ssa-nonkeeper-v0" \
  "source_evidence=296x-751,296x-752" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "hako_body_elapsed_ns=374000000" \
  "c_body_elapsed_ns=4699328" \
  "body_elapsed_ratio=79.586" \
  "gap_owner=compiler_lowering" \
  "gap_confidence=medium" \
  "selected_mir_body_owner=local_ssa_copy_materialization" \
  "selected_owner_confidence=high" \
  "dominant_dynamic_owner=local_ssa_copy_materialization" \
  "copy_count=51" \
  "local_ssa_copy_materialization_copy_count=20" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "call_operand_route_carrier_copy_count=13" \
  "compare_operand_route_carrier_copy_count=0" \
  "expression_materialization_copy_count=1" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "dominant_expression_origin=const" \
  "mir_call_origin_copy_count=0" \
  "const_origin_copy_count=1" \
  "closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_nonkeeper_safe_candidate_count=0" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_next=PHI-LIFECYCLE-FRESHNESS-DESIGN-001" \
  "selected_next_reason=body_gap_large_but_dominant_residue_requires_phi_lifecycle_and_block_entry_freshness_design" \
  "implementation_allowed=0" \
  "design_required=1" \
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

grep -F -q "PHI-LIFECYCLE-FRESHNESS-DESIGN-001:" "$CARD" || {
  echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] next design row is not documented" >&2
  exit 1
}

echo "[mimalloc-body-rebaseline-local-ssa-nonkeeper] ok"
