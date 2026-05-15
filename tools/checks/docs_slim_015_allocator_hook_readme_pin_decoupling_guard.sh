#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-015-allocator-hook-readme-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-293x/293x-422-DOCS-SLIM-015-ALLOCATOR-HOOK-README-PIN-DECOUPLING.md"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_015_allocator_hook_readme_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh"
  "tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh"
  "tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh"
  "tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh"
  "tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh"
  "tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh"
  "tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh"
  "tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh"
  "tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh"
  "tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh"
  "tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh"
  "tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-015 allocator hook README pin decoupling guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$ARCHIVE_POLICY" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT" \
  "${converted_scripts[@]}"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "DOCS-SLIM-015" "$CARD" "DOCS-SLIM-015 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Fifteenth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-015"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-015 guard"

for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'require_text "\$TASKBOARD"' "$script" "$script must keep taskboard assertions"
  if rg -n 'PHASE_README|phase README must list' "$script" >/tmp/"$TAG".history_pin 2>&1; then
    echo "[$TAG] ERROR: converted script still contains landed-history phase README pins: $script" >&2
    cat /tmp/"$TAG".history_pin >&2
    rm -f /tmp/"$TAG".history_pin
    exit 1
  fi
done
rm -f /tmp/"$TAG".history_pin

guard_require_no_phase_card_resolver_leak "$TAG" "$DEV_GATE" "$ALLOCATOR_GATE"

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${#converted_scripts[@]}"
