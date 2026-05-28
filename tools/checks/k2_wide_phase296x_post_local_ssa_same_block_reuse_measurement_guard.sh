#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/hako_mimalloc_post_local_ssa_same_block_reuse_measurement.py"
CARD="docs/development/current/main/phases/phase-296x/296x-168-POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-167-LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_local_ssa_same_block_reuse_measurement_guard.sh"

[[ -f "$TOOL" ]] || { echo "[row168-local-ssa-measurement] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row168-local-ssa-measurement] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row168-local-ssa-measurement] row168 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row168-local-ssa-measurement] row167 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-168-POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT"' "$STATE" || {
  echo "[row168-local-ssa-measurement] CURRENT_STATE latest_card must point to row168" >&2
  exit 1
}
grep -q 'current_blocker_token = "POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT-296X-001"' "$STATE" || {
  echo "[row168-local-ssa-measurement] CURRENT_STATE blocker must point to row168" >&2
  exit 1
}
grep -q '| 167 | `LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row168-local-ssa-measurement] taskboard row167 must be Landed" >&2
  exit 1
}
grep -q '| 168 | `POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row168-local-ssa-measurement] taskboard row168 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row168-local-ssa-measurement] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row168-local-ssa-measurement] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row168_local_ssa_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --sample-count 1 --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row168-local-ssa-measurement] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-local-ssa-same-block-reuse-measurement-v0"
require_line "input_contract=local-ssa-same-block-reuse-implementation-v0"
require_line "sample_count=1"
require_line "keeper=local_ssa_same_block_field_get_reuse"
require_line "previous_checkpoint_hako_elapsed_median_ms=550"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row168-local-ssa-measurement] ok"
