#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-732-RECORD-BOX-SURFACE-000.md"
SSOT="docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_record_box_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[record-box-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[record-box-surface] missing SSOT: $SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[record-box-surface] row732 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[record-box-surface] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[record-box-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for file in "$CARD" "$SSOT"; do
  require_line_in_file "$file" "record_box_surface_model=two_surface_one_substrate"
  require_line_in_file "$file" "record_identity_free_value_surface=1"
  require_line_in_file "$file" "box_identity_behavior_lifecycle_surface=1"
  require_line_in_file "$file" "source_surface_collapsed_to_box=0"
  require_line_in_file "$file" "record_methods_enabled=0"
  require_line_in_file "$file" "ordinary_box_with_enabled=0"
  require_line_in_file "$file" "automatic_record_to_box_copy=0"
  require_line_in_file "$file" "aggregate_storage_plan_shared_substrate=1"
  require_line_in_file "$file" "object_storage_plan_shared_substrate=1"
  require_line_in_file "$file" "mirbuilder_representation_owner=0"
done

require_line_in_file "$CARD" "output_contract=hako-record-box-surface-v0"
require_line_in_file "$CARD" "selected_next=RECORD-BOX-DOCS-001"
require_line_in_file "$CARD" "summary=ok"

echo "[record-box-surface] ok"
