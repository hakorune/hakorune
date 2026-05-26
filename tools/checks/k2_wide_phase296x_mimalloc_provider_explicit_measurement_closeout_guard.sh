#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-explicit-measurement-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_15="docs/development/current/main/phases/phase-296x/296x-15-MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT.md"
CARD_16="docs/development/current/main/phases/phase-296x/296x-16-MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
PREV_GUARD="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_repeated_measurement_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_measurement_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider explicit measurement closeout"

guard_require_files "$TAG" "$CARD_15" "$CARD_16" "$TASKBOARD" "$INDEX" "$CURRENT_STATE" "$PREV_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PREV_GUARD" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-16-MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001"' "$CURRENT_STATE" "current state must expose comparison contract blocker"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_15" "explicit repeated measurement must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_16" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001' "$CARD_16" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'provider-explicit-repeated-measurement=landed' "$CARD_16" "closeout must preserve repeated measurement landed state"
guard_expect_fixed_in_file "$TAG" 'provider_activation_lane=parked' "$CARD_16" "closeout must park activation"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_16" "closeout must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_16" "closeout must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_16" "closeout must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_16" "closeout must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001' "$CARD_16" "closeout must select 3-way comparison contract"
guard_expect_fixed_in_file "$TAG" 'hako_exact_exe' "$CARD_16" "closeout must name hako exact-EXE subject"
guard_expect_fixed_in_file "$TAG" 'c_mimalloc_explicit_runner' "$CARD_16" "closeout must name C mimalloc subject"
guard_expect_fixed_in_file "$TAG" 'provider_package_explicit_alloc_free' "$CARD_16" "closeout must name provider explicit subject"

guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose closeout row"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001' "$TASKBOARD" "taskboard must expose comparison contract row"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$TASKBOARD" "taskboard must define comparison subjects"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list closeout guard"

echo "[$TAG] ok"
