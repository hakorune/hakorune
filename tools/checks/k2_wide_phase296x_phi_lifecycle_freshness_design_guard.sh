#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-754-PHI-LIFECYCLE-FRESHNESS-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md"
PHI_SSOT="docs/development/current/main/design/phi-lifecycle-ssot.md"
PHI_IMPL="src/mir/builder/emission/phi_lifecycle.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_phi_lifecycle_freshness_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[phi-lifecycle-freshness-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[phi-lifecycle-freshness-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$PHI_SSOT" ]] || { echo "[phi-lifecycle-freshness-design] missing PHI SSOT: $PHI_SSOT" >&2; exit 1; }
[[ -f "$PHI_IMPL" ]] || { echo "[phi-lifecycle-freshness-design] missing PHI impl: $PHI_IMPL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[phi-lifecycle-freshness-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[phi-lifecycle-freshness-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[phi-lifecycle-freshness-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[phi-lifecycle-freshness-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-phi-lifecycle-freshness-design-v0" \
  "source_evidence=296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "body_elapsed_ratio=79.586" \
  "gap_owner=compiler_lowering" \
  "gap_confidence=medium" \
  "selected_mir_body_owner=local_ssa_copy_materialization" \
  "dominant_position=phi_edge" \
  "dominant_local_like_position=block_entry" \
  "phi_edge_copy_count=18" \
  "block_entry_copy_count=10" \
  "block_entry_route_none_count=7" \
  "block_entry_route_carrier_count=3" \
  "closed_nonkeeper_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_nonkeeper_safe_candidate_count=0" \
  "phi_lifecycle_truth_owner=src/mir/builder/emission/phi_lifecycle.rs" \
  "phi_lifecycle_contract=Reserve_Define_Populate_Finalize" \
  "variable_map_role=defined_value_emission_cache" \
  "local_ssa_role=block_local_operand_materialization" \
  "freshness_truth_owner_required=1" \
  "block_entry_copy_reopen_requires_freshness_proof=1" \
  "phi_edge_copy_reopen_requires_phi_lifecycle_proof=1" \
  "route_carrier_copy_reopen_requires_route_specific_operand_policy=1" \
  "broad_local_ssa_coalescing_allowed=0" \
  "implementation_allowed=0" \
  "design_required=1" \
  "next_task=PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001" \
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
  "variable_map_defined_only_invariant_preserved=1" \
  "phi_edge_rewrite_uses_phi_lifecycle=1" \
  "block_entry_freshness_proof_available=1" \
  "PHI-LIFECYCLE-FRESHNESS-GUARD-SURFACE-001:"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[phi-lifecycle-freshness-design] missing required contract text: $expected" >&2
    exit 1
  }
done

grep -F -q "Define 済み ValueId のみ" "$PHI_SSOT" || {
  echo "[phi-lifecycle-freshness-design] PHI SSOT missing variable_map defined-only invariant" >&2
  exit 1
}
grep -F -q "pub(in crate::mir::builder) struct PhiTxn" "$PHI_IMPL" || {
  echo "[phi-lifecycle-freshness-design] PHI impl missing PhiTxn boundary" >&2
  exit 1
}

echo "[phi-lifecycle-freshness-design] ok"
