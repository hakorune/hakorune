#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="tools/allocator/mir_expression_materialization_owner_selection.py"
CARD="docs/development/current/main/phases/phase-296x/296x-160-EXPRESSION-MATERIALIZATION-OWNER-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-159-LOCAL-SSA-COPY-BLOCK-POSITION-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_expression_materialization_owner_selection_guard.sh"

[[ -f "$APP" ]] || { echo "[row160-expression-owner] missing app: $APP" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[row160-expression-owner] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row160-expression-owner] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row160-expression-owner] row160 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row160-expression-owner] row159 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-160-EXPRESSION-MATERIALIZATION-OWNER-SELECTION"' "$STATE" || {
  echo "[row160-expression-owner] CURRENT_STATE latest_card must point to row160" >&2
  exit 1
}
grep -q 'current_blocker_token = "EXPRESSION-MATERIALIZATION-OWNER-SELECTION-296X-001"' "$STATE" || {
  echo "[row160-expression-owner] CURRENT_STATE blocker must point to row160" >&2
  exit 1
}
grep -q '| 159 | `LOCAL-SSA-COPY-BLOCK-POSITION-PROBE-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row160-expression-owner] taskboard row159 must be Landed" >&2
  exit 1
}
grep -q '| 160 | `EXPRESSION-MATERIALIZATION-OWNER-SELECTION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row160-expression-owner] taskboard row160 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row160-expression-owner] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row160-expression-owner] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row160_expression_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" --mir-json "$mir_json" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row160-expression-owner] missing report line: $expected" >&2
    echo "[row160-expression-owner] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-expression-materialization-owner-selection-v0"
require_line "input_contract=hako-mimalloc-local-ssa-copy-position-probe-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "expression_materialization_copy_count=29"
require_line "selected_owner=field_get_result_chain"
require_line "owner_confidence=medium"
require_line "field_get_result_chain_copy_count=28"
require_line "phi_result_chain_copy_count=1"
require_line "top_block_owner_0=block_552:field_get_result_chain"
require_line "top_block_owner_0_copy_count=14"
require_line "sample_0_owner=field_get_result_chain"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row160-expression-owner] ok"
