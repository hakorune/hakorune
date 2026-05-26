#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-existing-binary-manifest-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_20="docs/development/current/main/phases/phase-296x/296x-20-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT.md"
CARD_21="docs/development/current/main/phases/phase-296x/296x-21-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
TOOL="tools/allocator/provider_package_existing_binary_manifest.py"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider package existing-binary manifest closeout"

guard_require_files "$TAG" "$CARD_20" "$CARD_21" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$TOOL" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-21-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001"' "$CURRENT_STATE" "current state must expose CLI package pilot"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_20" "package helper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_21" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001' "$CARD_21" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$CARD_21" "closeout must preserve package contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_21" "closeout must keep load closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_21" "closeout must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_21" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_21" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_21" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_21" "closeout must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001' "$CARD_21" "closeout must select CLI package pilot"

guard_expect_fixed_in_file "$TAG" '| 21 | `MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 21 must be landed"
guard_expect_fixed_in_file "$TAG" '| 22 | `MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 22 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list closeout guard"

echo "[$TAG] ok"
