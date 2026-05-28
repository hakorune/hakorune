#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
REFRESH_TOOL="tools/allocator/mir_post_field_get_cleanup_owner_refresh.py"
MEASURE_TOOL="tools/allocator/hako_mimalloc_post_field_get_cleanup_measurement.py"
CARD="docs/development/current/main/phases/phase-296x/296x-169-ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-168-POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
LOCAL="src/mir/builder/ssa/local.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_rollback_local_ssa_same_block_reuse_guard.sh"

[[ -f "$CARD" ]] || { echo "[row169-rollback-local-ssa] missing card: $CARD" >&2; exit 1; }
[[ -f "$REFRESH_TOOL" ]] || { echo "[row169-rollback-local-ssa] missing tool: $REFRESH_TOOL" >&2; exit 1; }
[[ -f "$MEASURE_TOOL" ]] || { echo "[row169-rollback-local-ssa] missing tool: $MEASURE_TOOL" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row169-rollback-local-ssa] row169 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row169-rollback-local-ssa] row168 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-169-ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE"' "$STATE" || {
  echo "[row169-rollback-local-ssa] CURRENT_STATE latest_card must point to row169" >&2
  exit 1
}
grep -q 'current_blocker_token = "ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE-296X-001"' "$STATE" || {
  echo "[row169-rollback-local-ssa] CURRENT_STATE blocker must point to row169" >&2
  exit 1
}
grep -q '| 168 | `POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row169-rollback-local-ssa] taskboard row168 must be Landed" >&2
  exit 1
}
grep -q '| 169 | `ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row169-rollback-local-ssa] taskboard row169 must be Current" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row169-rollback-local-ssa] check index missing guard entry" >&2
  exit 1
}
if grep -q 'matches!(def_inst, Some(MirInstruction::FieldGet' "$LOCAL"; then
  echo "[row169-rollback-local-ssa] field_get same-block reuse must be removed" >&2
  exit 1
fi

tmp_dir="$(mktemp -d /tmp/hakorune_row169_rollback_local_ssa.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
refresh_report="$tmp_dir/refresh.out"
measure_report="$tmp_dir/measure.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$REFRESH_TOOL" --mir-json "$mir_json" --out "$refresh_report"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row169-rollback-local-ssa] missing report line: $expected" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$refresh_report" "instruction_count=180"
require_line "$refresh_report" "copy_count=88"
require_line "$refresh_report" "local_ssa_copy_count=38"
require_line "$refresh_report" "field_get_result_chain_copy_count=23"
require_line "$refresh_report" "summary=ok"

python3 "$MEASURE_TOOL" --sample-count 1 --out "$measure_report"
require_line "$measure_report" "sample_count=1"
require_line "$measure_report" "summary=ok"

echo "[row169-rollback-local-ssa] ok"
