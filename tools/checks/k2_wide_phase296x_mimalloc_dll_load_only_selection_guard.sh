#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-dll-load-only-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

ROADMAP="docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
PROVIDER_ABI="docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md"
CARD_07="docs/development/current/main/phases/phase-296x/296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT.md"
CARD_08="docs/development/current/main/phases/phase-296x/296x-08-MIMALLOC-DLL-LOAD-ONLY-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_dll_load_only_selection_guard.sh"

echo "[$TAG] checking phase-296x DLL load-only selection"

guard_require_files "$TAG" "$ROADMAP" "$PROVIDER_ABI" "$CARD_07" "$CARD_08" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_07" "exact-exe repeated measurement must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_08" "DLL load-only selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001' "$CARD_08" "selection card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001' "$CARD_08" "selection card must select metadata preflight"
guard_expect_fixed_in_file "$TAG" 'dll_mode=metadata-preflight' "$CARD_08" "selection card must choose metadata preflight mode"
guard_expect_fixed_in_file "$TAG" 'shared_library_load_executed=0' "$CARD_08" "selection card must keep shared-library load closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_08" "selection card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_08" "selection card must keep replacement inactive"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_08" "selection card must keep global allocator inactive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_08" "selection card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'manifest,' "$CARD_08" "selection card must mention manifest metadata"
guard_expect_fixed_in_file "$TAG" 'descriptor, hash, and host-side preflight' "$CARD_08" "selection card must follow provider package ABI ordering"
guard_expect_fixed_in_file "$TAG" 'Descriptor/manifest schema fixture and host preflight contract' "$PROVIDER_ABI" "provider ABI SSOT must define no-load preflight row"
guard_expect_fixed_in_file "$TAG" 'dll_mode=load-only' "$ROADMAP" "roadmap must define first DLL row as load-only"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001' "$TASKBOARD" "taskboard must expose load-only selection row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001' "$TASKBOARD" "taskboard must expose metadata preflight row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
