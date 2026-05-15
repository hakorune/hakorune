#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-018-allocator-provider-taskboard-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-293x/293x-425-DOCS-SLIM-018-ALLOCATOR-PROVIDER-TASKBOARD-PIN-DECOUPLING.md"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_018_allocator_provider_taskboard_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-018 allocator provider taskboard pin decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$DEV_GATE" \
  "$ALLOCATOR_GROUP" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-018" "$CARD" "DOCS-SLIM-018 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Eighteenth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-018"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-018 guard"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'require_text "\$TASKBOARD"' "$script" "$script must keep design taskboard assertions"
done

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GROUP"

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
