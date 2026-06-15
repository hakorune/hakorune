#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-724-EXACT-OBJECT-PILOT-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-723-EXACT-OBJECT-PILOT-001U.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_measurement_001_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-measurement-001] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-measurement-001] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-measurement-001] row724 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-measurement-001] row723 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-measurement-001] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-measurement-001] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-measurement-v0"
require_line_in_file "$CARD" "source_evidence=296x-723"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "pilot_exact_object_enabled=1"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "global_arc_retirement_claim=0"
require_line_in_file "$CARD" "body_elapsed_ratio_before=112.969"
require_line_in_file "$CARD" "body_elapsed_ratio_after=114.326"
require_line_in_file "$CARD" "hako_body_elapsed_ns_after=374000000"
require_line_in_file "$CARD" "c_body_elapsed_ns_after=3271344"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001"
require_line_in_file "$CARD" "summary=ok"

echo "[exact-object-measurement-001] ok"
