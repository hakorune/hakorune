#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-provider-package-explicit-comparison-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_40="docs/development/current/main/phases/phase-296x/296x-40-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT.md"
CARD_41="docs/development/current/main/phases/phase-296x/296x-41-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
ROADMAP="docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
PARITY_SSOT="docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_provider_package_explicit_comparison_closeout_guard.sh"

echo "[$TAG] checking phase-296x provider package explicit comparison closeout"

guard_require_files "$TAG" "$CARD_40" "$CARD_41" "$TASKBOARD" "$CURRENT_STATE" "$ROADMAP" "$PARITY_SSOT" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_40" "pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_41" "closeout card must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$CARD_41" "closeout card must identify blocker"
guard_expect_fixed_in_file "$TAG" 'output_contract=mimalloc-provider-explicit-comparison-adapter-v0' "$CARD_41" "closeout must preserve adapter contract"
guard_expect_fixed_in_file "$TAG" 'comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free' "$CARD_41" "closeout must preserve subjects"
guard_expect_fixed_in_file "$TAG" 'subject_count=3' "$CARD_41" "closeout must preserve subject count"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_41" "closeout must keep winner claims closed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$CARD_41" "closeout must select the closeout row"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001' "$CARD_41" "closeout must select the parity roadmap row"
guard_expect_fixed_in_file "$TAG" 'hakozuna reference-only' "$CARD_41" "closeout must keep hakozuna reference-only"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-41-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance to closeout"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select parity roadmap"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$TASKBOARD" "taskboard must expose comparison closeout row"
guard_expect_fixed_in_file "$TAG" '| 41 | `MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 41 must be landed"
guard_expect_fixed_in_file "$TAG" '| 42 | `HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 42 must be current"
guard_expect_fixed_in_file "$TAG" 'Hako Mimalloc Performance Parity' "$PARITY_SSOT" "parity SSOT must exist"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001' "$INDEX" "check index must list comparison closeout guard"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
