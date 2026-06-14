#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-697-POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-696-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-698-POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_cfg_stable_receiver_rewrite_measurement_guard.sh"

[[ -f "$CARD" ]] || { echo "[post-cfg-stable-receiver-measurement] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-cfg-stable-receiver-measurement] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[post-cfg-stable-receiver-measurement] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[post-cfg-stable-receiver-measurement] row697 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[post-cfg-stable-receiver-measurement] row696 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[post-cfg-stable-receiver-measurement] row698 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[post-cfg-stable-receiver-measurement] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[post-cfg-stable-receiver-measurement] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-post-cfg-stable-receiver-rewrite-measurement-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-696"
  require_line_in_file "$file" "measurement_repeat_count=4"
  require_line_in_file "$file" "hako_body_elapsed_ns=6500000"
  require_line_in_file "$file" "c_body_elapsed_ns=3962131"
  require_line_in_file "$file" "body_elapsed_gap_ns=2537869"
  require_line_in_file "$file" "body_elapsed_ratio=1.640"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=post_cfg_stable_receiver_rewrite_stability_measurement"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-697"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[post-cfg-stable-receiver-measurement] ok"
