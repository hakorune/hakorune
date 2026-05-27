#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-benchmark-return-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_39="docs/development/current/main/phases/phase-296x/296x-39-MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
PROVIDER_MEASURE="tools/allocator/provider_package_explicit_repeated_measurement.py"
COMPARISON_ADAPTER="tools/allocator/provider_explicit_comparison_adapter.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_benchmark_return_selection_guard.sh"

echo "[$TAG] checking phase-296x provider package benchmark return selection"

guard_require_files "$TAG" "$CARD_39" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_39" "selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001' "$CARD_39" "selection card must identify blocker"
guard_expect_fixed_in_file "$TAG" '.hako exact-EXE + C mimalloc repeated measurement' "$CARD_39" "selection card must return to hako/C measurement"
guard_expect_fixed_in_file "$TAG" '.hako-derived provider package explicit alloc/free repeated measurement' "$CARD_39" "selection card must select provider package measurement"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_39" "selection card must keep 3-way subjects"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_39" "selection card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$CARD_39" "selection card must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_39" "selection card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_39" "selection card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_39" "selection card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001' "$CARD_39" "selection card must select comparison pilot"

guard_expect_fixed_in_file "$TAG" '296x-39 Selected the provider package explicit comparison pilot' "$CURRENT_STATE" "current state landed tail must retain row 39"
guard_expect_fixed_in_file "$TAG" '| 39 | `MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 39 must be landed"
guard_expect_fixed_in_file "$TAG" '| 40 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 40 must be landed after pilot"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list benchmark return selection guard"

python3 -m py_compile "$PROVIDER_MEASURE" "$COMPARISON_ADAPTER"

echo "[$TAG] ok"
