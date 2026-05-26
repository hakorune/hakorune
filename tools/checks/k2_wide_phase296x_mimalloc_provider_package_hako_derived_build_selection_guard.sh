#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-hako-derived-build-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_29="docs/development/current/main/phases/phase-296x/296x-29-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
ARTIFACT_SSOT="docs/development/current/main/design/provider-package-artifact-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_build_selection_guard.sh"

echo "[$TAG] checking phase-296x .hako-derived provider package build selection"

guard_require_files "$TAG" "$CARD_29" "$TASKBOARD" "$ARTIFACT_SSOT" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_29" "selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001' "$CARD_29" "selection card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'selected .hako provider fixture' "$CARD_29" "selection must start from selected .hako source"
guard_expect_fixed_in_file "$TAG" 'MIR JSON emission preflight' "$CARD_29" "selection must require MIR JSON preflight"
guard_expect_fixed_in_file "$TAG" 'source_hash + mir_json_hash included in package contract/build metadata' "$CARD_29" "selection must tie package metadata to source and MIR hashes"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-package-hako-derived-build-v0' "$CARD_29" "selection must name new .hako-derived output contract"
guard_expect_fixed_in_file "$TAG" 'package_mode=hako-derived-provider-package' "$CARD_29" "selection must name package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=hako-derived-selected-fixture' "$CARD_29" "selection must name build mode"
guard_expect_fixed_in_file "$TAG" 'hako_source_checked=1' "$CARD_29" "selection must require source preflight"
guard_expect_fixed_in_file "$TAG" 'hako_mir_json_emitted=1' "$CARD_29" "selection must require MIR JSON emission"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$CARD_29" "selection must keep semantic provider codegen closed"
guard_expect_fixed_in_file "$TAG" 'shared_library_artifact_generated=1' "$CARD_29" "selection must still target a provider package artifact"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_29" "selection must keep package build no-load"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_29" "selection must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_29" "selection must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_29" "selection must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_29" "selection must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_29" "selection must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001' "$CARD_29" "selection must choose the minimal fixture pilot"

guard_expect_fixed_in_file "$TAG" '## Phase C Selection' "$ARTIFACT_SSOT" "artifact SSOT must document Phase C"
guard_expect_fixed_in_file "$TAG" 'selected .hako provider fixture' "$ARTIFACT_SSOT" "artifact SSOT must start Phase C from selected .hako"
guard_expect_fixed_in_file "$TAG" 'package_mode=hako-derived-provider-package' "$ARTIFACT_SSOT" "artifact SSOT must pin hako-derived package mode"
guard_expect_fixed_in_file "$TAG" 'build_mode=hako-derived-selected-fixture' "$ARTIFACT_SSOT" "artifact SSOT must pin hako-derived build mode"
guard_expect_fixed_in_file "$TAG" 'hako_semantic_provider_codegen=0' "$ARTIFACT_SSOT" "artifact SSOT must keep semantic codegen closed"

guard_expect_fixed_in_file "$TAG" '| 29 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 29 must be landed"
guard_expect_fixed_in_file "$TAG" '| 30 | `MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 30 must be landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list hako-derived selection guard"

echo "[$TAG] ok"
