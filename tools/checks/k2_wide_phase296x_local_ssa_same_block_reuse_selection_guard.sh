#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/mir_local_ssa_same_block_reuse_selection.py"
CARD="docs/development/current/main/phases/phase-296x/296x-166-LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-165-FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_same_block_reuse_selection_guard.sh"

[[ -f "$TOOL" ]] || { echo "[row166-local-ssa-selection] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row166-local-ssa-selection] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row166-local-ssa-selection] row166 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row166-local-ssa-selection] row165 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-166-LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION"' "$STATE" || {
  echo "[row166-local-ssa-selection] CURRENT_STATE latest_card must point to row166" >&2
  exit 1
}
grep -q 'current_blocker_token = "LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION-296X-001"' "$STATE" || {
  echo "[row166-local-ssa-selection] CURRENT_STATE blocker must point to row166" >&2
  exit 1
}
grep -q '| 165 | `FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row166-local-ssa-selection] taskboard row165 must be Landed" >&2
  exit 1
}
grep -q '| 166 | `LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row166-local-ssa-selection] taskboard row166 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row166-local-ssa-selection] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row166-local-ssa-selection] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row166_local_ssa_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row166-local-ssa-selection] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-local-ssa-same-block-reuse-selection-v0"
require_line "input_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0"
require_line "selected_owner=local_ssa_same_block_reuse"
require_line "selected_file=src/mir/builder/ssa/local.rs"
require_line "selected_function=ensure_inner"
require_line "selected_rule=return_original_value_when_def_block_is_current_block"
require_line "guarded_boundary=non_dominating_and_cross_block_values_keep_existing_copy_path"
require_line "implementation_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row166-local-ssa-selection] ok"
