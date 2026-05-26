#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-cli-package-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_22="docs/development/current/main/phases/phase-296x/296x-22-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT.md"
CARD_23="docs/development/current/main/phases/phase-296x/296x-23-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CLI_IMPL="src/cli/provider_package_existing_binary.rs"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider package CLI package closeout"

guard_require_files "$TAG" "$CARD_22" "$CARD_23" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$CLI_IMPL" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-23-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001"' "$CURRENT_STATE" "current state must expose usage docs row"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_22" "CLI package pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_23" "CLI package closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001' "$CARD_23" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" '--provider-package-existing-binary' "$CARD_23" "closeout must preserve CLI entry"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$CARD_23" "closeout must preserve output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_23" "closeout must keep loading closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_23" "closeout must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_23" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_23" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_23" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_23" "closeout must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001' "$CARD_23" "closeout must select usage docs"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT' "$CLI_IMPL" "CLI impl must keep contract owner"

guard_expect_fixed_in_file "$TAG" '| 23 | `MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 23 must be landed"
guard_expect_fixed_in_file "$TAG" '| 24 | `MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001` | Current |' "$TASKBOARD" "taskboard row 24 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list CLI closeout guard"

echo "[$TAG] ok"
