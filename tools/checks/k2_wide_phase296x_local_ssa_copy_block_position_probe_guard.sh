#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="tools/allocator/mir_local_ssa_copy_position_probe.py"
CARD="docs/development/current/main/phases/phase-296x/296x-159-LOCAL-SSA-COPY-BLOCK-POSITION-PROBE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-158-CALLSITE-COPY-OWNER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_copy_block_position_probe_guard.sh"

[[ -f "$APP" ]] || { echo "[row159-local-ssa-position] missing app: $APP" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[row159-local-ssa-position] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row159-local-ssa-position] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row159-local-ssa-position] row159 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row159-local-ssa-position] row158 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-159-LOCAL-SSA-COPY-BLOCK-POSITION-PROBE"' "$STATE" || {
  echo "[row159-local-ssa-position] CURRENT_STATE latest_card must point to row159" >&2
  exit 1
}
grep -q 'current_blocker_token = "LOCAL-SSA-COPY-BLOCK-POSITION-PROBE-296X-001"' "$STATE" || {
  echo "[row159-local-ssa-position] CURRENT_STATE blocker must point to row159" >&2
  exit 1
}
grep -q '| 158 | `CALLSITE-COPY-OWNER-SELECTION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row159-local-ssa-position] taskboard row158 must be Landed" >&2
  exit 1
}
grep -q '| 159 | `LOCAL-SSA-COPY-BLOCK-POSITION-PROBE-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row159-local-ssa-position] taskboard row159 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row159-local-ssa-position] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row159-local-ssa-position] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row159_local_ssa_position.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" --mir-json "$mir_json" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row159-local-ssa-position] missing report line: $expected" >&2
    echo "[row159-local-ssa-position] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-local-ssa-copy-position-probe-v0"
require_line "input_contract=hako-mimalloc-callsite-copy-owner-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "block_count=20"
require_line "copy_count=98"
require_line "local_like_copy_count=48"
require_line "dominant_position=call_adjacent"
require_line "dominant_local_like_position=expression_materialization"
require_line "expression_materialization_copy_count=29"
require_line "return_block_copy_count=0"
require_line "branch_condition_copy_count=6"
require_line "block_entry_copy_count=8"
require_line "call_adjacent_copy_count=40"
require_line "phi_edge_copy_count=10"
require_line "top_block_0_id=block_552"
require_line "top_block_0_copy_count=17"
require_line "sample_0_category=expression_materialization"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row159-local-ssa-position] ok"
