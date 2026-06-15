#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-741-CURRENT-ENTRY-MANUAL-SYNC-001.md"
CURRENT="docs/development/current/main/CURRENT_STATE.toml"
TASK="CURRENT_TASK.md"
LAYOUT="docs/development/current/main/DOCS_LAYOUT.md"
NOW="docs/development/current/main/10-Now.md"
DOCS_README="docs/README.md"
LANG_README="docs/reference/language/README.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_current_entry_manual_sync_guard.sh"

[[ -f "$CARD" ]] || { echo "[current-entry-manual-sync] missing card: $CARD" >&2; exit 1; }
grep -q '^Status: Landed$' "$CARD" || { echo "[current-entry-manual-sync] card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[current-entry-manual-sync] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[current-entry-manual-sync] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-current-entry-manual-sync-v0" \
  "current_task_read_next_latest_card=296x-741" \
  "current_now_read_next_latest_card=296x-741" \
  "current_state_latest_card=296x-741" \
  "docs_layout_manual_sync_bucket_visible=1" \
  "docs_readme_concurrency_nav_linked=1" \
  "language_reference_record_box_nav_linked=1" \
  "language_reference_concurrency_nav_linked=1" \
  "manual_sync_guard_indexed=1" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q 'latest_card = "CURRENT-ENTRY-MANUAL-SYNC-001"' "$CURRENT" || {
  echo "[current-entry-manual-sync] CURRENT_STATE latest_card must be 741" >&2
  exit 1
}
grep -F -q "296x-741-CURRENT-ENTRY-MANUAL-SYNC-001.md" "$CURRENT" || {
  echo "[current-entry-manual-sync] CURRENT_STATE missing latest card path" >&2
  exit 1
}
read_next_block="$(sed -n '/^## Read Next$/,$p' "$TASK")"
grep -F -q "2. \`docs/development/current/main/phases/phase-296x/296x-741-CURRENT-ENTRY-MANUAL-SYNC-001.md\`" <<<"$read_next_block" || {
  echo "[current-entry-manual-sync] CURRENT_TASK Read Next must point to 741" >&2
  exit 1
}
now_read_next_block="$(sed -n '/^## Read Next$/,$p' "$NOW")"
grep -F -q "2. \`docs/development/current/main/phases/phase-296x/296x-741-CURRENT-ENTRY-MANUAL-SYNC-001.md\`" <<<"$now_read_next_block" || {
  echo "[current-entry-manual-sync] 10-Now Read Next must point to 741" >&2
  exit 1
}
grep -F -q "manual/current-entry synchronization" "$LAYOUT" || {
  echo "[current-entry-manual-sync] DOCS_LAYOUT missing manual sync bucket" >&2
  exit 1
}
grep -F -q "reference/concurrency/semantics.md" "$DOCS_README" || {
  echo "[current-entry-manual-sync] docs README missing concurrency semantics link" >&2
  exit 1
}
grep -F -q "Record vs Box / Object Storage" "$LANG_README" || {
  echo "[current-entry-manual-sync] language README missing record/box nav" >&2
  exit 1
}
grep -F -q "Concurrency / Thread Boundary" "$LANG_README" || {
  echo "[current-entry-manual-sync] language README missing concurrency nav" >&2
  exit 1
}

echo "[current-entry-manual-sync] ok"
