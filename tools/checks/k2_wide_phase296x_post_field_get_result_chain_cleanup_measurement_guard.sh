#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TOOL="tools/allocator/hako_mimalloc_post_field_get_cleanup_measurement.py"
CARD="docs/development/current/main/phases/phase-296x/296x-163-POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-162-FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_field_get_result_chain_cleanup_measurement_guard.sh"

[[ -f "$TOOL" ]] || { echo "[row163-post-field-get-measurement] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row163-post-field-get-measurement] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row163-post-field-get-measurement] row163 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row163-post-field-get-measurement] row162 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-163-POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT"' "$STATE" || {
  echo "[row163-post-field-get-measurement] CURRENT_STATE latest_card must point to row163" >&2
  exit 1
}
grep -q 'current_blocker_token = "POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001"' "$STATE" || {
  echo "[row163-post-field-get-measurement] CURRENT_STATE blocker must point to row163" >&2
  exit 1
}
grep -q '| 162 | `FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row163-post-field-get-measurement] taskboard row162 must be Landed" >&2
  exit 1
}
grep -q '| 163 | `POST-FIELD-GET-RESULT-CHAIN-CLEANUP-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row163-post-field-get-measurement] taskboard row163 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row163-post-field-get-measurement] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row163-post-field-get-measurement] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row163_post_field_get_measure.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --sample-count 1 --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row163-post-field-get-measurement] missing report line: $expected" >&2
    echo "[row163-post-field-get-measurement] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-field-get-result-chain-cleanup-measurement-v0"
require_line "input_contract=field-get-result-chain-cleanup-implementation-v0"
require_line "workload_id=representative-object-lifecycle-small-block-v0"
require_line "sample_count=1"
require_line "keeper=field_get_result_chain_cleanup"
require_line "select_page_single_fast_path_count=524288"
require_line "select_page_single_fallback_count=0"
require_line "release_known_page_fast_path_count=524288"
require_line "release_known_page_fallback_count=0"
require_line "previous_checkpoint_hako_elapsed_median_ms=560"
require_line "structural_keeper=1"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

grep -q '^after_hako_elapsed_median_ms=[0-9][0-9]*$' "$report" || {
  echo "[row163-post-field-get-measurement] report must include numeric median" >&2
  cat "$report" >&2
  exit 1
}

echo "[row163-post-field-get-measurement] ok"
