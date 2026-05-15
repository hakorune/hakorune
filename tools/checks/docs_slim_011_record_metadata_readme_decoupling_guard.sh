#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-011-record-metadata-readme-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-293x/293x-418-DOCS-SLIM-011-RECORD-METADATA-README-DECOUPLING.md"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
SELF_SCRIPT="tools/checks/docs_slim_011_record_metadata_readme_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_record_decl_metadata_transport_guard.sh"
  "tools/checks/k2_wide_record_layout_plan_guard.sh"
  "tools/checks/k2_wide_record_local_scalar_metadata_guard.sh"
  "tools/checks/k2_wide_array_record_storage_descriptor_guard.sh"
  "tools/checks/k2_wide_arraybox_inline_record_storage_guard.sh"
  "tools/checks/k2_wide_allocator_metadata_record_declarations_guard.sh"
  "tools/checks/k2_wide_allocator_record_construction_read_guard.sh"
  "tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh"
  "tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-011 record metadata README decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$RECORD_SSOT" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_require_docs_slim_card_metadata \
  "$TAG" \
  "$CARD" \
  "$ARCHIVE_POLICY" \
  "$CHECK_INDEX" \
  "$SELF_SCRIPT" \
  "DOCS-SLIM-011" \
  "Eleventh Slimming Phase"

for script in "${converted_scripts[@]}"; do
  if rg -n 'PHASE_README|phase README must list' "$script" >/tmp/"$TAG".phase_pin 2>&1; then
    echo "[$TAG] ERROR: phase README landed-history pin still present in: $script" >&2
    cat /tmp/"$TAG".phase_pin >&2
    rm -f /tmp/"$TAG".phase_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".phase_pin

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
