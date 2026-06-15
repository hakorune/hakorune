#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-737-RECORD-WITH-BOX-GATE-000.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
EBNF="docs/reference/language/EBNF.md"
TYPES="docs/reference/language/types.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_record_with_box_gate_guard.sh"

[[ -f "$CARD" ]] || { echo "[record-with-box-gate] missing card: $CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[record-with-box-gate] missing SSOT: $SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[record-with-box-gate] row737 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[record-with-box-gate] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[record-with-box-gate] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-record-with-box-gate-v0" \
  "record_with_update_enabled=1" \
  "ordinary_box_with_enabled=0" \
  "automatic_record_to_box_copy=0" \
  "record_update_is_replacement=1" \
  "record_update_is_mutation=0" \
  "selected_next=SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "ordinary_box_with_enabled=0" \
  "automatic_record_to_box_copy=0"; do
  require_line_in_file "$SSOT" "$expected"
done

grep -F -q "record_update := expr 'with'" "$EBNF" || {
  echo "[record-with-box-gate] EBNF missing record_update with form" >&2
  exit 1
}
grep -F -q 'Ordinary boxes do not support `with`.' "$EBNF" || {
  echo "[record-with-box-gate] EBNF missing ordinary-box with prohibition" >&2
  exit 1
}
grep -F -q 'Ordinary boxes do not support `with` copy/update semantics.' "$TYPES" || {
  echo "[record-with-box-gate] types reference missing ordinary-box with prohibition" >&2
  exit 1
}

echo "[record-with-box-gate] ok"
