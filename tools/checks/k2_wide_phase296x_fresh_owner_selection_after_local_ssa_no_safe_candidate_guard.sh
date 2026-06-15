#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-752-FRESH-OWNER-SELECTION-AFTER-LOCAL-SSA-NO-SAFE-CANDIDATE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-751-LOCAL-SSA-BLOCK-ENTRY-PHI-EDGE-NO-SAFE-CANDIDATE-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fresh_owner_selection_after_local_ssa_no_safe_candidate_guard.sh"

[[ -f "$CARD" ]] || { echo "[fresh-owner-after-local-ssa-nonkeeper] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[fresh-owner-after-local-ssa-nonkeeper] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[fresh-owner-after-local-ssa-nonkeeper] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[fresh-owner-after-local-ssa-nonkeeper] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[fresh-owner-after-local-ssa-nonkeeper] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[fresh-owner-after-local-ssa-nonkeeper] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-fresh-owner-selection-after-local-ssa-no-safe-candidate-v0" \
  "source_evidence=296x-747,296x-748,296x-751" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "last_measured_hako_body_elapsed_ns=365000000" \
  "last_measured_c_body_elapsed_ns=3727908" \
  "last_measured_body_elapsed_ratio=97.910" \
  "closed_family=local_ssa_block_entry_phi_edge_copy_family" \
  "closed_family_safe_candidate_count=0" \
  "closed_family_selected_policy=none" \
  "closed_family_implementation_allowed=0" \
  "fresh_high_confidence_owner_selected=0" \
  "selected_next_action=body_timing_rebaseline_after_local_ssa_nonkeeper" \
  "selected_next_action_reason=last_measurement_precedes_no_safe_candidate_closeout_and_current_owner_is_closed" \
  "implementation_allowed=0" \
  "measurement_required=1" \
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

grep -F -q "MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001:" "$CARD" || {
  echo "[fresh-owner-after-local-ssa-nonkeeper] next rebaseline row is not documented" >&2
  exit 1
}

echo "[fresh-owner-after-local-ssa-nonkeeper] ok"
