#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-013-packed-record-history-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-420-DOCS-SLIM-013-PACKED-RECORD-HISTORY-PIN-DECOUPLING.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_013_packed_record_history_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_arraybox_inline_record_autouse_eligibility_guard.sh"
  "tools/checks/k2_wide_arraybox_inline_record_materialization_boundary_guard.sh"
  "tools/checks/k2_wide_arraybox_inline_record_autouse_pilot_guard.sh"
  "tools/checks/k2_wide_aligned_small_metadata_packed_store_pilot_guard.sh"
  "tools/checks/k2_wide_huge_page_metadata_packed_store_pilot_guard.sh"
  "tools/checks/k2_wide_packed_record_backend_failfast_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-013 packed record history pin decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$HELPER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$HELPER" "$SELF_SCRIPT"

guard_require_docs_slim_card_metadata \
  "$TAG" \
  "$CARD" \
  "$ARCHIVE_POLICY" \
  "$CHECK_INDEX" \
  "$SELF_SCRIPT" \
  "DOCS-SLIM-013" \
  "Thirteenth Slimming Phase"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" "phase_card_paths.sh" "$script" "$script must source phase card resolver helper"
  guard_expect_in_file "$TAG" "guard_require_phase293x_card" "$script" "$script must resolve cards via helper"
  if rg -n 'docs/development/current/main/phases/phase-293x/293x-[0-9][0-9][0-9]-' "$script" >/tmp/"$TAG".direct_path 2>&1; then
    echo "[$TAG] ERROR: converted script still contains direct phase-293x card paths: $script" >&2
    cat /tmp/"$TAG".direct_path >&2
    rm -f /tmp/"$TAG".direct_path
    exit 1
  fi
  if rg -n 'PHASE_README|phase README must list' "$script" >/tmp/"$TAG".history_pin 2>&1; then
    echo "[$TAG] ERROR: converted script still contains landed-history phase README pins: $script" >&2
    cat /tmp/"$TAG".history_pin >&2
    rm -f /tmp/"$TAG".history_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".direct_path
rm -f /tmp/"$TAG".history_pin

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GATE"

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
