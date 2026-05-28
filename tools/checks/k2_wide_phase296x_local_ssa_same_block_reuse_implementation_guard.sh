#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
REFRESH_TOOL="tools/allocator/mir_post_field_get_cleanup_owner_refresh.py"
FOLLOW_TOOL="tools/allocator/mir_field_get_result_chain_follow_on_probe.py"
CARD="docs/development/current/main/phases/phase-296x/296x-167-LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-166-LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
LOCAL="src/mir/builder/ssa/local.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_same_block_reuse_implementation_guard.sh"

[[ -f "$APP" ]] || { echo "[row167-local-ssa-impl] missing app: $APP" >&2; exit 1; }
[[ -f "$REFRESH_TOOL" ]] || { echo "[row167-local-ssa-impl] missing tool: $REFRESH_TOOL" >&2; exit 1; }
[[ -f "$FOLLOW_TOOL" ]] || { echo "[row167-local-ssa-impl] missing tool: $FOLLOW_TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row167-local-ssa-impl] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row167-local-ssa-impl] row167 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row167-local-ssa-impl] row166 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-167-LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION"' "$STATE" || {
  echo "[row167-local-ssa-impl] CURRENT_STATE latest_card must point to row167" >&2
  exit 1
}
grep -q 'current_blocker_token = "LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION-296X-001"' "$STATE" || {
  echo "[row167-local-ssa-impl] CURRENT_STATE blocker must point to row167" >&2
  exit 1
}
grep -q '| 166 | `LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row167-local-ssa-impl] taskboard row166 must be Landed" >&2
  exit 1
}
grep -q '| 167 | `LOCAL-SSA-SAME-BLOCK-REUSE-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row167-local-ssa-impl] taskboard row167 must be Current" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row167-local-ssa-impl] check index missing guard entry" >&2
  exit 1
}
grep -q 'if def_block == Some(bb)' "$LOCAL" || {
  echo "[row167-local-ssa-impl] local.rs must check same-block definition" >&2
  exit 1
}
grep -q 'matches!(def_inst, Some(MirInstruction::FieldGet' "$LOCAL" || {
  echo "[row167-local-ssa-impl] local.rs must keep same-block reuse narrowed to FieldGet" >&2
  exit 1
}
grep -q 'builder.local_ssa_map.insert(key, v);' "$LOCAL" || {
  echo "[row167-local-ssa-impl] local.rs must cache same-block original value" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row167_local_ssa_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
refresh_report="$tmp_dir/refresh.out"
follow_report="$tmp_dir/follow.out"
proof_log="$tmp_dir/proof.log"
exe_report="$tmp_dir/exe.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$REFRESH_TOOL" --mir-json "$mir_json" --out "$refresh_report"
python3 "$FOLLOW_TOOL" --mir-json "$mir_json" --out "$follow_report"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row167-local-ssa-impl] missing report line: $expected" >&2
    echo "[row167-local-ssa-impl] report follows ($file):" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$refresh_report" "instruction_count=160"
require_line "$refresh_report" "copy_count=68"
require_line "$refresh_report" "local_ssa_copy_count=18"
require_line "$refresh_report" "call_adjacent_copy_count=40"
require_line "$refresh_report" "expression_materialization_copy_count=7"
require_line "$refresh_report" "field_get_result_chain_copy_count=6"
require_line "$follow_report" "field_get_result_chain_copy_count=10"
require_line "$follow_report" "same_block_origin_copy_count=10"
require_line "$follow_report" "summary=ok"

timeout 180s env NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune "$APP" >"$proof_log"

require_line "$proof_log" "allocation_count=524288"
require_line "$proof_log" "free_count=524288"
require_line "$proof_log" "select_page_single_fast_path_count=524288"
require_line "$proof_log" "select_page_single_fallback_count=0"
require_line "$proof_log" "release_known_page_fast_path_count=524288"
require_line "$proof_log" "release_known_page_fallback_count=0"
require_line "$proof_log" "summary=ok"

python3 tools/allocator/hako_mimalloc_post_field_get_cleanup_measurement.py \
  --sample-count 1 \
  --out "$exe_report"

require_line "$exe_report" "sample_count=1"
require_line "$exe_report" "summary=ok"

echo "[row167-local-ssa-impl] ok"
