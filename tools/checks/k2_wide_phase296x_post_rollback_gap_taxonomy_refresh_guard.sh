#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/hako_mimalloc_post_rollback_gap_taxonomy_refresh.py"
CARD="docs/development/current/main/phases/phase-296x/296x-170-POST-ROLLBACK-GAP-TAXONOMY-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-169-ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_rollback_gap_taxonomy_refresh_guard.sh"

[[ -f "$TOOL" ]] || { echo "[row170-gap-taxonomy] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row170-gap-taxonomy] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row170-gap-taxonomy] row170 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row170-gap-taxonomy] row169 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-170-POST-ROLLBACK-GAP-TAXONOMY-REFRESH"' "$STATE" || {
  echo "[row170-gap-taxonomy] CURRENT_STATE latest_card must point to row170" >&2
  exit 1
}
grep -q 'current_blocker_token = "POST-ROLLBACK-GAP-TAXONOMY-REFRESH-296X-001"' "$STATE" || {
  echo "[row170-gap-taxonomy] CURRENT_STATE blocker must point to row170" >&2
  exit 1
}
grep -q '| 169 | `ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row170-gap-taxonomy] taskboard row169 must be Landed" >&2
  exit 1
}
grep -q '| 170 | `POST-ROLLBACK-GAP-TAXONOMY-REFRESH-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row170-gap-taxonomy] taskboard row170 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row170-gap-taxonomy] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row170-gap-taxonomy] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row170_gap_taxonomy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row170-gap-taxonomy] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0"
require_line "input_contract=rollback-local-ssa-same-block-reuse-v0"
require_line "workload_id=representative-object-lifecycle-small-block-v0"
require_line "current_c_exact_pair_available=0"
require_line "hako_body_elapsed_available=0"
require_line "body_elapsed_comparable=0"
require_line "mir_shape_timing_correlation=weak"
require_line "selected_gap_owner=measurement_contract_gap"
require_line "gap_confidence=high"
require_line "next_diagnostic=object_lifecycle_body_timing_and_exact_c_pair_contract"
require_line "next_optimization_allowed=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row170-gap-taxonomy] ok"
