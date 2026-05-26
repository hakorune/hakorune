#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-explicit-comparison-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_07="docs/development/current/main/phases/phase-296x/296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT.md"
CARD_15="docs/development/current/main/phases/phase-296x/296x-15-MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT.md"
CARD_17="docs/development/current/main/phases/phase-296x/296x-17-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_contract_guard.sh"

echo "[$TAG] checking phase-296x provider explicit comparison contract"

guard_require_files "$TAG" "$CARD_07" "$CARD_15" "$CARD_17" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_07" "exact-EXE repeated measurement must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_15" "provider explicit repeated measurement must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_17" "comparison contract card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-contract-v0' "$CARD_17" "card must define output contract"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_17" "card must define 3-way subjects"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=phase296x-provider-explicit-comparison-v0' "$CARD_17" "card must define profile"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_17" "card must preserve sample count"
guard_expect_fixed_in_file "$TAG" 'warmup_count=1' "$CARD_17" "card must preserve warmup count"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=128' "$CARD_17" "card must preserve operation repeat"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_17" "card must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_17" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_17" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_17" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001' "$CARD_17" "card must select adapter pilot"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$CARD_17" "card must consume exact-EXE/C repeated evidence"
guard_expect_fixed_in_file "$TAG" 'output_contract=hakorune-provider-explicit-repeated-measurement-v0' "$CARD_17" "card must consume provider repeated evidence"

guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001' "$TASKBOARD" "taskboard must expose comparison contract row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001' "$TASKBOARD" "taskboard must expose adapter pilot row"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list comparison contract guard"

echo "[$TAG] ok"
