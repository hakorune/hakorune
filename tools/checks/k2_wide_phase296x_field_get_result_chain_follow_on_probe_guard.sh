#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/mir_field_get_result_chain_follow_on_probe.py"
CARD="docs/development/current/main/phases/phase-296x/296x-165-FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-164-POST-FIELD-GET-CLEANUP-OWNER-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_result_chain_follow_on_probe_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$TOOL" ]] || { echo "[row165-field-get-follow-on] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row165-field-get-follow-on] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row165-field-get-follow-on] row165 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row165-field-get-follow-on] row164 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-165-FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE"' "$STATE" || {
  echo "[row165-field-get-follow-on] CURRENT_STATE latest_card must point to row165" >&2
  exit 1
}
grep -q 'current_blocker_token = "FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE-296X-001"' "$STATE" || {
  echo "[row165-field-get-follow-on] CURRENT_STATE blocker must point to row165" >&2
  exit 1
}
grep -q '| 164 | `POST-FIELD-GET-CLEANUP-OWNER-REFRESH-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row165-field-get-follow-on] taskboard row164 must be Landed" >&2
  exit 1
}
grep -q '| 165 | `FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row165-field-get-follow-on] taskboard row165 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row165-field-get-follow-on] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row165-field-get-follow-on] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row165_field_get_follow_on.XXXXXX)"
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
    echo "[row165-field-get-follow-on] missing report line: $expected" >&2
    echo "[row165-field-get-follow-on] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0"
require_line "input_contract=hako-mimalloc-post-field-get-cleanup-owner-refresh-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "field_get_result_chain_copy_count=30"
require_line "same_block_origin_copy_count=30"
require_line "selected_owner=local_ssa_same_block_field_get_reuse_probe"
require_line "owner_confidence=medium"
require_line "owner_reason=same_block_field_get_origins_and_internal_copy_chains_dominate"
require_line "consumer_copy_source_count=15"
require_line "consumer_phi_incoming_count=10"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row165-field-get-follow-on] ok"
