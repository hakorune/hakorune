#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-021-allocator-provider-taskboard-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-428-DOCS-SLIM-021-ALLOCATOR-PROVIDER-BOUNDARY-MANIFEST-TASK-BREAKDOWN-TASKBOARD-PIN-DECOUPLING.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_021_allocator_provider_taskboard_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh"
  "tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh"
  "tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-021 allocator provider taskboard pin decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$HELPER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GROUP" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$HELPER" "$SELF_SCRIPT"

guard_require_docs_slim_card_metadata \
  "$TAG" \
  "$CARD" \
  "$ARCHIVE_POLICY" \
  "$CHECK_INDEX" \
  "$SELF_SCRIPT" \
  "DOCS-SLIM-021" \
  "Twenty-first Slimming Phase"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'require_text "\$TASKBOARD"' "$script" "$script must keep design taskboard assertions"
  if rg -n 'REAL_APP_TASKBOARD|real-app taskboard' "$script" >/tmp/"$TAG".real_app_pin 2>&1; then
    echo "[$TAG] ERROR: converted script still contains landed-history real-app taskboard pins: $script" >&2
    cat /tmp/"$TAG".real_app_pin >&2
    rm -f /tmp/"$TAG".real_app_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".real_app_pin

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GROUP"

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
