#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-024-production-allocator-port-readme-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-431-DOCS-SLIM-024-PRODUCTION-ALLOCATOR-PORT-AND-MIMALLOC-CLOSEOUT-README-PIN-DECOUPLING.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh"
  "tools/checks/k2_wide_production_allocator_port_closeout_guard.sh"
  "tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh"
  "tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-024 production allocator port README pin decoupling guard"

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
  "DOCS-SLIM-024" \
  "Twenty-fourth Slimming Phase"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'require_text "\$TASKBOARD"' "$script" "$script must keep design taskboard assertions"
  guard_expect_in_file "$TAG" 'require_text "\$REAL_APP_TASKBOARD"' "$script" "$script must keep real-app taskboard assertions"
  if rg -n 'PHASE_README|phase README must list' "$script" >/tmp/"$TAG".phase_readme_pin 2>&1; then
    echo "[$TAG] ERROR: converted script still contains landed-history phase README pins: $script" >&2
    cat /tmp/"$TAG".phase_readme_pin >&2
    rm -f /tmp/"$TAG".phase_readme_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".phase_readme_pin

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GROUP"

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
