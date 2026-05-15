#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-025-docs-slim-card-metadata-helper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-432-DOCS-SLIM-025-DOCS-SLIM-CARD-METADATA-HELPER-EXTRACTION.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/guard_common.sh"
PHASE_HELPER="tools/checks/lib/phase_card_paths.sh"
SELF_SCRIPT="tools/checks/docs_slim_025_docs_slim_card_metadata_helper_guard.sh"

converted_scripts=(
  "tools/checks/docs_slim_022_allocator_provider_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_023_allocator_provider_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-025 docs-slim card metadata helper guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$HELPER" \
  "$PHASE_HELPER" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$PHASE_HELPER" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-025" "$CARD" "DOCS-SLIM-025 card must exist"
guard_expect_in_file "$TAG" "Twenty-fifth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-025"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-025 guard"
guard_expect_in_file "$TAG" "guard_require_docs_slim_card_metadata" "$HELPER" "guard_common must expose docs-slim card metadata helper"
guard_expect_in_file "$TAG" "guard_require_docs_slim_no_move_stop_line" "$HELPER" "guard_common must expose docs-slim stop-line helper"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'guard_require_docs_slim_card_metadata' "$script" "$script must call shared docs-slim card metadata helper"
done

if rg -n 'guard_expect_in_file "\$TAG" "DOCS-SLIM-022" "\$CARD"|guard_expect_in_file "\$TAG" "DOCS-SLIM-023" "\$CARD"|guard_expect_in_file "\$TAG" "DOCS-SLIM-024" "\$CARD"' "${converted_scripts[@]}" >/tmp/"$TAG".legacy_card_checks 2>&1; then
  echo "[$TAG] ERROR: converted scripts still contain legacy docs-slim card metadata assertions" >&2
  cat /tmp/"$TAG".legacy_card_checks >&2
  rm -f /tmp/"$TAG".legacy_card_checks
  exit 1
fi
rm -f /tmp/"$TAG".legacy_card_checks

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
