#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-733-RECORD-BOX-DOCS-001.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
TYPES="docs/reference/language/types.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_record_box_docs_guard.sh"

[[ -f "$CARD" ]] || { echo "[record-box-docs] missing card: $CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[record-box-docs] missing SSOT: $SSOT" >&2; exit 1; }
[[ -f "$TYPES" ]] || { echo "[record-box-docs] missing types reference: $TYPES" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[record-box-docs] row733 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[record-box-docs] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[record-box-docs] missing line in $file: $expected" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-record-box-docs-v0"
require_line_in_file "$CARD" "record_box_surface_model=two_surface_one_substrate"
require_line_in_file "$CARD" "reference_types_doc_updated=1"
require_line_in_file "$CARD" "record_is_data_value_wording=1"
require_line_in_file "$CARD" "box_is_thing_owner_behavior_wording=1"
require_line_in_file "$CARD" "performance_first_record_wording=0"
require_line_in_file "$CARD" "ordinary_box_with_enabled=0"
require_line_in_file "$CARD" "automatic_record_to_box_copy=0"
require_line_in_file "$CARD" "selected_next=AGG-STORAGE-PLAN-000"
require_line_in_file "$CARD" "summary=ok"

grep -F -q "record-box-two-surface-one-substrate-ssot.md" "$TYPES" || {
  echo "[record-box-docs] types reference must link record/box SSOT" >&2
  exit 1
}
grep -F -q "data/value:" "$TYPES" || { echo "[record-box-docs] missing data/value wording" >&2; exit 1; }
grep -F -q "thing/owner/behavior/lifecycle:" "$TYPES" || { echo "[record-box-docs] missing box wording" >&2; exit 1; }
grep -F -q 'Ordinary boxes do not support `with` copy/update semantics.' "$TYPES" || {
  echo "[record-box-docs] missing ordinary-box with prohibition" >&2
  exit 1
}

echo "[record-box-docs] ok"
