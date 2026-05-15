#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="docs-slim-026-phase-card-resolver-leak-helper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh
source tools/checks/lib/phase_card_paths.sh

CARD="$(guard_require_phase293x_card "$TAG" "293x-433-DOCS-SLIM-026-PHASE-CARD-RESOLVER-LEAK-HELPER-EXTRACTION.md")"
CHECK_INDEX="docs/tools/check-scripts-index.md"
ARCHIVE_POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
HELPER="tools/checks/lib/guard_common.sh"
PHASE_HELPER="tools/checks/lib/phase_card_paths.sh"
SELF_SCRIPT="tools/checks/docs_slim_026_phase_card_resolver_leak_helper_guard.sh"

converted_scripts=(
  "tools/checks/docs_slim_004_activation_closeout_resolver_guard.sh"
  "tools/checks/docs_slim_005_production_closeout_resolver_guard.sh"
  "tools/checks/docs_slim_006_m10c_runtime_decl_resolver_guard.sh"
  "tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh"
  "tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh"
  "tools/checks/docs_slim_009_proof_surface_resolver_guard.sh"
  "tools/checks/docs_slim_010_manifest_runner_decoupling_guard.sh"
  "tools/checks/docs_slim_013_packed_record_history_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_014_packed_record_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_015_allocator_hook_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_016_allocator_hook_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_017_allocator_provider_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_018_allocator_provider_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_019_allocator_provider_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_020_allocator_provider_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_021_allocator_provider_taskboard_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_022_allocator_provider_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_023_allocator_provider_readme_pin_decoupling_guard.sh"
  "tools/checks/docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh"
)

echo "[$TAG] running DOCS-SLIM-026 phase card resolver leak helper guard"

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

guard_require_docs_slim_card_metadata \
  "$TAG" \
  "$CARD" \
  "$ARCHIVE_POLICY" \
  "$CHECK_INDEX" \
  "$SELF_SCRIPT" \
  "DOCS-SLIM-026" \
  "Twenty-sixth Slimming Phase"

guard_expect_in_file "$TAG" "guard_require_no_phase_card_resolver_leak" "$HELPER" "guard_common must expose phase-card resolver leak helper"

actionable_count=0
for script in "${converted_scripts[@]}"; do
  guard_expect_in_file "$TAG" 'guard_require_no_phase_card_resolver_leak' "$script" "$script must call shared phase-card resolver leak helper"
  if rg -n 'phase_card_leak' "$script" >/tmp/"$TAG".legacy_phase_card_leak 2>&1; then
    echo "[$TAG] ERROR: converted script still contains raw phase-card leak assertions: $script" >&2
    cat /tmp/"$TAG".legacy_phase_card_leak >&2
    rm -f /tmp/"$TAG".legacy_phase_card_leak
    exit 1
  fi
  actionable_count=$((actionable_count + 1))
done
rm -f /tmp/"$TAG".legacy_phase_card_leak

for script in "${converted_scripts[@]}"; do
  bash -n "$script" >/dev/null
done

for script in "${converted_scripts[@]}"; do
  bash "$script" >/dev/null
done

echo "[$TAG] ok converted=${actionable_count}"
