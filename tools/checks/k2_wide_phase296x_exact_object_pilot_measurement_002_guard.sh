#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-730-EXACT-OBJECT-PILOT-MEASUREMENT-002.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-729-EXACT-OBJECT-PILOT-001V.md"
TOOL="tools/allocator/hako_exact_object_pilot_measurement_002.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_measurement_002_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-measurement-002] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-measurement-002] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-measurement-002] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-measurement-002] row730 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-measurement-002] row729 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-measurement-002] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-measurement-002] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-measurement-002-v0"
require_line_in_file "$CARD" "source_evidence=296x-729"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "pilot_exact_object_enabled=1"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "global_arc_retirement_claim=0"
require_line_in_file "$CARD" "body_elapsed_ratio_before=114.326"
require_line_in_file "$CARD" "body_elapsed_ratio_after=117.038"
require_line_in_file "$CARD" "hako_body_elapsed_ns_after=368000000"
require_line_in_file "$CARD" "c_body_elapsed_ns_after=3144269"
require_line_in_file "$CARD" "winner_claim=0"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-CLOSEOUT-001"
require_line_in_file "$CARD" "summary=ok"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
pair="$tmp_dir/pair.out"
report="$tmp_dir/report.out"
cat > "$pair" <<'PAIR'
output_contract=hako-mimalloc-object-lifecycle-body-timing-pair-v0
workload_id=representative-object-lifecycle-small-block-v0
body_elapsed_comparable=1
hako_body_elapsed_ns=368000000
c_body_elapsed_ns=3144269
body_elapsed_ratio=117.038
summary=ok
PAIR

python3 "$TOOL" --pair-report "$pair" --out "$report"

require_line_in_file "$report" "output_contract=hako-exact-object-pilot-measurement-002-v0"
require_line_in_file "$report" "body_elapsed_ratio_before=114.326"
require_line_in_file "$report" "body_elapsed_ratio_after=117.038"
require_line_in_file "$report" "winner_claim=0"
require_line_in_file "$report" "selected_next=EXACT-OBJECT-PILOT-CLOSEOUT-001"
require_line_in_file "$report" "summary=ok"

echo "[exact-object-measurement-002] ok"
