#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-768-MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-767-MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_measurement_state_provenance_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-measurement-state-provenance] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-measurement-state-provenance] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-measurement-state-provenance] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-measurement-state-provenance] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-measurement-state-provenance] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-measurement-state-provenance] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-measurement-state-provenance-inventory-v0" \
  "source_evidence=296x-767,296x-766,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "row753_runtime_config_profile=root" \
  "row753_hako_body_elapsed_ns=374000000" \
  "row753_body_elapsed_ratio=79.586" \
  "row753_hako_external_elapsed_ms=400" \
  "row753_external_peak_rss_bytes=9482240" \
  "row753_canonical_env_contract_recorded=0" \
  "row753_worker_front_mismatch_guard_recorded=0" \
  "row767_measurement_profile=canonical_direct_exact_pair_v0" \
  "row767_body_elapsed_ratio_median=2.119" \
  "row767_body_elapsed_ratio_max=2.363" \
  "row767_previous_outlier_reproduced=0" \
  "root_reprobe_hako_body_elapsed_ns=7000000" \
  "root_reprobe_external_elapsed_ms=10" \
  "empty_reprobe_hako_body_elapsed_ns=6000000" \
  "empty_reprobe_external_elapsed_ms=10" \
  "runtime_config_root_reproduces_outlier=0" \
  "runtime_config_mismatch_explains_outlier=0" \
  "row753_small_alloc_inst_count=157" \
  "current_small_alloc_inst_count=157" \
  "row753_small_alloc_copy_count=51" \
  "current_small_alloc_copy_count=51" \
  "row753_small_alloc_call_count=19" \
  "current_small_alloc_call_count=19" \
  "mir_shape_count_mismatch=0" \
  "old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier" \
  "old_large_gap_allowed_as_optimization_owner=0" \
  "current_reliable_body_ratio_floor=about_2x" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_owner=none" \
  "selected_owner_reason=old_large_gap_not_reproduced_and_current_2x_gap_needs_boundary_inventory" \
  "selected_next_action=runtime_boundary_inventory_for_current_2x_gap" \
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

grep -F -q "MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001:" "$CARD" || {
  echo "[mimalloc-measurement-state-provenance] next runtime boundary row is not documented" >&2
  exit 1
}

echo "[mimalloc-measurement-state-provenance] ok"
