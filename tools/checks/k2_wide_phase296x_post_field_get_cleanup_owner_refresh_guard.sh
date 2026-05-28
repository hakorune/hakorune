#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/mir_post_field_get_cleanup_owner_refresh.py"
CARD="docs/development/current/main/phases/phase-296x/296x-164-POST-FIELD-GET-CLEANUP-OWNER-REFRESH.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-163-POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_field_get_cleanup_owner_refresh_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$TOOL" ]] || { echo "[row164-owner-refresh] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row164-owner-refresh] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row164-owner-refresh] row164 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row164-owner-refresh] row163 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-164-POST-FIELD-GET-CLEANUP-OWNER-REFRESH"' "$STATE" || {
  echo "[row164-owner-refresh] CURRENT_STATE latest_card must point to row164" >&2
  exit 1
}
grep -q 'current_blocker_token = "POST-FIELD-GET-CLEANUP-OWNER-REFRESH-296X-001"' "$STATE" || {
  echo "[row164-owner-refresh] CURRENT_STATE blocker must point to row164" >&2
  exit 1
}
grep -q '| 163 | `POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row164-owner-refresh] taskboard row163 must be Landed" >&2
  exit 1
}
grep -q '| 164 | `POST-FIELD-GET-CLEANUP-OWNER-REFRESH-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row164-owner-refresh] taskboard row164 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row164-owner-refresh] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row164-owner-refresh] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row164_owner_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune \
  --backend mir \
  --emit-mir-json "$mir_json" \
  "$APP" >/dev/null

python3 "$TOOL" --mir-json "$mir_json" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row164-owner-refresh] missing report line: $expected" >&2
    echo "[row164-owner-refresh] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0"
require_line "input_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "instruction_count=180"
require_line "copy_count=88"
require_line "call_adjacent_copy_count=40"
require_line "expression_materialization_copy_count=24"
require_line "field_get_result_chain_copy_count=23"
require_line "dominant_position=call_adjacent"
require_line "dominant_expression_owner=field_get_result_chain"
require_line "selected_owner=field_get_result_chain_follow_on_probe"
require_line "owner_confidence=medium"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row164-owner-refresh] ok"
