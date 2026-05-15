#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-019-allocator-provider-taskboard-pin-decoupling"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-293x/293x-426-DOCS-SLIM-019-ALLOCATOR-PROVIDER-PROOF-REGISTRY-TASKBOARD-PIN-DECOUPLING.md"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/docs_slim_019_allocator_provider_taskboard_pin_decoupling_guard.sh"

converted_scripts=(
  "tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh"
  "tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh"
  "tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh"
  "tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh"
  "tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh"
  "tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh"
  "tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh"
  "tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-019 allocator provider proof/registry taskboard pin decoupling guard"

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

guard_expect_in_file "$TAG" "DOCS-SLIM-019" "$CARD" "DOCS-SLIM-019 card must exist"
guard_expect_in_file "$TAG" "Do not move numbered cards in this row" "$CARD" "card must keep no-move stop-line"
guard_expect_in_file "$TAG" "Nineteenth Slimming Phase" "$ARCHIVE_POLICY" "archive policy must record DOCS-SLIM-019"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$CHECK_INDEX" "check index must list DOCS-SLIM-019 guard"

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
