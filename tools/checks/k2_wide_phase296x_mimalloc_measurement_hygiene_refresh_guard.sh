#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-767-MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-766-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_measurement_hygiene_refresh_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-measurement-hygiene-refresh] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-measurement-hygiene-refresh] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-measurement-hygiene-refresh] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-measurement-hygiene-refresh] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-measurement-hygiene-refresh] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-measurement-hygiene-refresh] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-measurement-hygiene-refresh-v0" \
  "source_evidence=296x-766,296x-753" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "measurement_profile=canonical_direct_exact_pair_v0" \
  "sample_count=5" \
  "body_elapsed_ratio_min=1.501" \
  "body_elapsed_ratio_median=2.119" \
  "body_elapsed_ratio_max=2.363" \
  "previous_outlier_body_elapsed_ratio=79.586" \
  "previous_outlier_reproduced=0" \
  "refreshed_gap_owner=measurement_harness" \
  "refreshed_gap_confidence=low" \
  "measurement_state_drift_detected=1" \
  "fresh_high_confidence_implementation_owner_selected=0" \
  "selected_owner=none" \
  "selected_owner_reason=previous_large_body_gap_not_reproduced_under_canonical_direct_exact_pair" \
  "selected_next_action=measurement_state_provenance_inventory" \
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

grep -F -q "MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001:" "$CARD" || {
  echo "[mimalloc-measurement-hygiene-refresh] next provenance row is not documented" >&2
  exit 1
}

echo "[mimalloc-measurement-hygiene-refresh] ok"
