#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-phase-b-build-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_26="docs/development/current/main/phases/phase-296x/296x-26-MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
ABI_SSOT="docs/development/current/main/design/provider-abi-v1-ssot.md"
RUNTIME_SSOT="docs/development/current/main/design/provider-runtime-load-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_phase_b_build_selection_guard.sh"

echo "[$TAG] checking phase-296x provider package Phase B build selection"

guard_require_files "$TAG" "$CARD_26" "$TASKBOARD" "$CURRENT_STATE" "$ARTIFACT_SSOT" "$ABI_SSOT" "$RUNTIME_SSOT" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-26-MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001"' "$CURRENT_STATE" "current state must select selected-binary build contract pilot"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_26" "selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001' "$CARD_26" "card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'selected-provider-binary build/package lane' "$CARD_26" "card must select Phase B build lane"
guard_expect_fixed_in_file "$TAG" 'Phase C' "$CARD_26" "card must keep hako-derived package later"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001' "$CARD_26" "card must select next blocker"
guard_expect_fixed_in_file "$TAG" 'package_mode=selected-binary-build-package' "$CARD_26" "card must define selected build package mode"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_26" "selection must keep package build no-load"
guard_expect_fixed_in_file "$TAG" 'required_export_resolved=0' "$CARD_26" "selection must keep export resolution closed"
guard_expect_fixed_in_file "$TAG" 'descriptor_read_executed=0' "$CARD_26" "selection must keep descriptor read closed"
guard_expect_fixed_in_file "$TAG" 'provider_call_executed=0' "$CARD_26" "selection must keep provider calls closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_26" "selection must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_26" "selection must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_26" "selection must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_26" "selection must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_26" "selection must keep winners closed"
guard_expect_fixed_in_file "$TAG" 'run an arbitrary user shell command' "$CARD_26" "selection must forbid arbitrary shell build execution"
guard_expect_fixed_in_file "$TAG" 'compile .hako into a shared library' "$CARD_26" "selection must keep hako shared-library generation closed"

guard_expect_fixed_in_file "$TAG" '## Phase B Selection' "$ARTIFACT_SSOT" "artifact SSOT must document Phase B selection"
guard_expect_fixed_in_file "$TAG" 'repo-selected provider source or build fixture' "$ARTIFACT_SSOT" "artifact SSOT must define selected source boundary"
guard_expect_fixed_in_file "$TAG" 'package_mode=selected-binary-build-package' "$ARTIFACT_SSOT" "artifact SSOT must define selected build mode"
guard_expect_fixed_in_file "$TAG" 'Phase C, where `.hako` is compiled into a provider package, remains a later' "$ARTIFACT_SSOT" "artifact SSOT must keep Phase C later"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$ABI_SSOT" "ABI SSOT must keep v0 provider inactive"
guard_expect_fixed_in_file "$TAG" 'Provider package presence alone never activates a provider.' "$RUNTIME_SSOT" "runtime SSOT must keep activation separate"

guard_expect_fixed_in_file "$TAG" '| 26 | `MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 26 must be landed"
guard_expect_fixed_in_file "$TAG" '| 27 | `MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 27 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list Phase B selection guard"

echo "[$TAG] ok"
