#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-020-allocator-provider-taskboard-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-427-DOCS-SLIM-020-ALLOCATOR-PROVIDER-MANIFEST-READINESS-REGISTRY-TASKBOARD-PIN-DECOUPLING.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/phase_card_paths.sh"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_020_allocator_provider_taskboard_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh"
  "tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh"
  "tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh"
  "tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-020 allocator provider taskboard pin decoupling guard"

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

guard_expect_in_file "$TAG" "DOCS-SLIM-020" "$CARD" "DOCS-SLIM-020 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Twentieth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-020"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-020 guard"

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
