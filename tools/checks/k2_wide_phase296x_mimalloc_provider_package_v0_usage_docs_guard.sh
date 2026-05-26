#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-v0-usage-docs"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

DOC="docs/reference/runtime/provider-package-v0.md"
CARD_24="docs/development/current/main/phases/phase-296x/296x-24-MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
REFERENCE_INDEX="docs/reference/README.md"
CLI_IMPL="src/cli/provider_package_existing_binary.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_usage_docs_guard.sh"

echo "[$TAG] checking phase-296x provider package v0 usage docs"

guard_require_files "$TAG" "$DOC" "$CARD_24" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$REFERENCE_INDEX" "$CLI_IMPL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-24-MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001"' "$CURRENT_STATE" "current state must expose functional closeout row"

guard_expect_fixed_in_file "$TAG" 'Status: Active' "$DOC" "usage docs must be active reference docs"
guard_expect_fixed_in_file "$TAG" 'target/debug/hakorune' "$DOC" "usage docs must show Hakorune CLI command"
guard_expect_fixed_in_file "$TAG" '--provider-package-existing-binary' "$DOC" "usage docs must document package input flag"
guard_expect_fixed_in_file "$TAG" '--provider-package-out-dir' "$DOC" "usage docs must document output directory flag"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-existing-binary-manifest-v0' "$DOC" "usage docs must document package output contract"
guard_expect_fixed_in_file "$TAG" 'hakorune_provider.json' "$DOC" "usage docs must document manifest output"
guard_expect_fixed_in_file "$TAG" 'hakorune_provider.sha256' "$DOC" "usage docs must document sha256 output"
guard_expect_fixed_in_file "$TAG" 'provider_package_metadata_preflight.py' "$DOC" "usage docs must document metadata preflight path"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$DOC" "usage docs must keep package command no-load"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$DOC" "usage docs must keep descriptor closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$DOC" "usage docs must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$DOC" "usage docs must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$DOC" "usage docs must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$DOC" "usage docs must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$DOC" "usage docs must keep winners closed"
guard_expect_fixed_in_file "$TAG" '.hako-to-shared-library generation' "$DOC" "usage docs must keep hako shared library generation out of v0"
guard_expect_fixed_in_file "$TAG" 'docs/reference/runtime/provider-package-v0.md' "$REFERENCE_INDEX" "reference README must link provider package docs"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_24" "usage docs card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001' "$CARD_24" "card must identify docs blocker"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001' "$CARD_24" "card must select functional closeout"

guard_expect_fixed_in_file "$TAG" '| 24 | `MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001` | Landed |' "$TASKBOARD" "taskboard row 24 must be landed"
guard_expect_fixed_in_file "$TAG" '| 25 | `MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001` | Current |' "$TASKBOARD" "taskboard row 25 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list usage docs guard"
guard_expect_fixed_in_file "$TAG" 'OUTPUT_CONTRACT' "$CLI_IMPL" "CLI impl must keep package contract owner"

echo "[$TAG] ok"
