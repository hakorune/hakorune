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
  guard_expect_in_file "$TAG" 'require_text "\$PHASE_README"' "$script" "$script must keep phase README assertions"
  guard_expect_in_file "$TAG" 'require_text "\$TASKBOARD"' "$script" "$script must keep design taskboard assertions"
  if rg -n 'REAL_APP_TASKBOARD|real-app taskboard' "$script" >/tmp/"$TAG".real_app_pin 2>&1; then
    echo "[$TAG] ERROR: converted script still contains landed-history real-app taskboard pins: $script" >&2
    cat /tmp/"$TAG".real_app_pin >&2
    rm -f /tmp/"$TAG".real_app_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".real_app_pin

if rg -n 'phase_card_paths|guard_require_phase293x_card' "$DEV_GATE" "$ALLOCATOR_GROUP" >/tmp/"$TAG".gate_leak 2>&1; then
  echo "[$TAG] ERROR: phase-card resolver helper must not be wired into dev_gate or allocator-wide directly" >&2
  cat /tmp/"$TAG".gate_leak >&2
  rm -f /tmp/"$TAG".gate_leak
  exit 1
fi
rm -f /tmp/"$TAG".gate_leak

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
