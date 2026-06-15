#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-736-RECORD-METHODS-GATE-000.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
EBNF="docs/reference/language/EBNF.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_record_methods_gate_guard.sh"

[[ -f "$CARD" ]] || { echo "[record-methods-gate] missing card: $CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[record-methods-gate] missing SSOT: $SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[record-methods-gate] row736 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[record-methods-gate] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[record-methods-gate] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-record-methods-gate-v0" \
  "record_methods_enabled=0" \
  "record_fini_enabled=0" \
  "record_dynamic_dispatch_enabled=0" \
  "record_member_grammar_excludes_method_decl=1" \
  "box_owns_behavior_surface=1" \
  "selected_next=RECORD-WITH-BOX-GATE-000" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "record_methods_enabled=0" \
  "record_fini_enabled=0" \
  "record_dynamic_dispatch_enabled=0"; do
  require_line_in_file "$SSOT" "$expected"
done

grep -F -q "record_member:= record_field | invariant_member" "$EBNF" || {
  echo "[record-methods-gate] EBNF record members must exclude method_decl" >&2
  exit 1
}

echo "[record-methods-gate] ok"
