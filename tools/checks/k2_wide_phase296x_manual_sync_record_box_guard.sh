#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-739-MANUAL-SYNC-RECORD-BOX-001.md"
README="README.md"
BOX_README="docs/reference/boxes-system/README.md"
EIB="docs/reference/boxes-system/everything-is-box.md"
TYPES="docs/reference/language/types.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_manual_sync_record_box_guard.sh"

[[ -f "$CARD" ]] || { echo "[manual-sync-record-box] missing card: $CARD" >&2; exit 1; }
grep -q '^Status: Landed$' "$CARD" || { echo "[manual-sync-record-box] card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[manual-sync-record-box] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[manual-sync-record-box] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-manual-sync-record-box-v0" \
  "record_box_surface_model=two_surface_one_substrate" \
  "readme_box_first_architecture_removed=1" \
  "readme_everything_is_box_slogan_removed=1" \
  "boxes_system_historical_banner_added=1" \
  "record_methods_disabled_reference_visible=1" \
  "object_storage_plan_reference_linked=1" \
  "aggregate_storage_plan_reference_linked=1" \
  "ordinary_box_with_enabled=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

if grep -F -q "### 2. **Box-First Architecture**" "$README"; then
  echo "[manual-sync-record-box] README still advertises Box-First Architecture as current" >&2
  exit 1
fi
if grep -F -q "Where Everything is a Box" "$README"; then
  echo "[manual-sync-record-box] README still uses Everything is a Box slogan" >&2
  exit 1
fi

grep -F -q "### 2. **Boundary-First Architecture**" "$README" || {
  echo "[manual-sync-record-box] README missing Boundary-First Architecture section" >&2
  exit 1
}
grep -F -q "Historical note:" "$BOX_README" || { echo "[manual-sync-record-box] boxes README missing historical note" >&2; exit 1; }
grep -F -q "Historical note:" "$EIB" || { echo "[manual-sync-record-box] everything-is-box page missing historical note" >&2; exit 1; }
grep -F -q "no methods / fini / dynamic dispatch in v0" "$TYPES" || {
  echo "[manual-sync-record-box] types.md missing record method/fini/dynamic dispatch disabled wording" >&2
  exit 1
}
grep -F -q "object-storage-plan-boundary-ssot.md" "$TYPES" || {
  echo "[manual-sync-record-box] types.md missing ObjectStoragePlan link" >&2
  exit 1
}
grep -F -q "296x-734-AGG-STORAGE-PLAN-000.md" "$TYPES" || {
  echo "[manual-sync-record-box] types.md missing AggregateStoragePlan link" >&2
  exit 1
}

echo "[manual-sync-record-box] ok"
