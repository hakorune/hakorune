#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-v0-functional-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_22="docs/development/current/main/phases/phase-296x/296x-22-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT.md"
CARD_24="docs/development/current/main/phases/phase-296x/296x-24-MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS.md"
CARD_25="docs/development/current/main/phases/phase-296x/296x-25-MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT.md"
DOC="docs/reference/runtime/provider-package-v0.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CLI_IMPL="src/cli/provider_package_existing_binary.rs"
CLI_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh"
DOCS_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_usage_docs_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_functional_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider package v0 functional closeout"

guard_require_files "$TAG" "$CARD_22" "$CARD_24" "$CARD_25" "$DOC" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$CLI_IMPL" "$CLI_GUARD" "$DOCS_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$CLI_GUARD" "$DOCS_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-25-MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select Phase B build selection"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_22" "CLI package pilot must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_24" "usage docs row must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_25" "functional closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001' "$CARD_25" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$CARD_25" "closeout must preserve output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_25" "closeout must keep package command no-load"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$CARD_25" "closeout must keep export resolution closed"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$CARD_25" "closeout must keep descriptor read closed"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_25" "closeout must keep provider calls closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_25" "closeout must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_25" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_25" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_25" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_25" "closeout must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001' "$CARD_25" "closeout must select Phase B build selection"

guard_expect_fixed_in_file "$TAG" 'target/debug/hakorune' "$DOC" "reference docs must show Hakorune CLI command"
guard_expect_fixed_in_file "$TAG" 'provider_package_metadata_preflight.py' "$DOC" "reference docs must show preflight command"
guard_expect_fixed_in_file "$TAG" 'Provider package v0 is complete when the CLI creates the package and metadata' "$DOC" "reference docs must define completion"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT' "$CLI_IMPL" "CLI impl must own package output contract"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CLI_IMPL" "CLI impl must keep loading closed"

guard_expect_fixed_in_file "$TAG" '| 25 | `MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 25 must be landed"
guard_expect_fixed_in_file "$TAG" '| 26 | `MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 26 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list functional closeout guard"

echo "[$TAG] ok"
